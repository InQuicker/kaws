resource "template_file" "etcd_01_cloud_config" {
  filename = "${path.module}/templates/etcd_cloud_config.yml"

  vars {
    initial_cluster_state = "${var.etcd_01_initial_cluster_state}"
    name = "etcd_01"
  }
}

resource "template_file" "etcd_02_cloud_config" {
  filename = "${path.module}/templates/etcd_cloud_config.yml"

  vars {
    initial_cluster_state = "${var.etcd_02_initial_cluster_state}"
    name = "etcd_02"
  }
}

resource "template_file" "etcd_03_cloud_config" {
  filename = "${path.module}/templates/etcd_cloud_config.yml"

  vars {
    initial_cluster_state = "${var.etcd_03_initial_cluster_state}"
    name = "etcd_03"
  }
}

resource "template_file" "master_cloud_config" {
  filename = "${path.module}/templates/master_cloud_config.yml"

  vars {
    cluster = "${var.cluster}"
    domain = "${var.domain}"
    version = "${var.version}"
  }
}

resource "template_file" "node_cloud_config" {
  filename = "${path.module}/templates/node_cloud_config.yml"

  vars {
    cluster = "${var.cluster}"
    master_ip = "kubernetes.${var.domain}"
    version = "${var.version}"
  }
}
