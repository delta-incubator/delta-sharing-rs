mod fetcher;
use crate::server::utils::bootstrap::JwtKeys;
use once_cell::sync::Lazy;

pub(crate) static JWT_SECRET: Lazy<JwtKeys> = Lazy::new(|| {
    let secret = fetch::<String>("jwt_secret");
    JwtKeys::new(secret.as_bytes())
});

pub fn fetch<T>(flag: &str) -> T
where
    fetcher::Flag<String>: fetcher::Fetch<T>,
{
    let config = fetcher::CONFIG.clone();
    let flag = fetcher::Flag {
        key: String::from(flag),
    };
    <fetcher::Flag<String> as fetcher::Fetch<T>>::fetch(&flag, &config)
}
