variable "ca_cert" {
  description = "PEM-formatted X.509 certificate for the CA to be used by Kubernetes"
}

variable "cluster" {
  description = "The target cluster's name"
}

variable "coreos_ami" {
  description = "The AMI ID for the CoreOS image to use for servers, e.g. `ami-1234abcd`"
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

variable "master_cert" {
  description = "PEM-encoded X.509 certificate for kube-apiserver"
}

variable "master_key" {
  description = "Private key for kube-apiserver, encrypted by KMS and Base64-encoded"
}

variable "masters_max_size" {
  description = "The maximum number of EC2 instances the Kubernetes masters may autoscale to."
}

variable "masters_min_size" {
  description = "The minimum number of EC2 instances the Kubernetes masters may autoscale to."
}

variable "node_cert" {
  description = "PEM-encoded X.509 certificate for kubelet, encrypted by KMS and Base64-encoded"
}

variable "node_key" {
  description = "Private key for kubelet, encrypted by KMS and Base64-encoded"
}

variable "nodes_max_size" {
  description = "The maximum number of EC2 instances the Kubernetes nodes may autoscale to."
}

variable "nodes_min_size" {
  description = "The minimum number of EC2 instances the Kubernetes nodes may autoscale to."
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
