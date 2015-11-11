use std::fs::{DirEntry, read_dir, remove_file};
use std::io::ErrorKind;

use error::{Error, Result};
use log::Logger;
use process::execute_child_process;

pub struct TemporaryDecryption<'a> {
    pub encrypted_path: &'a str,
    pub logger: &'a Logger,
    pub unencrypted_path: &'a str,
}

impl<'a> TemporaryDecryption<'a> {
    pub fn decrypt(&self) -> Result {
        try!(self.logger.action(&format!("Decrypting {}", self.encrypted_path), || {
            execute_child_process("gpg2", &[
                "--output",
                self.unencrypted_path,
                "--decrypt",
                self.encrypted_path,
            ])
        }));

        Ok(None)
    }
}

impl<'a> Drop for TemporaryDecryption<'a> {
    fn drop(&mut self) {
        self.logger.action(&format!("Removing unencrypted file {}", self.unencrypted_path), || {
            if let Err(error) = remove_file(self.unencrypted_path) {
                match error.kind() {
                    ErrorKind::NotFound => {},
                    _ => panic!(
                        "Failed to remove unencrypted file! You should remove it yourself! Error: {}",
                        error
                    ),
                }
            }
        });
    }
}

fn import_public_key(entry: DirEntry) -> Result {
    let path = entry.path();
    let str_path = try!(path.to_str().ok_or_else(|| {
        Error::new("invalid utf8 in directory name".to_string())
    }));

    try!(execute_child_process("gpg2", &[
        "--import",
        str_path,
    ]));

    Ok(None)
}

pub fn import_public_keys(logger: &mut Logger) -> Result {
    logger.action("Synchronizing PGP public keys with the local keyring", || {
        for entry in try!(read_dir("pubkeys")) {
            try!(import_public_key(try!(entry)));
        }

        Ok(None)
    })
}
