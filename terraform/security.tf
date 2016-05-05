resource "aws_iam_instance_profile" "k8s_master" {
  name = "kaws-k8s-master-${var.cluster}"
  roles = ["${aws_iam_role.k8s_master.name}"]
}

resource "aws_iam_instance_profile" "k8s_node" {
  name = "kaws-k8s-node-${var.cluster}"
  roles = ["${aws_iam_role.k8s_node.name}"]
}

resource "aws_iam_role" "k8s_master" {
  name = "kaws-k8s-master-${var.cluster}"
  assume_role_policy = <<EOS
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "ec2.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOS
}

resource "aws_iam_role" "k8s_node" {
  name = "kaws-k8s-node-${var.cluster}"
  assume_role_policy = <<EOS
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "ec2.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOS
}

resource "aws_iam_role_policy" "k8s_master" {
  name = "kaws-k8s-master-${var.cluster}"
  role = "${aws_iam_role.k8s_master.id}"
  policy = <<EOS
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": ["ec2:*"],
      "Resource": ["*"]
    },
    {
      "Effect": "Allow",
      "Action": ["elasticloadbalancing:*"],
      "Resource": ["*"]
    }
  ]
}
EOS
}

resource "aws_iam_role_policy" "k8s_node" {
  name = "kaws-k8s-node-${var.cluster}"
  role = "${aws_iam_role.k8s_node.id}"
  policy = <<EOS
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": "ec2:Describe*",
      "Resource": "*"
    },
    {
      "Effect": "Allow",
      "Action": "ec2:AttachVolume",
      "Resource": "*"
    },
    {
      "Effect": "Allow",
      "Action": "ec2:DetachVolume",
      "Resource": "*"
    }
  ]
}
EOS
}

resource "aws_kms_key" "pki" {
  description = "kaws ${var.cluster} Kubernetes PKI"

  policy = "${file("clusters/${var.cluster}/kms-policy.json")}"
}

resource "aws_kms_alias" "pki" {
  name = "alias/kaws-${var.cluster}"
  target_key_id = "${aws_kms_key.pki.key_id}"
}

resource "aws_security_group" "balancers" {
  name = "kaws-balancers-${var.cluster}"
  description = "Load balancers"
  vpc_id = "${aws_vpc.kubernetes.id}"

  ingress {
    from_port = 80
    to_port = 80
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port = 443
    to_port = 443
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    Name = "kaws-balancers-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}

resource "aws_security_group" "bastion" {
  name = "kaws-bastion-${var.cluster}"
  description = "Bastion for SSH access"
  vpc_id = "${aws_vpc.kubernetes.id}"

  ingress {
    from_port = 22
    to_port = 22
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    Name = "kaws-bastion-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}

resource "aws_security_group" "etcd" {
  name = "kaws-etcd-${var.cluster}"
  description = "etcd quorum"
  vpc_id = "${aws_vpc.kubernetes.id}"

  ingress {
    from_port = 22
    to_port = 22
    protocol = "tcp"
    security_groups = ["${aws_security_group.bastion.id}"]
  }

  ingress {
    from_port = "2379"
    to_port = "2380"
    protocol = "tcp"
    security_groups = ["${aws_security_group.kubernetes.id}"]
    self = true
  }

  egress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    Name = "kaws-etcd-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}

resource "aws_security_group" "kubernetes" {
  name = "kaws-k8s-${var.cluster}"
  description = "Kubernetes masters and nodes"
  vpc_id = "${aws_vpc.kubernetes.id}"

  ingress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    self = true
  }

  ingress {
    from_port = 22
    to_port = 22
    protocol = "tcp"
    security_groups = ["${aws_security_group.bastion.id}"]
  }

  ingress {
    from_port = 80
    to_port = 80
    protocol = "tcp"
    security_groups = ["${aws_security_group.balancers.id}"]
  }

  ingress {
    from_port = 8080
    to_port = 8080
    protocol = "tcp"
    security_groups = ["${aws_security_group.balancers.id}"]
  }

  ingress {
    from_port = 443
    to_port = 443
    protocol = "tcp"
    security_groups = ["${aws_security_group.balancers.id}"]
  }

  egress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    Name = "kaws-k8s-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}
