resource "aws_vpc" "kubernetes" {
  cidr_block = "10.0.0.0/16"

  tags {
    Name = "kubernetes"
    Cluster = "${var.cluster}"
  }
}
