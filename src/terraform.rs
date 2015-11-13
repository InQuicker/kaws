use std::process::Command;

use clap::ArgMatches;

use aws::get_credentials;
use encryption::TemporaryDecryption;
use error::Error;
use log::Logger;
use error::Result;

pub struct Terraform<'a> {
    aws_credentials_path: Option<&'a str>,
    aws_credentials_profile: Option<&'a str>,
    cluster: &'a str,
    logger: Logger,
}

impl<'a> Terraform<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        Terraform {
            aws_credentials_path: matches.value_of("aws-credentials-path"),
            aws_credentials_profile: matches.value_of("aws-credentials-profile"),
            cluster: matches.value_of("cluster").expect("clap should have required cluster"),
            logger: Logger::new(matches.is_present("verbose")),
        }
    }

    pub fn apply(&self) -> Result {
        try!(self.get());

        let (aws_key, aws_secret) = try!(
            get_credentials(self.aws_credentials_path, self.aws_credentials_profile)
        );

        let encrypted_master_key_path = format!("clusters/{}/apiserver-key.pem.asc", self.cluster);
        let master_key_path = format!("clusters/{}/apiserver-key.pem", self.cluster);
        let encrypted_node_key_path = format!("clusters/{}/node-key.pem.asc", self.cluster);
        let node_key_path = format!("clusters/{}/node-key.pem", self.cluster);

        let master_key_decryption = TemporaryDecryption {
            encrypted_path: &encrypted_master_key_path,
            logger: &self.logger,
            unencrypted_path: &master_key_path,
        };
        try!(master_key_decryption.decrypt());

        let node_key_decryption = TemporaryDecryption {
            encrypted_path: &encrypted_node_key_path,
            logger: &self.logger,
            unencrypted_path: &node_key_path,
        };
        try!(node_key_decryption.decrypt());

        try!(Command::new("terraform").args(&[
            "apply",
            "-backup=-",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
            "terraform",
        ]).env("AWS_ACCESS_KEY_ID", aws_key).env("AWS_SECRET_ACCESS_KEY", aws_secret).status());

        Ok(None)
    }

    pub fn destroy(&self) -> Result {
        try!(self.get());

        let (aws_key, aws_secret) = try!(
            get_credentials(self.aws_credentials_path, self.aws_credentials_profile)
        );

        let exit_status = try!(Command::new("terraform").args(&[
            "destroy",
            "-backup=-",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
            "terraform",
        ]).env("AWS_ACCESS_KEY_ID", aws_key).env("AWS_SECRET_ACCESS_KEY", aws_secret).status());

        if exit_status.success() {
            Ok(Some(format!(
                "Destroyed cluster \"{}\"! You should remove clusters/{} from Git.",
                self.cluster,
                self.cluster,
            )))
        } else {
            Err(Error::new(format!("Failed to destroy cluster!")))
        }
    }

    pub fn plan(&self) -> Result {
        try!(self.get());

        let (aws_key, aws_secret) = try!(
            get_credentials(self.aws_credentials_path, self.aws_credentials_profile)
        );

        try!(Command::new("terraform").args(&[
            "plan",
            "-backup=-",
            "-module-depth=-1",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
            "terraform",
        ]).env("AWS_ACCESS_KEY_ID", aws_key).env("AWS_SECRET_ACCESS_KEY", aws_secret).status());

        Ok(None)
    }

    fn get(&self) -> Result {
        let exit_status = try!(Command::new("terraform").args(&[
            "get",
            "terraform",
        ]).status());

        if exit_status.success() {
            Ok(None)
        } else {
            Err(Error::new(format!("Failed to download Terraform module!")))
        }
    }
}
