use std::{error::Error, fmt, io};

use reqwest::StatusCode;

#[derive(Debug)]
pub struct KrakenError {
    status_code: Option<StatusCode>,
    errors: Vec<String>,
}

impl KrakenError {
    pub fn new(status_code: Option<StatusCode>, errors: Vec<String>) -> KrakenError {
        KrakenError {
            status_code,
            errors,
        }
    }
}

impl Error for KrakenError {}

impl fmt::Display for KrakenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Status code: {}. {}",
            self.status_code.unwrap_or(StatusCode::BAD_REQUEST),
            self.errors.join(";")
        )
    }
}

//impl From<io::Error> for KrakenError {
//    fn from(err: io::Error) -> KrakenError {
//        KrakenError {
//            status_code: None,
//            error_type: ErrorType::FailResponse(err.to_string()),
//        }
//    }
//}
