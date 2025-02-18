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

A set of managed sresources that goes beyond what the API surface exposes

-   credentials / storage locations
-   users
