use futures::*;
use std::pin::Pin;
use std::sync::mpsc::Sender;

#[derive(Clone, Debug)]
pub struct Dispatch {
    client: Sender<DispatchReply>,
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

pub fn dispatcher() -> Dispatcher<'static> {
    return |s: &str, dispatch: Dispatch| -> Option<DispatchPinBox<'_>> {
        match s {
            "gamesdb" => Some(Box::pin(targets::commands::gamesdb(dispatch))),
            "help" => Some(Box::pin(targets::commands::help(dispatch))),
            _ => None,
        }
    };
}

pub type DispatchResult<'a> = Result<(), Box<dyn std::error::Error>>;
pub type Dispatcher<'a> = fn(s: &str, dispatch: Dispatch) -> Option<DispatchPinBox<'a>>;
pub type DispatchPinBox<'a> = Pin<Box<dyn future::Future<Output = DispatchResult<'a>>>>;

fn is_loud(text: &String) -> bool {
    let chars_regex = regex::Regex::new("[a-zA-Z ]{5}").unwrap();
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
    pub fn new(
        client: Sender<DispatchReply>,
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
        let prefix = format!("{}: ", &self.nick);
        let text = self.text.trim_start_matches(prefix.as_str()).to_string();

        if is_loud(&text) {
            let mut d = self.clone();
            d.text = text;
            return targets::loud(d).await;
        } else if self.text != text {
            let mut parts = text.splitn(2, " ");

            if let Some(command) = parts.next() {
                let mut d = self.clone();

                d.text = match parts.next() {
                    Some(t) => t.to_string(),
                    None => String::from(""),
                };

                if let Some(cb) = dispatcher(command, d) {
                    return cb.await;
                }
            }

            let mut d = self.clone();
            d.text = String::from("");
            return targets::loud(d).await;
        }

        Ok(())
    }
}

mod targets {
    use crate::lib::dispatch::{Dispatch, DispatchReply, DispatchResult};
    use crate::lib::loudfile::LoudFile;

    pub async fn loud(dispatch: Dispatch) -> DispatchResult<'static> {
        let loudfile = LoudFile::new("loudfile.txt");

        if dispatch.text.len() > 0 {
            println!("LOUD RECORDED: <{}> {}", dispatch.target, dispatch.text);
            loudfile.append(&dispatch.text).unwrap();
        }

        if let Some(line) = loudfile.get_line() {
            return match dispatch.client.send(DispatchReply {
                target: dispatch.target,
                text: line,
            }) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            };
        }

        Ok(())
    }

    pub mod commands {
        use crate::lib::dispatch::{Dispatch, DispatchReply, DispatchResult};
        use openapi::apis::{self, games_api};
        use std::ops::Index;

        async fn fetch(
            search: Vec<&str>,
            categories: Vec<&str>,
            pages: Vec<&str>,
        ) -> Result<Vec<String>, apis::Error<games_api::GamesByGameNameError>> {
            let config = apis::configuration::Configuration::default();
            let mut joined_categories = categories.join(",");
            if joined_categories == "" {
                joined_categories = String::from("youtube,overview");
            }

            if let Ok(api_key) = std::env::var("API_KEY") {
                let res = games_api::games_by_game_name(
                    &config,
                    &api_key,
                    search.join(" ").as_str(),
                    Some(joined_categories.as_str()),
                    Some(""),
                    Some(""),
                    Some(0),
                )
                .await?;

                let mut ret: Vec<String> = Vec::new();
                let mut page_iter = pages.iter();

                while let Some(item) = page_iter.next() {
                    let mut inner_ret: Vec<String> = Vec::new();

                    let page = item.parse::<usize>().unwrap();

                    if res.data.count <= page as i32 {
                        continue;
                    }

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

                    ret.push(inner_ret.join(" / "));
                }

                return Ok(ret);
            }

            Ok(vec![String::from("No information found")])
        }

        pub async fn help(dispatch: Dispatch) -> DispatchResult<'static> {
            let help_vec = vec![
                "Try 'gamesdb <title>. Use a +category to fetch a specific category of data that we recognize. Use -# to fetch a specific index of the entries.'",
                "Example: mega man +youtube -1 -2 -3 # first three items, youtube link",
            ];

            let mut help = help_vec.iter();
            let target = dispatch.target;
            let sender = dispatch.sender;

            while let Some(message) = help.next() {
                match dispatch.client.send(DispatchReply {
                    target: target.to_string(),
                    text: format!("{}: {}", sender, message),
                }) {
                    Ok(_) => {}
                    Err(e) => return Err(Box::new(e)),
                }
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

        pub async fn gamesdb(dispatch: Dispatch) -> DispatchResult<'static> {
            if dispatch.text == "" {
                dispatch.client.send(DispatchReply {
                    target: dispatch.target,
                    text: String::from("Invalid query: try `help`"),
                })?;
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

                let search = parse(search_rx, "", &dispatch.text);
                println!("SEARCH {:?}", search);

                match fetch(search, categories, pages).await {
                    Ok(text) => {
                        if text.len() > 0 {
                            let mut iter = text.iter();
                            while let Some(t) = iter.next() {
                                if t.trim().len() != 0 {
                                    dispatch
                                        .client
                                        .send(DispatchReply {
                                            target: dispatch.target.to_string(),
                                            text: t.to_string(),
                                        })
                                        .unwrap();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        dispatch.client.send(DispatchReply {
                            target: dispatch.target,
                            text: String::from("Error fetching data"),
                        })?;
                    }
                }
            }
            Ok(())
        }
    }
}
