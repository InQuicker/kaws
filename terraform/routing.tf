resource "aws_route_table" "public" {
  vpc_id = "${aws_vpc.kubernetes.id}"

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = "${aws_internet_gateway.outgoing.id}"
  }

  tags {
    Name = "kaws_public"
    Cluster = "${var.cluster}"
  }
}

resource "aws_route_table_association" "public" {
  route_table_id = "${aws_route_table.public.id}"
  subnet_id = "${aws_subnet.public.id}"
}
