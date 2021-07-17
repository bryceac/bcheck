use serde::{ self, Serialize, Deserialize, Serializer, Deserializer };
use crate::transaction_type::TransactionType;
use chrono::prelude::*;
use ordered_float::OrderedFloat;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub date: DateTime<Local>,
    pub check_number: Option<u32>,
    pub vendor: String,
    pub memo: String,
    pub amount: OrderedFloat<f64>,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType
}