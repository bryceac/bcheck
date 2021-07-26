use chrono::prelude::*;
use std::io::Error;
use crate::is_proper_date_format;

pub trait LocalDateTimeExt {
    fn local_datetime(&self) -> Result<DateTime<Local>, String>;
}

impl LocalDateTimeExt for String {
    fn local_datetime(&self) -> Result<DateTime<Local>, String> {
        if is_proper_date_format(self) {
            let naive_date = NaiveDate::parse_from_str(self, crate::transaction::transaction_date_format::FORMAT).unwrap();

            let naive_datetime = naive_date.and_hms(0, 0, 0);

            let local_datetime = Local.from_local_datetime(&naive_datetime).unwrap();

            Ok(local_datetime)
        } else {
            Err(String::from("String must be in the format of YYYY-MM-DD."))
        }
        
    }
}