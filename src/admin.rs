use std::fs::create_dir_all;

use clap::ArgMatches;

use encryption::{import_public_keys, TemporaryDecryption};
use error::Result;
use log::Logger;
use process::execute_child_process;

pub struct Admin<'a> {
    cluster: &'a str,
    domain: Option<&'a str>,
    local_user: Option<&'a str>,
    logger: Logger,
    recipient: Option<&'a str>,
}

impl<'a> Admin<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        Admin {
            cluster: matches.value_of("cluster").expect("clap should have required cluster"),
            domain: matches.value_of("domain"),
            local_user: matches.value_of("uid"),
            logger: Logger::new(matches.is_present("verbose")),
            recipient: matches.value_of("recipient"),
        }
    }

    #[allow(unused_variables)]
    pub fn create(&mut self) -> Result {
        try!(import_public_keys(&mut self.logger));

        let local_user = self.local_user.expect("clap should have required uid");

        let admin_key_path = format!(
            "clusters/{}/{}-key.pem",
            self.cluster,
            local_user,
        );

        let encrypted_admin_key_path = format!("{}.asc", &admin_key_path);

        // This is only used for its Drop implementation to ensure the unencrypted key
        // is removed. It should be made more explicit so #[allow(unused_variables)] can
        // be removed from this method.
        let admin_key_decryption = TemporaryDecryption {
            encrypted_path: &encrypted_admin_key_path,
            logger: &self.logger,
            unencrypted_path: &admin_key_path,
        };

        let admin_csr_path = format!(
            "clusters/{}/{}.csr",
            self.cluster,
            local_user,
        );

        try!(self.logger.action("Creating directory for the new administrator's credentials", || {
            create_dir_all(format!("clusters/{}", self.cluster))
        }));

        // create private key
        try!(self.logger.action("Creating Kubernetes admin private key", || {
            execute_child_process("openssl", &[
                "genrsa",
                "-out",
                &admin_key_path,
                "2048",
            ])
        }));

        // create CSR
        try!(self.logger.action("Creating Kubernetes admin certificate signing request", || {
            execute_child_process("openssl", &[
                "req",
                "-new",
                "-key",
                &admin_key_path,
                "-out",
                &admin_csr_path,
                "-subj",
                &format!("/CN={}-{}", local_user, self.cluster),
            ])
        }));

        // encrypt private key
        try!(self.logger.action("Encrypting Kubernetes admin private key", || {
            execute_child_process("gpg2", &[
                "--encrypt",
                "--sign",
                "--local-user",
                local_user,
                "--recipient",
                local_user,
                "--output",
                &encrypted_admin_key_path,
                "--armor",
                &admin_key_path,
            ])
        }));

        Ok(Some(format!(
            "Certificate signing request created! Commit changes to Git and ask an\n\
            administrator to generate your client certificate."
        )))
    }

    pub fn install(&mut self) -> Result {
        try!(import_public_keys(&mut self.logger));

        let domain = self.domain.expect("clap should have required domain");
        let local_user = self.local_user.expect("clap should have required uid");

        let admin_key_path = format!("clusters/{}/{}-key.pem", self.cluster, local_user);
        let encrypted_admin_key_path = format!("{}.asc", &admin_key_path);

        // decrypt the key
        let admin_key_decryption = TemporaryDecryption {
            encrypted_path: &encrypted_admin_key_path,
            logger: &self.logger,
            unencrypted_path: &admin_key_path,
        };
        try!(self.logger.action("Decrypting Kubernetes admin private key", || {
            admin_key_decryption.decrypt()
        }));

        try!(self.logger.action("Configuring kubectl", || {
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
                &format!("{}-{}", local_user, self.cluster),
                &format!("--client-certificate=clusters/{}/{}.pem", self.cluster, local_user),
                &format!("--client-key=clusters/{}/{}-key.pem", self.cluster, local_user),
                "--embed-certs=true",
            ]));

            // set context
            execute_child_process("kubectl", &[
                "config",
                "set-context",
                self.cluster,
                &format!("--cluster={}", self.cluster),
                &format!("--user={}-{}", local_user, self.cluster),
            ])
        }));

        Ok(Some(format!(
            "Admin credentials for user \"{}\" installed for cluster \"{}\"!\n\
            To activate these settings as the current context, run:\n\n\
            kubectl config use-context {}\n\n\
            If the kubectl configuration file is ever removed or changed accidentally,\n\
            just run this command again to regenerate or reconfigure it.",
            local_user,
            self.cluster,
            self.cluster,
        )))
    }

    pub fn sign(&mut self) -> Result {
        try!(import_public_keys(&mut self.logger));

        let recipient = self.recipient.expect("clap should have required recipient");

        let admin_csr_path = format!("clusters/{}/{}.csr", self.cluster, recipient);
        let admin_cert_path = format!("clusters/{}/{}.pem", self.cluster, recipient);
        let ca_cert_path = format!("clusters/{}/ca.pem", self.cluster);
        let ca_key_path = format!("clusters/{}/ca-key.pem", self.cluster);
        let encrypted_ca_key_path = format!("{}.asc", &ca_key_path);

        // decrypt CA key
        let ca_key_decryption = TemporaryDecryption {
            encrypted_path: &encrypted_ca_key_path,
            logger: &self.logger,
            unencrypted_path: &ca_key_path,
        };
        try!(ca_key_decryption.decrypt());

        // generate admin cert
        try!(self.logger.action("Creating Kubernetes admin certificate", || {
            execute_child_process("openssl", &[
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
            ])
        }));

        Ok(Some(format!(
            "Client certificate for administrator \"{}\" created for cluster \"{}\"!\n\
            Commit changes to Git and ask the administrator to run `kaws admin install`.",
            recipient,
            self.cluster,
        )))
    }
}
