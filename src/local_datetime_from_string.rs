use chrono::prelude::*;
use std::io::Error;

pub trait LocalDateTimeExt {
    fn local_dateTime(&self) -> Result<DateTime<Local>, Error>;
}

impl LocalDateTimeExt for String {
    fn local_dateTime(&self) -> Result<DateTime<Local>, Error> {
        let naive_date = NaiveDate::parse_from_str(self, crate::transaction::transaction_date_format::FORMAT).unwrap();

        let naive_datetime = naive_date.and_hms(0, 0, 0);

        let local_datetime = Local.from_local_datetime(&naive_datetime).unwrap();

        Ok(local_datetime)
    }
}