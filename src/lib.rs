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

    use serde_json;
    #[test]
    fn serialize_record() {
        let record = Record::from(String::from("FF04C3DC-F0FE-472E-8737-0F4034C049F0"), Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), Some(1260 as u32), String::from("Sam Hill Credit Union"), String::from("Open Account"), OrderedFloat::<f64>(500 as f64), TransactionType::DEPOSIT, false), None);

        let expected_string = r#"
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
          }
        "#;

        if let Ok(json) = serde_json::to_string_pretty(&record) {
            assert_eq!(json, expected_string)
        }

        
    }
    /* fn deserialize_records() {
        let expected_record: Vec<Record> = vec![
            Record::from(String::from("FF04C3DC-F0FE-472E-8737-0F4034C049F0"), Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), Some(1260 as u32), String::from("Sam Hill Credit Union"), String::from("Open Account"), OrderedFloat::<f64>(500 as f64), TransactionType::DEPOSIT, false), None),
            Record::from(String::from("1422CBC6-7B0B-4584-B7AB-35167CC5647B"), Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), None, String::from("Fake Street Electronics"), String::from("Head set"), OrderedFloat::<f64>(200 as f64), TransactionType::WITHDRAWAL, false), None),
            Record::from(String::from("BB22187E-0BD3-41E8-B3D8-8136BD700865"), Transaction::from(Local.ymd(2021, 7, 8).and_hms(0, 0, 0), None, String::from("Velociraptor Entertainment"), String::new(), OrderedFloat::<f64>(50000 as f64), TransactionType::DEPOSIT, false), None)
        ];

        if let Ok(json_content) = File::open("/Users/bryce/Documents/transaction.bcheck") {
            let reader = BufReader::new(json_content);

            let records: Vec<Record> = serde_json::from_reader(reader)?;

            assert_eq!(records, expected_record)
        }
    } */
}
