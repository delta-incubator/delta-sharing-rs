output "arn" {
  value = aws_lb.this.arn
}

output "target_group_id" {
  value = aws_lb_target_group.this.id
}

output "dns_name" {
  value = aws_lb.this.dns_name
}

output "zone_id" {
  value = aws_lb.this.zone_id
}
