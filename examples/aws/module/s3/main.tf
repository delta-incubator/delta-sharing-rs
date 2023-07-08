# S3 Bucket
resource "aws_s3_bucket" "this" {
  bucket        = var.bucket
  force_destroy = true
  tags = {
    Name = "delta-sharing-s3"
  }
}

# Bucket Ownership
resource "aws_s3_bucket_ownership_controls" "this" {
  bucket = aws_s3_bucket.this.id
  rule {
    object_ownership = "BucketOwnerEnforced"
  }
}
