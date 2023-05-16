variable "access_key" {
  type = string
}

variable "secret_key" {
  type = string
}

variable "region" {
  type = string
}

variable "bucket" {
  type = string
}

provider "aws" {
  access_key = var.access_key
  secret_key = var.secret_key
  region     = var.region
}

# S3
resource "aws_s3_bucket" "this" {
  bucket        = var.bucket
  force_destroy = true
  tags = {
    Name = "kotosiro-sharing-s3"
  }
}

resource "aws_s3_bucket_ownership_controls" "this" {
  bucket = aws_s3_bucket.this.id
  rule {
    object_ownership = "BucketOwnerEnforced"
  }
}

output "bucket" {
  value = aws_s3_bucket.this.bucket
}
