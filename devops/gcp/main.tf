variable "project" {}
variable "bucket" {}
variable "location" {}

provider "google" {
  project = var.project
}

resource "google_storage_bucket" "kotosiro_sharing_gcs" {
  name          = var.bucket
  location      = var.location
  force_destroy = true
}

output "bucket" {
  value = google_storage_bucket.kotosiro_sharing_gcs.name
}
