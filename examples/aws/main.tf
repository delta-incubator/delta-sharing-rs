variable "region" {
  type = string
}

variable "availability_zone_1" {
  type = string
}

variable "availability_zone_2" {
  type = string
}

variable "security_group_name" {
  type = string
}


provider "aws" {
  region = var.region
}

module "network" {
  source              = "./module/network"
  availability_zone_1 = var.availability_zone_1
  availability_zone_2 = var.availability_zone_2
}

module "security_group" {
  source = "./module/security_group"
  vpc_id = module.network.vpc_id
  name   = var.security_group_name
}

module "ec2" {
  source                 = "./module/ec2"
  subnet_id_1            = module.network.subnet_id_1
  subnet_id_2            = module.network.subnet_id_2
  availability_zone_1    = var.availability_zone_1
  availability_zone_2    = var.availability_zone_2
  vpc_security_group_ids = module.security_group.security_group_ids
}
