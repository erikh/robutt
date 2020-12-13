mod config;
mod dispatch;
mod loudfile;

use anyhow::Result;
use discord::model::Event::MessageCreate;
use discord::Discord;
use dispatch::{Dispatch, DispatchResult, DispatchSource};
use futures::prelude::*;
use irc::client::prelude::*;

fn load_config() -> Result<config::Config> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        config::Config::new(args.get(1).unwrap().to_owned())
    } else {
        Ok(config::Config::default())
    }
}

#[tokio::main]
async fn main() -> DispatchResult {
    let config = load_config()?;

    if let Ok(discord_token) = std::env::var("DISCORD_TOKEN") {
        tokio::spawn(discord_loop(discord_token));
    }

    tokio::join!(irc_loop(config)).0?;

    Ok(())
}

pub async fn discord_loop(discord_token: String) -> DispatchResult {
    println!("Entering discord loop");
    if let Ok(discord) = Discord::from_bot_token(&discord_token) {
        if let Ok((mut discord_client, ready)) = discord.connect() {
            let state = discord::State::new(ready);

            loop {
                if let Ok(MessageCreate(message)) = discord_client.recv_event() {
                    if message.author.name != state.user().username {
                        println!(
                            "(user: {:?}) <{}> {}",
                            state.user().id,
                            message.author.name,
                            message.content
                        );
                        let mut d = Dispatch::new(
                            state.user().id.0,
                            state.user().username.to_string(),
                            message.author.name,
                            message.channel_id.to_string(),
                            message.content.to_string(),
                            DispatchSource::Discord,
                        );

                        let mut r = d.dispatch().await?;

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

pub async fn irc_loop(config: config::Config) -> DispatchResult {
    println!("Entering irc loop");
    let my_nickname = config.config.nickname()?;
    let mut irc_client = Client::from_config(config.config.clone()).await?;

    irc_client.identify()?;
    let mut stream = irc_client.stream()?;

    loop {
        let message = stream.select_next_some().await?;
        match message.clone().command {
            Command::PRIVMSG(prefix, text) => {
                println!("<{}> {}", prefix, text.to_string());
                if text.len() > 0 && text.as_bytes()[0] == 0x01 {
                    // CTCP, we just don't give a f
                    continue;
                }

                if let Some(prefix) = message.source_nickname() {
                    let mut d = Dispatch::new(
                        0,
                        my_nickname.to_string(),
                        prefix.to_string(),
                        message.response_target().unwrap().to_string(),
                        text.to_string(),
                        DispatchSource::IRC,
                    );

                    let mut r = d.dispatch().await?;

                    while let Some(reply) = r.recv().await {
                        irc_client.send_privmsg(reply.get_target(), reply.get_text())?;
                    }
                }
            }
            _ => print!("{}", message),
        };
    }
}
