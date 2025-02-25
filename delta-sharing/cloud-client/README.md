# cloud-client-rs

> [!IMPORTANT]
> This project is in large parts just the internal client from the [object_store] crate
> hoisted out into its own crate to make it useful for other projects.

By now there are comprehensive SDKs to interact with the APIs / services of the major cloud providers.
Each of these of course integrates with the respective cloud provider's identity and access management (IAM) system.

This works well in many scenarios, however in cases where 3rd party services are involved, that might be
running on a different cloud provider, or even on-premises, the situation is more complex. In these cases
one would have to pull in SDKs for each of the cloud providers involved, and manage the IAM for each of them.

While this is certainly possible, it is not ideal as one now needs to handle the superset of all
dependencies the SDKs bring in and translate between the different configuration systems.

This was one of the motivations for the development of [object_store].
In a similar vein, this project aims to provide a unified interface to integrate with the different
identity and access management systems of the major cloud providers.

Specifically, for scenarios where OSS or other projects want to build client libraries that can interact
that can make use of the various IdPs without having to pull in the SDKs of the respective cloud providers.

[object_store]: https://crates.io/crates/object_store
