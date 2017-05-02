use std::fs::{create_dir_all, File};
use std::io::Write;

use clap::ArgMatches;
use rusoto::ChainProvider;

use aws::credentials_provider;
use encryption::Encryptor;
use error::KawsResult;
use pki::CertificateAuthority;

pub struct Cluster<'a> {
    name: &'a str,
    region: &'a str,
}

pub struct ExistingCluster<'a> {
    aws_credentials_provider: ChainProvider,
    cluster: Cluster<'a>,
    domain: Option<&'a str>,
    kms_master_key_id: &'a str,
    subject: &'a str,
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

    fn etcd_ca_cert_path(&self) -> String {
        format!("clusters/{}/etcd-ca.pem", self.name)
    }

    fn etcd_encrypted_ca_key_path(&self) -> String {
        format!("clusters/{}/etcd-ca-key-encrypted.base64", self.name)
    }

    fn etcd_server_cert_path(&self) -> String {
        format!("clusters/{}/etcd-server.pem", self.name)
    }

    fn etcd_encrypted_server_key_path(&self) -> String {
        format!("clusters/{}/etcd-server-key-encrypted.base64", self.name)
    }

    fn etcd_client_cert_path(&self) -> String {
        format!("clusters/{}/etcd-client.pem", self.name)
    }

    fn etcd_encrypted_client_key_path(&self) -> String {
        format!("clusters/{}/etcd-client-key-encrypted.base64", self.name)
    }

    fn etcd_peer_ca_cert_path(&self) -> String {
        format!("clusters/{}/etcd-peer-ca.pem", self.name)
    }

    fn etcd_peer_encrypted_ca_key_path(&self) -> String {
        format!("clusters/{}/etcd-peer-ca-key-encrypted.base64", self.name)
    }

    fn etcd_peer_cert_path(&self) -> String {
        format!("clusters/{}/etcd-peer.pem", self.name)
    }

    fn etcd_peer_encrypted_key_path(&self) -> String {
        format!("clusters/{}/etcd-peer-key-encrypted.base64", self.name)
    }

    fn k8s_ca_cert_path(&self) -> String {
        format!("clusters/{}/k8s-ca.pem", self.name)
    }

    fn k8s_encrypted_ca_key_path(&self) -> String {
        format!("clusters/{}/k8s-ca-key-encrypted.base64", self.name)
    }

    fn k8s_encrypted_master_key_path(&self) -> String {
        format!("clusters/{}/k8s-master-key-encrypted.base64", self.name)
    }

    fn k8s_encrypted_node_key_path(&self) -> String {
        format!("clusters/{}/k8s-node-key-encrypted.base64", self.name)
    }

    fn gitignore_path(&self) -> String {
        format!("clusters/{}/.gitignore", self.name)
    }

    fn k8s_master_cert_path(&self) -> String {
        format!("clusters/{}/k8s-master.pem", self.name)
    }

    fn name(&self) -> &str {
        self.name
    }

    fn k8s_node_cert_path(&self) -> String {
        format!("clusters/{}/k8s-node.pem", self.name)
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
            domain: matches.value_of("domain"),
            kms_master_key_id: matches.value_of("kms-key").expect("missing kms-key"),
            subject: matches.value_of("subject").unwrap_or("ca"),
        }
    }

    pub fn generate_pki_all(&mut self) -> KawsResult {
        self.generate_etcd_pki()?;
        self.generate_etcd_peer_pki()?;
        self.generate_kubernetes_pki()?;

        Ok(None)
    }

    pub fn generate_etcd_pki(&self) -> KawsResult {
        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            self.cluster.region().parse()?,
            Some(self.kms_master_key_id),
        );

        let ca = if self.subject == "ca" {
            let ca = CertificateAuthority::generate(
                &format!("kaws-etcd-ca-{}", self.cluster.name)
            )?;

            ca.write_to_files(
                &mut encryptor,
                &self.cluster.etcd_ca_cert_path(),
                &self.cluster.etcd_encrypted_ca_key_path(),
            )?;

            ca
        } else {
            CertificateAuthority::from_files(
                &mut encryptor,
                &self.cluster.etcd_ca_cert_path(),
                &self.cluster.etcd_encrypted_ca_key_path(),
            )?
        };

        if self.subject == "ca" || self.subject == "server" {
            let (server_cert, server_key) = ca.generate_cert(
                &format!("kaws-etcd-server-{}", self.cluster.name),
                Some(&[
                    "10.0.1.4",
                    "10.0.1.5",
                    "10.0.1.6",
                ]),
                None,
            )?;

            server_cert.write_to_file(&self.cluster.etcd_server_cert_path())?;
            server_key.write_to_file(
                &mut encryptor,
                &self.cluster.etcd_encrypted_server_key_path(),
            )?;
        }

        if self.subject == "ca" || self.subject == "client" {
            let (client_cert, client_key) = ca.generate_cert(
                &format!("kaws-etcd-client-{}", self.cluster.name),
                None,
                None,
            )?;

            client_cert.write_to_file(&self.cluster.etcd_client_cert_path())?;
            client_key.write_to_file(
                &mut encryptor,
                &self.cluster.etcd_encrypted_client_key_path(),
            )?;
        }

        Ok(None)
    }

    pub fn generate_etcd_peer_pki(&self) -> KawsResult {
        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            self.cluster.region().parse()?,
            Some(self.kms_master_key_id),
        );

        let ca = if self.subject == "ca" {
            let ca = CertificateAuthority::generate(
                &format!("kaws-etcd-peer-ca-{}", self.cluster.name)
            )?;

            ca.write_to_files(
                &mut encryptor,
                &self.cluster.etcd_peer_ca_cert_path(),
                &self.cluster.etcd_peer_encrypted_ca_key_path(),
            )?;

            ca
        } else {
            CertificateAuthority::from_files(
                &mut encryptor,
                &self.cluster.etcd_peer_ca_cert_path(),
                &self.cluster.etcd_peer_encrypted_ca_key_path(),
            )?
        };

        let (peer_cert, peer_key) = ca.generate_cert(
            &format!("kaws-etcd-peer-{}", self.cluster.name),
            Some(&[
                "10.0.1.4",
                "10.0.1.5",
                "10.0.1.6",
            ]),
            None,
        )?;

        peer_cert.write_to_file(&self.cluster.etcd_peer_cert_path())?;
        peer_key.write_to_file(
            &mut encryptor,
            &self.cluster.etcd_peer_encrypted_key_path(),
        )?;

        Ok(None)
    }

    pub fn generate_kubernetes_pki(&self) -> KawsResult {
        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            self.cluster.region().parse()?,
            Some(self.kms_master_key_id),
        );

        let ca = if self.subject == "ca" {
            let ca = CertificateAuthority::generate(
                &format!("kaws-k8s-ca-{}", self.cluster.name)
            )?;

            ca.write_to_files(
                &mut encryptor,
                &self.cluster.k8s_ca_cert_path(),
                &self.cluster.k8s_encrypted_ca_key_path(),
            )?;

            ca
        } else {
            CertificateAuthority::from_files(
                &mut encryptor,
                &self.cluster.k8s_ca_cert_path(),
                &self.cluster.k8s_encrypted_ca_key_path(),
            )?
        };

        if self.subject == "ca" || self.subject == "masters" {
            let (master_cert, master_key) = ca.generate_cert(
                &format!("kaws-k8s-master-{}", self.cluster.name),
                Some(&[
                    "kubernetes",
                    "kubernetes.default",
                    "kubernetes.default.svc",
                    "kubernetes.default.svc.cluster.local",
                    &format!("kubernetes.{}", self.domain.expect("missing domain")),
                    "10.3.0.1",
                ]),
                None,
            )?;

            master_cert.write_to_file(&self.cluster.k8s_master_cert_path())?;
            master_key.write_to_file(
                &mut encryptor,
                &self.cluster.k8s_encrypted_master_key_path(),
            )?;
        }

        if self.subject == "ca" || self.subject == "nodes" {
            let (node_cert, node_key) = ca.generate_cert(
                &format!("kaws-k8s-node-{}", self.cluster.name),
                None,
                Some(&["system:nodes"]),
            )?;

            node_cert.write_to_file(&self.cluster.k8s_node_cert_path())?;
            node_key.write_to_file(
                &mut encryptor,
                &self.cluster.k8s_encrypted_node_key_path(),
            )?;
        }

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
            ssh_keys: matches.values_of("ssh-key").expect("missing ssh-keys").collect(),
            zone_id: matches.value_of("zone-id").expect("missing zone-id"),
        }
    }

    pub fn init(&mut self) -> KawsResult {
        self.create_directories()?;
        self.create_gitignore()?;
        self.create_tfvars()?;
        self.create_pki_stubs()?;

        Ok(Some(format!(
            "Cluster \"{name}\" initialized! Commit clusters/{name} to Git.",
            name = self.cluster.name,
        )))
    }

    fn create_directories(&self) -> KawsResult {
        log_wrap!("Creating directories for the new cluster", {
            create_dir_all(format!("clusters/{}", self.cluster.name))?;
        });

        Ok(None)
    }

    fn create_gitignore(&self) -> KawsResult {
        log_wrap!("Creating .gitignore file", {
            let mut file = File::create(&self.cluster.gitignore_path())?;

            write!(file, "*-key.pem")?;
        });

        Ok(None)
    }

    fn create_tfvars(&self) -> KawsResult {
        log_wrap!("Creating tfvars file", {
            let mut file = File::create(&self.cluster.tfvars_path())?;

            write!(
                file,
                "\
kaws_account_id = \"{}\"
kaws_availability_zone = \"{}\"
kaws_cluster = \"{}\"
kaws_coreos_ami = \"{}\"
kaws_domain = \"{}\"
kaws_iam_users = [{}]
kaws_instance_size = \"{}\"
kaws_masters_max_size = \"{}\"
kaws_masters_min_size = \"{}\"
kaws_nodes_max_size = \"{}\"
kaws_nodes_min_size = \"{}\"
kaws_propagating_vgws = []
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
                self.cluster.region(),
                self.ssh_keys.iter().map(|ssh_key| {
                    format!("\"{}\"", ssh_key)
                }).collect::<Vec<String>>().join(", "),
                self.kubernetes_version,
                self.zone_id,
            )?;
        });

        Ok(None)
    }

    fn create_pki_stubs(&self) -> KawsResult {
        let paths = [
            // etcd ca
            &self.cluster.etcd_ca_cert_path(),
            &self.cluster.etcd_encrypted_ca_key_path(),

            // etcd server
            &self.cluster.etcd_server_cert_path(),
            &self.cluster.etcd_encrypted_server_key_path(),

            // etcd clients
            &self.cluster.etcd_client_cert_path(),
            &self.cluster.etcd_encrypted_client_key_path(),

            // etcd peer ca
            &self.cluster.etcd_peer_ca_cert_path(),
            &self.cluster.etcd_peer_encrypted_ca_key_path(),

            // etcd peers
            &self.cluster.etcd_peer_cert_path(),
            &self.cluster.etcd_peer_encrypted_key_path(),

            // k8s ca
            &self.cluster.k8s_ca_cert_path(),
            &self.cluster.k8s_encrypted_ca_key_path(),

            // k8s masters
            &self.cluster.k8s_master_cert_path(),
            &self.cluster.k8s_encrypted_master_key_path(),

            // k8s nodes
            &self.cluster.k8s_node_cert_path(),
            &self.cluster.k8s_encrypted_node_key_path(),
        ];

        for path in paths.iter() {
            File::create(path)?;
        }

        Ok(None)
    }
}
