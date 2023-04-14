variable "access_key" {}
variable "secret_key" {}
variable "region" {}
variable "bucket" {}
variable "acl" {}

provider "aws" {
  access_key = var.access_key
  secret_key = var.secret_key
  region     = var.region
}

resource "aws_s3_bucket" "kotosiro_sharing_s3" {
  bucket        = var.bucket
  force_destroy = true
}

resource "aws_s3_bucket_acl" "kotosiro_sharing_s3" {
  bucket = aws_s3_bucket.kotosiro_sharing_s3.id
  acl    = var.acl
}

output "bucket" {
  value = aws_s3_bucket.kotosiro_sharing_s3.bucket
}
