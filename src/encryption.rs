use std::fs::{File, remove_file};
use std::io::{ErrorKind, Read, Write};

use hyper::Client as HyperClient;
use rusoto::{
    ChainProvider,
    DispatchSignedRequest,
    ProvideAwsCredentials,
    Region,
    default_tls_client,
};
use rusoto::kms::{
    DecryptError,
    DecryptRequest,
    DecryptResponse,
    EncryptError,
    EncryptRequest,
    EncryptResponse,
    KmsClient,
};
use rustc_serialize::base64::{FromBase64, STANDARD, ToBase64};

use error::{KawsError, KawsResult};

pub struct Encryptor<'a, P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
    client: KmsClient<P, D>,
    decrypted_files: Vec<String>,
    kms_master_key_id: Option<&'a str>,
}

impl<'a> Encryptor<'a, ChainProvider, HyperClient> {
    pub fn new(
        provider: ChainProvider,
        region: Region,
        kms_master_key_id: Option<&'a str>,
    ) -> Encryptor<'a, ChainProvider, HyperClient> {
        Encryptor {
            client: KmsClient::new(
                default_tls_client().expect("failed to create HTTP client with TLS"),
                provider,
                region,
            ),
            decrypted_files: vec![],
            kms_master_key_id: kms_master_key_id,
        }
    }

    pub fn decrypt_file_to_file<'b>(&mut self, source: &'b str, destination: &'b str)
    -> KawsResult {
        let mut src = try!(File::open(source));

        let mut encoded_data = String::new();

        try!(src.read_to_string(&mut encoded_data));

        let encrypted_data = try!(encoded_data.from_base64());
        let decrypted_data = try!(self.decrypt(encrypted_data));

        let mut dst = try!(File::create(destination));

        match decrypted_data.plaintext {
            Some(ref plaintext) => try!(dst.write_all(plaintext)),
            None => return Err(KawsError::new("No plaintext was returned from KMS".to_owned())),
        }

        self.decrypted_files.push(destination.to_owned());

        Ok(None)
    }

    pub fn encrypt_and_write_file(&mut self, data: &[u8], file_path: &str) -> KawsResult {
        let encrypted_data = self.encrypt(data.to_owned())?;
        let mut file = File::create(file_path)?;

        match encrypted_data.ciphertext_blob {
            Some(ref ciphertext_blob) => {
                let encoded_data = ciphertext_blob.to_base64(STANDARD);

                file.write_all(encoded_data.as_bytes())?;
            }
            None => return Err(KawsError::new("No ciphertext was returned from KMS".to_owned())),
        }

        Ok(None)
    }

    pub fn encrypt_file_to_file<'b>(&mut self, source: &'b str, destination: &'b str)
    -> KawsResult {
        let mut src = try!(File::open(source));

        let mut contents = vec![];

        try!(src.read_to_end(&mut contents));

        let encrypted_data = try!(self.encrypt(contents));

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

    // Private

    fn decrypt<'b>(&mut self, encrypted_data: Vec<u8>) -> Result<DecryptResponse, DecryptError> {
        let request = DecryptRequest {
            encryption_context: None,
            grant_tokens: None,
            ciphertext_blob: encrypted_data,
        };

        self.client.decrypt(&request)
    }

    fn encrypt<'b>(&mut self, decrypted_data: Vec<u8>) -> Result<EncryptResponse, EncryptError> {
        let request = EncryptRequest {
            plaintext: decrypted_data,
            encryption_context: None,
            key_id: self.kms_master_key_id.expect("KMS key must be supplied to encrypt").to_owned(),
            grant_tokens: None,
        };

        self.client.encrypt(&request)
    }

}

impl<'a, P, D> Drop for Encryptor<'a, P, D>
where P: ProvideAwsCredentials, D: DispatchSignedRequest {
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
