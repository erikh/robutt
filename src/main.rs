mod loudfile;

use irc::client::prelude::*;
use loudfile::LoudFile;
use std::ops::Index;

fn load_config() -> Config {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        match Config::load(args.index(1)) {
            Ok(config) => config,
            Err(e) => {
                println!("Error loading config: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        Config {
            nickname: Some("robutt-dev".to_owned()),
            server: Some("irc.freenode.net".to_owned()),
            channels: Some(vec!["#tinyci".to_owned()]),
            ..Config::default()
        }
    }
}

fn main() {
    let mut reactor = IrcReactor::new().unwrap();
    let config = load_config();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.register_client_with_handler(client, |client, message| {
        match message.clone().command {
            Command::PRIVMSG(channel, text) => {
                let target = message.response_target().unwrap();
                if text.to_uppercase() == text {
                    let loudfile = LoudFile::new("loudfile.txt");

                    println!("LOUD: {} <{}> {}", channel, target, text);

                    loudfile.append(&text).unwrap();

                    if let Some(line) = loudfile.get_line() {
                        client.send_privmsg(target, line)?;
                    }
                }
            }
            Command::PONG(_, _) => (),
            _ => print!("{}", message),
        }

        // And here we can do whatever we want with the messages.
        Ok(())
    });

    reactor.run().unwrap();
}
