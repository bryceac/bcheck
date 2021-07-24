use std::{ error::Error, fmt };

#[derive(Debug)]
pub enum TransactionTypeParseError {
    INVALID_TYPE(String)
}

impl fmt::Display for TransactionTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::INVALID_TYPE(s) => {
                let error_string = format!("{} is not a valid type", s);

                write!(f, "{}", error_string)
            }
        }
    }
}

impl Error for TransactionTypeParseError {}