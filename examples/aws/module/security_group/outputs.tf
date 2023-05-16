output "public_security_group_name" {
  value = aws_security_group.public.name
}

output "public_security_group_id" {
  value = aws_security_group.public.id
}

output "public_security_group_ids" {
  value = [aws_security_group.public.id]
}

output "private_security_group_name" {
  value = aws_security_group.private.name
}

output "private_security_group_id" {
  value = aws_security_group.private.id
}

output "private_security_group_ids" {
  value = [aws_security_group.private.id]
}
