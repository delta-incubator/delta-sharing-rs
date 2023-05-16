# RDS
resource "aws_db_subnet_group" "this" {
  name       = var.name
  subnet_ids = var.subnet_ids
}

# DB Instance
resource "aws_db_instance" "this" {
  identifier             = "${var.name}-db"
  allocated_storage      = var.allocated_storage
  storage_type           = var.storage_type
  engine                 = "postgres"
  engine_version         = "15.2"
  instance_class         = var.instance_class
  db_name                = var.db_name
  username               = var.db_username
  password               = var.db_password
  vpc_security_group_ids = var.vpc_security_group_ids
  db_subnet_group_name   = aws_db_subnet_group.this.name
  skip_final_snapshot    = true
}
