resource "aws_ebs_volume" "etcd_01" {
  availability_zone = "${var.availability_zone}"
  encrypted = true
  kms_key_id = "${aws_kms_key.etcd.key_id}"
  size = "10"
  type = "gp2"

  tags {
    Name = "kaws-etcd-${var.cluster}-01"
    Cluster = "${var.cluster}"
  }
}

resource "aws_ebs_volume" "etcd_02" {
  availability_zone = "${var.availability_zone}"
  encrypted = true
  kms_key_id = "${aws_kms_key.etcd.key_id}"
  size = "10"
  type = "gp2"

  tags {
    Name = "kaws-etcd-${var.cluster}-02"
    Cluster = "${var.cluster}"
  }
}

resource "aws_ebs_volume" "etcd_03" {
  availability_zone = "${var.availability_zone}"
  encrypted = true
  kms_key_id = "${aws_kms_key.etcd.key_id}"
  size = "10"
  type = "gp2"

  tags {
    Name = "kaws-etcd-${var.cluster}-03"
    Cluster = "${var.cluster}"
  }
}

resource "aws_volume_attachment" "etcd_01" {
  device_name = "/dev/xvdf"
  instance_id = "${aws_instance.etcd_01.id}"
  volume_id = "${aws_ebs_volume.etcd_01.id}"
}

resource "aws_volume_attachment" "etcd_02" {
  device_name = "/dev/xvdf"
  instance_id = "${aws_instance.etcd_02.id}"
  volume_id = "${aws_ebs_volume.etcd_02.id}"
}

resource "aws_volume_attachment" "etcd_03" {
  device_name = "/dev/xvdf"
  instance_id = "${aws_instance.etcd_03.id}"
  volume_id = "${aws_ebs_volume.etcd_03.id}"
}
