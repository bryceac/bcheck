use crate::Record;
use std::{fs::File, io::{ Write, BufRead, BufReader, Error} };
use serde_json;

pub trait Save {
    fn save(&self, path: &str) -> Result<(), io::Error>
}

impl Save for Vec<Record> {
    pub fn save(&self, path: &str) -> Result<(), io::Error> {
        let mut output = File::create(path)?;
        write!(output, serde_json::to_string_pretty(self))
    }
}