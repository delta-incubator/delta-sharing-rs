Kotosiro Sharing Server
==============================

TODO
==============================

[API Specification](https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md)

|                    | METHOD | URL                                                              |          |
| ------------------ | ------ | ---------------------------------------------------------------- | -------- |
| :heavy_check_mark: | GET    | {prefix}/shares                                                  | OFFICIAL |
| :heavy_check_mark: | GET    | {prefix}/shares/{share}                                          | OFFICIAL |
| :white_check_mark: | GET    | {prefix}/shares/{share}/schemas                                  | OFFICIAL |
| :white_check_mark: | GET    | {prefix}/shares/{share}/schemas/{schema}/tables                  | OFFICIAL |
| :white_check_mark: | GET    | {prefix}/shares/{share}/all-tables                               | OFFICIAL |
| :white_check_mark: | GET    | {prefix}/shares/{share}/schemas/{schema}/tables/{table}/version  | OFFICIAL |
| :white_check_mark: | GET    | {prefix}/shares/{share}/schemas/{schema}/tables/{table}/metadata | OFFICIAL |
| :white_check_mark: | POST   | {prefix}/shares/{share}/schemas/{schema}/tables/{table}/query    | OFFICIAL |
| :white_check_mark: | GET    | {prefix}/shares/{share}/schemas/{schema}/tables/{table}/changes  | OFFICIAL |
