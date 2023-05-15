# Hostzone
data "aws_route53_zone" "this" {
  name         = var.domain
  private_zone = false
}

# DNS Record
resource "aws_route53_record" "this" {
  zone_id = data.aws_route53_zone.this.id
  name    = var.domain
  type    = "A"
  alias {
    name                   = var.alb_dns_name
    zone_id                = var.alb_zone_id
    evaluate_target_health = true
  }
} 
