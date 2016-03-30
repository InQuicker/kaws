use std::str::from_utf8;

use rusoto::{IamProvider, Region};
use rusoto::kms::{DecryptRequest, KmsClient};
use rustc_serialize::base64::FromBase64;

pub struct Decryptor {
    client: KmsClient<IamProvider>,
}

impl Decryptor {
    pub fn new() -> Decryptor {
        Decryptor {
            client: KmsClient::new(IamProvider::new(), Region::UsEast1),
        }
    }

    pub fn decrypt<'a>(&mut self, encoded_ciphertext: &'a str) -> Result<String, String> {
        let request = DecryptRequest {
            encryption_context: None,
            grant_tokens: None,
            ciphertext_blob: encoded_ciphertext.as_bytes().to_vec(),
        };

        let decryption_response = match self.client.decrypt(&request) {
            Ok(data) => data,
            Err(error) => return Err(error.to_string()),
        };

        let encoded_plaintext = match decryption_response.plaintext {
            Some(plaintext) => plaintext,
            None => return Err("No plaintext was returned from KMS".to_owned()),
        };

        let plaintext_bytes = match encoded_plaintext.from_base64() {
            Ok(plaintext_bytes) => plaintext_bytes,
            Err(error) => return Err(error.to_string()),
        };

        match from_utf8(&plaintext_bytes) {
            Ok(plaintext) => Ok(plaintext.to_owned()),
            Err(error) => return Err(error.to_string()),
        }
    }
}
