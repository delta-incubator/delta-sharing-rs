output "private_key_pem" {
  value     = tls_private_key.this.private_key_pem
  sensitive = true
}

output "public_key_openssh" {
  value = tls_private_key.this.public_key_openssh
}

output "instance_id_1" {
  value = aws_instance.instance_1.id
}

output "instance_id_2" {
  value = aws_instance.instance_2.id
}
