use std::fs::{File, remove_file};
use std::io::{ErrorKind, Read, Write};

use rusoto::credentials::DefaultAWSCredentialsProviderChain;
use rusoto::kms::{
    DecryptRequest,
    DecryptResponse,
    EncryptRequest,
    EncryptResponse,
    KMSClient,
    Result as KMSResult,
};
use rusoto::regions::Region;

use error::{Error, Result};
use process::execute_child_process;

pub struct TemporaryDecryption<'a> {
    pub encrypted_path: &'a str,
    pub unencrypted_path: &'a str,
}

impl<'a> TemporaryDecryption<'a> {
    pub fn decrypt(&self) -> Result {
        log_wrap!(&format!("Decrypting {}", self.encrypted_path), {
            try!(execute_child_process("gpg2", &[
                "--output",
                self.unencrypted_path,
                "--decrypt",
                self.encrypted_path,
            ]));
        });

        Ok(None)
    }
}

impl<'a> Drop for TemporaryDecryption<'a> {
    fn drop(&mut self) {
        log_wrap!(&format!("Removing unencrypted file {}", self.unencrypted_path), {
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
    decrypted_files: Vec<String>,
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
            decrypted_files: vec![],
            kms_master_key_id: kms_master_key_id,
        }
    }

    pub fn decrypt<'b>(&mut self, encrypted_data: &'b str) -> KMSResult<DecryptResponse> {
        let request = DecryptRequest {
            EncryptionContext: None,
            GrantTokens: None,
            CiphertextBlob: encrypted_data.as_bytes().to_vec()
        };

        self.client.decrypt(&request)
    }

    pub fn decrypt_file<'b>(&mut self, source: &'b str, destination: &'b str) -> Result {
        let mut src = try!(File::open(source));

        let mut encrypted_data = String::new();

        try!(src.read_to_string(&mut encrypted_data));

        let decrypted_data = try!(self.decrypt(&encrypted_data));

        let mut dst = try!(File::create(destination));

        match decrypted_data.Plaintext {
            Some(ref plaintext) => try!(dst.write_all(plaintext)),
            None => return Err(Error::new("No plaintext was returned from KMS".to_owned())),
        }

        self.decrypted_files.push(destination.to_string());

        Ok(None)
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
            None => return Err(Error::new("No ciphertext was returned from KMS".to_owned())),
        }

        Ok(None)
    }
}

impl<'a> Drop for Encryptor<'a> {
    fn drop(&mut self) {
        let mut failures = vec![];

        for file in self.decrypted_files.iter() {
            log_wrap!(&format!("Removing unencrypted file {:?}", file), {
                if let Err(error) = remove_file(file) {
                    match error.kind() {
                        ErrorKind::NotFound => {},
                        _ => failures.push(error),
                    }
                }
            });
        }

        if !failures.is_empty() {
            panic!(
                "Failed to remove one or more encrypted files! You should remove these files \
                manually if they are present: {:?}",
                failures,
            );
        }
    }
}
