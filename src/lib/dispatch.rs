use tokio::sync::mpsc::{self, error::SendError};

#[derive(Clone, Debug)]
pub struct Dispatch {
    id: u64,
    nick: String,
    sender: String,
    target: String,
    text: String,
}

#[derive(Clone, Debug)]
pub struct DispatchReply {
    target: String,
    text: String,
}

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn new_from_error(e: &dyn std::error::Error) -> Self {
        Self {
            message: format!("{}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(t: std::io::Error) -> Self {
        Self {
            message: format!("{}", t),
        }
    }
}

impl From<SendError<DispatchReply>> for Error {
    fn from(t: SendError<DispatchReply>) -> Self {
        Self {
            message: format!("{}", t),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

pub type DispatchResult = Result<(), Error>;

async fn dispatcher(
    s: &str,
    dispatch: Dispatch,
    sender: &mut mpsc::Sender<DispatchReply>,
) -> DispatchResult {
    match s {
        "gamesdb" => targets::commands::gamesdb(dispatch, sender).await,
        "thoughts?" => targets::commands::thoughts(dispatch, sender).await,
        "help" => targets::commands::help(dispatch, sender).await,
        _ => targets::loud(dispatch, sender).await,
    }
}

pub fn is_loud(text: &String) -> bool {
    let chars_regex = regex::Regex::new("[A-Z ]{5}").unwrap();
    text.to_uppercase().eq(text) && chars_regex.is_match(text) && text.len() >= 5
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
    pub fn new(id: u64, nick: String, sender: String, target: String, text: String) -> Dispatch {
        Dispatch {
            id,
            nick,
            sender: sender.clone(),
            target,
            text,
        }
    }

    pub async fn dispatch(&self) -> (DispatchResult, mpsc::Receiver<DispatchReply>) {
        let prefix = format!("{}: ", &self.nick);
        let prefix_discord = format!("<@{}>", self.id);
        // kill me
        let prefix_discord2 = format!("<@!{}>", self.id);
        let text = if self.text.starts_with(prefix.as_str()) {
            self.text.trim_start_matches(prefix.as_str())
        } else if self.text.starts_with(prefix_discord.as_str()) {
            self.text.trim_start_matches(prefix_discord.as_str())
        } else if self.text.starts_with(prefix_discord2.as_str()) {
            self.text.trim_start_matches(prefix_discord2.as_str())
        } else {
            &self.text
        }
        .trim()
        .to_string();

        let (s, r) = mpsc::channel::<DispatchReply>(100);
        let res = if is_loud(&text) {
            let mut d = self.clone();
            d.text = text;
            let mut tmp_s = s.clone();
            targets::loud(d, &mut tmp_s).await
        } else if self.text != text {
            let mut parts = text.splitn(2, " ");
            let mut d = self.clone();

            if let Some(command) = parts.next() {
                match parts.next() {
                    Some(t) => {
                        d.text = t.to_string();
                    }
                    None => {}
                };

                dispatcher(command, d, &mut s.clone()).await
            } else {
                d.text = String::from("");
                targets::loud(d, &mut s.clone()).await
            }
        } else {
            Ok(())
        };

        drop(s);
        return (res, r);
    }
}

mod targets {
    use crate::lib::dispatch::is_loud;
    use crate::lib::dispatch::{Dispatch, DispatchReply, DispatchResult};
    use crate::lib::loudfile::LoudFile;
    use tokio::sync::mpsc;

    pub async fn loud(
        dispatch: Dispatch,
        sender: &mut mpsc::Sender<DispatchReply>,
    ) -> DispatchResult {
        let loudfile = LoudFile::new("loudfile.txt");

        if is_loud(&dispatch.text) && !dispatch.text.trim().is_empty() {
            println!("LOUD RECORDED: <{}> {}", dispatch.target, dispatch.text);
            loudfile.append(&dispatch.text).unwrap();
        }

        if let Some(line) = loudfile.get_line() {
            sender
                .send(DispatchReply {
                    target: dispatch.target,
                    text: line,
                })
                .await?;
        }

        Ok(())
    }

    pub mod commands {
        use crate::lib::dispatch::{Dispatch, DispatchReply, DispatchResult};
        use openapi::apis::{self, games_api};
        use rand::prelude::*;
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;
        use std::ops::Index;
        use tokio::sync::mpsc;

        async fn fetch(
            search: Vec<&str>,
            categories: Vec<&str>,
            pages: Vec<usize>, // using usize here so we can do some polymorphism around this later.
        ) -> Result<Vec<String>, apis::Error<games_api::GamesByGameNameError>> {
            let config = apis::configuration::Configuration::default();

            let mut joined_categories = String::from("youtube,overview");
            if categories.len() > 0 {
                joined_categories = categories.join(",");
            }

            if let Ok(api_key) = std::env::var("GAMESDB_API_KEY") {
                let res = games_api::games_by_game_name(
                    &config,
                    &api_key,
                    search.join(" ").as_str(),
                    Some(joined_categories.as_str()),
                    None,
                    None,
                    Some(1),
                )
                .await?;

                let mut ret: Vec<String> = Vec::new();
                let mut pgs: Vec<usize> = (1 as usize..res.data.count as usize).collect();
                if pages.len() > 0 {
                    pgs = pages;
                }

                for page in pgs {
                    let mut inner_ret: Vec<String> = Vec::new();
                    let obj = &res.data.games.index(page);

                    if let Some(title) = &obj.game_title {
                        inner_ret.push(format!("Title: {}", title))
                    }

                    if let Some(id) = &obj.id {
                        inner_ret.push(format!("URL: https://thegamesdb.net/game.php?id={}", id))
                    }

                    if let Some(youtube_url) = &obj.youtube {
                        if youtube_url != "" {
                            if youtube_url.starts_with("https://youtube.com")
                                || youtube_url.starts_with("https://youtu.be")
                            {
                                inner_ret.push(format!("Youtube: {}", youtube_url));
                            } else {
                                inner_ret.push(format!(
                                    "Youtube: https://youtube.com/watch?v={}",
                                    youtube_url
                                ));
                            }
                        }
                    }

                    if let Some(overview) = &obj.overview {
                        inner_ret.push(format!("Overview: {}", overview));
                    }

                    if inner_ret.len() > 0 {
                        ret.push(inner_ret.join(" / "));
                        break;
                    }
                }

                if ret.len() > 0 {
                    return Ok(ret);
                }
            }

            Ok(vec![String::from("No information found")])
        }

        pub async fn help(
            dispatch: Dispatch,
            send: &mut mpsc::Sender<DispatchReply>,
        ) -> DispatchResult {
            let help_vec = vec![
                "Try asking robutt what she thinks.",
                "Try 'gamesdb <title>. Use a +category to fetch a specific category of data that we recognize. Use -# to fetch a specific index of the entries.'",
                "Example: mega man +youtube -1 -2 -3 # first three items, youtube link",
            ];

            let mut help = help_vec.iter();
            let target = dispatch.target;
            let sender = dispatch.sender;

            while let Some(message) = help.next() {
                send.send(DispatchReply {
                    target: target.to_string(),
                    text: format!("{}: {}", sender, message),
                })
                .await?;
            }

            Ok(())
        }

        fn parse<'a>(rx: regex::Regex, lead: &str, text: &'a str) -> Vec<&'a str> {
            let caps = rx.captures_iter(text);
            return caps
                .map(|item| -> Vec<&str> {
                    item.iter()
                        .skip(1) // first match is always a dupe
                        .map(|i| i.unwrap().clone().as_str().trim_start_matches(lead))
                        .collect()
                })
                .flatten()
                .collect();
        }

        pub async fn gamesdb(
            dispatch: Dispatch,
            sender: &mut mpsc::Sender<DispatchReply>,
        ) -> DispatchResult {
            if dispatch.text == "" {
                sender
                    .send(DispatchReply {
                        target: dispatch.target,
                        text: String::from("Invalid query: try `help`"),
                    })
                    .await?;
            } else {
                // these are incredibly brittle.
                let categories_rx =
                    regex::Regex::new("(?:^|[^\\w]*)(\\+[a-z]+)(?:[^\\w]|$)").unwrap();
                let pages_rx = regex::Regex::new("(?:^|[^\\w]*)(-[0-9]+)(?:[^\\w]|$)").unwrap();
                let search_rx = regex::Regex::new("(?:^|\\s)([^-+][^\\s]+)").unwrap();

                let categories = parse(categories_rx, "+", &dispatch.text);
                println!("CATEGORIES: {:?}", categories);

                let pages = parse(pages_rx, "-", &dispatch.text);
                println!("PAGES: {:?}", pages);

                let int_pages = pages
                    .iter()
                    .map(|x| -> usize { x.parse::<usize>().unwrap_or_default() })
                    .collect();

                let search = parse(search_rx, "", &dispatch.text);
                println!("SEARCH {:?}", search);

                match fetch(search, categories, int_pages).await {
                    Ok(text) => {
                        if text.len() > 0 {
                            let mut iter = text.iter();
                            while let Some(t) = iter.next() {
                                if t.trim().len() != 0 {
                                    sender
                                        .send(DispatchReply {
                                            target: dispatch.target.to_string(),
                                            text: t.to_string(),
                                        })
                                        .await
                                        .unwrap();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        sender
                            .send(DispatchReply {
                                target: dispatch.target,
                                text: String::from("Error fetching data"),
                            })
                            .await?;
                    }
                }
            }
            Ok(())
        }

        // http://www.textfiles.com/humor/deep.txt
        const THOUGHTS_FILE: &str = "deep.txt";

        pub async fn thoughts(
            dispatch: Dispatch,
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
                    target: dispatch.target,
                    text: quote.to_string(),
                })
                .await?;
            Ok(())
        }
    }
}
