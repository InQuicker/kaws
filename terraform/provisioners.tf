resource "null_resource" "sync_pki" {
  connection {
    user = "core"
    host = "${aws_instance.etcd_01.private_ip}"
    bastion_host = "${aws_instance.bastion.public_ip}"
  }

  depends_on = ["aws_instance.etcd_01", "aws_instance.etcd_02", "aws_instance.etcd_03"]

  provisioner "remote-exec" {
    inline = [
      "etcdctl set /kaws/pki/ca-cert '${var.ca_cert}'",
      "etcdctl set /kaws/pki/master-cert '${var.master_cert}'",
      "etcdctl set /kaws/pki/master-key '${var.master_key}'",
      "etcdctl set /kaws/pki/node-cert '${var.node_cert}'",
      "etcdctl set /kaws/pki/node-key '${var.node_key}'",
    ]
  }

  triggers {
    ca_cert = "${var.ca_cert}"
    master_cert = "${var.master_cert}"
    master_key = "${var.master_key}"
    node_cert = "${var.node_cert}"
    node_key = "${var.node_key}"
  }
}

resource "null_resource" "start_kube_addons" {
  connection {
    user = "core"
    host = "${aws_instance.k8s_master_01.private_ip}"
    bastion_host = "${aws_instance.bastion.public_ip}"
  }

  depends_on = ["aws_instance.k8s_master_01"]

  provisioner "remote-exec" {
    inline = [
      "until curl --silent http://127.0.0.1:8080/version; do sleep 5; done",
      "curl --silent -X POST -d @/srv/kubernetes/kube-system.json http://127.0.0.1:8080/api/v1/namespaces",
      "curl --silent -X POST -d @/srv/kubernetes/kube-dns-rc.json http://127.0.0.1:8080/api/v1/namespaces/kube-system/replicationcontrollers",
      "curl --silent -X POST -d @/srv/kubernetes/kube-dns-svc.json http://127.0.0.1:8080/api/v1/namespaces/kube-system/services",
    ]
  }
}
