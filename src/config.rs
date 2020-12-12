use anyhow::Result;

pub struct Config {
    pub config: irc::client::prelude::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config: irc::client::prelude::Config {
                nickname: Some("robutt-dev".to_owned()),
                server: Some("irc.freenode.net".to_owned()),
                channels: vec!["#tinyci".to_owned()],
                ping_timeout: Some(180),
                ..irc::client::prelude::Config::default()
            },
        }
    }
}

impl Config {
    pub fn new(filename: String) -> Result<Self> {
        Ok(Self {
            config: irc::client::prelude::Config::load(filename)?,
        })
    }
}
