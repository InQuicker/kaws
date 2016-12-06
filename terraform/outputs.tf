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

output "kubernetes_security_group_id" {
  value = "${aws_security_group.kubernetes.id}"
}

output "main_route_table_id" {
  value = "${aws_vpc.kubernetes.main_route_table_id}"
}

output "pki_kms_key" {
  value = "${aws_kms_key.pki.key_id}"
}

output "region" {
  value = "${var.region}"
}

output "public_route_table_id" {
  value = "${aws_route_table.public.id}"
}

output "subnet_id" {
  value = "${aws_subnet.public.id}"
}

output "vpc_id" {
  value = "${aws_vpc.kubernetes.id}"
}
