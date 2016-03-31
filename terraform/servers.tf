resource "aws_instance" "bastion" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  instance_type = "t2.micro"
  key_name = "${var.ssh_key}"
  subnet_id = "${aws_subnet.public.id}"
  vpc_security_group_ids = ["${aws_security_group.bastion.id}"]

  tags {
    Name = "kaws_bastion"
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
    Name = "kaws_etcd_01"
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
    Name = "kaws_etcd_02"
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
    Name = "kaws_etcd_03"
    Cluster = "${var.cluster}"
  }
}

resource "aws_instance" "k8s_master_01" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  depends_on = ["null_resource.sync_pki"]
  iam_instance_profile = "${aws_iam_instance_profile.k8s_master.name}"
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  subnet_id = "${aws_subnet.public.id}"
  user_data = "${template_file.master_cloud_config.rendered}"
  security_groups = ["${aws_security_group.kubernetes.id}"]

  tags {
    Name = "kaws_k8s_master_01"
    Cluster = "${var.cluster}"
  }
}

resource "aws_instance" "k8s_master_02" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  depends_on = ["null_resource.sync_pki"]
  iam_instance_profile = "${aws_iam_instance_profile.k8s_master.name}"
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  subnet_id = "${aws_subnet.public.id}"
  user_data = "${template_file.master_cloud_config.rendered}"
  security_groups = ["${aws_security_group.kubernetes.id}"]

  tags {
    Name = "kaws_k8s_master_02"
    Cluster = "${var.cluster}"
  }
}

resource "aws_instance" "k8s_node_01" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  depends_on = ["aws_instance.k8s_master_01", "aws_instance.k8s_master_02"]
  iam_instance_profile = "${aws_iam_instance_profile.k8s_node.name}"
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  subnet_id = "${aws_subnet.public.id}"
  user_data = "${template_file.node_cloud_config.rendered}"
  security_groups = ["${aws_security_group.kubernetes.id}"]

  tags {
    Name = "kaws_k8s_node_01"
    Cluster = "${var.cluster}"
  }
}

resource "aws_instance" "k8s_node_02" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  depends_on = ["aws_instance.k8s_master_01", "aws_instance.k8s_master_02"]
  iam_instance_profile = "${aws_iam_instance_profile.k8s_node.name}"
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  subnet_id = "${aws_subnet.public.id}"
  user_data = "${template_file.node_cloud_config.rendered}"
  security_groups = ["${aws_security_group.kubernetes.id}"]

  tags {
    Name = "kaws_k8s_node_02"
    Cluster = "${var.cluster}"
  }
}
