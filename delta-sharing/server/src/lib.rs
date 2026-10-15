// `async_trait` adds `#[must_use]` to every trait method, which clippy reports as a duplicate of
// the `must_use` on the boxed future it returns. The attribute comes from the macro.
#![allow(clippy::double_must_use)]

pub use capabilities::*;

// #[cfg(feature = "grpc")]
// pub use grpc::run_server as run_grpc_server;
#[cfg(feature = "rest")]
pub use rest::run_server_full as run_rest_server_full;

mod capabilities;
// #[cfg(feature = "grpc")]
// mod grpc;
#[cfg(feature = "rest")]
mod rest;
mod shutdown;
