//! bcheck provides the ability to read bcheck files generated from the application BCheckbook
//! When using this crate in conjunction with serde_json in your own project, it is also possible to generate the files too, though I am hoping to find a way to extend Vector, so that it can all be done via this crate.
//! # Quick Start
//! The easiest way to get things working, after adding the crate as a dependency would be like this:
//! 
//! ```
//! use bcheck::{ Record, Transaction, TransactionType };
//! 
//! fn main() {
//!     if let Ok(records) = Record::from_file("/Users/bob/Docuents/transactions.bcheck") {
//!         for record in records {
//!             println!("{}", record.transaction.vendor)
//!         }
//!     }
//! }
//! ```
//! 
//! The above code will attempt to load a file, according the Mac OS file structure, and print out the vendor field of each record.
mod local_datetime_from_string;
mod transaction_type;
mod transaction;
mod record;
mod save_vec;
mod transaction_type_parse_error;

pub use crate::transaction::Transaction as Transaction;
pub use crate::transaction_type::TransactionType as TransactionType;
pub use crate::record::Record as Record;
pub use crate::save_vec::Save as Save;
pub use crate::local_datetime_from_string::LocalDateTimeExt as LocalDateTimeStringExt;
pub use crate::transaction::transaction_date_format::is_proper_format as is_proper_date_format;

#[cfg(test)]
mod tests {
    use crate::record::Record;
    use crate::transaction::Transaction;
    use crate::transaction_type::TransactionType;
    use crate::is_proper_date_format;
    use crate::LocalDateTimeStringExt;
    use pretty_assertions::{assert_eq};
    use crate::save_vec::Save;
    use std::str::FromStr;

    use serde_json;
    #[test]
    fn serialize_record() {
        let record = Record::from("FF04C3DC-F0FE-472E-8737-0F4034C049F0", Transaction::from(Some("2021-7-8"), Some(1260), "Sam Hill Credit Union", "Open Account", 500 as f64, TransactionType::Deposit, false).unwrap(), None);

        let expected_string = "{\n  \"id\": \"FF04C3DC-F0FE-472E-8737-0F4034C049F0\",\n  \"transaction\": {\n    \"date\": \"2021-07-08\",\n    \"check_number\": 1260,\n    \"vendor\": \"Sam Hill Credit Union\",\n    \"memo\": \"Open Account\",\n    \"amount\": 500.0,\n    \"type\": \"deposit\"\n  }\n}";

        if let Ok(json) = serde_json::to_string_pretty(&record) {
            assert_eq!(json, expected_string)
        }

        
    }

    #[test]
    fn deserialize_records() {
        let expected_record: Vec<Record> = vec![
            Record::from("FF04C3DC-F0FE-472E-8737-0F4034C049F0", Transaction::from(Some("2021-7-8"), Some(1260), "Sam Hill Credit Union", "Open Account", 500 as f64, TransactionType::Deposit, false).unwrap(), None),
            Record::from("1422CBC6-7B0B-4584-B7AB-35167CC5647B", Transaction::from(Some("2021-7-8"), None, "Fake Street Electronics", "Head set", 200 as f64, TransactionType::Withdrawal, false).unwrap(), None),
            Record::from("BB22187E-0BD3-41E8-B3D8-8136BD700865", Transaction::from(Some("2021-7-8"), None, "Velociraptor Entertainment", "", 50000 as f64, TransactionType::Deposit, false).unwrap(), None)
        ];

        let json = r#"
        [
            {
                "id" : "FF04C3DC-F0FE-472E-8737-0F4034C049F0",
                "transaction" : {
                    "amount" : 500,
                    "vendor" : "Sam Hill Credit Union",
                    "memo" : "Open Account",
                    "check_number" : 1260,
                    "type" : "deposit",
                    "date" : "2021-07-08"
                }
            },
            {
                "id" : "1422CBC6-7B0B-4584-B7AB-35167CC5647B",
                "transaction" : {
                    "amount" : 200,
                    "vendor" : "Fake Street Electronics",
                    "memo" : "Head set",
                    "type" : "withdrawal",
                    "date" : "2021-07-08"
                }
            },
            {
                "id" : "BB22187E-0BD3-41E8-B3D8-8136BD700865",
                "transaction" : {
                    "amount" : 50000,
                    "vendor" : "Velociraptor Entertainment",
                    "type" : "deposit",
                    "date" : "2021-07-08"
                }
            }
        ]
        "#;

        if let Ok(decoded_records) = serde_json::from_str::<Vec<Record>>(json) {
            assert_eq!(decoded_records, expected_record)
        }
    }

    #[test]
    fn transaction_throws_error_fix_with_improper_date() {
        let transaction = Transaction::from(Some("2021, 7, 8"), Some(1260), "Sam Hill Credit Union", "Open Account", 500 as f64, TransactionType::Deposit, false);

        assert!(transaction.is_err())
    }

    #[test]
    fn parse_deposit_type_from_string() {
        assert_eq!(TransactionType::from_str("deposit").unwrap(), TransactionType::Deposit)
    }

    #[test]
    fn parse_withdrawal_type_from_string() {
        assert_eq!(TransactionType::from_str("withdrawal").unwrap(), TransactionType::Withdrawal)
    }

    #[test]
    fn parse_type_from_string_errors_out() {
        assert!(TransactionType::from_str("boo").is_err())
    }

    #[test]
    fn verify_date_string_checking_function() {
        assert!(is_proper_date_format("2021-7-26"))
    }

    #[test]
    fn parse_date_from_string() {
        assert!(!String::from("2021-7-28").local_datetime().is_err())
    }

    #[test]
    fn parse_date_from_string_literal() {
        assert!(!"2021-7-26".local_datetime().is_err())
    }

    #[test]
    fn save_data() {
        let expected_record: Vec<Record> = vec![
            Record::from("FF04C3DC-F0FE-472E-8737-0F4034C049F0", Transaction::from(Some("2021-7-8"), Some(1260), "Sam Hill Credit Union", "Open Account", 500 as f64, TransactionType::Deposit, false).unwrap(), None),
            Record::from("1422CBC6-7B0B-4584-B7AB-35167CC5647B", Transaction::from(Some("2021-7-8"), None, "Fake Street Electronics", "Head set", 200 as f64, TransactionType::Withdrawal, false).unwrap(), None),
            Record::from("BB22187E-0BD3-41E8-B3D8-8136BD700865", Transaction::from(Some("2021-7-8"), None, "Velociraptor Entertainment", "", 50000 as f64, TransactionType::Deposit, false).unwrap(), None)
        ];

        assert!(!expected_record.save("test.bcheck").is_err())
    }
}
