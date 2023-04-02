variable "project" {}
variable "bucket" {}
variable "location" {}
variable "service_account" {}

provider "google" {
  project = var.project
}

resource "google_storage_bucket" "kotosiro_sharing_gcs" {
  name          = var.bucket
  location      = var.location
  force_destroy = true
}

resource "google_storage_bucket_iam_member" "kotosiro_sharing_gcs" {
  bucket     = google_storage_bucket.kotosiro_sharing_gcs.name
  role       = "roles/storage.objectAdmin"
  member     = var.service_account
  depends_on = [google_storage_bucket.kotosiro_sharing_gcs]
}

output "bucket" {
  value = google_storage_bucket.kotosiro_sharing_gcs.name
}
