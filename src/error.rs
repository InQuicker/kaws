use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Error as FmtError;
use std::str::Utf8Error;

use rusoto::AwsError;
use rustc_serialize::base64::FromBase64Error;

pub struct KawsError {
    message: String,
}

impl KawsError {
    pub fn new(message: String) -> KawsError {
        KawsError {
            message: message,
        }
    }
}

impl Debug for KawsError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{:?}", self.message)
    }
}

impl Display for KawsError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.message)
    }
}

impl Error for KawsError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<::std::io::Error> for KawsError {
    fn from(error: ::std::io::Error) -> Self {
        KawsError {
            message: format!("{}", error),
        }
    }
}

impl From<Utf8Error> for KawsError {
    fn from(error: Utf8Error) -> Self {
        KawsError {
            message: format!("{}", error),
        }
    }
}

impl From<AwsError> for KawsError {
    fn from(error: AwsError) -> Self {
        KawsError {
            message: format!("{}", error),
        }
    }
}

impl From<FromBase64Error> for KawsError {
    fn from(error: FromBase64Error) -> Self {
        KawsError {
            message: format!("{}", error),
        }
    }
}

pub type KawsResult = Result<Option<String>, KawsError>;
