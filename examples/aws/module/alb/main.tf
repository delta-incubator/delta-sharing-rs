# ALB
resource "aws_lb" "this" {
  name               = var.name
  load_balancer_type = "application"
  internal           = false
  subnets            = var.subnets
  security_groups    = var.security_groups
}

# Target Group
resource "aws_lb_target_group" "this" {
  name     = "${var.name}-target-group"
  vpc_id   = var.vpc_id
  port     = 80
  protocol = "HTTP"
}

# Target 1
resource "aws_lb_target_group_attachment" "target_1" {
  target_group_arn = aws_lb_target_group.this.arn
  target_id        = var.target_id_1
  port             = 80
}

# Target 2
resource "aws_lb_target_group_attachment" "target_2" {
  target_group_arn = aws_lb_target_group.this.arn
  target_id        = var.target_id_2
  port             = 80
}

# HTTP Listener
resource "aws_lb_listener" "http" {
  load_balancer_arn = aws_lb.this.arn
  port              = 80
  protocol          = "HTTP"
#  default_action {
#    type             = "forward"
#    target_group_arn = aws_lb_target_group.this.id
#  }
  default_action {
    type = "redirect"
    redirect {
      port        = 443
      protocol    = "HTTPS"
      status_code = "HTTP_301"
    }
  }
}
