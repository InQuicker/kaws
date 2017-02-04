use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

use hyper::Client;
use rusoto::ChainProvider;

use encryption::Encryptor;
use error::{KawsError, KawsResult};

pub struct Certificate {
    bytes: Vec<u8>,
}

pub struct CertificateAuthority {
    bytes: Vec<u8>,
    private_key: PrivateKey,
}

pub struct CertificateSigningRequest {
    bytes: Vec<u8>,
}

pub struct PrivateKey {
    bytes: Vec<u8>
}

impl Certificate {
    pub fn write_to_file(&self, file_path: &str) -> KawsResult {
        let mut file = File::create(file_path)?;
        file.write_all(self.as_bytes())?;

        Ok(None)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl CertificateAuthority {
    pub fn new(common_name: &str) -> Result<Self, KawsError> {
        let private_key = PrivateKey::new()?;

        let mut command = Command::new("openssl");

        command.args(&[
            "req",
            "-x509",
            "-new",
            "-nodes",
            "-key",
            "/dev/stdin",
            "-days",
            "10000",
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
            Ok(CertificateAuthority {
                bytes: output.stdout,
                private_key: private_key,
            })
        } else {
            Err(KawsError::new("Execution of `openssl req` failed.".to_owned()))
        }
    }

    pub fn sign(&self, csr: &CertificateSigningRequest) -> Result<Certificate, KawsError> {
        let mut command = Command::new("openssl");

        command.args(&[
            "x509",
            "-req",
            "-CA",
            "/dev/stdin",
            "-CAkey",
            "/dev/stdin",
            "-days",
            "3650",
            "-extensions",
            "v3_req",
            "-extfile",
            "/dev/stdin",
        ]);

        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn()?;
        Ok(Certificate {
            bytes: Vec::new(),
        })
    }

    pub fn write_to_files(
        &self,
        encryptor: &mut Encryptor<ChainProvider, Client>,
        cert_file_path: &str,
        key_file_path: &str,
    ) -> KawsResult {
        let mut cert_file = File::create(cert_file_path)?;
        cert_file.write_all(self.as_bytes())?;

        encryptor.encrypt_and_write_file(self.private_key.as_bytes(), key_file_path)?;

        Ok(None)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl CertificateSigningRequest {
    pub fn new(common_name: &str, private_key: &PrivateKey) -> Result<Self, KawsError> {
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
            Ok(CertificateSigningRequest {
                bytes: output.stdout,
            })
        } else {
            Err(KawsError::new("Execution of `openssl req` failed.".to_owned()))
        }
    }
}

impl PrivateKey {
    pub fn new() -> Result<Self, KawsError> {
        let mut command = Command::new("openssl");

        command.args(&["genrsa", "2048"]);

        let output = command.output()?;

        if output.status.success() {
            Ok(PrivateKey {
                bytes: output.stdout,
            })
        } else {
            Err(KawsError::new("Execution of `openssl genrsa` failed.".to_owned()))
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
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
