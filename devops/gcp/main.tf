variable "project" {
  type = string
}

variable "bucket" {
  type = string
}

variable "location" {
  type = string
}

variable "service_account" {
  type = string
}

provider "google" {
  project = var.project
}

resource "google_storage_bucket" "this" {
  name          = var.bucket
  location      = var.location
  force_destroy = true
}

resource "google_storage_bucket_iam_member" "this" {
  bucket     = google_storage_bucket.this.name
  role       = "roles/storage.objectAdmin"
  member     = var.service_account
  depends_on = [google_storage_bucket.this]
}

output "bucket" {
  value = google_storage_bucket.this.name
}
