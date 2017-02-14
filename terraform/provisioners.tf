resource "null_resource" "generate_pki" {
  depends_on = ["aws_kms_key.pki"]

  provisioner "local-exec" {
    command = "kaws cluster genpki ${var.cluster} --domain ${var.domain} --kms-key ${aws_kms_key.pki.key_id} --region ${var.region}"
  }

  triggers {
    etcd_ca = "${file("clusters/${var.cluster}/etcd-ca.pem")}"
    etcd_server_cert = "${file("clusters/${var.cluster}/etcd-server.pem")}"
    etcd_client_cert = "${file("clusters/${var.cluster}/etcd-client.pem")}"
    etcd_server_key = "${file("clusters/${var.cluster}/etcd-server-key-encrypted.base64")}"
    etcd_client_key = "${file("clusters/${var.cluster}/etcd-client-key-encrypted.base64")}"
    etcd_peer_ca = "${file("clusters/${var.cluster}/etcd-peer-ca.pem")}"
    etcd_peer_cert = "${file("clusters/${var.cluster}/etcd-peer.pem")}"
    etcd_peer_key = "${file("clusters/${var.cluster}/etcd-peer-key-encrypted.base64")}"
    k8s_ca = "${file("clusters/${var.cluster}/k8s-ca.pem")}"
    k8s_master_cert = "${file("clusters/${var.cluster}/k8s-master.pem")}"
    k8s_master_key = "${file("clusters/${var.cluster}/k8s-master-key-encrypted.base64")}"
    k8s_node_cert = "${file("clusters/${var.cluster}/k8s-node.pem")}"
    k8s_node_key = "${file("clusters/${var.cluster}/k8s-node-key-encrypted.base64")}"
  }
}
