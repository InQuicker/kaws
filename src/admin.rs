use std::fs::create_dir_all;
use std::process::Command;

use clap::ArgMatches;
use rusoto::ChainProvider;

use aws::credentials_provider;
use encryption::Encryptor;
use error::KawsResult;
use pki::{CertificateAuthority, CertificateSigningRequest};
use process::execute_child_process;

pub struct Admin<'a> {
    admin: &'a str,
    aws_credentials_provider: ChainProvider,
    cluster: &'a str,
    groups: Option<Vec<&'a str>>,
}

impl<'a> Admin<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        Admin {
            admin: matches.value_of("name").expect("clap should have required name"),
            aws_credentials_provider: credentials_provider(
                matches.value_of("aws-credentials-path"),
                matches.value_of("aws-credentials-profile"),
            ),
            cluster: matches.value_of("cluster").expect("clap should have required cluster"),
            groups: matches.values_of("group").map(|values| values.collect()),
        }
    }

    pub fn create(&mut self) -> KawsResult {
        log_wrap!("Creating directory for the new administrator's credentials", {
            try!(create_dir_all(format!("clusters/{}", self.cluster)));
        });

        let (csr, key) = CertificateSigningRequest::generate(self.admin, self.groups.as_ref())?;

        let csr_path = format!(
            "clusters/{}/{}-csr.pem",
            self.cluster,
            self.admin,
        );

        let key_path = format!(
            "clusters/{}/{}-key.pem",
            self.cluster,
            self.admin,
        );

        csr.write_to_file(&csr_path)?;
        key.write_to_file_unencrypted(&key_path)?;

        Ok(Some(format!(
            "Certificate signing request created! Commit changes to Git and ask an\n\
            administrator to generate your client certificate."
        )))
    }

    pub fn install(&mut self) -> KawsResult {
        let domain = try!(self.domain()).expect(
            "Terraform should have had a value for the domain output"
        );

        log_wrap!("Configuring kubectl", {
            // set cluster
            try!(execute_child_process("kubectl", &[
                "config",
                "set-cluster",
                &format!("kaws-{}", self.cluster),
                &format!("--server=https://kubernetes.{}", &domain),
                &format!("--certificate-authority=clusters/{}/k8s-ca.pem", self.cluster),
                "--embed-certs=true",
            ]));

            // set credentials
            try!(execute_child_process("kubectl", &[
                "config",
                "set-credentials",
                &format!("kaws-{}-{}", self.cluster, self.admin),
                &format!("--client-certificate=clusters/{}/{}.pem", self.cluster, self.admin),
                &format!("--client-key=clusters/{}/{}-key.pem", self.cluster, self.admin),
                "--embed-certs=true",
            ]));

            // set context
            try!(execute_child_process("kubectl", &[
                "config",
                "set-context",
                &format!("kaws-{}", self.cluster),
                &format!("--cluster=kaws-{}", self.cluster),
                &format!("--user=kaws-{}-{}", self.cluster, self.admin),
            ]));
        });

        Ok(Some(format!(
            "Admin credentials for user \"{admin}\" installed for cluster \"{cluster}\"!\n\
            To activate these settings as the current context, run:\n\n\
            kubectl config use-context kaws-{cluster}\n\n\
            If the kubectl configuration file is ever removed or changed accidentally,\n\
            just run this command again to regenerate or reconfigure it.",
            admin = self.admin,
            cluster = self.cluster,
        )))
    }

    pub fn sign(&mut self) -> KawsResult {
        let region = try!(self.region()).expect(
            "Terraform should have had a value for the region output"
        );

        let admin_csr_path = format!("clusters/{}/{}-csr.pem", self.cluster, self.admin);
        let admin_cert_path = format!("clusters/{}/{}.pem", self.cluster, self.admin);
        let ca_cert_path = format!("clusters/{}/ca.pem", self.cluster);
        let encrypted_ca_key_path = format!("clusters/{}/ca-key-encrypted.base64", self.cluster);

        let mut encryptor = Encryptor::new(
            self.aws_credentials_provider.clone(),
            region.parse()?,
            None,
        );

        let ca = CertificateAuthority::from_files(
            &mut encryptor,
            &ca_cert_path,
            &encrypted_ca_key_path,
        )?;
        let csr = CertificateSigningRequest::from_file(&admin_csr_path)?;

        let cert = ca.sign(&csr)?;

        cert.write_to_file(&admin_cert_path)?;

        Ok(Some(format!(
            "Client certificate for administrator \"{}\" created for cluster \"{}\"!\n\
            Commit changes to Git and ask the administrator to run `kaws admin install`.",
            self.admin,
            self.cluster,
        )))
    }

    fn domain(&self) -> KawsResult {
        self.output("domain")
    }

    fn region(&self) -> KawsResult {
        self.output("region")
    }

    fn output(&self, output_name: &str) -> KawsResult {
        let output = try!(
            Command::new("kaws").args(&["cluster", "output", self.cluster, output_name]).output()
        );

        Ok(Some(String::from_utf8_lossy(&output.stdout).trim_right().to_string()))
    }
}
