// import custom types, so they can be used.
use crate::{ transaction::Transaction, transaction_type::TransactionType };

// import serde, for easy serialization and deserialization
use serde::{ Serialize, Deserialize };

// import uuid crate, to easily generate ids
use uuid::Uuid;

// import OrderedFloat, so that balance can be calculated correctly.
use ordered_float::OrderedFloat;

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

    /** the record preceding this one.
     * This field defaults to None and is normally initialized as None.
    */
    #[serde(skip)]
    pub previous_record: Option<Box<Record>>
}

impl Record {
    /// create a new empty Record object.
    pub fn new() -> Record {
        Record {
            id: default_id(),
            transaction: Transaction::new(),
            previous_record: None
        }
    }

    /**
     * Create a record object with given values.
     * If id is an empty String, an id will be generated for you.
     * # Example
     * ```let record = Record::from(String::from("FF04C3DC-F0FE-472E-8737-0F4034C049F0"), Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), Some(1260 as u32), String::from("Sam Hill Credit Union"), String::from("Open Account"), OrderedFloat::<f64>(500 as f64), TransactionType::DEPOSIT, false), None);```
    */
    pub fn from(id: &str, transaction: Transaction, previous_record: Option<Record>) -> Record {
        Record {
            id: {
                if id.is_empty() {
                    default_id()
                } else {
                    String::from(id)
                }
            },
            transaction,
            previous_record: {
                match previous_record {
                    Some(record) => Some(Box::new(record)),
                    None => None
                }
            }
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

    /**retrieve the current balance as of this record.
     * If the previous_record field is None, it will simply return a value based on whether the transaction was a deposit or withdrawal.
     * Otherwise, it will retrieve the balance from the previous record and either add or subtract the amount specified in this record's transaction.
     * For best result, the previous record should be a record that has a date preceding the current record.
     */
    pub fn balance(&self) -> OrderedFloat<f64> {
        let mut value = OrderedFloat::from(0.0);

        if let Some(previous_record) = &self.previous_record {
            value = previous_record.balance();
        }

        match self.transaction.transaction_type {
            TransactionType::DEPOSIT => value = value + self.transaction.amount,
            TransactionType::WITHDRAWAL => value = value - self.transaction.amount
        }
        
        return value
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