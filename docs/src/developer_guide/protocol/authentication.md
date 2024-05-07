# Authentication middleware

Like other areas of the Delta Sharing server, it is possible to extend the server by implementing your own authentication middleware.

## How is authentication/authorization handled?

The handlers for all of the routes in the Delta Sharing protocol router expect a request extension with the `RecipientId`. If this extension is not set, the handler will return an error response saying the request is unauthenticated.
The `RecipientId` is the type that identifies the client that is calling the server (or is set to `RecipientId::Unknown` if the client could/should not be identified).
Once the request reaches the route handlers the `RecipientId` is used to determine if the client has the necessary permissions to access the requested data.

### Example

An example of custom middleware can be found below. In this example the middleware will authenticate resuests based on a hardcoded password. If the password is correct, the `RecipientId` is set to `RecipientId::anonymous()` and proceeds to the route handler. If the password is incorrect, the middleware will return an unauthorized response.

```rust
const SUPER_SECRET_PASSWORD: &str = "delta-sharing-is-caring";

async fn auth(mut request: Request, next: Next) -> Result<Response, ServerError> {
    if let Some(token) = request.headers().get(AUTHORIZATION) {
        let token = token.to_str().unwrap();
        if token == SUPER_SECRET_PASSWORD {
            tracing::info!(client_id=%client_id, "authorized");

            let client_id = RecipientId::anonymous();
            request.extensions_mut().insert(client_id);

            let response = next.run(request).await;
            return Ok(response);
        }
    }

    Err(ServerError::unauthorized(""))
}

let mut state = SharingServerState::new(...);
let svc = build_sharing_server_router(Arc::new(state));

// Add custom authentication middleware here
let app = svc
    .layer(middleware::from_fn(auth));

let listener = TcpListener::bind("127.0.0.1:0")
    .await
    .expect("Could not bind to socket");
axum::serve(listener, app).await.expect("server error");
```

// TODO: explain policy module


## What's in the box?

The Delta Sharing library comes with a pre-built authentication middleware that can be used out of the box.

// TODO: write about pre-built middleware


