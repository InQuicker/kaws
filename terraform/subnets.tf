resource "aws_subnet" "balancers" {
  availability_zone = "${var.availability_zone}"
  cidr_block = "10.0.0.0/24"
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "kaws-balancers-${var.cluster}"
    KubernetesCluster = "${var.cluster}"
  }
}

resource "aws_subnet" "etcd" {
  availability_zone = "${var.availability_zone}"
  cidr_block = "10.0.1.0/24"
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "kaws-etcd-${var.cluster}"
    KubernetesCluster = "${var.cluster}"
  }
}

resource "aws_subnet" "k8s" {
  availability_zone = "${var.availability_zone}"
  cidr_block = "${var.cidr}"
  vpc_id = "${aws_vpc.kubernetes.id}"

  tags {
    Name = "kaws-k8s-${var.cluster}"
    KubernetesCluster = "${var.cluster}"
  }
}
