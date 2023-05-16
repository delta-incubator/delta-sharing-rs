#
# Key Pair
#
resource "tls_private_key" "this" {
  algorithm = "RSA"
  rsa_bits  = 2048
}

locals {
  public_key_file  = "creds/${var.key_name}.id_rsa.pub"
  private_key_file = "creds/${var.key_name}.id_rsa"
}

resource "local_file" "this" {
  filename = local.private_key_file
  content  = tls_private_key.this.private_key_pem
}

resource "aws_key_pair" "this" {
  key_name   = var.key_name
  public_key = tls_private_key.this.public_key_openssh
}

# EC2 1
resource "aws_instance" "instance_1" {
  ami                    = var.ami
  instance_type          = var.instance_type
  subnet_id              = var.subnet_id_1
  availability_zone      = var.availability_zone_1
  vpc_security_group_ids = var.vpc_security_group_ids
  key_name               = var.key_name
  user_data              = <<-EOF
          #!/bin/bash
          sudo yum update -y
          sudo yum install -y git
          sudo yum install -y docker
          sudo usermod -a -G docker ec2-user
          sudo curl -L https://github.com/docker/compose/releases/download/v2.17.0/docker-compose-`uname -s`-`uname -m` | sudo tee /usr/local/bin/docker-compose > /dev/null
          sudo chmod +x /usr/local/bin/docker-compose
          sudo ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose
          sudo service docker start
          sudo chkconfig docker on
          EOF
}

# EC2 2
resource "aws_instance" "instance_2" {
  ami                    = var.ami
  instance_type          = var.instance_type
  subnet_id              = var.subnet_id_2
  availability_zone      = var.availability_zone_2
  vpc_security_group_ids = var.vpc_security_group_ids
  key_name               = var.key_name
  user_data              = <<-EOF
          #!/bin/bash 
          sudo yum update -y
          sudo yum install -y git
          sudo yum install -y docker
          sudo usermod -a -G docker ec2-user
          sudo curl -L https://github.com/docker/compose/releases/download/v2.17.0/docker-compose-`uname -s`-`uname -m` | sudo tee /usr/local/bin/docker-compose > /dev/null
          sudo chmod +x /usr/local/bin/docker-compose
          sudo ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose
          sudo service docker start
          sudo chkconfig docker on
          EOF
}
