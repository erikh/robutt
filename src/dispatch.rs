use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use tokio::sync::mpsc;

lazy_static! {
    static ref URL_REGEX: Regex = Regex::new("https?://[^ ]+").unwrap();
    static ref LOUD_REGEX: Regex = Regex::new("[A-Z ]{5}").unwrap();
}

#[derive(Clone, Debug)]
pub enum DispatchSource {
    IRC,
}

#[derive(Clone, Debug)]
pub struct Dispatch {
    nick: String,
    sender: String,
    target: String,
    text: String,
    source: DispatchSource,
}

#[derive(Clone, Debug)]
pub struct DispatchReply {
    pub target: String,
    pub text: String,
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

const TRIGGER_CHARACTER: &str = r"!";

pub fn is_url(text: &str) -> bool {
    URL_REGEX.is_match(text)
}

pub fn extract_urls(text: &str) -> Vec<url::Url> {
    let mut v: Vec<url::Url> = Vec::new();

    for m in URL_REGEX.find_iter(text) {
        match url::Url::parse(m.as_str()) {
            Ok(p) => v.push(p),
            Err(_) => {}
        }
    }

    v
}

impl Dispatch {
    pub fn new(
        nick: String,
        sender: String,
        target: String,
        text: String,
        source: DispatchSource,
    ) -> Dispatch {
        Dispatch {
            nick,
            sender,
            target,
            text,
            source,
        }
    }

    pub fn is_loud(&self) -> bool {
        self.text.to_uppercase().eq(&self.text)
            && LOUD_REGEX.is_match(&self.text)
            && self.text.len() >= 5
    }

    pub async fn dispatch(&mut self) -> Result<mpsc::Receiver<DispatchReply>> {
        let text = match self.source {
            DispatchSource::IRC => {
                let prefix = format!("{}: ", &self.nick);
                if self.text.starts_with(prefix.as_str()) {
                    self.text.trim_start_matches(prefix.as_str())
                } else if self.text.starts_with(TRIGGER_CHARACTER) {
                    self.text.trim_start_matches(TRIGGER_CHARACTER)
                } else {
                    &self.text
                }
            }
        }
        .trim()
        .to_string();

        let (mut s, r) = mpsc::channel::<DispatchReply>(100);
        if self.is_loud() {
            self.text = text.clone();
            targets::loud(self, &mut s).await?;
        } else if is_url(&text) {
            let mut d = self.clone();
            let urls = text.clone();
            d.text = text;
            let mut tmp_s = s.clone();
            targets::unroll_urls(d, &mut tmp_s, extract_urls(&urls)).await?;
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
    use lazy_static::lazy_static;
    use regex::Regex;
    use tokio::sync::mpsc;
    use trust_dns_resolver::AsyncResolver;

    lazy_static! {
        static ref TITLE_PATTERN: Regex = Regex::new("<title>([^<]+)</title>").unwrap();
    }

    pub async fn unroll_urls(
        dispatch: Dispatch,
        sender: &mut mpsc::Sender<DispatchReply>,
        urls: Vec<url::Url>,
    ) -> DispatchResult {
        let resolver = AsyncResolver::tokio_from_system_conf()?;
        let restricted_ips = vec!["10.", "172.16.", "192.168.", "127."];

        if urls.len() > 3 {
            return Ok(());
        }

        for url in urls {
            match url.host() {
                Some(host) => match resolver.lookup_ip(host.to_string()).await {
                    Ok(v) => {
                        let mut restricted = false;

                        for x in v {
                            let str_ip = x.to_string();
                            if restricted_ips.iter().any(|y| str_ip.starts_with(y)) {
                                restricted = true;
                            }
                        }

                        if !restricted {
                            let text = reqwest::get(url.clone()).await?.text().await?;
                            let title = match TITLE_PATTERN.captures(&text) {
                                Some(c) => match c.get(1) {
                                    Some(c) => c.as_str(),
                                    None => "",
                                },
                                None => "",
                            }
                            .trim();

                            if title != "" {
                                let mut i = 0;

                                for title_part in html_escape::decode_html_entities(title)
                                    .split("\n")
                                    .into_iter()
                                {
                                    if title_part.trim().len() == 0 {
                                        continue;
                                    }

                                    let mut title_part = title_part.to_string();

                                    if i > 0 {
                                        title_part = "... ".to_string() + &title_part;
                                    }

                                    sender
                                        .send(DispatchReply {
                                            target: dispatch.target.to_string(),
                                            text: format!(
                                                "[{}]: {}",
                                                url.host().unwrap(),
                                                title_part,
                                            ),
                                        })
                                        .await?;
                                    i += 1;
                                }
                            }
                        }
                    }
                    Err(_) => {}
                },
                None => {}
            }
        }
        Ok(())
    }

    pub async fn loud(
        dispatch: &mut Dispatch,
        sender: &mut mpsc::Sender<DispatchReply>,
    ) -> DispatchResult {
        let loudfile = LoudFile::new(String::from("loudfile.txt"));

        if dispatch.is_loud() && !dispatch.text.trim().is_empty() {
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
        use lazy_static::lazy_static;
        use regex::Regex;
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;
        use std::io::SeekFrom;
        use std::os::unix::prelude::MetadataExt;
        use tokio::sync::mpsc;

        lazy_static! {
            static ref DICE_REGEX: Regex =
                Regex::new(r"\s*(([1-9][0-9]*)d)?([1-9][0-9]*)(([+-][1-9][0-9]*))?").unwrap();
        }

        // http://www.textfiles.com/humor/deep.txt
        const THOUGHTS_FILE: &str = "deep.txt";

        pub async fn help(
            dispatch: &mut Dispatch,
            send: &mut mpsc::Sender<DispatchReply>,
        ) -> DispatchResult {
            let help_vec = vec![
                "The bot only responds to being addressed directly, e.g., `robutt: roll 1d4`, or with `!` prefixing, e.g., `!roll 1d4`",
                "otherwise, BE LOUD!",
                "Try asking robutt what she thinks.",
                "`roll 2d6` for hot die on die action",
            ];

            let help = help_vec.iter();
            let target = dispatch.target.clone();
            let sender = dispatch.sender.clone();

            for message in help {
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

        pub fn convert_capture(captures: &regex::Captures, index: usize, default: i32) -> i32 {
            match captures.get(index) {
                Some(x) => x.as_str().parse::<i32>().unwrap_or(default),
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

            let captures = match DICE_REGEX.captures(&dispatch.text) {
                Some(c) => c,
                None => {
                    return short_help(dispatch, sender).await;
                }
            };

            let num_dice = convert_capture(&captures, 2, 1);
            let die_size = convert_capture(&captures, 3, 10);
            let offset = convert_capture(&captures, 5, 0);

            let mut dice: Vec<i32> = Vec::new();
            let mut sum: i128 = 0;

            for _ in 0..num_dice {
                let mut result: i32 = rand::random();
                result = (result.abs() % die_size) + 1; // dice start at 1
                dice.push(result);
                sum += result as i128;
            }

            sender
                .send(DispatchReply {
                    target: dispatch.target.clone(),
                    text: format!("sum: {} | dice: {:?}", sum + offset as i128, dice),
                })
                .await?;

            Ok(())
        }

        lazy_static! {
            static ref PUNC_REGEX: Regex = regex::Regex::new("[!.?] ").unwrap();
            static ref WHITESPACE_COLLAPSE_REGEX: Regex = regex::Regex::new(r"\s{2,}").unwrap();
            static ref NEWLINE_REGEX: Regex = Regex::new(r"[\n\r]").unwrap();
        }

        const MIN_LINE_LEN: usize = 32;
        const MAX_LINE_LEN: usize = 128;

        pub async fn thoughts(
            dispatch: &mut Dispatch,
            sender: &mut mpsc::Sender<DispatchReply>,
        ) -> DispatchResult {
            let file = File::open(THOUGHTS_FILE)?;
            let stat = file.metadata()?;
            let mut br = BufReader::new(file);
            let seek = rand::random::<u64>() % stat.size();
            br.seek(SeekFrom::Start(seek))?;

            let mut out = String::default();
            let mut punc_found = false;

            let mut buf = String::default();
            br.read_line(&mut buf)?;

            let mut buf = String::default();
            br.read_line(&mut buf)?;

            let mut outlen = out.len();

            while outlen < MAX_LINE_LEN && outlen < MIN_LINE_LEN && !punc_found {
                if let Ok(_) = br.read_line(&mut buf) {
                    let buf = buf.trim();
                    let len = buf.len();

                    if let Some(idx) = PUNC_REGEX.find(&buf[..len]) {
                        if buf[..idx.end()].len() + outlen > MAX_LINE_LEN {
                            if outlen == 0 {
                                // first line is too long, whitespace returned
                                continue;
                            }
                            break;
                        }

                        out += &buf[..idx.end()].trim();
                        punc_found = true;
                    } else {
                        out += &buf[..len].trim();
                    }

                    out += " ";
                    outlen = out.len()
                } else {
                    break;
                }
            }

            sender
                .send(DispatchReply {
                    target: dispatch.target.clone(),
                    text: WHITESPACE_COLLAPSE_REGEX
                        .replace_all(&NEWLINE_REGEX.replace_all(&out, " "), " ")
                        .to_string(),
                })
                .await?;

            Ok(())
        }
    }
}
