variable "access_key" {}
variable "secret_key" {}
variable "region" {}
variable "az" {}
variable "bucket" {}
variable "key_name" {}

provider "aws" {
  access_key = var.access_key
  secret_key = var.secret_key
  region     = var.region
}

#
# VPC
#
resource "aws_vpc" "ks_vpc" {
  cidr_block           = "10.0.0.0/16"
  enable_dns_hostnames = true
}

resource "aws_subnet" "ks_sn" {
  vpc_id            = aws_vpc.ks_vpc.id
  cidr_block        = "10.0.0.0/24"
  availability_zone = var.az
}

resource "aws_internet_gateway" "ks_igw" {
  vpc_id = aws_vpc.ks_vpc.id
}

resource "aws_route_table" "ks_rt" {
  vpc_id = aws_vpc.ks_vpc.id
  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.ks_igw.id
  }
}

resource "aws_route_table_association" "ks_rt_associate" {
  subnet_id      = aws_subnet.ks_sn.id
  route_table_id = aws_route_table.ks_rt.id
}

resource "aws_security_group" "ks_sg" {
  name        = "kotosiro-sharing-sg"
  description = "For EC2 Linux"
  vpc_id      = aws_vpc.ks_vpc.id
  ingress {
    from_port = 8080
    to_port = 8080
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  egress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

#
# EC2 Key Pair
#
resource "tls_private_key" "ks_tls_private_key" {
  algorithm = "RSA"
  rsa_bits  = 2048
}

locals {
  public_key_file  = "creds/${var.key_name}.id_rsa.pub"
  private_key_file = "creds/${var.key_name}.id_rsa"
}

resource "local_file" "ks_private_key_pem" {
  filename = "${local.private_key_file}"
  content  = "${tls_private_key.ks_tls_private_key.private_key_pem}"
}

resource "aws_key_pair" "ks_key_pair" {
  key_name   = "${var.key_name}"
  public_key = "${tls_private_key.ks_tls_private_key.public_key_openssh}"
}

#
# EC2
#
resource "aws_instance" "ks_ec2"{
  ami                         = "ami-0e0820ad173f20fbb"
  instance_type               = "t2.micro"
  availability_zone           = "${var.az}"
  vpc_security_group_ids      = [aws_security_group.ks_sg.id]
  subnet_id                   = aws_subnet.ks_sn.id
  associate_public_ip_address = true
  key_name                    = "${var.key_name}"
}

#
# S3
#
resource "aws_s3_bucket" "kotosiro_sharing_s3" {
  bucket        = var.bucket
  force_destroy = true
  tags = {
    Name = "kotosiro-sharing-s3"
  }
}

resource "aws_s3_bucket_ownership_controls" "kotosiro_sharing_s3" {
  bucket = aws_s3_bucket.kotosiro_sharing_s3.id
  rule {
    object_ownership = "BucketOwnerEnforced"
  }
}

output "ec2_global_ips" {
  value = aws_instance.ks_ec2.*.public_ip
}

output "bucket" {
  value = aws_s3_bucket.kotosiro_sharing_s3.bucket
}
