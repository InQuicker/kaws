output "domain" {
  value = "${var.domain}"
}

output "kubernetes_security_group_id" {
  value = "${aws_security_group.kubernetes.id}"
}

output "region" {
  value = "${var.region}"
}

output "vpc_id" {
  value = "${aws_vpc.kubernetes.id}"
}
