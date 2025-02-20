# Architecture

The overall design of delta-sharing-rs focusses on modularity and ease of implementation -
i.e. performance is a secondary concern. As it turns out, while the API surface area
of the delta sharing protocol is quite narrow, there are a number of additional reosurces
that need to be managed in order to provide an operational system.

-   `StorageLocation`s and `Credential`s are required to read table data
-   `Shares`s and `Schema`s need to be created before they can be read
-   `Table`s need to be registered and organized in `Share`s

However, some services that wants to expose a delta-sharing compliant APIs may already have
facilities for managing such resources, as they are only partially specific to the sharing protocol.

On top of that, any productive system needs to have governance enabled, especially since the main
purpose of delta sharing is to share data with multiple external parties. The protocol defines
profile files that include an access token for clients to use when calling the API, but specific
implementations may decide to opt for other means of authentication.

We are thus pursuing two goals with this implementation.

-   make it simple for services to expose delta shaing APIs
-   provide a battieries inclluded implementation of a fully functional sharing server.

## Concepts

Let's dive deeper into the main abstractions and implementations that make up this imlementation.

### Resources

A core responsibility is to manage various `Resource`s that are required to provide the service.
These resources need to be identified and we need to agree on how we do that. There are two ways:

-   via a name and namespace - a namespace being an arbitrary length array of string
-   via a uuids, predominantly managed by the backends

Uuids are globally unique, while name / namespace references (may) require a resource label to
uniquely map to a specific resource.

### API Surfaces

Differnet functionality is broken down into separate API surfaces that can be adopted selectively
accroding to the needs of a service. The crate then defines REST routers or gPRC services that
can expose these API surfaces. The creation of these requires structs that implement certain traits.

One overarching piece of functionality that all APIs require is a [`Policy`](#policies). There is always
exactly one policy per server, but more on that later.

Each API surface defines a handler trait, which when combined with a Policy forms a manager: `Handler + Policy = Manager`

The handlers directly map to the request / response structure of the APIs, but what we really need is
to manage the resources exposed by the API.

The traits that are directly consumed by the API implementations are called `Managers`. In most cases
these traits do not need to be implemented directly, but rather via more aggregate traits.

=== "Discovery"

    ```rs
    --8<-- "delta-sharing/common/src/lib.rs:discovery-handler"
    ```

=== "Table Queries"

    ```rs
    --8<-- "delta-sharing/common/src/lib.rs:table-query-handler"
    ```

=== "Sharing Repository"

    ```rs
    --8<-- "delta-sharing/common/src/lib.rs:sharing-repository-handler"
    ```

=== "Storage Locations / Credentials"

### Policies

## TODO

-   repositories manage resources
-   handlers process requests

we define a set of services and an associated handler for each service.
Repositories manage resources and implememnt handlers for services.

A base assumtion is how resources are identified - i.e. namespace / uuid
the spec already defines some of this ... but not how that translates to management.

The very basic server exposes just the official sharing APIs, it requires:

-   A Discovery handler
-   A table resolution service
-   A policy for controlling access
