use std::fs::{create_dir_all, File, remove_file};
use std::io::Write;

use clap::ArgMatches;
use rusoto::ChainProvider;

use aws::credentials_provider;
use encryption::Encryptor;
use error::KawsResult;
use process::execute_child_process;

pub struct Cluster<'a> {
    aws_account_id: Option<&'a str>,
    aws_credentials_provider: ChainProvider,
    ca_cert_path: String,
    ca_key_path: String,
    coreos_ami: Option<&'a str>,
    domain: Option<&'a str>,
    encrypted_ca_key_path: String,
    encrypted_master_key_path: String,
    encrypted_node_key_path: String,
    gitignore_path: String,
    iam_users: Option<Vec<&'a str>>,
    instance_size: Option<&'a str>,
    kms_master_key_id: Option<&'a str>,
    kms_policy_path: String,
    kubernetes_version: Option<&'a str>,
    master_cert_path: String,
    master_csr_path: String,
    master_key_path: String,
    masters_max_size: Option<&'a str>,
    masters_min_size: Option<&'a str>,
    name: &'a str,
    node_cert_path: String,
    node_csr_path: String,
    node_key_path: String,
    nodes_max_size: Option<&'a str>,
    nodes_min_size: Option<&'a str>,
    openssl_config_path: String,
    region: &'a str,
    ssh_key: Option<&'a str>,
    tfvars_path: String,
    zone_id: Option<&'a str>,
}

impl<'a> Cluster<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        let name =  matches.value_of("cluster").expect("clap should have required cluster");

        Cluster {
            aws_account_id: matches.value_of("aws-account-id"),
            aws_credentials_provider: credentials_provider(
                matches.value_of("aws-credentials-path"),
                matches.value_of("aws-credentials-profile"),
            ),
            ca_cert_path: format!("clusters/{}/ca.pem", name),
            ca_key_path: format!("clusters/{}/ca-key.pem", name),
            coreos_ami: matches.value_of("ami"),
            domain: matches.value_of("domain"),
            encrypted_ca_key_path: format!("clusters/{}/ca-key-encrypted.base64", name),
            encrypted_master_key_path: format!("clusters/{}/master-key-encrypted.base64", name),
            encrypted_node_key_path: format!("clusters/{}/node-key-encrypted.base64", name),
            gitignore_path: format!("clusters/{}/.gitignore", name),
            name: name,
            iam_users: matches.values_of("iam-users").map(|values| values.collect()),
            instance_size: matches.value_of("size"),
            kms_master_key_id: matches.value_of("kms-key"),
            kms_policy_path: format!("clusters/{}/kms-policy.json", name),
            kubernetes_version: matches.value_of("k8s-version"),
            master_cert_path: format!("clusters/{}/master.pem", name),
            master_csr_path: format!("clusters/{}/master.csr", name),
            master_key_path: format!("clusters/{}/master-key.pem", name),
            masters_max_size: matches.value_of("masters-max-size"),
            masters_min_size: matches.value_of("masters-min-size"),
            node_cert_path: format!("clusters/{}/node.pem", name),
            node_csr_path: format!("clusters/{}/node.csr", name),
            node_key_path: format!("clusters/{}/node-key.pem", name),
            nodes_max_size: matches.value_of("nodes-max-size"),
            nodes_min_size: matches.value_of("nodes-min-size"),
            openssl_config_path: format!("clusters/{}/openssl.cnf", name),
            region: matches.value_of("region").expect("clap should have required region"),
            ssh_key: matches.value_of("ssh-key"),
            tfvars_path: format!("clusters/{}/terraform.tfvars", name),
            zone_id: matches.value_of("zone-id"),
        }
    }

    pub fn init(&mut self) -> KawsResult {
        try!(self.create_directories());
        try!(self.create_gitignore());
        try!(self.create_tfvars());
        try!(self.create_openssl_config());
        try!(self.create_kms_policy());
        try!(self.create_pki_stubs());

        Ok(Some(format!(
            "Cluster \"{name}\" initialized! Commit clusters/{name} to Git.",
            name = self.name,
        )))
    }

    pub fn generate_pki(&mut self) -> KawsResult {
        try!(self.create_ca());
        try!(self.create_master_credentials());
        try!(self.create_node_credentials());
        try!(self.encrypt_secrets(
            self.kms_master_key_id.expect("clap should have required kms-key"),
        ));

        Ok(None)
    }

    fn create_directories(&self) -> KawsResult {
        log_wrap!("Creating directories for the new cluster", {
            try!(create_dir_all(format!("clusters/{}", self.name)));
        });

        Ok(None)
    }

    fn create_kms_policy(&self) -> KawsResult {
        log_wrap!("Creating KMS policy file", {
            let mut file = try!(File::create(&self.kms_policy_path));

            let aws_account_id = self.aws_account_id.expect(
                "AWS account ID should have been required by clap"
            );

            let iam_user_arns = self.iam_users.as_ref().expect(
                "IAM users should have been required by clap"
            ).iter().map(|iam_user| {
                format!("\"arn:aws:iam::{}:user/{}\"", aws_account_id, iam_user)
            }).collect::<Vec<String>>().join(",");

            try!(write!(
                file,
                r#"{{
  "Version": "2012-10-17",
  "Statement": [
    {{
      "Sid": "Enable IAM User Permissions",
      "Effect": "Allow",
      "Principal": {{
        "AWS": [
          "arn:aws:iam::{aws_account_id}:root"
        ]
      }},
      "Action": "kms:*",
      "Resource": "*"
    }},
    {{
      "Sid": "Allow access for Key Administrators",
      "Effect": "Allow",
      "Principal": {{
        "AWS": [
          {iam_user_arns}
        ]
      }},
      "Action": [
        "kms:Create*",
        "kms:Describe*",
        "kms:Enable*",
        "kms:List*",
        "kms:Put*",
        "kms:Update*",
        "kms:Revoke*",
        "kms:Disable*",
        "kms:Get*",
        "kms:Delete*",
        "kms:ScheduleKeyDeletion",
        "kms:CancelKeyDeletion"
      ],
      "Resource": "*"
    }},
    {{
      "Sid": "Allow use of the key",
      "Effect": "Allow",
      "Principal": {{
        "AWS": [
          "arn:aws:iam::{aws_account_id}:role/kaws-k8s-master-{cluster}",
          "arn:aws:iam::{aws_account_id}:role/kaws-k8s-node-{cluster}",
          {iam_user_arns}
        ]
      }},
      "Action": [
        "kms:Encrypt",
        "kms:Decrypt",
        "kms:ReEncrypt*",
        "kms:GenerateDataKey*",
        "kms:DescribeKey"
      ],
      "Resource": "*"
    }},
    {{
      "Sid": "Allow attachment of persistent resources",
      "Effect": "Allow",
      "Principal": {{
        "AWS": [
          "arn:aws:iam::{aws_account_id}:role/kaws-k8s-master-{cluster}",
          "arn:aws:iam::{aws_account_id}:role/kaws-k8s-node-{cluster}",
          {iam_user_arns}
        ]
      }},
      "Action": [
        "kms:CreateGrant",
        "kms:ListGrants",
        "kms:RevokeGrant"
      ],
      "Resource": "*",
      "Condition": {{
        "Bool": {{
          "kms:GrantIsForAWSResource": true
        }}
      }}
    }}
  ]
}}
"#,
                aws_account_id = aws_account_id,
                cluster = self.name,
                iam_user_arns = iam_user_arns,
            ));
        });

        Ok(None)
    }

    fn create_master_credentials(&self) -> KawsResult {
        log_wrap!("Creating Kubernetes master private key", {
            try!(execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &self.master_key_path,
                "2048",
            ]));
        });

        log_wrap!("Creating Kubernetes master certificate signing request", {
            try!(execute_child_process("openssl", &[
                "req",
                "-new",
                "-key",
                &self.master_key_path,
                "-out",
                &self.master_csr_path,
                "-subj",
                &format!("/CN=kaws-master-{}", self.name),
                "-config",
                &self.openssl_config_path,
            ]));
        });

        log_wrap!("Creating Kubernetes master certificate", {
            try!(execute_child_process("openssl", &[
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
            ]));
        });

        log_wrap!("Removing Kubernetes master certificate signing request", {
            try!(remove_file(&self.master_csr_path));
        });

        Ok(None)
    }

    fn create_node_credentials(&self) -> KawsResult {
        log_wrap!("Creating Kubernetes node private key", {
            try!(execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &self.node_key_path,
                "2048",
            ]));
        });

        log_wrap!("Creating Kubernetes node certificate signing request", {
            try!(execute_child_process("openssl", &[
                "req",
                "-new",
                "-key",
                &self.node_key_path,
                "-out",
                &self.node_csr_path,
                "-subj",
                &format!("/CN=kaws-node-{}", self.name),
            ]));
        });

        log_wrap!("Creating Kubernetes node certificate", {
            try!(execute_child_process("openssl", &[
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
            ]));
        });

        log_wrap!("Removing Kubernetes node certificate signing request", {
            try!(remove_file(&self.node_csr_path));
        });

        Ok(None)
    }

    fn create_openssl_config(&self) -> KawsResult {
        log_wrap!("Creating OpenSSL config file", {
            let mut file = try!(File::create(&self.openssl_config_path));

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
                self.domain.expect("domain should have been required by clap"),
            ));
        });

        Ok(None)
    }

    fn create_ca(&self) -> KawsResult {
        log_wrap!("Creating Kubernetes certificate authority private key", {
            try!(execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &self.ca_key_path,
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
                &self.ca_key_path,
                "-days",
                "10000",
                "-out",
                &self.ca_cert_path,
                "-subj",
                &format!("/CN=kaws-ca-{}", self.name),
            ]));
        });

        Ok(None)
    }

    fn create_pki_stubs(&self) -> KawsResult {
        let paths = [
            &self.ca_cert_path,
            &self.master_cert_path,
            &self.encrypted_master_key_path,
            &self.node_cert_path,
            &self.encrypted_node_key_path,
        ];

        for path in paths.iter() {
            try!(File::create(path));
        }

        Ok(None)
    }

    fn create_gitignore(&self) -> KawsResult {
        log_wrap!("Creating .gitignore file", {
            let mut file = try!(File::create(&self.gitignore_path));

            try!(write!(file, "*-key.pem"));
        });

        Ok(None)
    }

    fn create_tfvars(&self) -> KawsResult {
        log_wrap!("Creating tfvars file", {
            let mut file = try!(File::create(&self.tfvars_path));

            try!(write!(
                file,
                "\
cluster = \"{}\"
coreos_ami = \"{}\"
domain = \"{}\"
etcd_01_initial_cluster_state = \"new\"
etcd_02_initial_cluster_state = \"new\"
etcd_03_initial_cluster_state = \"new\"
instance_size = \"{}\"
masters_max_size = \"{}\"
masters_min_size = \"{}\"
nodes_max_size = \"{}\"
nodes_min_size = \"{}\"
region = \"{}\"
ssh_key = \"{}\"
version = \"{}\"
zone_id = \"{}\"
",
                self.name,
                self.coreos_ami.expect("AMI should have been required by clap"),
                self.domain.expect("domain should have been required by clap"),
                self.instance_size.expect("instance size should have been required by clap"),
                self.masters_max_size.expect("masters max size should have been required by clap"),
                self.masters_min_size.expect("masters min size should have been required by clap"),
                self.nodes_max_size.expect("nodes max size should have been required by clap"),
                self.nodes_min_size.expect("nodes min size should have been required by clap"),
                self.region,
                self.ssh_key.expect("ssh key should have been required by clap"),
                self.kubernetes_version.expect("k8s version should have been required by clap"),
                self.zone_id.expect("zone ID should have been required by clap"),
            ));
        });

        Ok(None)
    }

    fn encrypt_secrets<'b>(&self, kms_key_id: &'b str) -> KawsResult {
        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            try!(self.region.parse()),
            Some(kms_key_id),
        );

        log_wrap!("Encrypting Kubernetes certificate authority private key", {
            try!(encryptor.encrypt_file(&self.ca_key_path, &self.encrypted_ca_key_path));
        });

        log_wrap!("Encrypting Kubernetes master private key", {
            try!(encryptor.encrypt_file(&self.master_key_path, &self.encrypted_master_key_path));
        });

        log_wrap!("Encrypting Kubernetes node private key", {
            try!(encryptor.encrypt_file(&self.node_key_path, &self.encrypted_node_key_path));
        });

        Ok(None)
    }
}
