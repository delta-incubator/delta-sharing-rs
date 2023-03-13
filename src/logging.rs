mod tracing;
use crate::config::Config;

pub fn setup(config: &Config) {
    tracing::init(config.use_json_log, &config.log_filter)
}
