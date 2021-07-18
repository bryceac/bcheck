use crate::Record;
use std::{fs::File, io::{ Write, Error } };
use serde_json;

pub trait Save {
    fn save(&self, path: &str) -> Result<(), Error>;
}

impl Save for Vec<Record> {
    fn save(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;
        let json_string = serde_json::to_string_pretty(self)?;

        match write!(output, "{}", format!("{}", json_string)) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }  
    }
}