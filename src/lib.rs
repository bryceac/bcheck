mod transaction_type;
mod transaction;
mod record;

#[cfg(test)]
mod tests {
    use crate::record::Record;
    use crate::transaction::Transaction;
    use crate::transaction_type::TransactionType;
    use chrono::prelude::*;
    use ordered_float::OrderedFloat;
    use pretty_assertions::{assert_eq};

    use serde_json;
    #[test]
    fn serialize_record() {
        let record = Record::from(String::from("FF04C3DC-F0FE-472E-8737-0F4034C049F0"), Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), Some(1260 as u32), String::from("Sam Hill Credit Union"), String::from("Open Account"), OrderedFloat::<f64>(500 as f64), TransactionType::DEPOSIT, false), None);

        let expected_string = "{\n  \"id\": \"FF04C3DC-F0FE-472E-8737-0F4034C049F0\",\n  \"transaction\": {\n    \"date\": \"2021-07-08\",\n    \"check_number\": 1260,\n    \"vendor\": \"Sam Hill Credit Union\",\n    \"memo\": \"Open Account\",\n    \"amount\": 500.0,\n    \"type\": \"deposit\"\n  }\n}";

        if let Ok(json) = serde_json::to_string_pretty(&record) {
            assert_eq!(json, expected_string)
        }

        
    }

    #[test]
    fn deserialize_records() {
        let expected_record: Vec<Record> = vec![
            Record::from(String::from("FF04C3DC-F0FE-472E-8737-0F4034C049F0"), Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), Some(1260 as u32), String::from("Sam Hill Credit Union"), String::from("Open Account"), OrderedFloat::<f64>(500 as f64), TransactionType::DEPOSIT, false), None),
            Record::from(String::from("1422CBC6-7B0B-4584-B7AB-35167CC5647B"), Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), None, String::from("Fake Street Electronics"), String::from("Head set"), OrderedFloat::<f64>(200 as f64), TransactionType::WITHDRAWAL, false), None),
            Record::from(String::from("BB22187E-0BD3-41E8-B3D8-8136BD700865"), Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), None, String::from("Velociraptor Entertainment"), String::new(), OrderedFloat::<f64>(50000 as f64), TransactionType::DEPOSIT, false), None)
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
}
