variable "name" {
  type = string
}

variable "subnets" {
  type = list(string)
}

variable "security_groups" {
  type = list(string)
}

variable "vpc_id" {
  type = string
}

variable "target_id_1" {
  type = string
}

variable "target_id_2" {
  type = string
}
