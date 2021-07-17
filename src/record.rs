use crate::{ transaction::Transaction, transaction_type::TransactionType };
use serde::{ Serialize, Deserialize, Deserializer };
use uuid::Uuid;
use ordered_float::OrderedFloat;

#[derive(Debug, Serialize, Deserialize, Eq, PartialOrd, Ord)]
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

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn default_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}