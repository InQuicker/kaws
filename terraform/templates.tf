data "template_file" "bastion_cloud_config" {
  template = "${file("${path.module}/templates/bastion_cloud_config.yml")}"

  vars {
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "etcd_01_cloud_config" {
  template = "${file("${path.module}/templates/etcd_cloud_config.yml")}"

  vars {
    initial_cluster_state = "${var.etcd_01_initial_cluster_state}"
    name = "etcd_01"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "etcd_02_cloud_config" {
  template = "${file("${path.module}/templates/etcd_cloud_config.yml")}"

  vars {
    initial_cluster_state = "${var.etcd_02_initial_cluster_state}"
    name = "etcd_02"
    ssh_public_keys = "${join(", ", var.ssh_keys)}"
  }
}

data "template_file" "etcd_03_cloud_config" {
  template = "${file("${path.module}/templates/etcd_cloud_config.yml")}"

  vars {
    initial_cluster_state = "${var.etcd_03_initial_cluster_state}"
    name = "etcd_03"
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
