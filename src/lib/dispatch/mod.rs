use crate::lib::loudfile::LoudFile;
use irc::client::prelude::*;

fn loud(
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

pub fn dispatch(
    client: &irc::client::IrcClient,
    target: String,
    text: String,
) -> Result<(), irc::error::IrcError> {
    if text.to_uppercase() == text {
        return loud(client, target, text);
    }

    Ok(())
}
