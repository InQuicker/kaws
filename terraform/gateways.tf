resource "aws_internet_gateway" "outgoing" {
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "kaws_outgoing"
    Cluster = "${var.cluster}"
  }
}
