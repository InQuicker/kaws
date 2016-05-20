resource "aws_vpc" "kubernetes" {
  enable_dns_hostnames = true
  cidr_block = "10.0.0.0/16"

  tags {
    Name = "kaws-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}
