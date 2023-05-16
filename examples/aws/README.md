Kotosiro Sharing Server on AWS
==============================

 In this README, we will provide a step-by-step guide on how to deploy your Kotosiro Sharing application with HTTPS
enabled in an AWS environment. The target architecture is as follows:

<p align="center">
  <img src="images/architecture.png" width="50%"/>
</p>

Acquiring Your Domain
==============================

 Before proceeding with the instructions, you will need to acquire your own domain name to enable HTTPS. We purchased
one from [Amazon Route 53](https://aws.amazon.com/route53/), but you can also find another (free) domain registry,
such as [Netfy](https://netfy.domains/).

Provisioning AWS Infrastructure
==============================

 To provision the AWS infrastructure, run the following commands in this directory:

```bash
 $ terraform init
 $ terraform apply
```

 You need to configure at least the following Terraform variables in `terraform.tfvars`:

```terraform: terraform.tfvars
name                = "kotosiro"
region              = "your-aws-region"
availability_zone_1 = "your-aws-availability-zone-a"
availability_zone_2 = "your-aws-availability-zone-b"
ssh_cidr_blocks     = ["0.0.0.0/0"]
domain              = "your.domain.org"
```

 Please note that `ssh_cidr_blocks` must be configured to only accept `ssh` connections from proper IP addresses for
the EC2 instances.
 To configure other variables, such as the Postgres admin password, please refer to the corresponding Terraform
modules and set the preferred values as you wish.
 
Deploying AWS credentials to EC2 Instances
==============================

 This is not a great way to handle AWS credentials. However, to deploy your AWS credentials to the EC2 servers,
you can follow these steps for each EC2 instance:

```bash
 $ chmod 400 creds/key.id_rsa
 $ ssh -i creds/key.id_rsa ec2-user@YOUR_EC2_IP_ADDRESS
 [ec2-user@ip-xxx-xxx-xxx-xxx ~]$ mkdir .aws
 [ec2-user@ip-xxx-xxx-xxx-xxx ~]$ logout
 $ scp -i creds/key.id_rsa path/to/aws/credentials ec2-user@YOUR_EC2_IP_ADDRESS:~/.aws/credentials
```

Setting Up Kotosiro Server
==============================

 To configure the Kotosiro Sharing server, follow these steps for each EC2 instance:

```bash
 $ ssh -i creds/key.id_rsa ec2-user@YOUR_EC2_IP_ADDRESS
 [ec2-user@ip-xxx-xxx-xxx-xxx ~]$ git clone https://github.com/kotosiro/docker-compose-files.git
 [ec2-user@ip-xxx-xxx-xxx-xxx ~]$ cd docker-compose-files/sharing
 [ec2-user@ip-xxx-xxx-xxx-xxx sharing]$ vi docker-compose.yaml
```

 The `docker-compose.yaml` file should look like this:

```yaml: docker-compose.yaml
version: "3.9"
services:
  app:
    image: kotosiro/sharing:latest
    restart: always
    environment:
      - AWS_SHARED_CREDENTIALS_FILE=/.aws/credentials
      - AWS_PROFILE=YOUR_AWS_PROFILE_NAME
      - AWS_REGION=YOUR_AWS_REGION
      - GOOGLE_APPLICATION_CREDENTIALS=/.gcp/service-account-file.json
      - KOTOSIRO_DB_URL=postgres://postgres:secretpassword@YOUR_AWS_RDS_ENDPOINT:5432/sharing
      - KOTOSIRO_SERVER_ADDR=https://your.domain.org
      - KOTOSIRO_SERVER_BIND=0.0.0.0:80
      - KOTOSIRO_ADMIN_NAME=kotosiro
      - KOTOSIRO_ADMIN_EMAIL=kotosiro@email.com
      - KOTOSIRO_ADMIN_PASSWORD=password
      - KOTOSIRO_ADMIN_NAMESPACE=kotosiro
      - KOTOSIRO_ADMIN_TTL=28800
      - KOTOSIRO_SIGNED_URL_TTL=28800
      - KOTOSIRO_JWT_SECRET=secret
      - KOTOSIRO_USE_JSON_LOG=true
      - KOTOSIRO_LOG_FILTER=warn,kotosiro_sharing=info
    ports:
      - 80:80
    volumes:
      - ~/.aws/credentials:/.aws/credentials
      - ~/.gcp/service-account-file.json:/.gcp/service-account-file.json
    command:
      server
```

 After finishing the configuration of your docker-compose.yaml file, run the following command to
start the Kotosiro Sharing daemon:

```bash
 [ec2-user@ip-xxx-xxx-xxx-xxx sharing]$ docker-compose up -d
```
