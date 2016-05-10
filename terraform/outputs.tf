output "domain" {
  value = "kubernetes.${var.domain}"
}

output "kms-master-key-id" {
  value = "${aws_kms_key.pki.key_id}"
}
