resource "aws_instance" "bastion" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  iam_instance_profile = "${aws_iam_instance_profile.bastion.name}"
  instance_type = "t2.micro"
  subnet_id = "${aws_subnet.k8s.id}"
  user_data = "${replace("${data.template_file.user_data.rendered}", "__FILE__", "bastion_cloud_config.yml")}"
  vpc_security_group_ids = ["${aws_security_group.bastion.id}"]

  tags {
    Name = "kaws-bastion-${var.cluster}"
    KubernetesCluster = "${var.cluster}"
  }
}

resource "aws_instance" "etcd_01" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  availability_zone = "${var.availability_zone}"
  iam_instance_profile = "${aws_iam_instance_profile.etcd.name}"
  instance_type = "${var.instance_size}"
  private_ip = "10.0.1.4"
  subnet_id = "${aws_subnet.etcd.id}"
  user_data = "${replace("${data.template_file.user_data.rendered}", "__FILE__", "etcd_01_cloud_config.yml")}"
  vpc_security_group_ids = ["${aws_security_group.etcd.id}"]

  tags {
    Name = "kaws-etcd-${var.cluster}-01"
    KubernetesCluster = "${var.cluster}"
  }
}

resource "aws_instance" "etcd_02" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  availability_zone = "${var.availability_zone}"
  iam_instance_profile = "${aws_iam_instance_profile.etcd.name}"
  instance_type = "${var.instance_size}"
  private_ip = "10.0.1.5"
  subnet_id = "${aws_subnet.etcd.id}"
  user_data = "${replace("${data.template_file.user_data.rendered}", "__FILE__", "etcd_02_cloud_config.yml")}"
  vpc_security_group_ids = ["${aws_security_group.etcd.id}"]

  tags {
    Name = "kaws-etcd-${var.cluster}-02"
    KubernetesCluster = "${var.cluster}"
  }
}

resource "aws_instance" "etcd_03" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  availability_zone = "${var.availability_zone}"
  iam_instance_profile = "${aws_iam_instance_profile.etcd.name}"
  instance_type = "${var.instance_size}"
  private_ip = "10.0.1.6"
  subnet_id = "${aws_subnet.etcd.id}"
  user_data = "${replace("${data.template_file.user_data.rendered}", "__FILE__", "etcd_03_cloud_config.yml")}"
  vpc_security_group_ids = ["${aws_security_group.etcd.id}"]

  tags {
    Name = "kaws-etcd-${var.cluster}-03"
    KubernetesCluster = "${var.cluster}"
  }
}

resource "aws_launch_configuration" "k8s_masters" {
  associate_public_ip_address = true
  iam_instance_profile = "${aws_iam_instance_profile.k8s_master.name}"
  image_id = "${var.coreos_ami}"
  instance_type = "${var.instance_size}"
  name_prefix = "kaws-k8s-masters-${var.cluster}-"
  security_groups = ["${aws_security_group.kubernetes.id}"]
  user_data = "${replace("${data.template_file.user_data.rendered}", "__FILE__", "master_cloud_config.yml")}"

  lifecycle {
    create_before_destroy = true
  }

  root_block_device {
    volume_type = "gp2"
    volume_size = 30
  }
}

resource "aws_autoscaling_group" "k8s_masters" {
  health_check_grace_period = 300
  health_check_type = "ELB"
  launch_configuration = "${aws_launch_configuration.k8s_masters.name}"
  load_balancers = ["${aws_elb.k8s_masters.name}"]
  max_size = "${var.masters_max_size}"
  min_size = "${var.masters_min_size}"
  name = "kaws-k8s-masters-${var.cluster}"
  vpc_zone_identifier = ["${aws_subnet.k8s.id}"]

  lifecycle {
    create_before_destroy = true
  }

  tag {
    key = "Name"
    value = "kaws-k8s-master-${var.cluster}"
    propagate_at_launch = true
  }

  tag {
    key = "KubernetesCluster"
    value = "${var.cluster}"
    propagate_at_launch = true
  }
}

resource "aws_launch_configuration" "k8s_nodes" {
  associate_public_ip_address = true
  iam_instance_profile = "${aws_iam_instance_profile.k8s_node.name}"
  image_id = "${var.coreos_ami}"
  instance_type = "${var.instance_size}"
  name_prefix = "kaws-k8s-nodes-${var.cluster}-"
  security_groups = ["${aws_security_group.kubernetes.id}"]
  user_data = "${replace("${data.template_file.user_data.rendered}", "__FILE__", "node_cloud_config.yml")}"

  lifecycle {
    create_before_destroy = true
  }

  root_block_device {
    volume_type = "gp2"
    volume_size = 30
  }
}

resource "aws_autoscaling_group" "k8s_nodes" {
  health_check_grace_period = 300
  health_check_type = "ELB"
  launch_configuration = "${aws_launch_configuration.k8s_nodes.name}"
  load_balancers = ["${aws_elb.k8s_nodes.name}"]
  max_size = "${var.nodes_max_size}"
  min_size = "${var.nodes_min_size}"
  name = "kaws-k8s-nodes-${var.cluster}"
  vpc_zone_identifier = ["${aws_subnet.k8s.id}"]

  lifecycle {
    create_before_destroy = true
  }

  tag {
    key = "Name"
    value = "kaws-k8s-node-${var.cluster}"
    propagate_at_launch = true
  }

  tag {
    key = "KubernetesCluster"
    value = "${var.cluster}"
    propagate_at_launch = true
  }
}
