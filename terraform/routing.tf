resource "aws_route_table" "public" {
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "kaws-public-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}

resource "aws_route" "outgoing_traffic" {
  route_table_id = "${aws_route_table.public.id}"
  destination_cidr_block = "0.0.0.0/0"
  gateway_id = "${aws_internet_gateway.outgoing.id}"
}

resource "aws_route_table_association" "public" {
  route_table_id = "${aws_route_table.public.id}"
  subnet_id = "${aws_subnet.public.id}"
}
