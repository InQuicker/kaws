use std::str::from_utf8;

use rusoto::IamProvider;
use rusoto::kms::{DecryptRequest, KmsClient};
use rustc_serialize::base64::FromBase64;

pub struct Decryptor {
    client: KmsClient<IamProvider>,
}

impl Decryptor {
    pub fn new(region_str: &str) -> Result<Decryptor, String> {
        let region = match region_str.parse() {
            Ok(region) => region,
            Err(error) => return Err(format!("{}", error)),
        };

        Ok(Decryptor {
            client: KmsClient::new(IamProvider::new(), region),
        })
    }

    pub fn decrypt<'a>(&mut self, encoded_ciphertext: &'a str) -> Result<String, String> {
        let ciphertext = match encoded_ciphertext.from_base64() {
            Ok(ciphertext) => ciphertext,
            Err(error) => return Err(error.to_string()),
        };

        let request = DecryptRequest {
            encryption_context: None,
            grant_tokens: None,
            ciphertext_blob: ciphertext,
        };

        let decryption_response = match self.client.decrypt(&request) {
            Ok(data) => data,
            Err(error) => return Err(error.to_string()),
        };

        let plaintext = match decryption_response.plaintext {
            Some(plaintext) => plaintext,
            None => return Err("No plaintext was returned from KMS".to_owned()),
        };

        match from_utf8(&plaintext) {
            Ok(utf8) => Ok(utf8.to_owned()),
            Err(error) => return Err(error.to_string()),
        }
    }
}
