# Security Group
resource "aws_security_group" "this" {
  vpc_id = var.vpc_id
  name   = var.name
}

# Ingress Rule
resource "aws_security_group_rule" "ingress_http" {
  security_group_id = aws_security_group.this.id
  type              = "ingress"
  from_port         = 80
  to_port           = 80
  protocol          = "tcp"
  cidr_blocks       = ["0.0.0.0/0"]
#  security_groups = [
#    "${aws_security_group.elb_sg.id}",
#  ]
}

# Ingress Rule
resource "aws_security_group_rule" "ingress_ssh" {
  security_group_id = aws_security_group.this.id
  type              = "ingress"
  from_port         = 22
  to_port           = 22
  protocol          = "tcp"
  cidr_blocks       = ["0.0.0.0/0"]
}

# Egress Rule
resource "aws_security_group_rule" "egress" {
  security_group_id = aws_security_group.this.id
  type              = "egress"
  from_port         = 0
  to_port           = 0
  protocol          = "-1"
  cidr_blocks       = ["0.0.0.0/0"]
}
