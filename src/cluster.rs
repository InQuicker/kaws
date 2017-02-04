use std::fs::{create_dir_all, File, remove_file};
use std::io::Write;

use clap::ArgMatches;
use rusoto::ChainProvider;

use aws::credentials_provider;
use encryption::Encryptor;
use error::KawsResult;
use pki::CertificateAuthority;
use process::execute_child_process;

pub struct Cluster<'a> {
    name: &'a str,
    region: &'a str,
}

pub struct ExistingCluster<'a> {
    aws_credentials_provider: ChainProvider,
    domain: &'a str,
    cluster: Cluster<'a>,
    kms_master_key_id: &'a str,
}

pub struct NewCluster<'a> {
    availability_zone: &'a str,
    aws_account_id: &'a str,
    cluster: Cluster<'a>,
    coreos_ami: &'a str,
    domain: &'a str,
    iam_users: Vec<&'a str>,
    instance_size: &'a str,
    kubernetes_version: &'a str,
    masters_max_size: &'a str,
    masters_min_size: &'a str,
    nodes_max_size: &'a str,
    nodes_min_size: &'a str,
    rbac_super_user: &'a str,
    ssh_keys: Vec<&'a str>,
    zone_id: &'a str,
}

impl<'a> Cluster<'a> {
    pub fn new(name: &'a str, region: &'a str) -> Self {
        Cluster {
            name: name,
            region: region,
        }
    }

    fn ca_cert_path(&self) -> String {
        format!("clusters/{}/ca.pem", self.name)
    }

    fn ca_key_path(&self) -> String {
        format!("clusters/{}/ca-key.pem", self.name)
    }

    fn encrypted_ca_key_path(&self) -> String {
        format!("clusters/{}/ca-key-encrypted.base64", self.name)
    }

    fn encrypted_master_key_path(&self) -> String {
        format!("clusters/{}/master-key-encrypted.base64", self.name)
    }

    fn encrypted_node_key_path(&self) -> String {
        format!("clusters/{}/node-key-encrypted.base64", self.name)
    }

    fn gitignore_path(&self) -> String {
        format!("clusters/{}/.gitignore", self.name)
    }

    fn master_cert_path(&self) -> String {
        format!("clusters/{}/master.pem", self.name)
    }

    fn master_csr_path(&self) -> String {
        format!("clusters/{}/master-csr.pem", self.name)
    }

    fn master_key_path(&self) -> String {
        format!("clusters/{}/master-key.pem", self.name)
    }

    fn name(&self) -> &str {
        self.name
    }

    fn node_cert_path(&self) -> String {
        format!("clusters/{}/node.pem", self.name)
    }

    fn node_csr_path(&self) -> String {
        format!("clusters/{}/node-csr.pem", self.name)
    }

    fn node_key_path(&self) -> String {
        format!("clusters/{}/node-key.pem", self.name)
    }

    fn openssl_config_path(&self) -> String {
        format!("clusters/{}/openssl.cnf", self.name)
    }

    fn region(&self) -> &str {
        self.region
    }

    fn tfvars_path(&self) -> String {
        format!("clusters/{}/terraform.tfvars", self.name)
    }
}

impl<'a> ExistingCluster<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        ExistingCluster {
            aws_credentials_provider: credentials_provider(
                matches.value_of("aws-credentials-path"),
                matches.value_of("aws-credentials-profile"),
            ),
            cluster: Cluster::new(
                matches.value_of("cluster").expect("missing cluster name"),
                matches.value_of("region").expect("missing region"),
            ),
            domain: matches.value_of("domain").expect("missing domain"),
            kms_master_key_id: matches.value_of("kms-key").expect("missing kms-key"),
        }
    }

    pub fn generate_pki(&mut self) -> KawsResult {
        // self.generate_etcd_pki()?;
        // self.generate_etcd_peer_pki()?;
        self.generate_k8s_pki(self.kms_master_key_id)?;

        try!(self.create_ca());
        try!(self.create_master_credentials());
        try!(self.create_node_credentials());

        try!(self.encrypt_secrets(self.kms_master_key_id));

        Ok(None)
    }

    fn generate_k8s_pki(&self, kms_key_id: &str) -> KawsResult {
        let ca = CertificateAuthority::generate(&format!("kaws-k8s-ca-{}", self.cluster.name))?;

        let (master_cert, master_key) = ca.generate_cert(
            &format!("kaws-k8s-master-{}", self.cluster.name),
            Some(&[
                "kubernetes",
                "kubernetes.default",
                "kubernetes.default.svc",
                "kubernetes.default.svc.cluster.local",
                &format!("kubernetes.{}", self.domain),
                "10.3.0.1",
            ]),
        )?;

        let (node_cert, node_key) = ca.generate_cert(
            &format!("kaws-k8s-node-{}", self.cluster.name),
            None,
        )?;

        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            self.cluster.region().parse()?,
            Some(kms_key_id),
        );

        ca.write_to_files(
            &mut encryptor,
            &format!("clusters/{}/k8s-ca.pem", self.cluster.name),
            &format!("clusters/{}/k8s-ca-key-encrypted.base64", self.cluster.name),
        )?;

        master_cert.write_to_file(&format!("clusters/{}/k8s-master.pem", self.cluster.name))?;
        master_key.write_to_file(
            &mut encryptor,
            &format!("clusters/{}/k8s-master-key-encrypted.base64", self.cluster.name),
        )?;

        node_cert.write_to_file(&format!("clusters/{}/k8s-node.pem", self.cluster.name))?;
        node_key.write_to_file(
            &mut encryptor,
            &format!("clusters/{}/k8s-node-key-encrypted.base64", self.cluster.name),
        )?;

        Ok(None)
    }

    fn create_master_credentials(&self) -> KawsResult {
        log_wrap!("Creating Kubernetes master private key", {
            try!(execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &self.cluster.master_key_path(),
                "2048",
            ]));
        });

        log_wrap!("Creating Kubernetes master certificate signing request", {
            try!(execute_child_process("openssl", &[
                "req",
                "-new",
                "-key",
                &self.cluster.master_key_path(),
                "-out",
                &self.cluster.master_csr_path(),
                "-subj",
                &format!("/CN=kaws-master"),
                "-config",
                &self.cluster.openssl_config_path(),
            ]));
        });

        log_wrap!("Creating Kubernetes master certificate", {
            try!(execute_child_process("openssl", &[
                "x509",
                "-req",
                "-in",
                &self.cluster.master_csr_path(),
                "-CA",
                &self.cluster.ca_cert_path(),
                "-CAkey",
                &self.cluster.ca_key_path(),
                "-CAcreateserial",
                "-out",
                &self.cluster.master_cert_path(),
                "-days",
                "365",
                "-extensions",
                "v3_req",
                "-extfile",
                &self.cluster.openssl_config_path(),
            ]));
        });

        log_wrap!("Removing Kubernetes master certificate signing request", {
            try!(remove_file(&self.cluster.master_csr_path()));
        });

        Ok(None)
    }

    fn create_node_credentials(&self) -> KawsResult {
        log_wrap!("Creating Kubernetes node private key", {
            try!(execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &self.cluster.node_key_path(),
                "2048",
            ]));
        });

        log_wrap!("Creating Kubernetes node certificate signing request", {
            try!(execute_child_process("openssl", &[
                "req",
                "-new",
                "-key",
                &self.cluster.node_key_path(),
                "-out",
                &self.cluster.node_csr_path(),
                "-subj",
                &format!("/CN=kaws-node"),
            ]));
        });

        log_wrap!("Creating Kubernetes node certificate", {
            try!(execute_child_process("openssl", &[
                "x509",
                "-req",
                "-in",
                &self.cluster.node_csr_path(),
                "-CA",
                &self.cluster.ca_cert_path(),
                "-CAkey",
                &self.cluster.ca_key_path(),
                "-CAcreateserial",
                "-out",
                &self.cluster.node_cert_path(),
                "-days",
                "365",
            ]));
        });

        log_wrap!("Removing Kubernetes node certificate signing request", {
            try!(remove_file(&self.cluster.node_csr_path()));
        });

        Ok(None)
    }

    fn create_ca(&self) -> KawsResult {
        log_wrap!("Creating Kubernetes certificate authority private key", {
            try!(execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &self.cluster.ca_key_path(),
                "2048",
            ]));
        });

        log_wrap!("Creating Kubernetes certificate authority certificate", {
            try!(execute_child_process("openssl", &[
                "req",
                "-x509",
                "-new",
                "-nodes",
                "-key",
                &self.cluster.ca_key_path(),
                "-days",
                "10000",
                "-out",
                &self.cluster.ca_cert_path(),
                "-subj",
                &format!("/CN=kaws-ca"),
            ]));
        });

        Ok(None)
    }

    fn encrypt_secrets<'b>(&self, kms_key_id: &'b str) -> KawsResult {
        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            try!(self.cluster.region().parse()),
            Some(kms_key_id),
        );

        log_wrap!("Encrypting Kubernetes certificate authority private key", {
            try!(encryptor.encrypt_file_to_file(
                &self.cluster.ca_key_path(),
                &self.cluster.encrypted_ca_key_path(),
            ));
        });

        log_wrap!("Encrypting Kubernetes master private key", {
            try!(encryptor.encrypt_file_to_file(
                &self.cluster.master_key_path(),
                &self.cluster.encrypted_master_key_path(),
            ));
        });

        log_wrap!("Encrypting Kubernetes node private key", {
            try!(encryptor.encrypt_file_to_file(
                &self.cluster.node_key_path(),
                &self.cluster.encrypted_node_key_path(),
            ));
        });

        Ok(None)
    }
}

impl<'a> NewCluster<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        NewCluster {
            availability_zone: matches
                .value_of("availability-zone")
                .expect("missing availability-zone"),
            aws_account_id: matches.value_of("aws-account-id").expect("missing aws-account-id"),
            cluster: Cluster::new(
                matches.value_of("cluster").expect("missing cluster name"),
                matches.value_of("region").expect("missing region"),
            ),
            coreos_ami: matches.value_of("ami").expect("missing ami"),
            domain: matches.value_of("domain").expect("missing domain"),
            iam_users: matches
                .values_of("iam-user")
                .expect("missing iam-users")
                .collect(),
            instance_size: matches.value_of("size").expect("missing instance size"),
            kubernetes_version: matches.value_of("k8s-version").expect("missing k8s-version"),
            masters_max_size: matches
                .value_of("masters-max-size")
                .expect("missing masters-max-size"),
            masters_min_size: matches
                .value_of("masters-min-size")
                .expect("missing masters-min-size"),
            nodes_max_size: matches
                .value_of("nodes-max-size")
                .expect("missing nodes-max-size"),
            nodes_min_size: matches
                .value_of("nodes-min-size")
                .expect("missing nodes-min-size"),
            rbac_super_user: matches.value_of("rbac-super-user").expect("missing rbac-super-user"),
            ssh_keys: matches.values_of("ssh-key").expect("missing ssh-keys").collect(),
            zone_id: matches.value_of("zone-id").expect("missing zone-id"),
        }
    }

    pub fn init(&mut self) -> KawsResult {
        try!(self.create_directories());
        try!(self.create_gitignore());
        try!(self.create_tfvars());
        try!(self.create_openssl_config());
        try!(self.create_pki_stubs());

        Ok(Some(format!(
            "Cluster \"{name}\" initialized! Commit clusters/{name} to Git.",
            name = self.cluster.name,
        )))
    }

    fn create_directories(&self) -> KawsResult {
        log_wrap!("Creating directories for the new cluster", {
            try!(create_dir_all(format!("clusters/{}", self.cluster.name)));
        });

        Ok(None)
    }

    fn create_gitignore(&self) -> KawsResult {
        log_wrap!("Creating .gitignore file", {
            let mut file = try!(File::create(&self.cluster.gitignore_path()));

            try!(write!(file, "*-key.pem"));
        });

        Ok(None)
    }

    fn create_tfvars(&self) -> KawsResult {
        log_wrap!("Creating tfvars file", {
            let mut file = try!(File::create(&self.cluster.tfvars_path()));

            try!(write!(
                file,
                "\
kaws_account_id = \"{}\"
kaws_availability_zone = \"{}\"
kaws_cluster = \"{}\"
kaws_coreos_ami = \"{}\"
kaws_domain = \"{}\"
kaws_etcd_01_initial_cluster_state = \"new\"
kaws_etcd_02_initial_cluster_state = \"new\"
kaws_etcd_03_initial_cluster_state = \"new\"
kaws_iam_users = [{}]
kaws_instance_size = \"{}\"
kaws_masters_max_size = \"{}\"
kaws_masters_min_size = \"{}\"
kaws_nodes_max_size = \"{}\"
kaws_nodes_min_size = \"{}\"
kaws_propagating_vgws = []
kaws_rbac_super_user = \"{}\"
kaws_region = \"{}\"
kaws_ssh_keys = [{}]
kaws_version = \"{}\"
kaws_zone_id = \"{}\"
",
                self.aws_account_id,
                self.availability_zone,
                self.cluster.name(),
                self.coreos_ami,
                self.domain,
                self.iam_users.iter().map(|iam_user| {
                    format!("\"{}\"", iam_user)
                }).collect::<Vec<String>>().join(", "),
                self.instance_size,
                self.masters_max_size,
                self.masters_min_size,
                self.nodes_max_size,
                self.nodes_min_size,
                self.rbac_super_user,
                self.cluster.region(),
                self.ssh_keys.iter().map(|ssh_key| {
                    format!("\"{}\"", ssh_key)
                }).collect::<Vec<String>>().join(", "),
                self.kubernetes_version,
                self.zone_id,
            ));
        });

        Ok(None)
    }

    fn create_openssl_config(&self) -> KawsResult {
        log_wrap!("Creating OpenSSL config file", {
            let mut file = try!(File::create(&self.cluster.openssl_config_path()));

            try!(write!(
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
                self.domain,
            ));
        });

        Ok(None)
    }

    fn create_pki_stubs(&self) -> KawsResult {
        let paths = [
            &self.cluster.ca_cert_path(),
            &self.cluster.master_cert_path(),
            &self.cluster.encrypted_master_key_path(),
            &self.cluster.node_cert_path(),
            &self.cluster.encrypted_node_key_path(),
        ];

        for path in paths.iter() {
            try!(File::create(path));
        }

        Ok(None)
    }
}
