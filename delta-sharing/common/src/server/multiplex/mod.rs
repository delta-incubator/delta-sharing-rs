use std::path::Path;
use std::sync::Arc;

use axum::{extract::Request, routing::get, Router};
use futures::{future::BoxFuture, ready};
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tokio::signal;
use tonic::IntoRequest;
use tower::{Service, ServiceExt};
use tower_http::trace::TraceLayer;

use self::service::*;
use crate::error::{Error, Result};
use crate::models::v1::delta_sharing_service_server::DeltaSharingServiceServer;
use crate::policies::ConstantPolicy;
use crate::rest::auth::{AnonymousAuthenticator, AuthenticationLayer};
use crate::rest::get_rest_router;
use crate::{DeltaSharingHandler, InMemoryConfig, InMemoryHandler, KernelQueryHandler};

mod service;

async fn run_hybrid_server(
    config: impl AsRef<Path>,
    host: impl AsRef<str>,
    port: u16,
) -> Result<()> {
    let config = std::fs::read_to_string(config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;
    let config = serde_yml::from_str::<InMemoryConfig>(&config)
        .map_err(|_| Error::Generic("malformed config".to_string()))?;

    let discovery = Arc::new(InMemoryHandler::new(config));
    let state = DeltaSharingHandler {
        query: KernelQueryHandler::new_multi_thread(discovery.clone(), Default::default()),
        discovery,
        policy: Arc::new(ConstantPolicy::default()),
    };

    let listener = TcpListener::bind(format!("{}:{}", host.as_ref(), port))
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    let rest = get_rest_router(state.clone())
        .layer(AuthenticationLayer::new(AnonymousAuthenticator))
        .layer(TraceLayer::new_for_http());

    let grpc = tonic::transport::Server::builder()
        .add_service(DeltaSharingServiceServer::new(state))
        .into_service();

    let service = MultiplexService::new(rest, grpc);

    // axum::serve(listener, tower::make::Shared::new(service));
    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        // let io = TokioIo::new(stream);

        // let svc = service.clone();

        // Spawn a task to handle the connection. That way we can handle multiple connections
        // concurrently.
        tokio::spawn(async move {
            // Hyper has its own `AsyncRead` and `AsyncWrite` traits and doesn't use tokio.
            // `TokioIo` converts between them.
            let socket = TokioIo::new(stream);

            // Hyper also has its own `Service` trait and doesn't use tower. We can use
            // `hyper::service::service_fn` to create a hyper `Service` that calls our app through
            // `tower::Service::call`.
            let hyper_service = service_fn(move |request: Request<Incoming>| {
                // We have to clone `tower_service` because hyper's `Service` uses `&self` whereas
                // tower's `Service` requires `&mut self`.
                //
                // We don't need to call `poll_ready` since `Router` is always ready.
                service.clone().call(request)
            });
            let router = TowerToHyperService::new(service.clone());

            // let builder = server::conn::auto::Builder::new(TokioExecutor::new());

            // `server::conn::auto::Builder` supports both http1 and http2.
            //
            // `TokioExecutor` tells hyper to use `tokio::spawn` to spawn tasks.
            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                // `serve_connection_with_upgrades` is required for websockets. If you don't need
                // that you can use `serve_connection` instead.
                .serve_connection_with_upgrades(socket, hyper_service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });

        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        // let service = service_fn(move |_req| async move {
        //     let response = svc.call();
        //     Ok::<_, Error>(Response::new(Full::new(Bytes::from(format!(
        //         "Request #{}",
        //         count
        //     )))))
        // });

        // Spawn a tokio task to serve multiple connections concurrently
        // tokio::task::spawn(async move {
        //     // Bind the incoming connection to our service
        //     if let Err(err) = http2::Builder::new(TokioExecutor)
        //         // `service_fn` converts our function in a `Service`
        //         .serve_connection(io, svc)
        //         .await
        //     {
        //         eprintln!("Error serving connection: {:?}", err);
        //     }
        // });
    }

    Ok(())
}
