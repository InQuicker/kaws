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
        try!(create_dir_all(format!("{}/clusters", self.name)));
        try!(create_dir_all(format!("{}/terraform", self.name)));

        let mut gitignore = try!(File::create(format!("{}/.gitignore", self.name)));
        try!(writeln!(&mut gitignore, ".terraform"));

        let mut main_tf = try!(File::create(format!("{}/terraform/main.tf", self.name)));
        try!(write!(
            &mut main_tf,
r#"module "kaws" {{
    source = "{}"

    ca_cert = "${{file("clusters/${{var.cluster}}/ca.pem")}}"
    cluster = "${{var.cluster}}"
    coreos_ami = "${{var.coreos_ami}}"
    domain = "${{var.domain}}"
    etcd_01_initial_cluster_state = "${{var.etcd_01_initial_cluster_state}}"
    etcd_02_initial_cluster_state = "${{var.etcd_02_initial_cluster_state}}"
    etcd_03_initial_cluster_state = "${{var.etcd_03_initial_cluster_state}}"
    instance_size = "${{var.instance_size}}"
    master_cert = "${{file("clusters/${{var.cluster}}/master.pem")}}"
    master_key = "${{file("clusters/${{var.cluster}}/master-key-encrypted.base64")}}"
    node_cert = "${{file("clusters/${{var.cluster}}/node.pem")}}"
    node_key = "${{file("clusters/${{var.cluster}}/node-key-encrypted.base64")}}"
    ssh_key = "${{var.ssh_key}}"
    version = "${{var.version}}"
    zone_id = "${{var.zone_id}}"
}}

variable "coreos_ami" {{
  description = "The AMI ID for the CoreOS image to use for servers, e.g. `ami-1234abcd`"
}}

variable "cluster" {{
  description = "The target cluster's name"
}}

variable "domain" {{
  description = "The domain name for the cluster"
}}

variable "etcd_01_initial_cluster_state" {{
  description = "The initial cluster state for the first etcd node. One of `new` or `existing`"
}}

variable "etcd_02_initial_cluster_state" {{
  description = "The initial cluster state for the second etcd node. One of `new` or `existing`"
}}

variable "etcd_03_initial_cluster_state" {{
  description = "The initial cluster state for the third etcd node. One of `new` or `existing`"
}}

variable "instance_size" {{
  description = "The EC2 instance size"
}}

variable "ssh_key" {{
  description = "Name of the SSH key in AWS that should have acccess to EC2 instances"
}}

variable "version" {{
  description = "Version of Kubernetes to use, e.g. 1.0.0"
}}

variable "zone_id" {{
  description = "Zone ID of the Route 53 hosted zone"
}}
"#,
            self.terraform_source,
        ));

        Ok(Some(format!("New repository \"{}\" created!", self.name)))
    }
}
