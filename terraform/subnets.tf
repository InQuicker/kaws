resource "aws_subnet" "public" {
  availability_zone = "${var.availability_zone}"
  cidr_block = "10.0.1.0/24"
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "kaws-public-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}
