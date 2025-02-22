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
