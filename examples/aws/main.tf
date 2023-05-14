variable "region" {}

variable "az_1" {}

variable "az_2" {}

provider "aws" {
  region = var.region
}

module "network" {
  source = "./module/network"
  az_1   = var.az_1
  az_2   = var.az_2
}
