variable "name" {
  type = string
}

variable "region" {
  type = string
}

variable "availability_zone_1" {
  type = string
}

variable "availability_zone_2" {
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
  name   = "${var.name}-security-group"
  vpc_id = module.network.vpc_id
}

module "ec2" {
  source                 = "./module/ec2"
  subnet_id_1            = module.network.subnet_id_1
  subnet_id_2            = module.network.subnet_id_2
  availability_zone_1    = var.availability_zone_1
  availability_zone_2    = var.availability_zone_2
  vpc_security_group_ids = module.security_group.security_group_ids
}

module "alb" {
  source          = "./module/alb"
  name            = "${var.name}-alb"
  subnets         = module.network.subnet_ids
  security_groups = module.security_group.security_group_ids
  vpc_id          = module.network.vpc_id
  target_id_1     = module.ec2.instance_id_1
  target_id_2     = module.ec2.instance_id_2
}
