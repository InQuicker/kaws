resource "aws_route_table" "k8s" {
  propagating_vgws = ["${var.propagating_vgws}"]
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "kaws-k8s-${var.cluster}"
    KubernetesCluster = "${var.cluster}"
  }
}

resource "aws_route_table_association" "k8s" {
  route_table_id = "${aws_route_table.k8s.id}"
  subnet_id = "${aws_subnet.k8s.id}"
}

resource "aws_route" "k8s_outgoing_traffic" {
  route_table_id = "${aws_route_table.k8s.id}"
  destination_cidr_block = "0.0.0.0/0"
  gateway_id = "${aws_internet_gateway.outgoing.id}"
}

resource "aws_route" "main_outgoing_traffic" {
  route_table_id = "${aws_vpc.kubernetes.main_route_table_id}"
  destination_cidr_block = "0.0.0.0/0"
  gateway_id = "${aws_internet_gateway.outgoing.id}"
}
