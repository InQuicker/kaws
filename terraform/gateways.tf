resource "aws_internet_gateway" "outgoing" {
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "outgoing"
    Cluster = "${var.cluster}"
  }
}
