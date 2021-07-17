use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAWAL
}