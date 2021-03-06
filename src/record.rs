// import custom types, so they can be used.
use crate::transaction::Transaction;

// import serde, for easy serialization and deserialization
use serde::{ Serialize, Deserialize };

// import uuid crate, to easily generate ids
use uuid::Uuid;

// import File and io stuff, so that data can be loaded from a file.
use std::{fs::File, io::{ self, Read } };

// import serde_json crate, to facilitate deserialization from JSON.
use serde_json;

/// Represents an entry in a check register
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub struct Record {
    /// record identifier.
    #[serde(default = "default_id")]
    pub id: String,

    /// the record's transaction
    pub transaction: Transaction,
}

impl Record {
    /// create a new empty Record object.
    pub fn new() -> Record {
        Record {
            id: default_id(),
            transaction: Transaction::new()
        }
    }

    /**
     * Create a record object with given values.
     * If id is an empty String, an id will be generated for you.
     * # Example
     * ```let record = Record::from("FF04C3DC-F0FE-472E-8737-0F4034C049F0", Transaction::from(Some("2021-7-8"), Some(1260 as u32), String::from("Sam Hill Credit Union"), String::from("Open Account"), OrderedFloat::<f64>(500 as f64), TransactionType::DEPOSIT, false).unwrap());```
    */
    pub fn from(id: &str, transaction: Transaction) -> Record {
        Record {
            id: {
                if id.is_empty() {
                    default_id()
                } else {
                    String::from(id)
                }
            },
            transaction
        }
    }

    /**load vector containing Records from a given file path.
     * This method attempts to read a file containing record data, returning a vector if successful, but will give out an error if something goes wrong, either with loading the file or parsing it.
    */
    pub fn from_file(f: &str) -> Result<Vec<Record>, String> {
        /* if let Ok(content) = file_contents_from(f) {
            if let Ok(decoded_records) = serde_json::from_str::<Vec<Record>>(&content) {
                Ok(decoded_records)
            } else {
                Err(String::from("Could not parse data successfully."))
            }
        } else {
            Err(String::from("Could not read file"))
        } */

        match file_contents_from(f) {
            Ok(content) => {
                match serde_json::from_str::<Vec<Record>>(&content) {
                    Ok(decoded_records) => Ok(decoded_records),
                    Err(error) => Err(format!("{}", error))
                }
            },
            Err(error) => Err(format!("{}", error))
        }
    }
}

// apply trait necessary to check for equality
impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn default_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}

fn file_contents_from(f: &str) -> Result<String, io::Error> {
        let mut file_content = String::new();
        File::open(f)?.read_to_string(&mut file_content)?;

        Ok(file_content)
}