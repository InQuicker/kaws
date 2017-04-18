/*
Note: `kms_key_id` is only included as a variable below to force a dependency on the generated
TLS assets. Using Terraform's `depends_on` argument causes Terraform to incorrectly see the
aws_s3_bucket_object resources as having changed, forcing each cloud config template to change, in
turn forcing all the servers to be recreated on each `terraform apply`. It's unclear why this
happens.
*/

data template_file "user_data" {
  template = "${file("${path.module}/templates/user_data.yml")}"

  vars {
    s3_uri = "s3://${aws_s3_bucket.cloud_config.id}"
    region = "${var.region}"
  }
}

data "template_file" "bastion_cloud_config" {
  template = "${file("${path.module}/templates/bastion_cloud_config.yml")}"

  vars {
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "etcd_01_cloud_config" {
  template = "${file("${path.module}/templates/etcd_cloud_config.yml")}"

  vars {
    etcd_ca_cert = "${base64encode(file("clusters/${var.cluster}/etcd-ca.pem"))}",
    etcd_peer_ca_cert = "${base64encode(file("clusters/${var.cluster}/etcd-peer-ca.pem"))}",
    etcd_peer_cert = "${base64encode(file("clusters/${var.cluster}/etcd-peer.pem"))}",
    etcd_peer_key = "${file("clusters/${var.cluster}/etcd-peer-key-encrypted.base64")}",
    etcd_server_cert = "${base64encode(file("clusters/${var.cluster}/etcd-server.pem"))}",
    etcd_server_key = "${file("clusters/${var.cluster}/etcd-server-key-encrypted.base64")}",
    kms_key_id = "${aws_kms_key.pki.key_id}"
    name = "etcd_01"
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "etcd_02_cloud_config" {
  template = "${file("${path.module}/templates/etcd_cloud_config.yml")}"

  vars {
    etcd_ca_cert = "${base64encode(file("clusters/${var.cluster}/etcd-ca.pem"))}",
    etcd_peer_ca_cert = "${base64encode(file("clusters/${var.cluster}/etcd-peer-ca.pem"))}",
    etcd_peer_cert = "${base64encode(file("clusters/${var.cluster}/etcd-peer.pem"))}",
    etcd_peer_key = "${file("clusters/${var.cluster}/etcd-peer-key-encrypted.base64")}",
    etcd_server_cert = "${base64encode(file("clusters/${var.cluster}/etcd-server.pem"))}",
    etcd_server_key = "${file("clusters/${var.cluster}/etcd-server-key-encrypted.base64")}",
    kms_key_id = "${aws_kms_key.pki.key_id}"
    name = "etcd_02"
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "etcd_03_cloud_config" {
  template = "${file("${path.module}/templates/etcd_cloud_config.yml")}"

  vars {
    etcd_ca_cert = "${base64encode(file("clusters/${var.cluster}/etcd-ca.pem"))}",
    etcd_peer_ca_cert = "${base64encode(file("clusters/${var.cluster}/etcd-peer-ca.pem"))}",
    etcd_peer_cert = "${base64encode(file("clusters/${var.cluster}/etcd-peer.pem"))}",
    etcd_peer_key = "${file("clusters/${var.cluster}/etcd-peer-key-encrypted.base64")}",
    etcd_server_cert = "${base64encode(file("clusters/${var.cluster}/etcd-server.pem"))}",
    etcd_server_key = "${file("clusters/${var.cluster}/etcd-server-key-encrypted.base64")}",
    kms_key_id = "${aws_kms_key.pki.key_id}"
    name = "etcd_03"
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "master_cloud_config" {
  template = "${file("${path.module}/templates/master_cloud_config.yml")}"

  vars {
    cluster = "${var.cluster}"
    domain = "${var.domain}"
    etcd_ca_cert = "${base64encode(file("clusters/${var.cluster}/etcd-ca.pem"))}",
    etcd_client_cert = "${base64encode(file("clusters/${var.cluster}/etcd-client.pem"))}",
    etcd_client_key = "${file("clusters/${var.cluster}/etcd-client-key-encrypted.base64")}",
    k8s_ca_cert = "${base64encode(file("clusters/${var.cluster}/k8s-ca.pem"))}",
    k8s_master_cert = "${base64encode(file("clusters/${var.cluster}/k8s-master.pem"))}",
    k8s_master_key = "${file("clusters/${var.cluster}/k8s-master-key-encrypted.base64")}",
    kms_key_id = "${aws_kms_key.pki.key_id}"
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
    version = "${var.version}"
  }
}

data "template_file" "node_cloud_config" {
  template = "${file("${path.module}/templates/node_cloud_config.yml")}"

  vars {
    cluster = "${var.cluster}"
    etcd_ca_cert = "${base64encode(file("clusters/${var.cluster}/etcd-ca.pem"))}",
    etcd_client_cert = "${base64encode(file("clusters/${var.cluster}/etcd-client.pem"))}",
    etcd_client_key = "${file("clusters/${var.cluster}/etcd-client-key-encrypted.base64")}",
    k8s_ca_cert = "${base64encode(file("clusters/${var.cluster}/k8s-ca.pem"))}",
    k8s_node_cert = "${base64encode(file("clusters/${var.cluster}/k8s-node.pem"))}",
    k8s_node_key = "${file("clusters/${var.cluster}/k8s-node-key-encrypted.base64")}",
    kms_key_id = "${aws_kms_key.pki.key_id}"
    master_ip = "kubernetes.${var.domain}"
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
    version = "${var.version}"
  }
}
