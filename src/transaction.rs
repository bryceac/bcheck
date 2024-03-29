// import Serde, so that type can be serialized and deserialized
use serde::{ Serialize, Deserialize, Serializer, Deserializer };

// import custom enumeration, so that is can be lined up properly.
use crate::transaction_type::TransactionType;

use crate::LocalDateTimeStringExt;

// import chrono crate, so that ate can be included in seriallization and properly set.
use chrono::prelude::*;

// import OrderedFloat, so that transactions can be compared by amount
use ordered_float::OrderedFloat;

// import to use regex verification
use regex::Regex;

// import to allow formatting as string.
use std::fmt;

/// Represent a transaction made.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub struct Transaction {
    #[serde(with = "transaction_date_format")]
    #[serde(default = "Local::now")]
    pub date: DateTime<Local>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
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
            category: None,
            vendor: String::new(),
            memo: String::new(),
            amount: default_float(),
            transaction_type: TransactionType::Withdrawal,
            is_reconciled: default_reconciled()

        }
    }

    /**
     * Create a transaction object with given values.
     * This function will throw an Error if a given date string is not in the proper format, which is "yyyy-mm-dd" or "yyyy-m-d".
     * # Example
     * ```let transaction = Transaction::from(None, Some(1260), Some("Opening Balance"), "Sam Hill Credit Union", "Open Account", 500 as f64, TransactionType::Deposit, false);```
     */
    pub fn from(date: Option<&str>, check_number: Option<u32>, category: Option<&str>, vendor: &str, memo: &str, amount: f64, transaction_type: TransactionType, is_reconciled: bool) -> Result<Transaction, String> {
        if let Some(date_string) = date {
            match date_string.local_datetime() {
                Ok(date_time) => Ok(Transaction {
                    date: date_time,
                    check_number,
                    category: if let Some(category) = category {
                        Some(String::from(category))
                    } else {
                        None
                    },
                    vendor: String::from(vendor),
                    memo: String::from(memo),
                    amount: OrderedFloat(amount),
                    transaction_type,
                    is_reconciled
                }),
                Err(error) => Err(error)
            }
        } else {
            Ok(Transaction {
                date: Local::now(),
                check_number,
                category: if let Some(category) = category {
                    Some(String::from(category))
                } else {
                    None
                },
                vendor: String::from(vendor),
                memo: String::from(memo),
                amount: OrderedFloat(amount),
                transaction_type,
                is_reconciled
            })
        }
    }

    /// create a transaction directly from a string.
    pub fn from_string(s: &str) -> Transaction {
        let components: Vec<&str> = s.split('\t').collect();

        Transaction {
            date: if let Ok(transaction_date) = components[0].to_string().local_datetime() {
                transaction_date
            } else {
                Local::now()
            },
            check_number: if components[1].to_string().is_empty() {
                None
            } else {
                if let Ok(value) = components[1].parse::<u32>() {
                    Some(value)
                } else {
                    None
                }
            },
            category: if components[3].to_string().is_empty() {
                None
            } else {
                Some(components[3].to_string())
            },
            vendor: components[4].to_string(),
            memo: components[5].to_string(),
            amount: if components[6].to_string().is_empty() {
                OrderedFloat(components[7].parse::<f64>().ok().unwrap_or_else(|| 0.0))
            } else {
                OrderedFloat(components[6].parse::<f64>().ok().unwrap_or_else(|| 0.0))
            },
            transaction_type: if components[6].to_string().is_empty() {
                TransactionType::Withdrawal
            } else {
                TransactionType::Deposit
            },
            is_reconciled: components[2].to_string() == "Y"
        }
    }

    /// presents a string version of the transaction.
    pub fn to_string(&self) -> String {
        let mut transaction_string = String::new();

        let date_string = format!("{}\t", self.date.format("%Y-%m-%d"));

        transaction_string.push_str(&date_string);

        if let Some(check_number) = self.check_number {
            let check_number_string = format!("{}\t", check_number);
            transaction_string.push_str(&check_number_string);
        } else {
            transaction_string.push_str("\t");
        }

        transaction_string.push_str(if self.is_reconciled {
            "Y\t"
        } else { "N\t" });

        if let Some(category) = self.category.clone() {
            let category_string = format!("{}\t", category);

            transaction_string.push_str(&category_string);
        } else {
            transaction_string.push_str("\t");
        }

        let vendor_string = format!("{}\t", self.vendor);

        transaction_string.push_str(&vendor_string);

        let memo_string = format!("{}\t", self.memo);

        transaction_string.push_str(&memo_string);

        let amount_string = if let TransactionType::Withdrawal = self.transaction_type {
            format!("\t{:.2}", self.amount)
        } else {
            format!("{:.2}\t", self.amount)
        };

        transaction_string.push_str(&amount_string);

        return transaction_string;
    }
}

// implement trait need to deal with equality
impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && 
        self.check_number == other.check_number &&
        self.category == other.category &&
        self.vendor == other.vendor &&
        self.memo == other.memo &&
        self.amount == other.amount &&
        self.transaction_type == other.transaction_type &&
        self.is_reconciled == other.is_reconciled
    }
}

// implement trait needed to display item as string
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// create module to custom the serialization and deserialization of dates.
/** 
 * module containing stuff to deal with serializing, deserializing, and verifying dates.
 * This module is only made public, to help ease verification of date format.
 */
pub mod transaction_date_format {
    use super::*;

    /// the format for the date that is expected.
    pub const FORMAT: &'static str = "%Y-%m-%d";

    /// serialize dates as strings. This is typically not used directly, as serde grabs it automatically.
    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        let date_string = format!("{}", date.format(&FORMAT));
        serializer.serialize_str(&date_string)
    }

    /// deserialize dates from strings. Like the serialize method, this is typically never used.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error> where D: Deserializer<'de>, {
        let date_string = String::deserialize(deserializer)?;
        let naive_date = NaiveDate::parse_from_str(&date_string, FORMAT).map_err(serde::de::Error::custom)?;
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();

        let local_datetime = Local.from_local_datetime(&naive_datetime).unwrap();

        Ok(local_datetime)
    }

    /// verify a given string uses the appropriate date format.
    pub fn is_proper_format(s: &str) -> bool {
        let re = Regex::new(r"^\d{4}-\d{1,2}-\d{1,2}$").unwrap();

        re.is_match(s)
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