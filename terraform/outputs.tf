output "domain" {
  value = "${var.domain}"
}

output "kubernetes_nodes_elb_name" {
  value = "${aws_elb.k8s_nodes.name}"
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
