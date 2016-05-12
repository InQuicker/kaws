output "domain" {
  value = "${var.domain}"
}

output "region" {
  value = "${var.region}"
}

output "vpc" {
  value = "${aws_vpc.kubernetes.id}"
}
