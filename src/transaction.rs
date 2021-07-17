use serde::{ self, Serialize, Deserialize, Serializer, Deserializer };
use crate::transaction_type::TransactionType;
use chrono::prelude::*;

pub struct Transaction {
    pub date: DateTime<Local>,
    pub check_number: Option<u32>,
    pub vendor: String,
    pub memo: String,
    pub amount
}