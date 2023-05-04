variable "access_key" {}
variable "secret_key" {}
variable "region" {}
variable "bucket" {}

provider "aws" {
  access_key = var.access_key
  secret_key = var.secret_key
  region     = var.region
}

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
