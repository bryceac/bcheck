use serde::{ de, Serialize, Deserialize, Serializer, Deserializer };
use crate::transaction_type::TransactionType;
use chrono::prelude::*;
use ordered_float::OrderedFloat;

#[derive(Debug, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub struct Transaction {
    #[serde(with = "transaction_date_format")]
    #[serde(default = "Local::now")]
    pub date: DateTime<Local>,
    pub check_number: Option<u32>,
    pub vendor: String,
    #[serde(default = "String::new")]
    pub memo: String,
    #[serde(default = "default_float")]
    pub amount: OrderedFloat<f64>,
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub transaction_type: TransactionType,
    #[serde(default = "default_reconciled")]
    pub is_reconciled: bool
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            date: Local::now(),
            check_number: None,
            vendor: String::new(),
            memo: String::new(),
            amount: default_float(),
            transaction_type: default_type(),
            is_reconciled: default_reconciled()

        }
    }

    pub fn from(date: DateTime<Local>, check_number: Option<u32>, vendor: String, memo: String, amount: OrderedFloat<f64>, transaction_type: TransactionType, is_reconciled: bool) -> Transaction {
        Transaction {
            date,
            check_number,
            vendor,
            memo,
            amount,
            transaction_type,
            is_reconciled
        }
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && 
        self.check_number == other.check_number &&
        self.vendor == other.vendor &&
        self.memo == other.memo &&
        self.amount == other.amount &&
        self.transaction_type == other.transaction_type &&
        self.is_reconciled == other.is_reconciled
    }
}

mod transaction_date_format {
    use super::*;

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        let date_string = format!("{}", date.format(&FORMAT));
        serializer.serialize_str(&date_string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error> where D: Deserializer<'de>, {
        let date_string = String::deserialize(deserializer)?;
        Local.datetime_from_str(&date_string, &FORMAT).map_err(de::Error::custom)
    }
}

fn default_float() -> OrderedFloat<f64> {
    OrderedFloat::<f64>(0.0)
}

fn default_reconciled() -> bool {
    false
}

fn default_type() -> TransactionType {
    TransactionType::WITHDRAWAL
}