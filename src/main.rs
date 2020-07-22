mod lib;

use irc::client::prelude::*;
use lib::config::load_config;
use lib::dispatch::{default_dispatcher, dispatch};

fn main() {
    let mut reactor = IrcReactor::new().unwrap();
    let config = load_config();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.register_client_with_handler(client, |client, message| {
        match message.clone().command {
            Command::PRIVMSG(_, text) => {
                if let Some(prefix) = message.source_nickname() {
                    match dispatch(
                        client,
                        prefix.to_string(),
                        message.clone().response_target().unwrap().to_string(),
                        text,
                        default_dispatcher(),
                    ) {
                        Ok(_) => (),
                        Err(e) => println!("IRC ERROR: {}", e),
                    }
                }
            }
            Command::PING(_, _) => (),
            Command::PONG(_, _) => (),
            _ => print!("{}", message),
        }

        Ok(())
    });

    reactor.run().unwrap();
}
