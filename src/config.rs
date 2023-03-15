pub mod fetcher;

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
