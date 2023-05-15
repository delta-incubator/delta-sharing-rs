output "dns_record_name" {
  value = aws_route53_record.dns.name
}

output "zone_id" {
  value = data.aws_route53_zone.this.id
}
