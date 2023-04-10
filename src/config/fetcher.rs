use config::Config;
use config::File;
use glob::glob;
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let conf_dir = std::env::var("KOTOSIRO_CONF_DIR").unwrap_or("config/dev".to_string());
    let glob_path = format!("{}/*", conf_dir);
    let mut builder = Config::builder();
    if let Ok(paths) = glob(&glob_path) {
        for entry in paths {
            match entry {
                Ok(path) => {
                    builder = builder.add_source(File::from(path));
                }
                Err(e) => tracing::warn!(?e),
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

impl<V> Fetch<usize> for Flag<V>
where
    V: std::fmt::Debug + std::fmt::Display,
{
    fn fetch(&self, config: &Config) -> usize {
        let value = match config.get::<String>(&self.key.to_string()) {
            Ok(value) => value.to_string(),
            _ => String::new(),
        };
        match value.parse::<usize>() {
            Ok(value) => value,
            Err(e) => panic!(
                r#"Unknown error parsing configuration flag "{}": {:?}"#,
                &self.key, e
            ),
        }
    }
}

impl<V> Fetch<u32> for Flag<V>
where
    V: std::fmt::Debug + std::fmt::Display,
{
    fn fetch(&self, config: &Config) -> u32 {
        let value = match config.get::<String>(&self.key.to_string()) {
            Ok(value) => value.to_string(),
            _ => String::new(),
        };
        match value.parse::<u32>() {
            Ok(value) => value,
            Err(e) => panic!(
                r#"Unknown error parsing configuration flag "{}": {:?}"#,
                &self.key, e
            ),
        }
    }
}

impl<V> Fetch<i32> for Flag<V>
where
    V: std::fmt::Debug + std::fmt::Display,
{
    fn fetch(&self, config: &Config) -> i32 {
        let value = match config.get::<String>(&self.key.to_string()) {
            Ok(value) => value.to_string(),
            _ => String::new(),
        };
        match value.parse::<i32>() {
            Ok(value) => value,
            Err(e) => panic!(
                r#"Unknown error parsing configuration flag "{}": {:?}"#,
                &self.key, e
            ),
        }
    }
}

impl<V> Fetch<u64> for Flag<V>
where
    V: std::fmt::Debug + std::fmt::Display,
{
    fn fetch(&self, config: &Config) -> u64 {
        let value = match config.get::<String>(&self.key.to_string()) {
            Ok(value) => value.to_string(),
            _ => String::new(),
        };
        match value.parse::<u64>() {
            Ok(value) => value,
            Err(e) => panic!(
                r#"Unknown error parsing configuration flag "{}": {:?}"#,
                &self.key, e
            ),
        }
    }
}

impl<V> Fetch<i64> for Flag<V>
where
    V: std::fmt::Debug + std::fmt::Display,
{
    fn fetch(&self, config: &Config) -> i64 {
        let value = match config.get::<String>(&self.key.to_string()) {
            Ok(value) => value.to_string(),
            _ => String::new(),
        };
        match value.parse::<i64>() {
            Ok(value) => value,
            Err(e) => panic!(
                r#"Unknown error parsing configuration flag "{}": {:?}"#,
                &self.key, e
            ),
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
        assert!(matches!(config.get::<String>("server_addr"), Ok(_)));
        assert!(matches!(config.get::<String>("server_bind"), Ok(_)));
        assert!(matches!(config.get::<String>("admin_name"), Ok(_)));
        assert!(matches!(config.get::<String>("admin_email"), Ok(_)));
        assert!(matches!(config.get::<String>("admin_password"), Ok(_)));
        assert!(matches!(config.get::<String>("admin_namespace"), Ok(_)));
        assert!(matches!(config.get::<i64>("admin_ttl"), Ok(_)));
        assert!(matches!(config.get::<String>("jwt_secret"), Ok(_)));
        assert!(matches!(config.get::<String>("gcp_sa_private_key"), Ok(_)));
        assert!(matches!(config.get::<String>("aws_profile"), Ok(_)));
        assert!(matches!(config.get::<String>("aws_region"), Ok(_)));
        assert!(matches!(config.get::<String>("use_json_log"), Ok(_)));
        assert!(matches!(config.get::<String>("log_filter"), Ok(_)));
    }
}
