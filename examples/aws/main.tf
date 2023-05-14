variable "region" {}

variable "az_1" {}

variable "az_2" {}

variable "security_group_name" {}

provider "aws" {
  region = var.region
}

module "network" {
  source = "./module/network"
  az_1   = var.az_1
  az_2   = var.az_2
}

module "security_group" {
  source = "./module/security_group"
  vpc_id = module.network.vpc_id
  name   = var.security_group_name
}
