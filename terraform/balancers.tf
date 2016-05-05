resource "aws_elb" "k8s_masters" {
  connection_draining = true
  name = "kaws-k8s-masters-${var.cluster}"
  security_groups = ["${aws_security_group.balancers.id}"]
  subnets = ["${aws_subnet.public.id}"]

  listener {
    instance_port = 443
    instance_protocol = "tcp"
    lb_port = 443
    lb_protocol = "tcp"
  }

  health_check {
    healthy_threshold = 2
    interval = 30
    target = "http:8080/healthz"
    timeout = 3
    unhealthy_threshold = 2
  }

  tags {
    Name = "kaws-k8s-masters"
    Cluster = "${var.cluster}"
  }
}
