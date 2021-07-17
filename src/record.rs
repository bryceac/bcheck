use crate::transaction::Transaction;
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub struct Record {
    #[serde(default = "default_id")]
    pub id: String,
    pub transaction: Transaction,
    #[serde(skip)]
    pub previous_record: Option<Box<Record>>
}

impl Record {
    pub fn new() -> Record {
        Record {
            id: default_id(),
            transaction: Transaction::new(),
            previous_record: None
        }
    }

    pub fn from(id: String, transaction: Transaction, previous_record: Option<Record>) -> Record {
        Record {
            id,
            transaction,
            previous_record: {
                match previous_record {
                    Some(record) => Some(Box::new(record)),
                    None => None
                }
            }
        }
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn default_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}