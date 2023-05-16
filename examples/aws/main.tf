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

variable "domain" {
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
  subnet_id_1            = module.network.public_subnet_id_1
  subnet_id_2            = module.network.public_subnet_id_2
  availability_zone_1    = var.availability_zone_1
  availability_zone_2    = var.availability_zone_2
  vpc_security_group_ids = module.security_group.public_security_group_ids
}

module "alb" {
  source          = "./module/alb"
  name            = "${var.name}-alb"
  subnets         = module.network.public_subnet_ids
  security_groups = module.security_group.public_security_group_ids
  vpc_id          = module.network.vpc_id
  target_id_1     = module.ec2.instance_id_1
  target_id_2     = module.ec2.instance_id_2
}

module "route53" {
  source       = "./module/route53"
  domain       = var.domain
  alb_dns_name = module.alb.dns_name
  alb_zone_id  = module.alb.zone_id
}

module "acm" {
  source              = "./module/acm"
  dns_record_name     = module.route53.dns_record_name
  zone_id             = module.route53.zone_id
  alb_arn             = module.alb.arn
  alb_target_group_id = module.alb.target_group_id
  security_group_id   = module.security_group.public_security_group_id
}

module "s3" {
  source = "./module/s3"
  bucket = "${var.name}-s3"
}
