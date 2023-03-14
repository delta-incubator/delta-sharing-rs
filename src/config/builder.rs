use config::builder::DefaultState;
use config::ConfigBuilder;
use config::Environment;
use config::File;
use config::FileFormat;
use std::path::Path;

pub fn new(path: Option<&Path>) -> ConfigBuilder<DefaultState> {
    let mut builder = config::Config::builder();
    builder = builder.add_source(File::from_str(
        include_str!("defaults.toml"),
        FileFormat::Toml,
    ));
    if let Some(path) = path {
        builder = builder.add_source(File::from(path));
    }
    builder.add_source(Environment::with_prefix("KOTOSIRO_SHARING").try_parsing(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::path::Path;

    #[test]
    #[serial]
    fn test_new_some() {
        let db_url: String = testutils::rand::url();
        let server_addr: String = testutils::rand::ip();
        let server_bind: String = testutils::rand::ip();
        let use_json_log: bool = testutils::rand::bool();
        let log_filter: String = testutils::rand::string(20);
        let config = format!(
            include_str!("config.tmpl"),
            db_url = &db_url,
            server_addr = &server_addr,
            server_bind = &server_bind,
            use_json_log = &use_json_log,
            log_filter = &log_filter
        );
        let path = testutils::io::persist(&config, Path::new("./config.toml"))
            .expect("path should be created");
        let config: crate::config::Config = new(Some(path))
            .build()
            .expect("builder should be able to build configuration")
            .try_deserialize()
            .expect("config object must be loaded");
        assert_eq!(&db_url, &config.db_url);
        assert_eq!(&server_addr, &config.server_addr);
        assert_eq!(&server_bind, &config.server_bind);
        assert_eq!(&use_json_log, &config.use_json_log);
        assert_eq!(&log_filter, &config.log_filter);
        testutils::io::remove(&path).expect("temporary confiiguration file should be removed");
    }

    #[test]
    #[serial]
    fn test_new_none() {
        let db_url: String = testutils::rand::url();
        let server_addr: String = testutils::rand::ip();
        let server_bind: String = testutils::rand::ip();
        let use_json_log: bool = testutils::rand::bool();
        let log_filter: String = testutils::rand::string(20);
        env::set_var("KOTOSIRO_SHARING_DB_URL", &db_url);
        env::set_var("KOTOSIRO_SHARING_SERVER_ADDR", &server_addr);
        env::set_var("KOTOSIRO_SHARING_SERVER_BIND", &server_bind);
        env::set_var("KOTOSIRO_SHARING_USE_JSON_LOG", use_json_log.to_string());
        env::set_var("KOTOSIRO_SHARING_LOG_FILTER", &log_filter);
        let config: crate::config::Config = new(None)
            .build()
            .expect("builder should be able to build configuration")
            .try_deserialize()
            .expect("config object must be loaded");
        assert_eq!(&db_url, &config.db_url);
        assert_eq!(&server_addr, &config.server_addr);
        assert_eq!(&server_bind, &config.server_bind);
        assert_eq!(&use_json_log, &config.use_json_log);
        assert_eq!(&log_filter, &config.log_filter);
        env::remove_var("KOTOSIRO_SHARING_DB_URL");
        env::remove_var("KOTOSIRO_SHARING_SERVER_ADDR");
        env::remove_var("KOTOSIRO_SHARING_SERVER_BIND");
        env::remove_var("KOTOSIRO_SHARING_USE_JSON_LOG");
        env::remove_var("KOTOSIRO_SHARING_LOG_FILTER");
    }
}
