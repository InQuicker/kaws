use std::process::Command;

use clap::ArgMatches;
use rusoto::{ChainProvider, ProvideAWSCredentials};

use aws::credentials_provider;
use error::{KawsError, KawsResult};

pub struct Terraform<'a> {
    aws_credentials_provider: ChainProvider,
    cluster: &'a str,
}

impl<'a> Terraform<'a> {
    pub fn new(matches: &'a ArgMatches) -> Terraform<'a> {
        Terraform {
            aws_credentials_provider: credentials_provider(
                matches.value_of("aws-credentials-path"),
                matches.value_of("aws-credentials-profile"),
            ),
            cluster: matches.value_of("cluster").expect("clap should have required cluster"),
        }
    }

    pub fn apply(&mut self) -> KawsResult {
        try!(self.get());

        try!(Command::new("terraform").args(&[
            "apply",
            "-backup=-",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
            "terraform",
        ]).env(
            "AWS_ACCESS_KEY_ID",
            self.aws_credentials_provider.credentials().expect("Failed to get AWS credentials").aws_access_key_id(),
        ).env(
            "AWS_SECRET_ACCESS_KEY",
            self.aws_credentials_provider.credentials().expect("Failed to get AWS credentials").aws_secret_access_key(),
        ).status());

        Ok(None)
    }

    pub fn destroy(&mut self) -> KawsResult {
        try!(self.get());

        let exit_status = try!(Command::new("terraform").args(&[
            "destroy",
            "-backup=-",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
            "terraform",
        ]).env(
            "AWS_ACCESS_KEY_ID",
            self.aws_credentials_provider.credentials().expect("Failed to get AWS credentials").aws_access_key_id(),
        ).env(
            "AWS_SECRET_ACCESS_KEY",
            self.aws_credentials_provider.credentials().expect("Failed to get AWS credentials").aws_secret_access_key(),
        ).status());

        if exit_status.success() {
            Ok(Some(format!(
                "Destroyed cluster \"{}\"! You should remove clusters/{} from Git.",
                self.cluster,
                self.cluster,
            )))
        } else {
            Err(KawsError::new(format!("Failed to destroy cluster!")))
        }
    }

    pub fn plan(&mut self) -> KawsResult {
        try!(self.get());

        try!(Command::new("terraform").args(&[
            "plan",
            "-backup=-",
            "-module-depth=-1",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
            "terraform",
        ]).env(
            "AWS_ACCESS_KEY_ID",
            self.aws_credentials_provider.credentials().expect("Failed to get AWS credentials").aws_access_key_id(),
        ).env(
            "AWS_SECRET_ACCESS_KEY",
            self.aws_credentials_provider.credentials().expect("Failed to get AWS credentials").aws_secret_access_key(),
        ).status());

        Ok(None)
    }

    fn get(&self) -> KawsResult {
        let exit_status = try!(Command::new("terraform").args(&[
            "get",
            "terraform",
        ]).status());

        if exit_status.success() {
            Ok(None)
        } else {
            Err(KawsError::new(format!("Failed to download Terraform module!")))
        }
    }
}
