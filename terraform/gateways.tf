resource "aws_internet_gateway" "outgoing" {
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "kaws-outgoing-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}
