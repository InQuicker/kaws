resource "aws_instance" "bastion" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  depends_on = ["aws_instance.etcd_01", "aws_instance.etcd_02", "aws_instance.etcd_03"]
  instance_type = "t2.micro"
  key_name = "${var.ssh_key}"
  subnet_id = "${aws_subnet.public.id}"
  vpc_security_group_ids = ["${aws_security_group.bastion.id}"]

  tags {
    Name = "kaws-bastion-${var.cluster}"
    Cluster = "${var.cluster}"
  }
}

resource "aws_instance" "etcd_01" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  private_ip = "10.0.1.4"
  subnet_id = "${aws_subnet.public.id}"
  user_data = "${template_file.etcd_01_cloud_config.rendered}"
  vpc_security_group_ids = ["${aws_security_group.etcd.id}"]

  tags {
    Name = "kaws-etcd-${var.cluster}-01"
    Cluster = "${var.cluster}"
  }
}

resource "aws_instance" "etcd_02" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  private_ip = "10.0.1.5"
  subnet_id = "${aws_subnet.public.id}"
  user_data = "${template_file.etcd_02_cloud_config.rendered}"
  vpc_security_group_ids = ["${aws_security_group.etcd.id}"]

  tags {
    Name = "kaws-etcd-${var.cluster}-02"
    Cluster = "${var.cluster}"
  }
}

resource "aws_instance" "etcd_03" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  private_ip = "10.0.1.6"
  subnet_id = "${aws_subnet.public.id}"
  user_data = "${template_file.etcd_03_cloud_config.rendered}"
  vpc_security_group_ids = ["${aws_security_group.etcd.id}"]

  tags {
    Name = "kaws-etcd-${var.cluster}-03"
    Cluster = "${var.cluster}"
  }
}

resource "aws_launch_configuration" "k8s_masters" {
  associate_public_ip_address = true
  iam_instance_profile = "${aws_iam_instance_profile.k8s_master.name}"
  image_id = "${var.coreos_ami}"
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  name_prefix = "kaws-k8s-masters-${var.cluster}-"
  security_groups = ["${aws_security_group.kubernetes.id}"]
  user_data = "${template_file.master_cloud_config.rendered}"

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_autoscaling_group" "k8s_masters" {
  depends_on = ["null_resource.sync_pki", "aws_internet_gateway.outgoing"]
  health_check_grace_period = 300
  health_check_type = "ELB"
  launch_configuration = "${aws_launch_configuration.k8s_masters.name}"
  load_balancers = ["${aws_elb.k8s_masters.name}"]
  max_size = "${var.masters_max_size}"
  min_size = "${var.masters_min_size}"
  name = "kaws-k8s-masters-${var.cluster}"
  vpc_zone_identifier = ["${aws_subnet.public.id}"]

  lifecycle {
    create_before_destroy = true
  }

  tag {
    key = "Name"
    value = "kaws-k8s-master-${var.cluster}"
    propagate_at_launch = true
  }

  tag {
    key = "Cluster"
    value = "${var.cluster}"
    propagate_at_launch = true
  }
}

resource "aws_launch_configuration" "k8s_nodes" {
  associate_public_ip_address = true
  iam_instance_profile = "${aws_iam_instance_profile.k8s_node.name}"
  image_id = "${var.coreos_ami}"
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  name_prefix = "kaws-k8s-nodes-${var.cluster}-"
  security_groups = ["${aws_security_group.kubernetes.id}"]
  user_data = "${template_file.node_cloud_config.rendered}"

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_autoscaling_group" "k8s_nodes" {
  depends_on = ["aws_autoscaling_group.k8s_masters", "aws_internet_gateway.outgoing"]
  health_check_grace_period = 300
  health_check_type = "ELB"
  launch_configuration = "${aws_launch_configuration.k8s_nodes.name}"
  load_balancers = ["${aws_elb.k8s_nodes.name}"]
  max_size = "${var.nodes_max_size}"
  min_size = "${var.nodes_min_size}"
  name = "kaws-k8s-nodes-${var.cluster}"
  vpc_zone_identifier = ["${aws_subnet.public.id}"]

  lifecycle {
    create_before_destroy = true
  }

  tag {
    key = "Name"
    value = "kaws-k8s-node-${var.cluster}"
    propagate_at_launch = true
  }

  tag {
    key = "Cluster"
    value = "${var.cluster}"
    propagate_at_launch = true
  }
}
