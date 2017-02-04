use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

use hyper::Client;
use rusoto::ChainProvider;
use serde_json::from_slice;

use encryption::Encryptor;
use error::{KawsError, KawsResult};

pub struct Certificate(Vec<u8>);

pub struct CertificateAuthority {
    cert: Certificate,
    key: PrivateKey,
}

pub struct CertificateSigningRequest(Vec<u8>);

pub struct PrivateKey(Vec<u8>);

#[derive(Deserialize)]
struct CfsslGencertResponse {
    cert: Vec<u8>,
    key: Vec<u8>,
}

#[derive(Deserialize)]
struct CfsslSignResponse {
    cert: Vec<u8>,
}

impl Certificate {
    pub fn write_to_file(&self, file_path: &str) -> KawsResult {
        let mut file = File::create(file_path)?;
        file.write_all(self.as_bytes())?;

        Ok(None)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Certificate {
    fn from(vec: Vec<u8>) -> Self {
        Certificate(vec)
    }
}

impl CertificateAuthority {
    pub fn generate(common_name: &str) -> Result<Self, KawsError> {
        let mut command = Command::new("cfssl");

        command.args(&[
            "gencert",
            "-initca",
            "-",
        ]);

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn()?;

        match child.stdin.as_mut() {
            Some(stdin) => {
                stdin.write_all(
                    &format!(
                        r#"{{"CN":"{}","key":{{"algo":"rsa","size":2048}}}}}}"#,
                        common_name
                    ).as_bytes()
                )?;
            }
            None => {
                return Err(
                    KawsError::new("failed to acquire handle to stdin of child process".to_owned())
                );
            }
        }

        let output = child.wait_with_output()?;

        if output.status.success() {
            let raw: CfsslGencertResponse = from_slice(&output.stdout)?;
            Ok(raw.into())
        } else {
            Err(KawsError::new("Execution of `cfssl genkey` failed.".to_owned()))
        }
    }

    pub fn generate_cert(&self, common_name: &str, san: Option<&[&str]>)
    -> Result<(Certificate, PrivateKey), KawsError> {
        let mut command = Command::new("cfssl");

        command.args(&[
            "gencert",
            "-ca",
            "{}", // TODO: write cert to tempdir and include path here
            "-ca-key",
            "{}", // TODO: write key to tempdir and include path here
        ]);

        if let Some(san) = san {
            command.args(&[
                "-hostname",
                &san.join(","),
            ]);
        }

        command.arg("-");

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn()?;

        match child.stdin.as_mut() {
            Some(stdin) => {
                stdin.write_all(
                    &format!(
                        r#"{{"CN":"{}","key":{{"algo":"rsa","size":2048}}}}}}"#,
                        common_name
                    ).as_bytes()
                )?;
            }
            None => {
                return Err(
                    KawsError::new("failed to acquire handle to stdin of child process".to_owned())
                );
            }
        }

        let output = child.wait_with_output()?;

        if output.status.success() {
            let raw: CfsslGencertResponse = from_slice(&output.stdout)?;

            Ok((raw.cert.into(), raw.key.into()))
        } else {
            Err(KawsError::new("Execution of `cfssl gencert` failed.".to_owned()))
        }
    }

    pub fn sign(&self, csr: &CertificateSigningRequest) -> Result<Certificate, KawsError> {
        let mut command = Command::new("cfssl");

        command.args(&[
            "sign",
            "-ca",
            "{}", // TODO: write cert to tempdir and include path here
            "-ca-key",
            "{}", // TODO: write key to tempdir and include path here
            "-"
        ]);

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn()?;

        match child.stdin.as_mut() {
            Some(stdin) => {
                stdin.write_all(csr.as_bytes())?;
            }
            None => {
                return Err(
                    KawsError::new("failed to acquire handle to stdin of child process".to_owned())
                );
            }
        }

        let output = child.wait_with_output()?;

        if output.status.success() {
            let response: CfsslSignResponse = from_slice(&output.stdout)?;

            Ok(response.cert.into())
        } else {
            Err(KawsError::new("Execution of `cfssl gencert` failed.".to_owned()))
        }
    }

    pub fn write_to_files(
        &self,
        encryptor: &mut Encryptor<ChainProvider, Client>,
        cert_file_path: &str,
        key_file_path: &str,
    ) -> KawsResult {
        let mut cert_file = File::create(cert_file_path)?;
        cert_file.write_all(self.as_bytes())?;

        encryptor.encrypt_and_write_file(self.key.as_bytes(), key_file_path)?;

        Ok(None)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.cert.as_bytes()
    }
}

impl From<CfsslGencertResponse> for CertificateAuthority {
    fn from(raw: CfsslGencertResponse) -> Self {
        CertificateAuthority {
            cert: raw.cert.into(),
            key: raw.key.into(),
        }
    }
}

impl CertificateSigningRequest {
    pub fn generate(common_name: &str, private_key: &PrivateKey) -> Result<Self, KawsError> {
        let mut command = Command::new("openssl");

        command.args(&[
            "req",
            "-new",
            "-key",
            "/dev/stdin",
            "-subj",
            &format!("/CN={}", common_name),
        ]);

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn()?;

        match child.stdin.as_mut() {
            Some(stdin) => {
                stdin.write_all(private_key.as_bytes())?;
            }
            None => {
                return Err(
                    KawsError::new("failed to acquire handle to stdin of child process".to_owned())
                );
            }
        };

        let output = child.wait_with_output()?;

        if output.status.success() {
            Ok(CertificateSigningRequest(output.stdout))
        } else {
            Err(KawsError::new("Execution of `openssl req` failed.".to_owned()))
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for CertificateSigningRequest {
    fn from(vec: Vec<u8>) -> Self {
        CertificateSigningRequest(vec)
    }
}

impl PrivateKey {
    pub fn generate() -> Result<Self, KawsError> {
        let mut command = Command::new("openssl");

        command.args(&["genrsa", "2048"]);

        let output = command.output()?;

        if output.status.success() {
            Ok(PrivateKey(output.stdout))
        } else {
            Err(KawsError::new("Execution of `openssl genrsa` failed.".to_owned()))
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn write_to_file(
        &self,
        encryptor: &mut Encryptor<ChainProvider, Client>,
        file_path: &str,
    ) -> KawsResult {
        encryptor.encrypt_and_write_file(self.as_bytes(), file_path)?;

        Ok(None)
    }
}

impl From<Vec<u8>> for PrivateKey {
    fn from(vec: Vec<u8>) -> Self {
        PrivateKey(vec)
    }
}
