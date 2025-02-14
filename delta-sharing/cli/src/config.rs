use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub backend: BackendConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BackendConfig {
    pub database: String,
}
