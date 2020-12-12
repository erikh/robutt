use rand::prelude::*;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader, Lines, Write};
use std::ops::Index;

pub struct LoudFile {
    filename: String,
}

impl LoudFile {
    pub fn new(filename: String) -> LoudFile {
        return LoudFile { filename };
    }

    fn get_file(&self) -> io::Result<Lines<BufReader<File>>> {
        let file = File::open(self.filename.clone())?;
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

    pub fn append(&self, line: String) -> io::Result<()> {
        let bucket: Option<Vec<String>> = match self.bucket() {
            Ok(bucket) => Some(bucket),
            Err(_) => None,
        };

        if let Some(b) = bucket {
            if b.contains(&line) {
                return Ok(());
            }
        }

        let mut file = match OpenOptions::new().append(true).open(self.filename.clone()) {
            Ok(f) => f,
            Err(_) => File::create(self.filename.clone())?,
        };
        file.write_fmt(format_args!("{}\n", line))?;
        return Ok(());
    }
}
