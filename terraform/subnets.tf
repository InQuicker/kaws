resource "aws_subnet" "public" {
  cidr_block = "10.0.1.0/24"
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "kaws_public"
    Cluster = "${var.cluster}"
  }
}
