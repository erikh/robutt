use irc::client::prelude::*;
use rand::prelude::*;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader, Lines, Write};
use std::ops::Index;

pub struct LoudFile<'a> {
    filename: &'a str,
}

impl LoudFile<'_> {
    pub fn new(filename: &str) -> LoudFile {
        return LoudFile::<'_> { filename };
    }

    fn get_file(&self) -> io::Result<Lines<BufReader<File>>> {
        let file = File::open(self.filename)?;
        let reader = BufReader::new(file);
        Ok(reader.lines())
    }

    fn bucket(&self) -> io::Result<Vec<String>> {
        let mut vec: Vec<String> = Vec::new();
        let mut hs = HashSet::new();
        let mut lines = self.get_file()?;

        while let Some(line) = lines.next() {
            let b = Box::new(line.unwrap());
            if !hs.contains(b.as_ref()) {
                hs.insert(b.to_string());
                vec.push(b.to_string());
            }
        }

        return Ok(vec);
    }

    pub fn get_line(&self) -> Option<String> {
        match self.bucket() {
            Ok(bucket) => {
                let vec: Vec<String> = bucket;
                Some(
                    vec.index(rand::thread_rng().gen::<usize>() % vec.len())
                        .to_string(),
                )
            }
            Err(_) => None,
        }
    }

    pub fn append(&self, line: &str) -> io::Result<()> {
        let bucket: Option<Vec<String>> = match self.bucket() {
            Ok(bucket) => Some(bucket),
            Err(_) => None,
        };

        if let Some(b) = bucket {
            if b.contains(&line.to_string()) {
                return Ok(());
            }
        }

        let mut file = match OpenOptions::new().append(true).open(self.filename) {
            Ok(f) => f,
            Err(_) => File::create(self.filename)?,
        };
        file.write_fmt(format_args!("{}\n", line))?;
        return Ok(());
    }
}

fn load_config() -> Config {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        match Config::load(args.index(1)) {
            Ok(config) => config,
            Err(e) => {
                println!("Error loading config: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        Config {
            nickname: Some("robutt-dev".to_owned()),
            server: Some("irc.freenode.net".to_owned()),
            channels: Some(vec!["#tinyci".to_owned()]),
            ..Config::default()
        }
    }
}

fn main() {
    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&load_config()).unwrap();
    client.identify().unwrap();

    reactor.register_client_with_handler(client, |client, message| {
        match message.clone().command {
            Command::PRIVMSG(channel, text) => {
                if text.to_uppercase() == text {
                    let loudfile = LoudFile::new("loudfile.txt");

                    println!(
                        "LOUD: {} <{}> {}",
                        channel,
                        message.response_target().unwrap(),
                        text
                    );
                    loudfile.append(&text).unwrap();
                    if let Some(line) = loudfile.get_line() {
                        client.send_privmsg(channel, line)?;
                    }

                    drop(loudfile);
                }
            }
            Command::PONG(_, _) => (),
            _ => print!("{}", message),
        }

        // And here we can do whatever we want with the messages.
        Ok(())
    });

    reactor.run().unwrap();
}
