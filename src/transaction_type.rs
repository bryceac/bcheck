use serde::{ Deserialize, Serialize };
use crate::transaction_type_parse_error::TransactionTypeParseError;
use std::str::FromStr;

/// Represent the type of the transaction initiated.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal
}

impl FromStr for TransactionType {
    type Err = TransactionTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "deposit" => Ok(Self::Deposit),
            "withdrawal" => Ok(Self::Withdrawal),
            _ => Err(TransactionTypeParseError::InvalidType(s.to_lowercase()))
        }
    }
}