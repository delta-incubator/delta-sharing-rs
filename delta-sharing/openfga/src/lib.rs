// Generated tonic code and the hand-written client return tonic types in `Result`. The error type
// wraps those types directly and is kept as-is to avoid changing the public API.
#![allow(clippy::result_large_err)]

mod client;
mod error;

#[allow(
    clippy::enum_variant_names,
    clippy::empty_docs,
    clippy::large_enum_variant,
    clippy::doc_overindented_list_items,
    dead_code
)]
pub(crate) mod gen {
    pub mod v1 {
        include!("gen/openfga.v1.rs");
    }
}

pub use client::{ClientConfig, OpenFgaClient};

#[cfg(test)]
mod tests {
    macro_rules! maybe_skip_fga {
        () => {
            if std::env::var("FGA_STORE_ID").is_err() {
                eprintln!("Skipping integration test - set FGA_STORE_ID");
                return;
            }
        };
    }
    pub(crate) use maybe_skip_fga;
}
