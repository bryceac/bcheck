// import Serde, so that type can be serialized and deserialized
use serde::{ Serialize, Deserialize, Serializer, Deserializer };

// import custom enumeration, so that is can be lined up properly.
use crate::transaction_type::TransactionType;

// import chrono crate, so that ate can be included in seriallization and properly set.
use chrono::prelude::*;

// import OrderedFloat, so that transactions can be compared by amount
use ordered_float::OrderedFloat;

/// Represent a transaction made.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub struct Transaction {
    #[serde(with = "transaction_date_format")]
    #[serde(default = "Local::now")]
    pub date: DateTime<Local>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<u32>,
    pub vendor: String,
    #[serde(default = "String::new", skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[serde(default = "default_float")]
    pub amount: OrderedFloat<f64>,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    #[serde(default = "default_reconciled", skip_serializing_if = "is_default_reconciled")]
    pub is_reconciled: bool
}

impl Transaction {
    /// create empty Transaction object.
    pub fn new() -> Transaction {
        Transaction {
            date: Local::now(),
            check_number: None,
            vendor: String::new(),
            memo: String::new(),
            amount: default_float(),
            transaction_type: TransactionType::WITHDRAWAL,
            is_reconciled: default_reconciled()

        }
    }

    /**
     * Create a transaction object with given values.
     * # Example
     * ```let transaction = Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), Some(1260), "Sam Hill Credit Union", "Open Account", 500 as f64, TransactionType::DEPOSIT, false);```
     */
    pub fn from(date: DateTime<Local>, check_number: Option<u32>, vendor: &str, memo: &str, amount: f64, transaction_type: TransactionType, is_reconciled: bool) -> Transaction {
        Transaction {
            date,
            check_number,
            vendor: String::from(vendor),
            memo: String::from(memo),
            amount: OrderedFloat(amount),
            transaction_type,
            is_reconciled
        }
    }
}

// implement trait need to deal with equality
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

// create module to custom the serialization and deserialization of dates.
mod transaction_date_format {
    use super::*;

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        let date_string = format!("{}", date.format(&FORMAT));
        serializer.serialize_str(&date_string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error> where D: Deserializer<'de>, {
        let date_string = String::deserialize(deserializer)?;
        let naive_date = NaiveDate::parse_from_str(&date_string, FORMAT).map_err(serde::de::Error::custom)?;
        let naive_datetime = naive_date.and_hms(0, 0, 0);

        let local_datetime = Local.from_local_datetime(&naive_datetime).unwrap();

        Ok(local_datetime)
    }
}

// create functions for use with setting default values.
fn default_float() -> OrderedFloat<f64> {
    OrderedFloat::<f64>(0.0)
}

fn default_reconciled() -> bool {
    false
}

fn is_default_reconciled(v: &bool) -> bool {
    *v == default_reconciled()
}