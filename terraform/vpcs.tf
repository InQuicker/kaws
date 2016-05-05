resource "aws_vpc" "kubernetes" {
  cidr_block = "10.0.0.0/16"

  tags {
    Name = "kaws-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}
