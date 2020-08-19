mod lib;

use discord::model::Event::MessageCreate;
use discord::Discord;
use futures::*;
use irc::client::prelude::*;
use lib::config::load_config;
use lib::dispatch::Dispatch;
use tokio::runtime::Builder;
use tokio::task;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    let mut r = Builder::new().threaded_scheduler().enable_all().build()?;

    if let Ok(discord_token) = std::env::var("DISCORD_TOKEN") {
        r.spawn(irc_loop(config));
        r.block_on(discord_loop(discord_token));
    } else {
        r.block_on(irc_loop(config)).unwrap();
    }

    Ok(())
}

pub fn err_exit(e: Box<dyn std::error::Error>) {
    println!("{}", e);
    std::process::exit(1);
}

pub async fn discord_loop(discord_token: String) -> Result<(), ()> {
    println!("Entering discord loop");
    task::yield_now().await;
    if let Ok(discord) = Discord::from_bot_token(&discord_token) {
        if let Ok((mut discord_client, ready)) = discord.connect() {
            let state = discord::State::new(ready);

            loop {
                if let Ok(MessageCreate(message)) = discord_client.recv_event() {
                    if message.author.name != state.user().username {
                        println!("<{}> {}", message.author.name, message.content);
                        let d = Dispatch::new(
                            state.user().username.to_string(),
                            message.author.name,
                            message.channel_id.to_string(),
                            message.content.to_string(),
                        );

                        let (dispatch, mut r) = d.dispatch().await;
                        match dispatch {
                            Ok(_) => {}
                            Err(e) => println!("DISCORD ERROR: {:?}", e),
                        }

                        while let Some(reply) = r.recv().await {
                            discord
                                .send_message(message.channel_id, &reply.get_text(), "", false)
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn irc_loop(config: Config) -> Result<(), ()> {
    println!("Entering irc loop");
    if let Ok(my_nickname) = config.nickname() {
        if let Ok(mut irc_client) = Client::from_config(config.clone()).await {
            match irc_client.identify() {
                Ok(_) => {}
                Err(_) => {
                    println!("Couldn't login");
                    std::process::exit(1);
                }
            }

            if let Ok(mut stream) = irc_client.stream() {
                loop {
                    match stream.next().await {
                        Some(Ok(message)) => match message.clone().command {
                            Command::PRIVMSG(prefix, text) => {
                                println!("<{}> {}", prefix, text.to_string());
                                if let Some(prefix) = message.source_nickname() {
                                    let d = Dispatch::new(
                                        my_nickname.to_string(),
                                        prefix.to_string(),
                                        message.response_target().unwrap().to_string(),
                                        text.to_string(),
                                    );

                                    let (dispatch, mut r) = d.dispatch().await;
                                    match dispatch {
                                        Ok(_) => {}
                                        Err(e) => println!("IRC ERROR: {:?}", e),
                                    }

                                    while let Some(reply) = r.recv().await {
                                        irc_client
                                            .send_privmsg(reply.get_target(), reply.get_text())
                                            .unwrap();
                                    }
                                }
                            }
                            _ => print!("{}", message),
                        },
                        Some(Err(e)) => {
                            println!("Error: {}", e);
                            std::process::exit(1);
                        }
                        None => {}
                    }
                }
            }
        }
    }
    Ok(())
}
