mod dispatch;
mod loudfile;

use anyhow::Result;
use dispatch::{Dispatch, DispatchResult, DispatchSource};
use futures::prelude::*;
use irc::client::prelude::*;

pub fn default_config() -> Config {
    Config {
        nickname: Some("robutt-dev".to_string()),
        server: Some("irc.hugops.org".to_string()),
        channels: vec!["#bots".to_string()],
        ping_timeout: Some(180),
        ..Default::default()
    }
}

fn load_config() -> Result<Config> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        Ok(Config::load(args.get(1).unwrap().to_owned())?)
    } else {
        Ok(default_config())
    }
}

#[tokio::main]
async fn main() -> DispatchResult {
    Ok(irc_loop(load_config()?).await?)
}

pub async fn irc_loop(config: Config) -> DispatchResult {
    println!("Entering irc loop");
    let my_nickname = config.nickname()?;

    let mut irc_client = Client::from_config(config.clone()).await?;

    irc_client.send_sasl_plain()?;
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
                        my_nickname.to_string(),
                        prefix.to_string(),
                        message.response_target().unwrap().to_string(),
                        text.to_string(),
                        DispatchSource::IRC,
                    );

                    let mut r = d.dispatch().await?;

                    while let Some(reply) = r.recv().await {
                        irc_client.send_privmsg(reply.target, reply.text)?;
                    }
                }
            }
            _ => print!("{}", message),
        };
    }
}
