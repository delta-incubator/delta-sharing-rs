# Public Security Group
resource "aws_security_group" "public" {
  vpc_id = var.vpc_id
  name   = "${var.name}-public"
}

# Public Ingress Rule
resource "aws_security_group_rule" "ingress_http" {
  security_group_id = aws_security_group.public.id
  type              = "ingress"
  from_port         = 80
  to_port           = 80
  protocol          = "tcp"
  cidr_blocks       = ["0.0.0.0/0"]
#  security_groups = [
#    "${aws_security_group.elb_sg.id}",
#  ]
}

# Public Ingress Rule
resource "aws_security_group_rule" "ingress_ssh" {
  security_group_id = aws_security_group.public.id
  type              = "ingress"
  from_port         = 22
  to_port           = 22
  protocol          = "tcp"
  cidr_blocks       = var.ssh_cidr_blocks
}

# Public Egress Rule
resource "aws_security_group_rule" "egress_public" {
  security_group_id = aws_security_group.public.id
  type              = "egress"
  from_port         = 0
  to_port           = 0
  protocol          = "-1"
  cidr_blocks       = ["0.0.0.0/0"]
}

# Private Security Group
resource "aws_security_group" "private" {
  vpc_id = var.vpc_id
  name   = "${var.name}-private"
}

# Private Ingress Rule
resource "aws_security_group_rule" "ingress_postgres" {
  security_group_id = aws_security_group.private.id
  type              = "ingress"
  from_port         = 5432
  to_port           = 5432
  protocol          = "tcp"
  cidr_blocks       = ["10.0.0.0/24", "10.0.1.0/24"]
}

# Private Egress Rule
resource "aws_security_group_rule" "egress_private" {
  security_group_id = aws_security_group.private.id
  type              = "egress"
  from_port         = 0
  to_port           = 0
  protocol          = "-1"
  cidr_blocks       = ["0.0.0.0/0"]
}
