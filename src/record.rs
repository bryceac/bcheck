use crate::transaction::Transaction;
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Record {
    #[serde(default = "default_id")]
    pub id: String,
    pub transaction: Transaction,
    #[serde(skip)]
    pub previous_record: Option<Box<Record>>
}

impl Record {
    fn new() -> Record {
        Record {
            id: default_id(),
            transaction: Transaction::new(),
            previous_record: None
        }
    }
}

fn default_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}