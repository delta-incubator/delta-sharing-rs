Kotosiro Sharing Server
==============================

TODO
==============================

[API Specification](https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md)

| Check              | Method | URL                                                                | Official           |
| ------------------ | ------ | ------------------------------------------------------------------ | ------------------ |
| :heavy_check_mark: | GET    | *{prefix}/shares*                                                  | :heavy_check_mark: |
| :heavy_check_mark: | GET    | *{prefix}/shares/{share}*                                          | :heavy_check_mark: |
|                    | GET    | *{prefix}/shares/{share}/schemas*                                  | :heavy_check_mark: |
|                    | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables*                  | :heavy_check_mark: |
|                    | GET    | *{prefix}/shares/{share}/all-tables*                               | :heavy_check_mark: |
|                    | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/version*  | :heavy_check_mark: |
|                    | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/metadata* | :heavy_check_mark: |
|                    | POST   | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/query*    | :heavy_check_mark: |
|                    | GET    | *{prefix}/shares/{share}/schemas/{schema}/tables/{table}/changes*  | :heavy_check_mark: |
