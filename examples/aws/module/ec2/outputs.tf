output "private_key_pem" {
  value     = tls_private_key.this.private_key_pem
  sensitive = true
}

output "public_key_openssh" {
  value = tls_private_key.this.public_key_openssh
}
