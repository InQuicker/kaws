output "domain" {
  value = "${var.domain}"
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

output "region" {
  value = "${var.region}"
}

output "subnet_id" {
  value = "${aws_subnet.public.id}"
}

output "vpc_id" {
  value = "${aws_vpc.kubernetes.id}"
}
