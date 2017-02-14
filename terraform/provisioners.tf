resource "null_resource" "generate_pki" {
  depends_on = ["aws_kms_key.pki"]

  provisioner "local-exec" {
    command = "kaws cluster genpki ${var.cluster} --domain ${var.domain} --kms-key ${aws_kms_key.pki.key_id} --region ${var.region}"
  }
}
