resource "aws_route53_record" "bastion" {
  zone_id = "${var.zone_id}"
  name = "bastion.${var.domain}"
  type = "A"
  ttl = "300"
  records = ["${aws_instance.bastion.public_ip}"]
}

resource "aws_route53_record" "kubernetes" {
  zone_id = "${var.zone_id}"
  name = "kubernetes.${var.domain}"
  type = "A"

  alias {
    name = "${aws_elb.k8s_masters.dns_name}"
    zone_id = "${aws_elb.k8s_masters.zone_id}"
    evaluate_target_health = false
  }
}
