Kotosiro Sharing Server
==============================

API
==============================

[SEE ALSO](https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md)

| Status             | Official       | Method | URL                                                                |
|:------------------:|:--------------:|--------|--------------------------------------------------------------------|
| :heavy_check_mark: | :red_square:   | GET    | */swagger-ui*                                                      |
| :heavy_check_mark: | :red_square:   | POST   | *{prefix}/admin/login*                                             |
|                    | :red_square:   | GET    | *{prefix}/admin/token*                                             |
| :heavy_check_mark: | :red_square:   | GET    | *{prefix}/admin/accounts*                                          |
| :heavy_check_mark: | :red_square:   | POST   | *{prefix}/admin/accounts*                                          |
| :heavy_check_mark: | :red_square:   | GET    | *{prefix}/admin/accounts/{account}*                                |
| :heavy_check_mark: | :red_square:   | POST   | *{prefix}/admin/shares*                                            |
| :heavy_check_mark: | :red_square:   | GET    | *{prefix}/admin/tables*                                            |
| :heavy_check_mark: | :red_square:   | POST   | *{prefix}/admin/tables*                                            |
| :heavy_check_mark: | :red_square:   | GET    | *{prefix}/admin/tables/{table}*                                    |
| :heavy_check_mark: | :red_square:   | POST   | *{prefix}/admin/shares/{share}/schemas/{schema}/tables*            |
|                    | :red_square:   | POST   | *{prefix}/admin/shares/{share}/all-tables*                         |
| :heavy_check_mark: | :green_square: | GET    | *{prefix}/shares*                                                  |
| :heavy_check_mark: | :green_square: | GET    | *{prefix}/shares/{share}*                                          |
| :heavy_check_mark: | :green_square: | GET    | *{prefix}/shares/{share}/schemas*                                  |
| :heavy_check_mark: | :green_square: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables*                  |
| :heavy_check_mark: | :green_square: | GET    | *{prefix}/shares/{share}/all-tables*                               |
|                    | :green_square: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/version*  |
|                    | :green_square: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/metadata* |
|                    | :green_square: | POST   | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/query*    |
|                    | :green_square: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/changes*  |

TODO
==============================

 - [ ] README
 - [ ] Wiki
 - [ ] Dockerfile
 - [ ] Kubernetes
 - [ ] React/Frontend
 - [ ] CORS
