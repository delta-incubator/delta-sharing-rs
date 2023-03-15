use config::Config;
use config::File;
use glob::glob;
use once_cell::sync::Lazy;
use tracing::info;
use tracing::warn;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut glob_path = "config/dev/*";
    let mode = match std::env::var("RUST_ENV") {
        Ok(value) => value,
        Err(_) => String::new(),
    };
    if mode.eq("production") {
        glob_path = "config/prod/*";
        info!(r#"RUST_ENV = "{}""#, mode);
    }
    let mut builder = Config::builder();
    if let Ok(paths) = glob(glob_path) {
        for entry in paths {
            match entry {
                Ok(path) => {
                    builder = builder.add_source(File::from(path));
                }
                Err(e) => warn!(?e),
            }
        }
    }
    builder.build().unwrap_or(Config::default())
});

pub struct Flag<V> {
    pub key: V,
}

pub trait Fetch<T>: private::Sealed {
    fn fetch(&self, config: &Config) -> T;
}

impl<V> Fetch<bool> for Flag<V>
where
    V: std::fmt::Debug + std::fmt::Display,
{
    fn fetch(&self, config: &Config) -> bool {
        let value = match config.get::<String>(&self.key.to_string()) {
            Ok(value) => value.to_string(),
            _ => String::new(),
        };
        if value.eq("true") || value.eq("false") {
            match value.parse::<bool>() {
                Ok(value) => value,
                Err(e) => panic!(
                    r#"Unknown error parsing configuration flag "{}": {:?}"#,
                    &self.key, e
                ),
            }
        } else {
            false
        }
    }
}

impl<V> Fetch<String> for Flag<V>
where
    V: std::fmt::Debug + std::fmt::Display,
{
    fn fetch(&self, config: &Config) -> String {
        match config.get::<String>(&self.key.to_string()) {
            Ok(value) => value.to_string(),
            _ => String::new(),
        }
    }
}

mod private {
    pub trait Sealed {}
    impl<V> Sealed for super::Flag<V> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_get() {
        let config = CONFIG.clone();
        assert!(matches!(config.get::<String>("db_url"), Ok(_)));
        assert!(matches!(config.get::<String>("kvs_url"), Ok(_)));
        assert!(matches!(config.get::<String>("server_addr"), Ok(_)));
        assert!(matches!(config.get::<String>("server_bind"), Ok(_)));
        assert!(matches!(config.get::<String>("use_json_log"), Ok(_)));
        assert!(matches!(config.get::<String>("log_filter"), Ok(_)));
    }
}
