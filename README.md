Kotosiro Sharing Server
==============================

TODO
==============================

 1. **[API Specification](https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md)**

| Check              | Official           | Method | URL                                                                |
| ------------------ | ------------------ | ------ | ------------------------------------------------------------------ |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/login*                                             |
|                    |                    | POST   | *{prefix}/admin/token*                                             |
| :heavy_check_mark: |                    | GET    | *{prefix}/admin/accounts*                                          |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/accounts*                                          |
| :heavy_check_mark: |                    | GET    | *{prefix}/admin/accounts/{account}*                                |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/shares*                                            |
| :heavy_check_mark: |                    | GET    | *{prefix}/admin/tables*                                            |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/tables*                                            |
| :heavy_check_mark: |                    | GET    | *{prefix}/admin/tables/{table}*                                    |
| :heavy_check_mark: |                    | POST   | *{prefix}/admin/shares/{share}/schemas/{schema}/tables*            |
|                    |                    | POST   | *{prefix}/admin/shares/{share}/all-tables*                         |
| :heavy_check_mark: | :heavy_check_mark: | GET    | *{prefix}/shares*                                                  |
| :heavy_check_mark: | :heavy_check_mark: | GET    | *{prefix}/shares/{share}*                                          |
| :heavy_check_mark: | :heavy_check_mark: | GET    | *{prefix}/shares/{share}/schemas*                                  |
| :heavy_check_mark: | :heavy_check_mark: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables*                  |
|                    | :heavy_check_mark: | GET    | *{prefix}/shares/{share}/all-tables*                               |
|                    | :heavy_check_mark: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/version*  |
|                    | :heavy_check_mark: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/metadata* |
|                    | :heavy_check_mark: | POST   | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/query*    |
|                    | :heavy_check_mark: | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/changes*  |

 2. **Infrastracture**
 
 - [ ] README
 - [ ] Wiki
 - [ ] Dockerfile
 - [ ] Kubernetes
 - [ ] React/Frontend
 - [ ] CORS
 - [ ] Token + Email + Date DB
