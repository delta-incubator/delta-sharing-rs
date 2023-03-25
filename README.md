Kotosiro Sharing Server
==============================

TODO
==============================

[API Specification](https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md)

|                    | METHOD | URL                                                              |          |
| ------------------ | ------ | ---------------------------------------------------------------- | -------- |
| :heavy_check_mark: | GET    | {prefix}/shares                                                  | OFFICIAL |
| :heavy_check_mark: | GET    | {prefix}/shares/{share}                                          | OFFICIAL |
|                    | GET    | {prefix}/shares/{share}/schemas                                  | OFFICIAL |
|                    | GET    | {prefix}/shares/{share}/schemas/{schema}/tables                  | OFFICIAL |
|                    | GET    | {prefix}/shares/{share}/all-tables                               | OFFICIAL |
|                    | GET    | {prefix}/shares/{share}/schemas/{schema}/tables/{table}/version  | OFFICIAL |
|                    | GET    | {prefix}/shares/{share}/schemas/{schema}/tables/{table}/metadata | OFFICIAL |
|                    | POST   | {prefix}/shares/{share}/schemas/{schema}/tables/{table}/query    | OFFICIAL |
|                    | GET    | {prefix}/shares/{share}/schemas/{schema}/tables/{table}/changes  | OFFICIAL |
