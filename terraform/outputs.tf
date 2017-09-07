output "domain" {
  value = "${var.domain}"
}

output "internet_gateway_id" {
  value = "${aws_internet_gateway.outgoing.id}"
}

output "kubernetes_nodes_elb_dns_name" {
  value = "${aws_elb.k8s_nodes.dns_name}"
}

output "kubernetes_nodes_elb_zone_id" {
  value = "${aws_elb.k8s_nodes.zone_id}"
}

output "kubernetes_route_table_id" {
  value = "${aws_route_table.k8s.id}"
}

output "kubernetes_security_group_id" {
  value = "${aws_security_group.kubernetes.id}"
}

output "kubernetes_subnet_id" {
  value = "${aws_subnet.k8s.id}"
}

output "main_route_table_id" {
  value = "${aws_vpc.kubernetes.main_route_table_id}"
}

output "pki_kms_key" {
  value = "${aws_kms_key.pki.key_id}"
}

output "ssh_bastion_security_group_id" {
  value = "${aws_security_group.bastion.id}"
}

output "region" {
  value = "${var.region}"
}

output "vpc_id" {
  value = "${aws_vpc.kubernetes.id}"
}
