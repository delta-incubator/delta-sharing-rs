pub use capabilities::*;

#[cfg(feature = "grpc")]
pub use grpc::run_server as run_grpc_server;
#[cfg(feature = "rest")]
pub use rest::{
    run_server as run_rest_server, run_server_full as run_rest_server_full,
    run_server_full2 as run_rest_server_full2,
};

mod capabilities;
#[cfg(feature = "grpc")]
mod grpc;
#[cfg(feature = "rest")]
mod rest;
mod shutdown;

#[cfg(feature = "rest")]
#[cfg(test)]
mod tests {
    macro_rules! maybe_skip_dat {
        () => {
            if testutils::dat::find_dat_dir().is_none() {
                eprintln!("Skipping integration test - set DAT_DATA_DIR");
                return;
            }
        };
    }
    pub(crate) use maybe_skip_dat;
}
