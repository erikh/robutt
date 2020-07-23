use irc::client::prelude::*;
use std::ops::Index;

pub fn load_config() -> Config {
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
            channels: vec!["#tinyci".to_owned()],
            ..Config::default()
        }
    }
}
