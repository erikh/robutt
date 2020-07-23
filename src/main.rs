mod lib;

use futures::*;
use irc::client::prelude::*;
use lib::config::load_config;
use lib::dispatch::{default_dispatcher, Dispatch};

#[tokio::main]
pub async fn main() -> Result<(), irc::error::Error> {
    let config = load_config();
    let my_nickname = config.nickname()?;
    let mut client = Client::from_config(config.clone()).await?;

    client.identify()?;

    let mut stream = client.stream()?;

    loop {
        let dispatcher = default_dispatcher();
        match stream.next().await {
            Some(Ok(message)) => match message.clone().command {
                Command::PRIVMSG(prefix, text) => {
                    println!("<{}> {}", prefix, text.to_string());
                    if let Some(prefix) = message.source_nickname() {
                        let d = Dispatch {
                            client: client.sender(),
                            nick: my_nickname.to_string(),
                            sender: prefix.to_string(),
                            target: message.response_target().unwrap().to_string(),
                            text: text.to_string(),
                        };

                        match d.dispatch(&dispatcher).await {
                            Ok(_) => println!("{}", message),
                            Err(e) => println!("IRC ERROR: {}", e),
                        }
                    }
                }
                command => println!("{:?}", command),
            },
            Some(Err(e)) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
            None => {}
        }
    }
}
