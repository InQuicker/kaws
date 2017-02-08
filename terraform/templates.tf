data "template_file" "bastion_cloud_config" {
  template = "${file("${path.module}/templates/bastion_cloud_config.yml")}"

  vars {
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "etcd_01_cloud_config" {
  template = "${file("${path.module}/templates/etcd_cloud_config.yml")}"

  vars {
    etcd_ca_cert: "${file("clusters/${var.cluster}/etcd-ca.pem")}",
    etcd_server_cert: "${file("clusters/${var.cluster}/etcd-server.pem")}",
    etcd_server_key: "${file("clusters/${var.cluster}/etcd-server-key-encrypted.base64")}",
    etcd_peer_ca_cert: "${file("clusters/${var.cluster}/etcd-peer-ca.pem")}",
    etcd_peer_cert: "${file("clusters/${var.cluster}/etcd-peer.pem")}",
    etcd_peer_key: "${file("clusters/${var.cluster}/etcd-peer-key-encrypted.base64")}",
    name = "etcd_01"
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "etcd_02_cloud_config" {
  template = "${file("${path.module}/templates/etcd_cloud_config.yml")}"

  vars {
    name = "etcd_02"
    etcd_ca_cert: "${file("clusters/${var.cluster}/etcd-ca.pem")}",
    etcd_server_cert: "${file("clusters/${var.cluster}/etcd-server.pem")}",
    etcd_server_key: "${file("clusters/${var.cluster}/etcd-server-key-encrypted.base64")}",
    etcd_peer_ca_cert: "${file("clusters/${var.cluster}/etcd-peer-ca.pem")}",
    etcd_peer_cert: "${file("clusters/${var.cluster}/etcd-peer.pem")}",
    etcd_peer_key: "${file("clusters/${var.cluster}/etcd-peer-key-encrypted.base64")}",
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "etcd_03_cloud_config" {
  template = "${file("${path.module}/templates/etcd_cloud_config.yml")}"

  vars {
    name = "etcd_03"
    etcd_ca_cert: "${file("clusters/${var.cluster}/etcd-ca.pem")}",
    etcd_server_cert: "${file("clusters/${var.cluster}/etcd-server.pem")}",
    etcd_server_key: "${file("clusters/${var.cluster}/etcd-server-key-encrypted.base64")}",
    etcd_peer_ca_cert: "${file("clusters/${var.cluster}/etcd-peer-ca.pem")}",
    etcd_peer_cert: "${file("clusters/${var.cluster}/etcd-peer.pem")}",
    etcd_peer_key: "${file("clusters/${var.cluster}/etcd-peer-key-encrypted.base64")}",
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "master_cloud_config" {
  template = "${file("${path.module}/templates/master_cloud_config.yml")}"

  vars {
    cluster = "${var.cluster}"
    domain = "${var.domain}"
    rbac_super_user = "${var.rbac_super_user}"
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
    version = "${var.version}"
  }
}

data "template_file" "node_cloud_config" {
  template = "${file("${path.module}/templates/node_cloud_config.yml")}"

  vars {
    cluster = "${var.cluster}"
    master_ip = "kubernetes.${var.domain}"
    region = "${var.region}"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
    version = "${var.version}"
  }
}
