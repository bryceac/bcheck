use serde::{ de, Serialize, Deserialize, Serializer, Deserializer };
use crate::transaction_type::TransactionType;
use chrono::prelude::*;
use ordered_float::OrderedFloat;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    #[serde(with = "transaction_date_format")]
    #[serde(default = "Local::now")]
    pub date: DateTime<Local>,
    #[serde(default = "default_checkNumber")]
    pub check_number: Option<u32>,
    pub vendor: String,
    #[serde(default = "String::new")]
    pub memo: String,
    #[serde(default = "default_float")]
    pub amount: OrderedFloat<f64>,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub is_reconciled: bool
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

fn default_checkNumber() -> Option<u32> {
    None
}

fn default_float() -> OrderedFloat<f64> {
    OrderedFloat::<f64>(0.0)
}