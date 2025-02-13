mod client;
mod error;

#[allow(
    clippy::enum_variant_names,
    clippy::empty_docs,
    clippy::large_enum_variant
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
