use std::error::Error as BaseError;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Error as FmtError;
use std::result::Result as BaseResult;
use std::str::Utf8Error;

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error {
            message: message,
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> BaseResult<(), FmtError> {
        write!(f, "{:?}", self.message)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> BaseResult<(), FmtError> {
        write!(f, "{}", self.message)
    }
}

impl BaseError for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<::std::io::Error> for Error {
    fn from(error: ::std::io::Error) -> Self {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error {
            message: format!("{}", error),
        }
    }
}

pub type Result = BaseResult<Option<String>, Error>;
