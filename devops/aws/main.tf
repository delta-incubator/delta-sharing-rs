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

//resource "aws_iam_policy" "kotosiro_sharing_s3" {
//  name   = "kotosiro_sharing_s3"
//  policy = jsonencode(
//    {
//      "Version": "2012-10-17",
//      "Statement": [
//	{
//	  "Action": [
//	    "s3:AbortMultipartUpload",
//	    "s3:Get*",
//	    "s3:List*",
//	    "s3:Delete*",
//	    "s3:PutObject"
//	  ],
//	  "Effect": "Allow",
//	  "Resource": "arn:aws:s3:::${aws_s3_bucket.kotosiro_sharing_s3.bucket}"
//	}
//      ]
//    })
//}

//resource "aws_iam_user_policy_attachment" "s3_bucket_policy_for_IAM_user" {
//  user       = var.user
//  policy_arn = aws_iam_policy.kotosiro_sharing_s3.arn
//}

output "bucket" {
  value = aws_s3_bucket.kotosiro_sharing_s3.bucket
}
