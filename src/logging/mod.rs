mod tracing;
use crate::config;

pub fn setup() {
    tracing::init(
        &config::fetch::<bool>("use_json_log"),
        &config::fetch::<String>("log_filter"),
    )
}
