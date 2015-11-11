resource "aws_instance" "bastion" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  instance_type = "t2.micro"
  key_name = "${var.ssh_key}"
  subnet_id = "${aws_subnet.public.id}"
  vpc_security_group_ids = ["${aws_security_group.bastion.id}"]

  tags {
    Name = "bastion"
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
    Name = "etcd_01"
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
    Name = "etcd_02"
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
    Name = "etcd_03"
    Cluster = "${var.cluster}"
  }
}

resource "aws_instance" "k8s_master_01" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  depends_on = ["aws_instance.etcd_01", "aws_instance.etcd_02", "aws_instance.etcd_03"]
  iam_instance_profile = "${aws_iam_instance_profile.k8s_master.name}"
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  subnet_id = "${aws_subnet.public.id}"
  user_data = "${template_file.master_cloud_config.rendered}"
  security_groups = ["${aws_security_group.kubernetes.id}"]

  connection {
    user = "core"
    host = "${self.private_ip}"
    bastion_host = "${aws_instance.bastion.public_ip}"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mkdir -p /etc/kubernetes/ssl",
      "sudo chgrp core /etc/kubernetes/ssl",
      "sudo chmod g+w /etc/kubernetes/ssl",
    ]
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/ca.pem"
    destination = "/etc/kubernetes/ssl/ca.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/apiserver.pem"
    destination = "/etc/kubernetes/ssl/apiserver.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/apiserver-key.pem"
    destination = "/etc/kubernetes/ssl/apiserver-key.pem"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo systemctl start kubelet",
      "sudo systemctl enable kubelet",
      "sudo systemctl start start_kube_addons",
    ]
  }

  tags {
    Name = "k8s_master_01"
    Cluster = "${var.cluster}"
  }
}

resource "aws_instance" "k8s_master_02" {
  ami = "${var.coreos_ami}"
  associate_public_ip_address = true
  depends_on = ["aws_instance.etcd_01", "aws_instance.etcd_02", "aws_instance.etcd_03"]
  iam_instance_profile = "${aws_iam_instance_profile.k8s_master.name}"
  instance_type = "${var.instance_size}"
  key_name = "${var.ssh_key}"
  subnet_id = "${aws_subnet.public.id}"
  user_data = "${template_file.master_cloud_config.rendered}"
  security_groups = ["${aws_security_group.kubernetes.id}"]

  connection {
    user = "core"
    host = "${self.private_ip}"
    bastion_host = "${aws_instance.bastion.public_ip}"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mkdir -p /etc/kubernetes/ssl",
      "sudo chgrp core /etc/kubernetes/ssl",
      "sudo chmod g+w /etc/kubernetes/ssl",
    ]
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/ca.pem"
    destination = "/etc/kubernetes/ssl/ca.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/apiserver.pem"
    destination = "/etc/kubernetes/ssl/apiserver.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/apiserver-key.pem"
    destination = "/etc/kubernetes/ssl/apiserver-key.pem"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo systemctl start kubelet",
      "sudo systemctl enable kubelet",
    ]
  }

  tags {
    Name = "k8s_master_02"
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

  connection {
    user = "core"
    host = "${self.private_ip}"
    bastion_host = "${aws_instance.bastion.public_ip}"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mkdir -p /etc/kubernetes/ssl",
      "sudo chgrp core /etc/kubernetes/ssl",
      "sudo chmod g+w /etc/kubernetes/ssl",
    ]
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/ca.pem"
    destination = "/etc/kubernetes/ssl/ca.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/node.pem"
    destination = "/etc/kubernetes/ssl/node.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/node-key.pem"
    destination = "/etc/kubernetes/ssl/node-key.pem"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo systemctl start kubelet",
      "sudo systemctl enable kubelet",
    ]
  }

  tags {
    Name = "k8s_node_01"
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

  provisioner "remote-exec" {
    inline = [
      "sudo mkdir -p /etc/kubernetes/ssl",
      "sudo chgrp core /etc/kubernetes/ssl",
      "sudo chmod g+w /etc/kubernetes/ssl",
    ]
  }

  connection {
    user = "core"
    host = "${self.private_ip}"
    bastion_host = "${aws_instance.bastion.public_ip}"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/ca.pem"
    destination = "/etc/kubernetes/ssl/ca.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/node.pem"
    destination = "/etc/kubernetes/ssl/node.pem"
  }

  provisioner "file" {
    source = "clusters/${var.cluster}/node-key.pem"
    destination = "/etc/kubernetes/ssl/node-key.pem"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo systemctl start kubelet",
      "sudo systemctl enable kubelet",
    ]
  }

  tags {
    Name = "k8s_node_02"
    Cluster = "${var.cluster}"
  }
}
