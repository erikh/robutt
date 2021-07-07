use anyhow::Result;

pub struct Config {
    pub config: irc::client::prelude::Config,
}

impl Default for Config {
    fn default() -> Self {
        let config = irc::client::prelude::Config {
            nickname: Some("robutt-dev".to_owned()),
            server: Some("irc.hugops.org".to_owned()),
            channels: vec!["#bots".to_owned()],
            ping_timeout: Some(180),
            ..irc::client::prelude::Config::default()
        };

        Self { config }
    }
}

impl Config {
    pub fn new(filename: String) -> Result<Self> {
        Ok(Self {
            config: irc::client::prelude::Config::load(filename)?,
        })
    }
}
