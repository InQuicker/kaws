variable "cluster" {
  description = "The target cluster's name, e.g. `production`"
}

variable "coreos_ami" {
  description = "The AMI ID for the CoreOS image to use for servers, e.g. `ami-1234abcd`"
}

variable "domain" {
  description = "The domain name for the cluster, e.g. `example.com`"
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

variables "rbac_super_user" {
  description = "The Kubernetes username of an administrator who will set up initial RBAC policies."
}

variable "region" {
  description = "The AWS Region where the cluster will live, e.g. `us-east-1`"
}

variable "ssh_key" {
  description = "Name of the SSH key in AWS that should have acccess to EC2 instances, e.g. `jimmy`"
}

variable "version" {
  description = "Version of Kubernetes to use, e.g. `1.0.0`"
}

variable "zone_id" {
  description = "Zone ID of the Route 53 hosted zone, e.g. `Z111111QQQQQQQ`"
}
