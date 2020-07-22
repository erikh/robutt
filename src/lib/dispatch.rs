use irc::client::prelude::*;
use std::collections::HashMap;

pub fn default_dispatch() -> Dispatcher {
    let mut d = Dispatcher::new();
    d.insert("gamesdb".to_string(), targets::commands::gamesdb);
    d
}

pub type DispatchError = Result<(), irc::error::IrcError>;

pub type Dispatcher = HashMap<
    String,
    fn(
        client: &irc::client::IrcClient,
        sender: String,
        target: String,
        text: String,
    ) -> Result<(), irc::error::IrcError>,
>;

pub fn dispatch(
    client: &irc::client::IrcClient,
    sender: String,
    target: String,
    text: String,
    dispatch: Dispatcher,
) -> Result<(), irc::error::IrcError> {
    if text.to_uppercase() == text {
        return targets::loud(client, target, text);
    };

    if text.trim().starts_with(client.config().nickname()?) {
        return targets::addressed(client, sender, target, text, dispatch);
    };

    Ok(())
}

mod targets {
    use crate::lib::dispatch::Dispatcher;
    use crate::lib::loudfile::LoudFile;
    use irc::client::prelude::*;

    pub fn addressed(
        client: &irc::client::IrcClient,
        sender: String,
        target: String,
        text: String,
        dispatch: Dispatcher,
    ) -> Result<(), irc::error::IrcError> {
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

    pub fn loud(
        client: &irc::client::IrcClient,
        target: String,
        text: String,
    ) -> Result<(), irc::error::IrcError> {
        let loudfile = LoudFile::new("loudfile.txt");

        println!("LOUD: <{}> {}", target, text);

        loudfile.append(&text).unwrap();

        if let Some(line) = loudfile.get_line() {
            client.send_privmsg(target, line)?;
        }

        Ok(())
    }

    pub mod commands {
        use crate::lib::dispatch::DispatchError;
        use irc::client::prelude::*;

        pub fn gamesdb(
            client: &irc::client::IrcClient,
            sender: String,
            target: String,
            text: String,
        ) -> DispatchError {
            if text == "" {
                client.send_privmsg(target, format!("{}: Try 'gamesdb <title>'", sender))?;
            } else {
                println!("Title: '{}'", text);
            }

            Ok(())
        }
    }
}
