use anyhow::Result;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum DispatchSource {
    IRC,
    Discord,
}

#[derive(Clone, Debug)]
pub struct Dispatch {
    id: u64,
    nick: String,
    sender: String,
    target: String,
    text: String,
    source: DispatchSource,
}

#[derive(Clone, Debug)]
pub struct DispatchReply {
    target: String,
    text: String,
}

pub type DispatchResult = Result<()>;

async fn dispatcher(
    s: &str,
    dispatch: &mut Dispatch,
    sender: &mut mpsc::Sender<DispatchReply>,
) -> DispatchResult {
    match s {
        "thoughts?" => targets::commands::thoughts(dispatch, sender).await,
        "roll" => targets::commands::roll(dispatch, sender).await,
        "help" => targets::commands::help(dispatch, sender).await,
        _ => targets::loud(dispatch, sender).await,
    }
}

impl DispatchReply {
    pub fn get_target(&self) -> String {
        return self.target.to_string();
    }

    pub fn get_text(&self) -> String {
        return self.text.to_string();
    }
}

impl Dispatch {
    pub fn new(
        id: u64,
        nick: String,
        sender: String,
        target: String,
        text: String,
        source: DispatchSource,
    ) -> Dispatch {
        Dispatch {
            id,
            nick,
            sender: sender.clone(),
            target,
            text,
            source,
        }
    }

    pub fn is_loud(&self, text: &String) -> bool {
        let chars_regex = regex::Regex::new("[A-Z ]{5}").unwrap();
        text.to_uppercase().eq(text) && chars_regex.is_match(text) && text.len() >= 5
    }

    pub async fn dispatch(&mut self) -> Result<mpsc::Receiver<DispatchReply>> {
        let text = match self.source {
            DispatchSource::IRC => {
                let prefix = format!("{}: ", &self.nick);
                if self.text.starts_with(prefix.as_str()) {
                    self.text.trim_start_matches(prefix.as_str())
                } else {
                    &self.text
                }
            }
            DispatchSource::Discord => {
                let prefix_discord = format!("<@{}>", self.id);
                // kill me
                let prefix_discord2 = format!("<@!{}>", self.id);
                if self.text.starts_with(prefix_discord.as_str()) {
                    self.text.trim_start_matches(prefix_discord.as_str())
                } else if self.text.starts_with(prefix_discord2.as_str()) {
                    self.text.trim_start_matches(prefix_discord2.as_str())
                } else {
                    &self.text
                }
            }
        }
        .trim()
        .to_string();

        let (mut s, r) = mpsc::channel::<DispatchReply>(100);
        if self.is_loud(&text) {
            self.text = text;
            targets::loud(self, &mut s).await?;
        } else if self.text.trim() != text {
            let mut parts = text.splitn(2, " ");

            if let Some(command) = parts.next() {
                match parts.next() {
                    Some(t) => {
                        self.text = t.to_string();
                    }
                    None => {}
                };

                dispatcher(command, self, &mut s).await?;
            } else {
                self.text = String::new();
                targets::loud(self, &mut s).await?;
            }
        }

        drop(s);
        return Ok(r);
    }
}

mod targets {
    use crate::dispatch::{Dispatch, DispatchReply, DispatchResult};
    use crate::loudfile::LoudFile;
    use tokio::sync::mpsc;

    pub async fn loud(
        dispatch: &mut Dispatch,
        sender: &mut mpsc::Sender<DispatchReply>,
    ) -> DispatchResult {
        let loudfile = LoudFile::new(String::from("loudfile.txt"));

        if dispatch.is_loud(&dispatch.text) && !dispatch.text.trim().is_empty() {
            println!("LOUD RECORDED: <{}> {}", dispatch.target, dispatch.text);
            loudfile.append(dispatch.text.clone()).unwrap();
        }

        if let Some(line) = loudfile.get_line() {
            sender
                .send(DispatchReply {
                    target: dispatch.target.clone(),
                    text: line,
                })
                .await?;
        }

        Ok(())
    }

    pub mod commands {
        use crate::dispatch::{Dispatch, DispatchReply, DispatchResult};
        use rand::prelude::*;
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;
        use tokio::sync::mpsc;

        pub async fn help(
            dispatch: &mut Dispatch,
            send: &mut mpsc::Sender<DispatchReply>,
        ) -> DispatchResult {
            let help_vec = vec![
                "The bot only responds to being addressed directly, e.g., `robutt: roll 1d4`",
                "otherwise, BE LOUD!",
                "Try asking robutt what she thinks.",
                "`roll 2d6` for hot die on die action",
            ];

            let mut help = help_vec.iter();
            let target = dispatch.target.clone();
            let sender = dispatch.sender.clone();

            while let Some(message) = help.next() {
                send.send(DispatchReply {
                    target: target.clone(),
                    text: format!("{}: {}", sender, message),
                })
                .await?;
            }

            Ok(())
        }

        pub async fn short_help(
            dispatch: &mut Dispatch,
            sender: &mut mpsc::Sender<DispatchReply>,
        ) -> DispatchResult {
            sender
                .send(DispatchReply {
                    target: dispatch.target.clone(),
                    text: String::from("Invalid query: try `help`"),
                })
                .await?;
            Ok(())
        }

        pub fn convert_capture(captures: &regex::Captures, index: usize, default: u8) -> u8 {
            match captures.get(index) {
                Some(x) => x.as_str().parse::<u8>().unwrap_or(default),
                None => default,
            }
        }

        pub async fn roll(
            dispatch: &mut Dispatch,
            sender: &mut mpsc::Sender<DispatchReply>,
        ) -> DispatchResult {
            if dispatch.text == "" {
                return short_help(dispatch, sender).await;
            }

            let dice_rx =
                regex::Regex::new(r"\s*(([1-9][0-9]*)d)?([1-9][0-9]*)(\+([1-9][0-9]*))?").unwrap();
            let captures = match dice_rx.captures(&dispatch.text) {
                Some(c) => c,
                None => {
                    return short_help(dispatch, sender).await;
                }
            };

            let num_dice = convert_capture(&captures, 2, 1);
            let die_size = convert_capture(&captures, 3, 10);
            let offset = convert_capture(&captures, 5, 0);

            let mut dice: Vec<u8> = Vec::new();
            let mut sum: u128 = 0;

            for _x in 0..num_dice {
                let mut result: u8 = rand::random();
                result = (result % die_size) + 1 + offset; // dice start at 1
                dice.push(result);
                sum += result as u128;
            }

            let outs = vec![format!("dice: {:?}", dice), format!("sum: {}", sum)];

            for out in outs {
                sender
                    .send(DispatchReply {
                        target: dispatch.target.clone(),
                        text: out,
                    })
                    .await?;
            }

            Ok(())
        }

        // http://www.textfiles.com/humor/deep.txt
        const THOUGHTS_FILE: &str = "deep.txt";

        pub async fn thoughts(
            dispatch: &mut Dispatch,
            sender: &mut mpsc::Sender<DispatchReply>,
        ) -> DispatchResult {
            let file = File::open(THOUGHTS_FILE).unwrap();
            let br = BufReader::new(file);
            let mut lines = br.lines();
            let mut quotes: Vec<String> = Vec::new();
            let mut tmp = String::from("");

            while let Some(Ok(line)) = lines.next() {
                if line.trim().is_empty() {
                    quotes.push(tmp.to_string());
                    tmp = String::from("");
                }

                if tmp != "" {
                    tmp += " "
                }

                tmp += &line;
                tmp = tmp.trim().to_string();
            }

            let quote = &quotes[random::<usize>() % quotes.len()];
            sender
                .send(DispatchReply {
                    target: dispatch.target.clone(),
                    text: quote.to_string(),
                })
                .await?;
            Ok(())
        }
    }
}
