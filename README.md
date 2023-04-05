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


 To access the S3 Delta Table backend, you need to create an IAM user with an Amazon S3 permissions policy.
Once you've created the IAM user, you must configure the profile name and region to allow Kotosiro Sharing
Server to access the S3 bucket. The location of the credentials file is specified by the environment variable
`AWS_SHARED_CREDENTIALS_FILE`. If this variable is not set, the credentials file should be located at `~/.aws/credentials`.
 
  2. **Google GCP**

 To access the GCS Delta Table backend, you need to create a GCS service account. Once you've created the service account,
you must configure the location of the GCP service account private key JSON.

  3. **Microsoft Azure**
  
 Microsoft Azure backed Delta Tables will be supported in the near future.

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
 - [ ] Audit (namespace, tenant, log etc...)
