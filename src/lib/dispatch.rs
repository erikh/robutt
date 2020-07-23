use futures::*;
use std::collections::HashMap;
use std::pin::Pin;

#[derive(Clone)]
pub struct Dispatch<'a> {
    client: irc::client::Sender,
    nick: String,
    sender: String,
    target: String,
    text: String,
}

pub fn default_dispatcher<'a>() -> Dispatcher<'a> {
    let mut d = Dispatcher::new();
    d.insert("gamesdb".to_string(), Pin::new(&targets::commands::gamesdb));
    d
}

pub type DispatchResult<'a> = Result<(), irc::error::Error>;
pub type DispatchFuture<'a> = dyn future::Future<Output = DispatchResult<'a>>;
pub type DispatchFunc<'a> = &'a for<'b> fn(Dispatch<'b>) -> DispatchFuture<'a>;
pub type DispatchPinBox<'a> = Pin<DispatchFunc<'a>>;
pub type Dispatcher<'a> = HashMap<String, DispatchPinBox<'a>>;

impl Dispatch<'static> {
    pub async fn dispatch(&self, dispatcher: &Dispatcher<'static>) -> DispatchResult<'static> {
        if self.text.to_uppercase() == self.text {
            return targets::loud(self.clone()).await;
        };

        if self.text.trim().starts_with(&self.nick) {
            return targets::addressed(self.clone(), *dispatcher).await;
        };

        Ok(())
    }
}

mod targets {
    use crate::lib::dispatch::{Dispatch, DispatchResult, Dispatcher};
    use crate::lib::loudfile::LoudFile;
    use futures::*;
    use std::pin::Pin;

    pub async fn addressed<'a>(
        dispatch: Dispatch<'a>,
        dispatcher: Dispatcher<'static>,
    ) -> DispatchResult<'static> {
        let res = dispatch.text.splitn(2, " ");

        match res.last() {
            Some(inner) => {
                let mut keys = dispatcher.keys();
                while let Some(key) = keys.next() {
                    if inner.trim().starts_with(key) {
                        let new_text = inner.trim_start_matches(key).trim();
                        if let Some(f) = dispatcher.get(key) {
                            let mut d2 = dispatch.clone();
                            d2.text = new_text.to_string();
                            let f2 = Pin::into_inner(*f);
                            f2(d2).await?;
                        }
                    }
                }
            }
            None => (),
        };

        return Ok(());
    }

    pub async fn loud(dispatch: Dispatch<'_>) -> DispatchResult<'static> {
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
            let config = Default::default();
            if let Ok(api_key) = std::env::var("API_KEY") {
                let res = games_api::games_by_game_name(
                    &config,
                    &api_key,
                    &text,
                    None,
                    None,
                    None,
                    Some(0),
                )
                .await;

                match res {
                    Ok(games) => {
                        if let Some(youtube_url) = &games.data.games.first().unwrap().youtube {
                            return Ok(youtube_url.to_string());
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

            Ok(String::from("No youtube url found"))
        }

        pub async fn gamesdb(dispatch: Dispatch<'_>) -> DispatchResult<'static> {
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
                    Ok(url) => dispatch
                        .client
                        .send_privmsg(dispatch.target, format!("Youtube: {}", url)),
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
