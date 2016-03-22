use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Error as FmtError;
use std::str::Utf8Error;

use rusoto::AWSError;
use rusoto::kms::KMSError;

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

impl From<AWSError> for KawsError {
    fn from(error: AWSError) -> Self {
        KawsError {
            message: format!("{}", error),
        }
    }
}

impl From<KMSError> for KawsError {
    fn from(error: KMSError) -> Self {
        KawsError {
            message: format!("{:?}", error),
        }
    }
}
pub type KawsResult = Result<Option<String>, KawsError>;
