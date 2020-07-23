use std::collections::HashMap;

pub fn default_dispatcher() -> Dispatcher {
    let mut d = Dispatcher::new();
    d.insert("gamesdb".to_string(), targets::commands::gamesdb);
    d
}

pub type DispatchError = Result<(), irc::error::Error>;

pub type Dispatcher = HashMap<
    String,
    fn(client: &irc::client::Client, sender: String, target: String, text: String) -> DispatchError,
>;

pub fn dispatch<'a>(
    client: &irc::client::Client,
    nick: String,
    sender: String,
    target: String,
    text: String,
    dispatch: Dispatcher,
) -> DispatchError {
    if text.to_uppercase() == text {
        return targets::loud(client, target, text);
    };

    if text.trim().starts_with(&nick) {
        return targets::addressed(client, sender, target, text, dispatch);
    };

    Ok(())
}

mod targets {
    use crate::lib::dispatch::{DispatchError, Dispatcher};
    use crate::lib::loudfile::LoudFile;

    pub fn addressed(
        client: &irc::client::Client,
        sender: String,
        target: String,
        text: String,
        dispatch: Dispatcher,
    ) -> DispatchError {
        let res = text.splitn(2, " ");

        match res.last() {
            Some(inner) => {
                let mut keys = dispatch.keys();
                while let Some(key) = keys.next() {
                    if inner.trim().starts_with(key) {
                        let new_text = inner.trim_start_matches(key).trim();
                        if let Some(f) = dispatch.get(key) {
                            f(client, sender.clone(), target.clone(), new_text.to_string())?;
                        }
                    }
                }
            }
            None => (),
        };

        return Ok(());
    }

    pub fn loud(client: &irc::client::Client, target: String, text: String) -> DispatchError {
        let loudfile = LoudFile::new("loudfile.txt");

        println!("LOUD: <{}> {}", target, text);

        loudfile.append(&text).unwrap();

        if let Some(line) = loudfile.get_line() {
            return match client.send_privmsg(target, line) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
        }

        Ok(())
    }

    pub mod commands {
        use crate::lib::dispatch::DispatchError;
        use futures::executor;
        use openapi::apis::{self, games_api};

        fn fetch(text: String) -> Result<String, apis::Error<games_api::GamesByGameNameError>> {
            let config = Default::default();
            if let Ok(api_key) = std::env::var("API_KEY") {
                let res = executor::block_on(games_api::games_by_game_name(
                    &config,
                    &api_key,
                    &text,
                    None,
                    None,
                    None,
                    Some(0),
                ));

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

        pub fn gamesdb(
            client: &irc::client::Client,
            sender: String,
            target: String,
            text: String,
        ) -> DispatchError {
            if text == "" {
                match client.send_privmsg(target, format!("{}: Try 'gamesdb <title>'", sender)) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(irc::error::Error::from(e)),
                }
            } else {
                match fetch(text) {
                    Ok(url) => client.send_privmsg(target, format!("Youtube: {}", url)),
                    Err(apis::Error::Io(e)) => Err(irc::error::Error::from(e)),
                    Err(e) => {
                        println!("Error: {:?}", e);
                        client.send_privmsg(target, "Error fetching data")
                    }
                }
            }
        }
    }
}
