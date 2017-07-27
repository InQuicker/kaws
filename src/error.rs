use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Error as FmtError;
use std::str::Utf8Error;

use rusoto_core::ParseRegionError;
use rusoto_kms::{DecryptError, EncryptError};
use rustc_serialize::base64::FromBase64Error;
use serde_json::Error as SerdeJsonError;

pub struct KawsError {
    message: String,
    stderr: Option<String>,
    stdout: Option<String>,
}

impl KawsError {
    pub fn new(message: String) -> KawsError {
        KawsError {
            message: message,
            stderr: None,
            stdout: None,
        }
    }

    pub fn with_std_streams(message: String, stdout: String, stderr: String) -> KawsError {
        KawsError {
            message: message,
            stderr: Some(stderr),
            stdout: Some(stdout),
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
        if self.stdout.is_some() && self.stderr.is_some() {
            write!(f,
                "{}

                Standard streams from the underlying command that failed:

                stdout:
                {}

                stderr:
                {}",
                self.message,
                self.stdout.as_ref().expect("accessing self.stdout"),
                self.stderr.as_ref().expect("accessing self.stderr")
            )
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl Error for KawsError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<::std::io::Error> for KawsError {
    fn from(error: ::std::io::Error) -> Self {
        KawsError::new(format!("{}", error))
    }
}

impl From<Utf8Error> for KawsError {
    fn from(error: Utf8Error) -> Self {
        KawsError::new(format!("{}", error))
    }
}

impl From<DecryptError> for KawsError {
    fn from(error: DecryptError) -> Self {
        KawsError::new(format!("{}", error))
    }
}

impl From<EncryptError> for KawsError {
    fn from(error: EncryptError) -> Self {
        KawsError::new(format!("{}", error))
    }
}

impl From<FromBase64Error> for KawsError {
    fn from(error: FromBase64Error) -> Self {
        KawsError::new(format!("{}", error))
    }
}

impl From<ParseRegionError> for KawsError {
    fn from(error: ParseRegionError) -> Self {
        KawsError::new(format!("{}", error))
    }
}

impl From<SerdeJsonError> for KawsError {
    fn from(error: SerdeJsonError) -> Self {
        KawsError::new(format!("{}", error))
    }
}

pub type KawsResult = Result<Option<String>, KawsError>;
