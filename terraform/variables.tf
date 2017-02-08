variable "account_id" {
  description = "Numerical account ID of the AWS account to use, e.g. `12345678`"
}

variable "availability_zone" {
    description = "Availability Zone for etcd instances and EBS volumes, e.g. `us-east-1a`"
}

variable "cluster" {
  description = "The target cluster's name, e.g. `production`"
}

variable "coreos_ami" {
  description = "The AMI ID for the CoreOS image to use for servers, e.g. `ami-1234abcd`"
}

variable "domain" {
  description = "The domain name for the cluster, e.g. `example.com`"
}

variable "iam_users" {
  description = "A list of IAM user names who will have access to cluster PKI secrets"
  type = "list"
}

variable "instance_size" {
  description = "The EC2 instance size, e.g. `m3.medium`"
}

variable "masters_max_size" {
  description = "The maximum number of EC2 instances the Kubernetes masters may autoscale to"
}

variable "masters_min_size" {
  description = "The minimum number of EC2 instances the Kubernetes masters may autoscale to"
}

variable "nodes_max_size" {
  description = "The maximum number of EC2 instances the Kubernetes nodes may autoscale to"
}

variable "nodes_min_size" {
  description = "The minimum number of EC2 instances the Kubernetes nodes may autoscale to"
}

variable "propagating_vgws" {
  description = "A list of virtual gateways that should propagate routes to the route table"
  type = "list"
}

variable "rbac_super_user" {
  description = "The Kubernetes username of an administrator who will set up initial RBAC policies."
}

variable "region" {
  description = "The AWS Region where the cluster will live, e.g. `us-east-1`"
}

variable "ssh_keys" {
  description = "SSH public keys to add to ~/.ssh/authorized_keys on each server"
  type = "list"
}

variable "version" {
  description = "Version of Kubernetes to use, e.g. `1.0.0`"
}

variable "zone_id" {
  description = "Zone ID of the Route 53 hosted zone, e.g. `Z111111QQQQQQQ`"
}
