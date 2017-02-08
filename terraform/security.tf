data "aws_iam_policy_document" "kms_key" {
  statement {
    actions = [
      "kms:*",
    ]

    principals {
      type = "AWS"
      identifiers = [
        "arn:aws:iam::${var.account_id}:root",
      ]
    }

    resources = [
      "*",
    ]

    sid = "Enable IAM User Permissions"
  }

  statement {
    actions = [
        "kms:Create*",
        "kms:Describe*",
        "kms:Enable*",
        "kms:List*",
        "kms:Put*",
        "kms:Update*",
        "kms:Revoke*",
        "kms:Disable*",
        "kms:Get*",
        "kms:Delete*",
        "kms:ScheduleKeyDeletion",
        "kms:CancelKeyDeletion",
    ]

    principals {
      type = "AWS"
      identifiers = ["${formatlist("arn:aws:iam::%s:user/%s", var.account_id, var.iam_users)}"]
    }

    resources = [
      "*",
    ]

    sid = "Allow access for Key Administrators"
  }

  statement {
    actions = [
      "kms:Encrypt",
      "kms:Decrypt",
      "kms:ReEncrypt*",
      "kms:GenerateDataKey*",
      "kms:DescribeKey",
    ]

    principals {
      type = "AWS"
      identifiers = [
        "${
          concat(
            formatlist(
              "arn:aws:iam::%s:role/kaws-k8s-%s",
              var.account_id,
              list("master-${var.cluster}", "node-${var.cluster}")
            ),
            formatlist("arn:aws:iam::%s:user/%s", var.account_id, var.iam_users)
          )
        }"
      ]
    }

    resources = [
      "*",
    ]

    sid = "Allow use of the key"
  }

  statement {
    actions = [
      "kms:CreateGrant",
      "kms:ListGrants",
      "kms:RevokeGrant",
    ]

    condition {
      test = "Bool"

      values = [
        true,
      ]

      variable = "kms:GrantIsForAWSResource"
    }

    resources = [
      "*",
    ]

    sid = "Allow attachment of persistent resources"
  }
}

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
    },
    {
      "Effect": "Allow",
      "Action": [
        "ecr:GetAuthorizationToken",
        "ecr:BatchCheckLayerAvailability",
        "ecr:GetDownloadUrlForLayer",
        "ecr:GetRepositoryPolicy",
        "ecr:DescribeRepositories",
        "ecr:ListImages",
        "ecr:BatchGetImage"
      ],
      "Resource": "*"
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
    },
    {
      "Effect": "Allow",
      "Action": [
        "ecr:GetAuthorizationToken",
        "ecr:BatchCheckLayerAvailability",
        "ecr:GetDownloadUrlForLayer",
        "ecr:GetRepositoryPolicy",
        "ecr:DescribeRepositories",
        "ecr:ListImages",
        "ecr:BatchGetImage"
      ],
      "Resource": "*"
    }
  ]
}
EOS
}

resource "aws_kms_key" "pki" {
  depends_on = ["aws_iam_role.k8s_master", "aws_iam_role.k8s_node"]
  description = "kaws ${var.cluster} etcd and k8s PKI"
  policy = "${data.aws_iam_policy_document.kms_key.json}"
}

resource "aws_kms_alias" "pki" {
  name = "alias/kaws-${var.cluster}"
  target_key_id = "${aws_kms_key.pki.key_id}"
}

resource "aws_kms_key" "etcd" {
  description = "kaws ${var.cluster} EBS encryption for etcd"
}

resource "aws_kms_alias" "etcd" {
  name = "alias/kaws-${var.cluster}-etcd"
  target_key_id = "${aws_kms_key.etcd.key_id}"
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
    KubernetesCluster = "${var.cluster}"
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
    KubernetesCluster = "${var.cluster}"
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
    KubernetesCluster = "${var.cluster}"
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

  # SSH
  ingress {
    from_port = 22
    to_port = 22
    protocol = "tcp"
    security_groups = ["${aws_security_group.bastion.id}"]
  }

  # ELB health checks for masters (kube-apiserver /healthz)
  ingress {
    from_port = 8080
    to_port = 8080
    protocol = "tcp"
    security_groups = ["${aws_security_group.balancers.id}"]
  }

  # Kubernetes API
  ingress {
    from_port = 443
    to_port = 443
    protocol = "tcp"
    security_groups = ["${aws_security_group.balancers.id}"]
  }

  # ELB health checks for nodes (kube-proxy /heatlhz)
  ingress {
    from_port = 10249
    to_port = 10249
    protocol = "tcp"
    security_groups = ["${aws_security_group.balancers.id}"]
  }

  # HTTP/S exposed via ingress controller
  ingress {
    from_port = 30000
    to_port = 30001
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
    KubernetesCluster = "${var.cluster}"
  }
}
