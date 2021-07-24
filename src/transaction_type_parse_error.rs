use std::{ error::Error, fmt };

#[derive(Debug)]
pub enum TransactionTypeParseError {
    InvalidType(String)
}

impl fmt::Display for TransactionTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidType(s) => {
                let error_string = format!("{} is not a valid type", s);

                write!(f, "{}", error_string)
            }
        }
    }
}

impl Error for TransactionTypeParseError {}