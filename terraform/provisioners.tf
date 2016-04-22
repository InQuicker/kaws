resource "null_resource" "generate_pki" {
  depends_on = ["aws_instance.etcd_01", "aws_instance.etcd_02", "aws_instance.etcd_03"]

  provisioner "local-exec" {
    command = "kaws cluster genpki ${var.cluster} --kms-key ${aws_kms_key.pki.key_id}"
  }
}

resource "null_resource" "sync_pki" {
  connection {
    user = "core"
    host = "${aws_instance.etcd_01.private_ip}"
    bastion_host = "${aws_instance.bastion.public_ip}"
  }

  depends_on = ["null_resource.generate_pki"]

  provisioner "remote-exec" {
    inline = [
      "etcdctl set /kaws/pki/ca-cert '${file("clusters/${var.cluster}/ca.pem")}'",
      "etcdctl set /kaws/pki/master-cert '${file("clusters/${var.cluster}/master.pem")}'",
      "etcdctl set /kaws/pki/master-key '${file("clusters/${var.cluster}/master-key-encrypted.base64")}'",
      "etcdctl set /kaws/pki/node-cert '${file("clusters/${var.cluster}/node.pem")}'",
      "etcdctl set /kaws/pki/node-key '${file("clusters/${var.cluster}/node-key-encrypted.base64")}'",
    ]
  }

  triggers {
    ca_cert = "${file("clusters/${var.cluster}/ca.pem")}"
    master_cert = "${file("clusters/${var.cluster}/master.pem")}"
    master_key = "${file("clusters/${var.cluster}/master-key-encrypted.base64")}"
    node_cert = "${file("clusters/${var.cluster}/node.pem")}"
    node_key = "${file("clusters/${var.cluster}/node-key-encrypted.base64")}"
  }
}

resource "null_resource" "start_kube_addons" {
  connection {
    user = "core"
    host = "${aws_instance.k8s_master_01.private_ip}"
    bastion_host = "${aws_instance.bastion.public_ip}"
  }

  depends_on = ["aws_autoscaling_group.k8s_masters"]

  provisioner "remote-exec" {
    inline = [
      "until curl --silent http://127.0.0.1:8080/version; do sleep 5; done",
      "curl --silent -X POST -d @/srv/kubernetes/kube-system.json http://127.0.0.1:8080/api/v1/namespaces",
      "curl --silent -X POST -d @/srv/kubernetes/kube-dns-rc.json http://127.0.0.1:8080/api/v1/namespaces/kube-system/replicationcontrollers",
      "curl --silent -X POST -d @/srv/kubernetes/kube-dns-svc.json http://127.0.0.1:8080/api/v1/namespaces/kube-system/services",
    ]
  }
}
