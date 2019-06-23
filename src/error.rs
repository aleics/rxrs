use crossbeam::RecvError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RxError {
    SubscribeError(RecvError),
    CustomError(String)
}

impl From<RecvError> for RxError {
    fn from(err: RecvError) -> RxError {
        RxError::SubscribeError(err)
    }
}

impl Error for RxError {
    fn description(&self) -> &str {
        match self {
            RxError::SubscribeError(ref err) => err.description(),
            RxError::CustomError(ref err) => err.as_str()
        }
    }
}

impl fmt::Display for RxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RxError::SubscribeError(ref err) => fmt::Display::fmt(err, f),
            RxError::CustomError(ref err) =>  fmt::Display::fmt(err, f)
        }
    }
}