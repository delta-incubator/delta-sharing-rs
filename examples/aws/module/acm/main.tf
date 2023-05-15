# SSL Certificate
resource "aws_acm_certificate" "this" {
  domain_name               = var.dns_record_name
  subject_alternative_names = []
  validation_method         = "DNS"
  lifecycle {
    create_before_destroy = true
  }
}

# Validation Record
resource "aws_route53_record" "validation" {
  for_each = {
    for dvo in aws_acm_certificate.this.domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  }
  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = var.zone_id
}

# Certificate Validation
resource "aws_acm_certificate_validation" "this" {
  certificate_arn         = aws_acm_certificate.this.arn
  validation_record_fqdns = [for record in aws_route53_record.validation : record.fqdn]
}

# HTTPS Listener
resource "aws_lb_listener" "https" {
  load_balancer_arn = var.alb_arn
  port              = 443
  protocol          = "HTTPS"
  certificate_arn   = aws_acm_certificate.this.arn
  ssl_policy        = "ELBSecurityPolicy-2016-08"
  default_action {
    type             = "forward"
    target_group_arn = var.alb_target_group_id
  }
  depends_on = [
    aws_acm_certificate_validation.this
  ]
}

# Ingress Rule
resource "aws_security_group_rule" "ingress_https" {
  security_group_id = var.security_group_id
  type              = "ingress"
  from_port         = 443
  to_port           = 443
  protocol          = "tcp"
  cidr_blocks       = ["0.0.0.0/0"]
#  security_groups = [
#    "${aws_security_group.elb_sg.id}",
#  ]
}
