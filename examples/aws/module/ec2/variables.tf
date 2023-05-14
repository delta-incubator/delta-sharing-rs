variable "key_name" {
  type    = string
  default = "key"
}

variable "ami" {
  type    = string
  default = "ami-0e0820ad173f20fbb"
}

variable "instance_type" {
  type    = string
  default = "t2.micro"
}

variable "subnet_id_1" {
  type = string
}

variable "subnet_id_2" {
  type = string
}

variable "availability_zone_1" {
  type = string
}

variable "availability_zone_2" {
  type = string
}

variable "vpc_security_group_ids" {
  type = list(string)
}
