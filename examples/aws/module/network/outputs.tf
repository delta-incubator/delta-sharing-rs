output "vpc_id" {
  value = aws_vpc.this.id
}

output "vpc_cidr" {
  value = aws_vpc.this.cidr_block
}

output "public_subnet_ids" {
  value = [aws_subnet.public_1.id, aws_subnet.public_2.id]
}

output "public_subnet_id_1" {
  value = aws_subnet.public_1.id
}

output "public_subnet_id_2" {
  value = aws_subnet.public_2.id
}

output "private_subnet_ids" {
  value = [aws_subnet.private_1.id, aws_subnet.private_2.id]
}

output "private_subnet_id_1" {
  value = aws_subnet.private_1.id
}

output "private_subnet_id_2" {
  value = aws_subnet.private_2.id
}
