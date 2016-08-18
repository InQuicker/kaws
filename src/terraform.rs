use std::process::{Command, Stdio};

use clap::ArgMatches;
use rusoto::{ChainProvider, ProvideAwsCredentials};

use aws::credentials_provider;
use error::{KawsError, KawsResult};

pub struct Terraform<'a> {
    aws_credentials_provider: ChainProvider,
    cluster: &'a str,
    output: Option<&'a str>,
    terraform_args: Option<Vec<&'a str>>,
}

impl<'a> Terraform<'a> {
    pub fn new(matches: &'a ArgMatches) -> Terraform<'a> {
        Terraform {
            aws_credentials_provider: credentials_provider(
                matches.value_of("aws-credentials-path"),
                matches.value_of("aws-credentials-profile"),
            ),
            cluster: matches.value_of("cluster").expect("clap should have required cluster"),
            output: matches.value_of("output"),
            terraform_args: matches.values_of("terraform-args").map(|values| values.collect()),
        }
    }

    pub fn apply(&mut self) -> KawsResult {
        try!(self.get());

        let mut command = Command::new("terraform");

        command.args(&[
            "apply",
            "-backup=-",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
        ]);

        if self.terraform_args.is_some() {
            command.args(self.terraform_args.as_ref().unwrap());
        }

        command.arg("terraform").env(
            "AWS_ACCESS_KEY_ID",
            self.aws_credentials_provider.credentials().expect(
                "Failed to get AWS credentials"
            ).aws_access_key_id(),
        ).env(
            "AWS_SECRET_ACCESS_KEY",
            self.aws_credentials_provider.credentials().expect(
                "Failed to get AWS credentials"
            ).aws_secret_access_key(),
        );

        try!(command.status());

        Ok(None)
    }

    pub fn destroy(&mut self) -> KawsResult {
        try!(self.get());

        let mut command = Command::new("terraform");

        command.args(&[
            "destroy",
            "-backup=-",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
        ]);

        if self.terraform_args.is_some() {
            command.args(self.terraform_args.as_ref().unwrap());
        }

        command.arg("terraform").env(
            "AWS_ACCESS_KEY_ID",
            self.aws_credentials_provider.credentials().expect(
                "Failed to get AWS credentials"
            ).aws_access_key_id(),
        ).env(
            "AWS_SECRET_ACCESS_KEY",
            self.aws_credentials_provider.credentials().expect(
                "Failed to get AWS credentials"
            ).aws_secret_access_key(),
        );

        let exit_status = try!(command.status());

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

    pub fn output(&mut self) -> KawsResult {
        try!(self.get());

        let mut command = Command::new("terraform");

        command.args(&[
            "output",
            "-module=kaws",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
        ]);

        if let Some(output) = self.output {
            command.arg(output);
        }

        try!(command.status());

        Ok(None)
    }

    pub fn plan(&mut self) -> KawsResult {
        try!(self.get());

        let mut command = Command::new("terraform");

        command.args(&[
            "plan",
            "-module-depth=-1",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
        ]);

        if self.terraform_args.is_some() {
            command.args(self.terraform_args.as_ref().unwrap());
        }

        command.arg("terraform").env(
            "AWS_ACCESS_KEY_ID",
            self.aws_credentials_provider.credentials().expect(
                "Failed to get AWS credentials"
            ).aws_access_key_id(),
        ).env(
            "AWS_SECRET_ACCESS_KEY",
            self.aws_credentials_provider.credentials().expect(
                "Failed to get AWS credentials"
            ).aws_secret_access_key(),
        );

        try!(command.status());

        Ok(None)
    }

    pub fn refresh(&mut self) -> KawsResult {
        try!(self.get());

        let mut command = Command::new("terraform");

        command.args(&[
            "refresh",
            "-backup=-",
            &format!("-state=clusters/{}/terraform.tfstate", self.cluster),
            &format!("-var-file=clusters/{}/terraform.tfvars", self.cluster),
        ]);

        if self.terraform_args.is_some() {
            command.args(self.terraform_args.as_ref().unwrap());
        }

        command.arg("terraform").env(
            "AWS_ACCESS_KEY_ID",
            self.aws_credentials_provider.credentials().expect(
                "Failed to get AWS credentials"
            ).aws_access_key_id(),
        ).env(
            "AWS_SECRET_ACCESS_KEY",
            self.aws_credentials_provider.credentials().expect(
                "Failed to get AWS credentials"
            ).aws_secret_access_key(),
        );

        try!(command.status());

        Ok(None)
    }

    fn get(&self) -> KawsResult {
        let exit_status = try!(Command::new("terraform").args(&[
            "get",
            "terraform",
        ]).stdout(Stdio::null()).status());

        if exit_status.success() {
            Ok(None)
        } else {
            Err(KawsError::new(format!("Failed to download Terraform module!")))
        }
    }
}
