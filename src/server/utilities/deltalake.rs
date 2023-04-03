use crate::config;
use anyhow::Context;
use anyhow::Result;
use deltalake::delta::open_table_with_storage_options;
use deltalake::delta::DeltaTable;
use std::collections::hash_map::HashMap;

pub struct Utility;

impl Utility {
    pub async fn open_table(location: &str) -> Result<DeltaTable> {
        open_table_with_storage_options(
            location,
            HashMap::from([
                (
                    String::from("google_service_account_path"),
                    config::fetch::<String>("gcp_sa_private_key"),
                ),
                (
                    String::from("region"),
                    config::fetch::<String>("aws_region"),
                ),
                (
                    String::from("profile"),
                    config::fetch::<String>("aws_profile"),
                ),
            ]),
        )
        .await
        .context("failed to open delta table")
    }
}
