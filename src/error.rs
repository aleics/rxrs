use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RxError {
    CustomError(String)
}

impl Error for RxError {
    fn description(&self) -> &str {
        match self {
            RxError::CustomError(ref err) => err.as_str()
        }
    }
}

impl fmt::Display for RxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RxError::CustomError(ref err) =>  fmt::Display::fmt(err, f)
        }
    }
}