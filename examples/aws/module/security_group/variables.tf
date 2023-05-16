variable "name" {
  type = string
}

variable "vpc_id" {
  type = string
}

variable "ssh_cidr_blocks" {
  type = list(string)
}
