# delta-sharing-rs

[Delta Sharing](https://delta.io/sharing) is an open protocol for secure real-time exchange of
large datasets, which enables organizations to share data in real time regardless of which
computing platforms they use. It is a simple [REST protocol] that securely shares access to part
of a cloud dataset and leverages modern cloud storage systems, such as S3, ADLS,or GCS, to reliably
transfer data.

While the [reference implementation] focuses on providing providing a service that can be used
to test connectors or run small instances with static or slow changing configuration, this project
aims to provide a more flexible and scalable implementation that can be tailored to specific use cases.

[reference implementation](https://github.com/delta-io/delta-sharing)
[REST protocol](https://github.com/delta-io/delta-sharing/blob/main/PROTOCOL.md)

## Getting Started

> [!IMPORTANT]
> This project is still under active development and is not ready for production use.
> Currently there are no releases yet, but you can build and run the project from source.

```sh
cargo run --bin delta-sharing rest
```
