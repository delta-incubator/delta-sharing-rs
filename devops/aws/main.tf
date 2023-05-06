variable "access_key" {}
variable "secret_key" {}
variable "region" {}
variable "az" {}
variable "bucket" {}

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

resource "aws_security_group" "allow_http" {
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
