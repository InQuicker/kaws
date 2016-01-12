use std::fs::{File, remove_file};
use std::io::{ErrorKind, Read, Write};

use rusoto::credentials::DefaultAWSCredentialsProviderChain;
use rusoto::kms::{EncryptRequest, EncryptResponse, KMSClient, Result as KMSResult};
use rusoto::regions::Region;

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

pub struct Encryptor<'a> {
    client: KMSClient<'a>,
    kms_master_key_id: &'a str,
}

impl<'a> Encryptor<'a> {
    pub fn new(
        provider: DefaultAWSCredentialsProviderChain,
        region: &'a Region,
        kms_master_key_id: &'a str,
    ) -> Encryptor<'a> {
        Encryptor {
            client: KMSClient::new(provider, region),
            kms_master_key_id: kms_master_key_id,
        }
    }

    pub fn encrypt<'b>(&mut self, decrypted_data: &'b str) -> KMSResult<EncryptResponse> {
        let request = EncryptRequest {
            Plaintext: decrypted_data.as_bytes().to_vec(),
            EncryptionContext: None,
            KeyId: self.kms_master_key_id.to_owned(),
            GrantTokens: None,
        };

        self.client.encrypt(&request)
    }

    pub fn encrypt_file<'b>(&mut self, source: &'b str, destination: &'b str) -> Result {
        let mut src = try!(File::open(source));

        let mut contents = String::new();

        try!(src.read_to_string(&mut contents));

        let encrypted_data = try!(self.encrypt(&contents));

        let mut dst = try!(File::create(destination));

        match encrypted_data.CiphertextBlob {
            Some(ref ciphertext_blob) => try!(dst.write_all(ciphertext_blob)),
            None => return Err(Error::new("No ciphertext blob was returned from KMS".to_owned())),
        }

        Ok(None)
    }
}
