use std::fs::{File, remove_file};
use std::io::{ErrorKind, Read, Write};

use rusoto::{AwsResult, ChainProvider, Region};
use rusoto::kms::{
    DecryptRequest,
    DecryptResponse,
    EncryptRequest,
    EncryptResponse,
    KmsClient,
};
use rustc_serialize::base64::{FromBase64, STANDARD, ToBase64};

use error::{KawsError, KawsResult};

pub struct Encryptor<'a> {
    client: KmsClient<'a>,
    decrypted_files: Vec<String>,
    kms_master_key_id: &'a str,
}

impl<'a> Encryptor<'a> {
    pub fn new(
        provider: ChainProvider,
        region: &'a Region,
        kms_master_key_id: &'a str,
    ) -> Encryptor<'a> {
        Encryptor {
            client: KmsClient::new(provider, region),
            decrypted_files: vec![],
            kms_master_key_id: kms_master_key_id,
        }
    }

    pub fn decrypt<'b>(&mut self, encrypted_data: &'b str) -> AwsResult<DecryptResponse> {
        let request = DecryptRequest {
            encryption_context: None,
            grant_tokens: None,
            ciphertext_blob: encrypted_data.as_bytes().to_vec()
        };

        self.client.decrypt(&request)
    }

    pub fn decrypt_file<'b>(&mut self, source: &'b str, destination: &'b str) -> KawsResult {
        let mut src = try!(File::open(source));

        let mut encoded_data = String::new();

        try!(src.read_to_string(&mut encoded_data));

        let encrypted_data = try!(encoded_data.from_base64());
        let decrypted_data = try!(self.decrypt(&String::from_utf8_lossy(&encrypted_data)));

        let mut dst = try!(File::create(destination));

        match decrypted_data.plaintext {
            Some(ref plaintext) => try!(dst.write_all(plaintext)),
            None => return Err(KawsError::new("No plaintext was returned from KMS".to_owned())),
        }

        self.decrypted_files.push(destination.to_owned());

        Ok(None)
    }

    pub fn encrypt<'b>(&mut self, decrypted_data: &'b str) -> AwsResult<EncryptResponse> {
        let request = EncryptRequest {
            plaintext: decrypted_data.as_bytes().to_vec(),
            encryption_context: None,
            key_id: self.kms_master_key_id.to_owned(),
            grant_tokens: None,
        };

        self.client.encrypt(&request)
    }

    pub fn encrypt_file<'b>(&mut self, source: &'b str, destination: &'b str) -> KawsResult {
        let mut src = try!(File::open(source));

        let mut contents = String::new();

        try!(src.read_to_string(&mut contents));

        let encrypted_data = try!(self.encrypt(&contents));

        let mut dst = try!(File::create(destination));

        match encrypted_data.ciphertext_blob {
            Some(ref ciphertext_blob) => {
                let encoded_data = ciphertext_blob.to_base64(STANDARD);

                try!(dst.write_all(encoded_data.as_bytes()));
            }
            None => return Err(KawsError::new("No ciphertext was returned from KMS".to_owned())),
        }

        self.decrypted_files.push(source.to_owned());

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
