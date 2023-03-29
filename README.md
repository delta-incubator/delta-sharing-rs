Kotosiro Sharing Server
==============================

API
==============================

[SEE ALSO](https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md)

| Status             | Official           | Method | URL                                                                |
|:------------------:|:------------------:| ------ | ------------------------------------------------------------------ |
| :heavy_check_mark: |                    | GET    | */swagger-ui*                                                      |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/login*                                             |
|                    |                    | GET    | *{prefix}/admin/token*                                             |
| :heavy_check_mark: |                    | GET    | *{prefix}/admin/accounts*                                          |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/accounts*                                          |
| :heavy_check_mark: |                    | GET    | *{prefix}/admin/accounts/{account}*                                |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/shares*                                            |
| :heavy_check_mark: |                    | GET    | *{prefix}/admin/tables*                                            |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/tables*                                            |
| :heavy_check_mark: |                    | GET    | *{prefix}/admin/tables/{table}*                                    |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/shares/{share}/schemas/{schema}/tables*            |
|                    |                    | POST   | *{prefix}/admin/shares/{share}/all-tables*                         |
| :heavy_check_mark: | :white_check_mark: | GET    | *{prefix}/shares*                                                  |
| :heavy_check_mark: | :white_check_mark: | GET    | *{prefix}/shares/{share}*                                          |
| :heavy_check_mark: | :white_check_mark: | GET    | *{prefix}/shares/{share}/schemas*                                  |
| :heavy_check_mark: | :white_check_mark: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables*                  |
| :heavy_check_mark: | :white_check_mark: | GET    | *{prefix}/shares/{share}/all-tables*                               |
|                    | :white_check_mark: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/version*  |
|                    | :white_check_mark: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/metadata* |
|                    | :white_check_mark: | POST   | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/query*    |
|                    | :white_check_mark: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/changes*  |

TODO
==============================

 - [ ] README
 - [ ] Wiki
 - [ ] Dockerfile
 - [ ] Kubernetes
 - [ ] React/Frontend
 - [ ] CORS
