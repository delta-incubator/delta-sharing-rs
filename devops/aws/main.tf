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
resource "aws_vpc" "kotosiro_sharing_vpc" {
  cidr_block = "10.0.0.0/16"
  enable_dns_hostnames = true
}

resource "aws_internet_gateway" "kotosiro_sharing_internet_gateway" {
  vpc_id = aws_vpc.kotosiro_sharing_vpc.id
}

resource "aws_route" "kotosiro_sharing_to_internet" {
  route_table_id = aws_vpc.kotosiro_sharing_vpc.main_route_table_id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id = aws_internet_gateway.kotosiro_sharing_internet_gateway.id
}

resource "aws_subnet" "kotosiro_sharing_public_subnet" {
  vpc_id = aws_vpc.kotosiro_sharing_vpc.id
  availability_zone = var.az
  cidr_block = "10.0.0.0/18"
  map_public_ip_on_launch = true
  depends_on = [aws_internet_gateway.kotosiro_sharing_internet_gateway]
}

resource "aws_security_group" "kotosiro_sharing_sg" {
    name = "allow_http"
    ingress {
        from_port = 80
        to_port = 80
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
resource "tls_private_key" "kotosiro_sharing_tls_private_key" {
  algorithm = "RSA"
  rsa_bits  = 2048
}

locals {
  public_key_file  = "~/.kotosiro/${var.key_name}.id_rsa.pub"
  private_key_file = "~/.kotosiro/${var.key_name}.id_rsa"
}

resource "local_file" "handson_private_key_pem" {
  filename = "${local.private_key_file}"
  content  = "${tls_private_key.kotosiro_sharing_tls_private_key.private_key_pem}"
}

resource "aws_key_pair" "kotosiro_sharing_key_pair" {
  key_name   = "${var.key_name}"
  public_key = "${tls_private_key.kotosiro_sharing_tls_private_key.public_key_openssh}"
}

#
# EC2
#
#data "aws_ssm_parameter" "amzn2_latest_ami" {
#  name = "/aws/service/ami-amazon-linux-latest/amzn2-ami-hvm-x86_64-gp2"
#}

resource "aws_instance" "kotosiro_sharing_ec2"{
  ami                         = "ami-0e0820ad173f20fbb"
  instance_type               = "t2.micro"
  availability_zone           = "${var.az}"
  vpc_security_group_ids      = [aws_security_group.kotosiro_sharing_sg.id]
  associate_public_ip_address = "true"
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

output "bucket" {
  value = aws_s3_bucket.kotosiro_sharing_s3.bucket
}
