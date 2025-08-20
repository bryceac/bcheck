// import custom type for use in specifying particular vector
use crate::Record;

// import things needed for dealing with files, so saving can work.
use std::{fs::File, io::{ Write, Error } };

// import serde_json, to save contents as JSON
use serde_json;

/// enables the ability to save data when implemented.
pub trait Save {
    fn save(&self, path: &str) -> Result<(), Error>;
    fn save_tsv(&self, path: &str) -> Result<(), Error>;
}

// add implementation of Save trait to Vector of Records.
impl Save for Vec<Record> {
    fn save(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;
        let json_string = serde_json::to_string_pretty(self)?;

        match write!(output, "{}", format!("{}", json_string)) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }  
    }

    fn save_tsv(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;
        let tsv_string: String = self.iter().map(|record| record.to_string() + "\r\n").collect();

        match write!(output, "{}", format!("{}", tsv_string)) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        } 
    }
}