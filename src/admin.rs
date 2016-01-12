use std::fs::{create_dir_all, remove_file};

use clap::ArgMatches;
use rusoto::credentials::DefaultAWSCredentialsProviderChain;
use rusoto::regions::Region;

use encryption::Encryptor;
use error::Result;
use process::execute_child_process;

pub struct Admin<'a> {
    aws_credentials_provider: DefaultAWSCredentialsProviderChain,
    cluster: &'a str,
    domain: Option<&'a str>,
    kms_master_key_id: &'a str,
    name: Option<&'a str>,
}

impl<'a> Admin<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        let mut provider = DefaultAWSCredentialsProviderChain::new();

        provider.set_profile(matches.value_of("aws-credentials-profile").unwrap_or("default"));

        Admin {
            aws_credentials_provider: provider,
            cluster: matches.value_of("cluster").expect("clap should have required cluster"),
            domain: matches.value_of("domain"),
            kms_master_key_id: matches.value_of("kms-key").expect("clap should have required kms-key"),
            name: matches.value_of("name"),
        }
    }

    pub fn create(&mut self) -> Result {
        let name = self.name.expect("clap should have required name");

        let admin_key_path = format!(
            "clusters/{}/{}-key.pem",
            self.cluster,
            name,
        );

        let encrypted_admin_key_path = format!("{}.asc", &admin_key_path);

        let admin_csr_path = format!(
            "clusters/{}/{}.csr",
            self.cluster,
            name,
        );

        log_wrap!("Creating directory for the new administrator's credentials", {
            try!(create_dir_all(format!("clusters/{}", self.cluster)));
        });

        // create private key
        log_wrap!("Creating Kubernetes admin private key", {
            try!(execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &admin_key_path,
                "2048",
            ]));
        });

        // create CSR
        log_wrap!("Creating Kubernetes admin certificate signing request", {
            try!(execute_child_process("openssl", &[
                "req",
                "-new",
                "-key",
                &admin_key_path,
                "-out",
                &admin_csr_path,
                "-subj",
                &format!("/CN={}-{}", name, self.cluster),
            ]));
        });

        // encrypt private key
        let region = Region::UsEast1;
        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            &region,
            self.kms_master_key_id,
        );

        log_wrap!("Encrypting Kubernetes admin private key", {
            try!(encryptor.encrypt_file(&admin_key_path, &encrypted_admin_key_path));
        });

        // Remove unencrypted private key
        log_wrap!(&format!("Removing unencrypted file {}", admin_key_path), {
            try!(remove_file(admin_key_path));
        });

        Ok(Some(format!(
            "Certificate signing request created! Commit changes to Git and ask an\n\
            administrator to generate your client certificate."
        )))
    }

    pub fn install(&mut self) -> Result {
        let domain = self.domain.expect("clap should have required domain");
        let name = self.name.expect("clap should have required name");

        let admin_key_path = format!("clusters/{}/{}-key.pem", self.cluster, name);
        let encrypted_admin_key_path = format!("{}.asc", &admin_key_path);

        let region = Region::UsEast1;
        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            &region,
            self.kms_master_key_id,
        );

        // decrypt the key
        log_wrap!("Decrypting Kubernetes admin private key", {
            try!(encryptor.decrypt_file(&encrypted_admin_key_path, &admin_key_path));
        });

        log_wrap!("Configuring kubectl", {
            // set cluster
            try!(execute_child_process("kubectl", &[
                "config",
                "set-cluster",
                self.cluster,
                &format!("--server=https://kubernetes.{}", &domain),
                &format!("--certificate-authority=clusters/{}/ca.pem", self.cluster),
                "--embed-certs=true",
            ]));

            // set credentials
            try!(execute_child_process("kubectl", &[
                "config",
                "set-credentials",
                &format!("{}-{}", name, self.cluster),
                &format!("--client-certificate=clusters/{}/{}.pem", self.cluster, name),
                &format!("--client-key=clusters/{}/{}-key.pem", self.cluster, name),
                "--embed-certs=true",
            ]));

            // set context
            try!(execute_child_process("kubectl", &[
                "config",
                "set-context",
                self.cluster,
                &format!("--cluster={}", self.cluster),
                &format!("--user={}-{}", name, self.cluster),
            ]));
        });

        Ok(Some(format!(
            "Admin credentials for user \"{}\" installed for cluster \"{}\"!\n\
            To activate these settings as the current context, run:\n\n\
            kubectl config use-context {}\n\n\
            If the kubectl configuration file is ever removed or changed accidentally,\n\
            just run this command again to regenerate or reconfigure it.",
            name,
            self.cluster,
            self.cluster,
        )))
    }

    pub fn sign(&mut self) -> Result {
        let name = self.name.expect("clap should have required name");

        let admin_csr_path = format!("clusters/{}/{}.csr", self.cluster, name);
        let admin_cert_path = format!("clusters/{}/{}.pem", self.cluster, name);
        let ca_cert_path = format!("clusters/{}/ca.pem", self.cluster);
        let ca_key_path = format!("clusters/{}/ca-key.pem", self.cluster);
        let encrypted_ca_key_path = format!("{}.asc", &ca_key_path);

        let region = Region::UsEast1;
        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            &region,
            self.kms_master_key_id,
        );

        // decrypt CA key
        try!(encryptor.decrypt_file(&encrypted_ca_key_path, &ca_key_path));

        // generate admin cert
        log_wrap!("Creating Kubernetes admin certificate", {
            try!(execute_child_process("openssl", &[
                "x509",
                "-req",
                "-in",
                &admin_csr_path,
                "-CA",
                &ca_cert_path,
                "-CAkey",
                &ca_key_path,
                "-CAcreateserial",
                "-out",
                &admin_cert_path,
                "-days",
                "365",
            ]));
        });

        Ok(Some(format!(
            "Client certificate for administrator \"{}\" created for cluster \"{}\"!\n\
            Commit changes to Git and ask the administrator to run `kaws admin install`.",
            name,
            self.cluster,
        )))
    }
}
