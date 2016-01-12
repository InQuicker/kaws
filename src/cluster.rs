use std::fs::{create_dir_all, File, remove_file};
use std::io::Write;

use clap::ArgMatches;
use rusoto::credentials::DefaultAWSCredentialsProviderChain;
use rusoto::regions::Region;

use encryption::{Encryptor, TemporaryDecryption};
use error::Result;
use log::Logger;
use process::execute_child_process;

pub struct Cluster<'a> {
    aws_credentials_provider: DefaultAWSCredentialsProviderChain,
    ca_cert_path: String,
    ca_key_path: String,
    coreos_ami: Option<&'a str>,
    domain: Option<&'a str>,
    encrypted_ca_key_path: String,
    encrypted_master_key_path: String,
    encrypted_node_key_path: String,
    instance_size: Option<&'a str>,
    kms_master_key_id: Option<&'a str>,
    kubernetes_version: Option<&'a str>,
    logger: Logger,
    master_cert_path: String,
    master_csr_path: String,
    master_key_path: String,
    name: &'a str,
    node_cert_path: String,
    node_csr_path: String,
    node_key_path: String,
    openssl_config_path: String,
    ssh_key: Option<&'a str>,
    tfvars_path: String,
    zone_id: Option<&'a str>,
}

impl<'a> Cluster<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        let name =  matches.value_of("cluster").expect("clap should have required cluster");

        let mut provider = DefaultAWSCredentialsProviderChain::new();

        provider.set_profile(matches.value_of("aws-credentials-profile").unwrap_or("default"));


        Cluster {
            aws_credentials_provider: provider,
            ca_cert_path: format!("clusters/{}/ca.pem", name),
            ca_key_path: format!("clusters/{}/ca-key.pem", name),
            coreos_ami: matches.value_of("ami"),
            domain: matches.value_of("domain"),
            encrypted_ca_key_path: format!("clusters/{}/ca-key.pem.asc", name),
            encrypted_master_key_path: format!("clusters/{}/apiserver-key.pem.asc", name),
            encrypted_node_key_path: format!("clusters/{}/node-key.pem.asc", name),
            name: name,
            instance_size: matches.value_of("size"),
            kms_master_key_id: matches.value_of("kms-key"),
            kubernetes_version: matches.value_of("k8s-version"),
            logger: Logger::new(matches.is_present("verbose")),
            master_cert_path: format!("clusters/{}/apiserver.pem", name),
            master_csr_path: format!("clusters/{}/apiserver.csr", name),
            master_key_path: format!("clusters/{}/apiserver-key.pem", name),
            node_cert_path: format!("clusters/{}/node.pem", name),
            node_csr_path: format!("clusters/{}/node.csr", name),
            node_key_path: format!("clusters/{}/node-key.pem", name),
            openssl_config_path: format!("clusters/{}/openssl.cnf", name),
            ssh_key: matches.value_of("ssh-key"),
            tfvars_path: format!("clusters/{}/terraform.tfvars", name),
            zone_id: matches.value_of("zone-id"),
        }
    }

    pub fn init(&mut self) -> Result {
        try!(self.create_directories());
        try!(self.create_tfvars());
        try!(self.create_openssl_config());
        try!(self.create_ca());
        try!(self.create_master_credentials());
        try!(self.create_node_credentials());
        try!(self.encrypt_secrets(false));

        Ok(Some(format!(
            "Cluster \"{}\" initialized! Commit clusters/{} to Git.",
            self.name,
            self.name,
        )))
    }

    pub fn reencrypt(&mut self) -> Result {
        try!(self.encrypt_secrets(true));

        Ok(None)
    }

    fn create_directories(&self) -> Result {
        try!(self.logger.action("Creating directories for the new cluster", || {
            create_dir_all(format!("clusters/{}", self.name))
        }));

        Ok(None)
    }

    fn create_master_credentials(&self) -> Result {
        try!(self.logger.action("Creating Kubernetes master private key", || {
            execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &self.master_key_path,
                "2048",
            ])
        }));

        try!(self.logger.action("Creating Kubernetes master certificate signing request", || {
            execute_child_process("openssl", &[
                "req",
                "-new",
                "-key",
                &self.master_key_path,
                "-out",
                &self.master_csr_path,
                "-subj",
                &format!("/CN=kube-{}-apiserver", self.name),
                "-config",
                &self.openssl_config_path,
            ])
        }));

        try!(self.logger.action("Creating Kubernetes master certificate", || {
            execute_child_process("openssl", &[
                "x509",
                "-req",
                "-in",
                &self.master_csr_path,
                "-CA",
                &self.ca_cert_path,
                "-CAkey",
                &self.ca_key_path,
                "-CAcreateserial",
                "-out",
                &self.master_cert_path,
                "-days",
                "365",
                "-extensions",
                "v3_req",
                "-extfile",
                &self.openssl_config_path,
            ])
        }));

        try!(self.logger.action("Removing Kubernetes master certificate signing request", || {
            remove_file(&self.master_csr_path)
        }));

        Ok(None)
    }

    fn create_node_credentials(&self) -> Result {
        try!(self.logger.action("Creating Kubernetes node private key", || {
            execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &self.node_key_path,
                "2048",
            ])
        }));

        try!(self.logger.action("Creating Kubernetes node certificate signing request", || {
            execute_child_process("openssl", &[
                "req",
                "-new",
                "-key",
                &self.node_key_path,
                "-out",
                &self.node_csr_path,
                "-subj",
                &format!("/CN=kube-{}-node", self.name),
            ])
        }));

        try!(self.logger.action("Creating Kubernetes node certificate", || {
            execute_child_process("openssl", &[
                "x509",
                "-req",
                "-in",
                &self.node_csr_path,
                "-CA",
                &self.ca_cert_path,
                "-CAkey",
                &self.ca_key_path,
                "-CAcreateserial",
                "-out",
                &self.node_cert_path,
                "-days",
                "365",
            ])
        }));

        try!(self.logger.action("Removing Kubernetes node certificate signing request", || {
            remove_file(&self.node_csr_path)
        }));

        Ok(None)
    }

    fn create_openssl_config(&self) -> Result {
        try!(self.logger.action("Creating OpenSSL config file", || {
            let mut file = try!(File::create(&self.openssl_config_path));

            write!(
                file,
                "\
[req]
req_extensions = v3_req
distinguished_name = req_distinguished_name
[req_distinguished_name]
[v3_req]
basicConstraints = CA:FALSE
keyUsage = nonRepudiation, digitalSignature, keyEncipherment
subjectAltName = @alt_names
[alt_names]
DNS.1 = kubernetes
DNS.2 = kubernetes.default
DNS.3 = kubernetes.{}
IP.1 = 10.3.0.1
",
                self.domain.expect("domain should have been required by clap"),
            )
        }));

        Ok(None)
    }

    fn create_ca(&self) -> Result {
        try!(self.logger.action("Creating Kubernetes certificate authority private key", || {
            execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &self.ca_key_path,
                "2048",
            ])
        }));

        try!(self.logger.action("Creating Kubernetes certificate authority certificate", || {
            execute_child_process("openssl", &[
                "req",
                "-x509",
                "-new",
                "-nodes",
                "-key",
                &self.ca_key_path,
                "-days",
                "10000",
                "-out",
                &self.ca_cert_path,
                "-subj",
                &format!("/CN=kube-{}-ca", self.name),
            ])
        }));

        Ok(None)
    }

    fn create_tfvars(&self) -> Result {
        try!(self.logger.action("Creating tfvars file", || {
            let mut file = try!(File::create(&self.tfvars_path));

            write!(
                file,
                "\
domain = \"{}\"
coreos_ami = \"{}\"
cluster = \"{}\"
etcd_01_initial_cluster_state = \"new\"
etcd_02_initial_cluster_state = \"new\"
etcd_03_initial_cluster_state = \"new\"
instance_size = \"{}\"
ssh_key = \"{}\"
version = \"{}\"
zone_id = \"{}\"
",
                self.domain.expect("domain should have been required by clap"),
                self.coreos_ami.expect("AMI should have been required by clap"),
                self.name,
                self.instance_size.expect("instance size should have been required by clap"),
                self.ssh_key.expect("ssh key should have been required by clap"),
                self.kubernetes_version.expect("k8s version should have been required by clap"),
                self.zone_id.expect("zone ID should have been required by clap"),
            )
        }));

        Ok(None)
    }

    fn encrypt_secrets(&self, decrypt_existing: bool) -> Result {
        let ca_key_decryption = TemporaryDecryption {
            encrypted_path: &self.encrypted_ca_key_path,
            logger: &self.logger,
            unencrypted_path: &self.ca_key_path,
        };

        let master_key_decryption = TemporaryDecryption {
            encrypted_path: &self.encrypted_master_key_path,
            logger: &self.logger,
            unencrypted_path: &self.master_key_path,
        };

        let node_key_decryption = TemporaryDecryption {
            encrypted_path: &self.encrypted_node_key_path,
            logger: &self.logger,
            unencrypted_path: &self.node_key_path,
        };

        if decrypt_existing {
            try!(ca_key_decryption.decrypt());
            try!(master_key_decryption.decrypt());
            try!(node_key_decryption.decrypt());
        }

        let region = Region::UsEast1;

        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            &region,
            self.kms_master_key_id.expect("KMS master key ID not provided"),
        );

        // println!("Encrypting Kubernetes certificate authority private key");
        try!(encryptor.encrypt_file(&self.ca_key_path, &self.encrypted_ca_key_path));

        // println!("Encrypting Kubernetes master private key");
        try!(encryptor.encrypt_file(&self.master_key_path, &self.encrypted_master_key_path));

        // println!("Encrypting Kubernetes node private key");
        try!(encryptor.encrypt_file(&self.node_key_path, &self.encrypted_node_key_path));

        Ok(None)
    }
}
