Kotosiro Sharing Server
==============================

 Kotosiro Sharing is a Rust-based Delta Sharing Server that includes administration functionality.
Unlike [the reference implementation of a Delta Sharing Server](https://github.com/delta-io/delta-sharing),
which primarily focuses on the API specification and uses static file-based sharing information,
Kotosiro Sharing manages its sharing information through an API.

<p align="center">
  <img src="images/delta-sharing.png" width="85%"/>
</p>

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
 
Starting the Development Server
==============================

 Since the implementation is still in the early stages, only the development server is currently available. A Helm
chart will be added to the project in the near future.
 
 To run the development server, execute the following commands in this directory:

```bash
  $ just docker
  $ just server
 ```

 To run the unit tests, execute the following commands in this directory:

 ```bash
  $ just docker
  $ just test
  $ just testdb
 ```
 
Create a New Sharing via the API
==============================

 Once you've started the development server, you can create a new sharing via the API. Follow these steps:
 
 1. Log in to Kotosiro Sharing and get the admin access token by running the following command:
 
 ```bash
 $ curl -s -X POST http://localhost:8080/admin/login -H "Content-Type: application/json" -d '{"account": "kotosiro", "password": "password"}' | jq '.'
{
  "profile": {
    "shareCredentialsVersion": 1,
    "endpoint": "http://127.0.0.1:8080",
    "bearerToken": "YOUR_ADMIN_ACCESS_TOKEN",
    "expirationTime": "2023-04-09 19:34:04 UTC"
  }
}
 ```
 
 2. Register a new share by running the following command:
 
 ```bash
  $ curl -s -X POST "http://localhost:8080/admin/shares" -H "Authorization: Bearer YOUR_ADMIN_ACCESS_TOKEN" -H "Content-Type: application/json" -d'{ "name": "share1" }' | jq '.'
{
  "share": {
    "id": "6986c361-5e6a-4554-b698-11875d6598e0",
    "name": "share1"
  }
}
 ```
 
 3. Register a new table by running the following command:

```bash
 $ curl -s -X POST "http://localhost:8080/admin/tables" -H "Authorization: Bearer YOUR_ADMIN_ACCESS_TOKEN" -H "Content-Type: application/json" -d'{ "name": "table1", "location": "s3://kotosiro-sharing-test/examination" }' | jq '.'
{
  "table": {
    "id": "579df9cd-a674-459d-9599-d38d54583cd0",
    "name": "table1",
    "location": "s3://kotosiro-sharing-test/examination"
  }
}
```

 4. Register a new table as a part of schema1 in the share1 by running the following command:
 
```bash
 $ curl -s -X POST "http://localhost:8080/admin/shares/share1/schemas/schema1/tables" -H "Authorization: Bearer YOUR_ADMIN_ACCESS_TOKEN" -H "Content-Type: application/json" -d'{ "table": "table1" }' | jq '.'
{
  "schema": {
    "id": "689ed733-bec8-4796-a2dd-4f82dce6beab",
    "name": "schema1"
  }
}
```

 5. Issue a new recipient profile by running the following command:

```bash
 $ curl -s -X GET "http://localhost:8080/admin/profile" -H "Authorization: Bearer YOUR_ADMIN_ACCESS_TOKEN" -H "Content-Type: application/json" | jq '.'
{
  "profile": {
    "shareCredentialsVersion": 1,
    "endpoint": "http://127.0.0.1:8080",
    "bearerToken": "YOUR_RECIPIENT_ACCESS_TOKEN",
    "expirationTime": "2023-04-09 19:55:19 UTC"
  }
}
```

Kotosiro Sharing per Server Configuration
==============================

 Will be available in the near future. [SEE ALSO](/config)

API
==============================

[SEE ALSO](https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md)

| Status             | Official       | Method | URL                                                                |
|:------------------:|:--------------:|--------|--------------------------------------------------------------------|
| :heavy_check_mark: | :red_square:   | GET    | */swagger-ui*                                                      |
| :heavy_check_mark: | :red_square:   | POST   | */admin/login*                                                     |
| :heavy_check_mark: | :red_square:   | GET    | */admin/profile*                                                   |
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
| :heavy_check_mark: | :green_square: | POST   | */shares/{share}/schemas/{schema}/tables/{table}/query*            |
|                    | :green_square: | GET    | */shares/{share}/schemas/{schema}/tables/{table}/changes*          |

TODO
==============================

- [ ] API
  - [ ] CDF Related API
- [ ] Documentation
  - [x] README
  - [ ] Wiki
- [ ] DevOps
  - [ ] Dockerfile
  - [ ] Helm Chart
- [ ] Admin Console (React/Frontend)
- [ ] Data Access Audit
  - [ ] Enrich Access Log
  - [ ] Share Namespaces
  - [ ] Token Blacklist
