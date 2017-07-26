resource "aws_elb" "k8s_masters" {
  connection_draining = true
  idle_timeout = 3600
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
    KubernetesCluster = "${var.cluster}"
  }
}

resource "aws_elb" "k8s_nodes" {
  connection_draining = true
  idle_timeout = 3600
  name = "kaws-k8s-nodes-${var.cluster}"
  security_groups = ["${aws_security_group.balancers.id}"]
  subnets = ["${aws_subnet.public.id}"]

  listener {
    instance_port = 30000
    instance_protocol = "tcp"
    lb_port = 80
    lb_protocol = "tcp"
  }

  listener {
    instance_port = 30001
    instance_protocol = "tcp"
    lb_port = 443
    lb_protocol = "tcp"
  }

  health_check {
    healthy_threshold = 2
    interval = 30
    target = "http:10249/healthz"
    timeout = 3
    unhealthy_threshold = 2
  }

  tags {
    Name = "kaws-k8s-nodes"
    KubernetesCluster = "${var.cluster}"
  }
}

resource "aws_load_balancer_policy" "k8s_masters_proxy" {
  load_balancer_name = "${aws_elb.k8s_masters.name}"
  policy_name = "k8s-masters-proxy"
  policy_type_name = "ProxyProtocolPolicyType"

  policy_attribute = {
    name = "ProxyProtocol"
    value = "true"
  }
}

resource "aws_load_balancer_policy" "k8s_nodes_proxy" {
  load_balancer_name = "${aws_elb.k8s_nodes.name}"
  policy_name = "k8s-nodes-proxy"
  policy_type_name = "ProxyProtocolPolicyType"

  policy_attribute = {
    name = "ProxyProtocol"
    value = "true"
  }
}

resource "aws_load_balancer_backend_server_policy" "k8s_masters_proxy" {
  instance_port = 443
  load_balancer_name = "${aws_elb.k8s_masters.name}"
  policy_names = [
    "${aws_load_balancer_policy.k8s_masters_proxy.policy_name}",
  ]
}

resource "aws_load_balancer_backend_server_policy" "k8s_nodes_proxy_http" {
  instance_port = 30000
  load_balancer_name = "${aws_elb.k8s_nodes.name}"
  policy_names = [
    "${aws_load_balancer_policy.k8s_nodes_proxy.policy_name}",
  ]
}

resource "aws_load_balancer_backend_server_policy" "k8s_nodes_proxy_https" {
  instance_port = 30001
  load_balancer_name = "${aws_elb.k8s_nodes.name}"
  policy_names = [
    "${aws_load_balancer_policy.k8s_nodes_proxy.policy_name}",
  ]
}
