resource "null_resource" "generate_pki" {
  depends_on = ["aws_instance.etcd_01", "aws_instance.etcd_02", "aws_instance.etcd_03"]

  provisioner "local-exec" {
    command = "kaws cluster genpki ${var.cluster} --kms-key ${aws_kms_key.pki.key_id} --region ${var.region}"
  }
}

resource "null_resource" "sync_pki" {
  connection {
    user = "core"
    host = "${aws_instance.etcd_01.private_ip}"
    bastion_host = "${aws_instance.bastion.public_ip}"
  }

  depends_on = ["null_resource.generate_pki"]

  provisioner "file" {
    source = "clusters/${var.cluster}/ca.pem"
    destionation = "/tmp/ca.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/master.pem"
    destionation = "/tmp/master.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/master-key-encrypted.base64"
    destionation = "/tmp/master-key-encrypted.base64"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/node.pem"
    destionation = "/tmp/node.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/node-key-encrypted.base64"
    destionation = "/tmp/node-key-encrypted.base64"
  }

  provisioner "remote-exec" {
    inline = [
      "etcdctl set /kaws/pki/ca.pem < /tmp/ca.pem",
      "etcdctl set /kaws/pki/master.pem < /tmp/master.pem",
      "etcdctl set /kaws/pki/master-key-encrypted.base64 < /tmp/master-key-encrypted.base64",
      "etcdctl set /kaws/pki/node.pem < /tmp/node.pem",
      "etcdctl set /kaws/pki/node-key-encrypted.base64 < /tmp/node-key-encrypted.base64",
      "rm /tmp/{ca.pem,master.pem,master-key-encrypted.base64,node.pem,node-key-encrypted.base64}",
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
