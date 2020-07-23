mod lib;

use irc::client::prelude::*;
use lib::config::load_config;
use lib::dispatch::{default_dispatcher, dispatch};
use tokio::prelude::*;
use tokio::stream::StreamExt;

#[tokio::main]
pub async fn main() -> Result<(), irc::error::Error> {
    let config = load_config();
    let my_nickname = config.nickname()?;
    let mut client = Client::from_config(config.clone()).await?;
    let mut stream = client.stream()?;

    client.identify()?;

    loop {
        match stream.next().await {
            Some(Ok(message)) => match &message.command {
                Command::PRIVMSG(_, text) => {
                    if let Some(prefix) = message.source_nickname() {
                        match dispatch(
                            &client,
                            my_nickname.to_string(),
                            prefix.to_string(),
                            message.clone().response_target().unwrap().to_string(),
                            text.to_string(),
                            default_dispatcher(),
                        ) {
                            Ok(_) => println!("{}", message),
                            Err(e) => println!("IRC ERROR: {}", e),
                        }
                    }
                }
                _ => println!("{:?}", message.command),
            },
            Some(Err(e)) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
            None => tokio::time::delay_for(std::time::Duration::new(1, 0)).await,
        }
    }
}
