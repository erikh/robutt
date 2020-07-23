use futures::*;
use std::pin::Pin;

#[derive(Clone)]
pub struct Dispatch {
    client: irc::client::Sender,
    nick: String,
    sender: String,
    target: String,
    text: String,
}

pub fn dispatcher() -> Dispatcher<'static> {
    return |s: &str, dispatch: Dispatch| -> Option<DispatchPinBox<'_>> {
        match s {
            "gamesdb" => Some(Box::pin(targets::commands::gamesdb(dispatch))),
            _ => None,
        }
    };
}

pub type DispatchResult<'a> = Result<(), irc::error::Error>;
pub type Dispatcher<'a> = fn(s: &str, dispatch: Dispatch) -> Option<DispatchPinBox<'a>>;
pub type DispatchPinBox<'a> = Pin<Box<dyn future::Future<Output = DispatchResult<'a>>>>;

impl Dispatch {
    pub fn new(
        client: irc::client::Sender,
        nick: String,
        sender: String,
        target: String,
        text: String,
    ) -> Dispatch {
        Dispatch {
            client,
            nick,
            sender,
            target,
            text,
        }
    }

    pub async fn dispatch(&self, dispatcher: Dispatcher<'static>) -> DispatchResult<'static> {
        if self.text.to_uppercase() == self.text {
            return targets::loud(self.clone()).await;
        };

        if self.text.trim().starts_with(&self.nick) {
            let prefix = format!("{}: ", &self.nick);
            let text = self.text.trim_start_matches(prefix.as_str());
            let mut parts = text.splitn(2, " ");
            let mut d = self.clone();
            if let Some(command) = parts.next() {
                if let Some(t) = parts.next() {
                    d.text = t.to_string();
                    if let Some(cb) = dispatcher(command, d) {
                        return cb.await;
                    }
                }
            }
        };

        Ok(())
    }
}

mod targets {
    use crate::lib::dispatch::{Dispatch, DispatchResult};
    use crate::lib::loudfile::LoudFile;

    pub async fn loud(dispatch: Dispatch) -> DispatchResult<'static> {
        let loudfile = LoudFile::new("loudfile.txt");

        println!("LOUD: <{}> {}", dispatch.target, dispatch.text);

        loudfile.append(&dispatch.text).unwrap();

        if let Some(line) = loudfile.get_line() {
            return match dispatch.client.send_privmsg(dispatch.target, line) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
        }

        Ok(())
    }

    pub mod commands {
        use crate::lib::dispatch::{Dispatch, DispatchResult};
        use openapi::apis::{self, games_api};

        async fn fetch(
            text: String,
        ) -> Result<String, apis::Error<games_api::GamesByGameNameError>> {
            let config = apis::configuration::Configuration::default();
            if let Ok(api_key) = std::env::var("API_KEY") {
                let res = games_api::games_by_game_name(
                    &config,
                    &api_key,
                    &text,
                    Some("youtube,overview"),
                    Some(""),
                    Some(""),
                    Some(0),
                )
                .await?;

                let mut ret: Vec<String> = Vec::new();

                if let Some(obj) = &res.data.games.first() {
                    if let Some(title) = &obj.game_title {
                        ret.push(format!("Title: {}", title))
                    }

                    if let Some(youtube_url) = &obj.youtube {
                        if youtube_url != "" {
                            if youtube_url.starts_with("https://youtube.com") {
                                ret.push(format!("Youtube: {}", youtube_url));
                            } else {
                                ret.push(format!(
                                    "Youtube: https://youtube.com/watch?v={}",
                                    youtube_url
                                ));
                            }
                        }
                    }

                    if let Some(overview) = &obj.overview {
                        ret.push(format!("Overview: {}", overview));
                    }

                    return Ok(ret.join(" / "));
                }
            }

            Ok(String::from("No information found"))
        }

        pub async fn gamesdb(dispatch: Dispatch) -> DispatchResult<'static> {
            if dispatch.text == "" {
                match dispatch.client.send_privmsg(
                    dispatch.target,
                    format!("{}: Try 'gamesdb <title>'", dispatch.sender),
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(irc::error::Error::from(e)),
                }
            } else {
                match fetch(dispatch.text).await {
                    Ok(text) => dispatch.client.send_privmsg(dispatch.target, text),
                    Err(apis::Error::Io(e)) => Err(irc::error::Error::from(e)),
                    Err(e) => {
                        println!("Error: {:?}", e);
                        dispatch
                            .client
                            .send_privmsg(dispatch.target, "Error fetching data")
                    }
                }
            }
        }
    }
}
