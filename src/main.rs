mod lib;

use discord::Discord;
use futures::*;
use irc::client::prelude::*;
use lib::config::load_config;
use lib::dispatch::{dispatcher, Dispatch, DispatchReply};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    let my_nickname = config.nickname()?;
    let mut irc_client = Client::from_config(config.clone()).await?;

    irc_client.identify()?;

    let mut stream = irc_client.stream()?;

    let discord = Discord::from_bot_token("")?;

    let res = tokio::join!(
        irc_loop(irc_client, &mut stream, my_nickname.to_string()),
        discord_loop(discord)
    );

    match res.0 {
        Ok(_) => {}
        Err(e) => err_exit(e),
    }

    match res.1 {
        Ok(_) => {}
        Err(e) => err_exit(e),
    }

    Ok(())
}

pub fn err_exit(e: Box<dyn std::error::Error>) {
    println!("{}", e);
    std::process::exit(1);
}

pub async fn discord_loop(discord: Discord) -> Result<(), Box<dyn std::error::Error>> {
    // if let Ok(mut discord_client) = discord.connect() {
    //     loop {
    //         match discord_client.0.recv_event()? {
    //             discord::model::Event::MessageCreate(message) => {
    //                 let dp = dispatcher();
    //                 println!("<{}> {}", message.author.name, message.content);
    //                 let d = Dispatch::new(
    //                     message.author.name,
    //                     message.message.channel_id.to_string(),
    //                     text.to_string(),
    //                 );
    //
    //                 match d.dispatch(dp).await {
    //                     Ok(_) => {}
    //                     Err(e) => println!("IRC ERROR: {}", e),
    //                 }
    //             }
    //         }
    //     }
    // }
    Ok(())
}

pub async fn irc_loop(
    irc_client: irc::client::Client,
    stream: &mut irc::client::ClientStream,
    my_nickname: String,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let dp = dispatcher();
        let (s, r) = std::sync::mpsc::channel::<DispatchReply>();
        match stream.next().await {
            Some(Ok(message)) => match message.clone().command {
                Command::PRIVMSG(prefix, text) => {
                    println!("<{}> {}", prefix, text.to_string());
                    if let Some(prefix) = message.source_nickname() {
                        let d = Dispatch::new(
                            s,
                            my_nickname.to_string(),
                            prefix.to_string(),
                            message.response_target().unwrap().to_string(),
                            text.to_string(),
                        );

                        match d.dispatch(dp).await {
                            Ok(_) => {}
                            Err(e) => println!("IRC ERROR: {}", e),
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

        for reply in r.try_iter() {
            irc_client.send_privmsg(reply.get_target(), reply.get_text())?;
        }
    }
}
