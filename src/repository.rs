use std::fs::{create_dir_all, File};
use std::io::Write;

use clap::ArgMatches;

use error::KawsResult;

pub struct Repository<'a> {
    name: &'a str,
    terraform_source: &'a str,
}

impl<'a> Repository<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        Repository {
            name: matches.value_of("name").expect("clap should have required name"),
            terraform_source: matches.value_of("terraform-source").unwrap_or(
                concat!("github.com/InQuicker/kaws//terraform?ref=v", env!("CARGO_PKG_VERSION")),
            ),
        }
    }

    pub fn create(&self) -> KawsResult {
        create_dir_all(format!("{}/clusters", self.name))?;
        create_dir_all(format!("{}/terraform", self.name))?;

        let mut gitignore = File::create(format!("{}/.gitignore", self.name))?;
        writeln!(&mut gitignore, ".terraform")?;

        let mut main_tf = File::create(format!("{}/terraform/kaws.tf", self.name))?;
        write!(
            &mut main_tf,
r#"module "kaws" {{
    source = "{}"

    account_id = "${{var.kaws_account_id}}"
    availability_zone = "${{var.kaws_availability_zone}}"
    cluster = "${{var.kaws_cluster}}"
    coreos_ami = "${{var.kaws_coreos_ami}}"
    domain = "${{var.kaws_domain}}"
    etcd_01_initial_cluster_state = "${{var.kaws_etcd_01_initial_cluster_state}}"
    etcd_02_initial_cluster_state = "${{var.kaws_etcd_02_initial_cluster_state}}"
    etcd_03_initial_cluster_state = "${{var.kaws_etcd_03_initial_cluster_state}}"
    iam_users = ["${{var.kaws_iam_users}}"]
    instance_size = "${{var.kaws_instance_size}}"
    masters_max_size = "${{var.kaws_masters_max_size}}"
    masters_min_size = "${{var.kaws_masters_min_size}}"
    nodes_max_size = "${{var.kaws_nodes_max_size}}"
    nodes_min_size = "${{var.kaws_nodes_min_size}}"
    propagating_vgws = ["${{var.kaws_propagating_vgws}}"]
    rbac_super_user = "${{var.kaws_rbac_super_user}}"
    region = "${{var.kaws_region}}"
    ssh_keys = ["${{var.kaws_ssh_keys}}"]
    version = "${{var.kaws_version}}"
    zone_id = "${{var.kaws_zone_id}}"
}}

variable "kaws_account_id" {{
  description = "Numerical account ID of the AWS account to use, e.g. `12345678`"
}}

variable "kaws_availability_zone" {{
  description = "Availability Zone for etcd instances and EBS volumes, e.g. `us-east-1a`"
}}

variable "kaws_cluster" {{
  description = "The target cluster's name, e.g. `production`"
}}

variable "kaws_coreos_ami" {{
  description = "The AMI ID for the CoreOS image to use for servers, e.g. `ami-1234abcd`"
}}

variable "kaws_domain" {{
  description = "The domain name for the cluster, e.g. `example.com`"
}}

variable "kaws_etcd_01_initial_cluster_state" {{
  description = "The initial cluster state for the first etcd node. One of `new` or `existing`"
}}

variable "kaws_etcd_02_initial_cluster_state" {{
  description = "The initial cluster state for the second etcd node. One of `new` or `existing`"
}}

variable "kaws_etcd_03_initial_cluster_state" {{
  description = "The initial cluster state for the third etcd node. One of `new` or `existing`"
}}

variable "kaws_iam_users" {{
  description = "A list of IAM user names who will have access to cluster PKI secrets"
  type = "list"
}}

variable "kaws_instance_size" {{
  description = "The EC2 instance size, e.g. `m3.medium`"
}}

variable "kaws_masters_max_size" {{
  description = "The maximum number of EC2 instances the Kubernetes masters may autoscale to"
}}

variable "kaws_masters_min_size" {{
  description = "The minimum number of EC2 instances the Kubernetes masters may autoscale to"
}}

variable "kaws_nodes_max_size" {{
  description = "The maximum number of EC2 instances the Kubernetes nodes may autoscale to"
}}

variable "kaws_nodes_min_size" {{
  description = "The minimum number of EC2 instances the Kubernetes nodes may autoscale to"
}}

variable "kaws_propagating_vgws" {{
  description = "A list of virtual gateways that should propagate routes to the route table"
  type = "list"
}}

variable "kaws_rbac_super_user" {{
  description = "The Kubernetes username of an administrator who will set up initial RBAC policies."
}}

variable "kaws_region" {{
  description = "The AWS Region where the cluster will live, e.g. `us-east-1`"
}}

variable "kaws_ssh_keys" {{
  description = "SSH public keys to add to ~/.ssh/authorized_keys on each server"
  type = "list"
}}

variable "kaws_version" {{
  description = "Version of Kubernetes to use, e.g. `1.0.0`"
}}

variable "kaws_zone_id" {{
  description = "Zone ID of the Route 53 hosted zone, e.g. `Z111111QQQQQQQ`"
}}
"#,
            self.terraform_source,
        )?;

        Ok(Some(format!("New repository \"{}\" created!", self.name)))
    }
}
