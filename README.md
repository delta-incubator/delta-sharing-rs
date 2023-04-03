Kotosiro Sharing Server
==============================

Supported Platforms
==============================

| Amazon AWS      | Google GCP       | Microsoft Azure |
|:---------------:|:----------------:|:---------------:|
| :green_square:  | :green_square:   | :red_square:    |

Cofigure Credentials for Cloud Storage Backends
==============================
 1. **Amazon AWS**
 To access the S3 Delta Table backend, you must prepare an IAM user with an Amazon S3 permissions policy.
 After creating the IAM user, you must configure the profile name and the region to enable Kotosiro Sharing
 Server can access the S3 bucket. The location of credentials file is specified with the environment variable
 `AWS_SHARED_CREDENTIALS_FILE` otherwise "~/.aws/credentials".
 
  1. **Google GCP**
 To access the GCS Delta Table backend, you must prepare a GCS service account. After creating the service account,
 you must configure the location of the GCP service account private key JSON.

API
==============================

[SEE ALSO](https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md)

| Status             | Official       | Method | URL                                                                |
|:------------------:|:--------------:|--------|--------------------------------------------------------------------|
| :heavy_check_mark: | :red_square:   | GET    | */swagger-ui*                                                      |
| :heavy_check_mark: | :red_square:   | POST   | */admin/login*                                                     |
|                    | :red_square:   | GET    | */admin/token*                                                     |
| :heavy_check_mark: | :red_square:   | GET    | */admin/accounts*                                                  |
| :heavy_check_mark: | :red_square:   | POST   | */admin/accounts*                                                  |
| :heavy_check_mark: | :red_square:   | GET    | */admin/accounts/{account}*                                        |
| :heavy_check_mark: | :red_square:   | POST   | */admin/shares*                                                    |
| :heavy_check_mark: | :red_square:   | GET    | */admin/tables*                                                    |
| :heavy_check_mark: | :red_square:   | POST   | */admin/tables*                                                    |
| :heavy_check_mark: | :red_square:   | GET    | */admin/tables/{table}*                                            |
| :heavy_check_mark: | :red_square:   | POST   | */admin/shares/{share}/schemas/{schema}/tables*                    |
|                    | :red_square:   | POST   | */admin/shares/{share}/all-tables*                                 |
| :heavy_check_mark: | :green_square: | GET    | */shares*                                                          |
| :heavy_check_mark: | :green_square: | GET    | */shares/{share}*                                                  |
| :heavy_check_mark: | :green_square: | GET    | */shares/{share}/schemas*                                          |
| :heavy_check_mark: | :green_square: | GET    | */shares/{share}/schemas/{schema}/tables*                          |
| :heavy_check_mark: | :green_square: | GET    | */shares/{share}/all-tables*                                       |
| :heavy_check_mark: | :green_square: | GET    | */shares/{share}/schemas/{schema}/tables/{table}/version*          |
| :heavy_check_mark: | :green_square: | GET    | */shares/{share}/schemas/{schema}/tables/{table}/metadata*         |
|                    | :green_square: | POST   | */shares/{share}/schemas/{schema}/tables/{table}/query*            |
|                    | :green_square: | GET    | */shares/{share}/schemas/{schema}/tables/{table}/changes*          |

TODO
==============================

 - [ ] README
 - [ ] Wiki
 - [ ] Dockerfile
 - [ ] Kubernetes
 - [ ] React/Frontend
 - [ ] CORS
