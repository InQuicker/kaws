variable "coreos_ami" {
  description = "The AMI ID for the CoreOS image to use for servers, e.g. `ami-1234abcd`"
}

variable "cluster" {
  description = "The target cluster's name"
}

variable "domain" {
  description = "The domain name for the cluster"
}

variable "etcd_01_initial_cluster_state" {
  description = "The initial cluster state for the first etcd node. One of `new` or `existing`"
}

variable "etcd_02_initial_cluster_state" {
  description = "The initial cluster state for the second etcd node. One of `new` or `existing`"
}

variable "etcd_03_initial_cluster_state" {
  description = "The initial cluster state for the third etcd node. One of `new` or `existing`"
}

variable "instance_size" {
  description = "The EC2 instance size"
}

variable "ssh_key" {
  description = "Name of the SSH key in AWS that should have acccess to EC2 instances"
}

variable "version" {
  description = "Version of Kubernetes to use, e.g. 1.0.0"
}

variable "zone_id" {
  description = "Zone ID of the Route 53 hosted zone"
}
