use chrono::NaiveDateTime;
use chrono::Utc;
use rand::Rng;
use uuid::Uuid;

static CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub fn i32(lower: i32, upper: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower..upper)
}

pub fn i64(lower: i64, upper: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower..upper)
}

pub fn usize(upper: usize) -> usize {
    rand::random::<usize>() % upper
}

pub fn bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..CHARSET.len()) % 2 == 0
}

pub fn string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let ret: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    ret
}

pub fn bytes(length: usize) -> Vec<u8> {
    (0..length).map(|_| rand::random::<u8>()).collect()
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn port() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..65536)
}

pub fn ip() -> String {
    format!(
        "{}.{}.{}.{}:{}",
        i32(0, 255),
        i32(0, 255),
        i32(0, 255),
        i32(0, 255),
        port()
    )
}

pub fn url() -> String {
    format!("{}://{}:{}", string(5), string(10), port())
}

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn choice<T>(candidates: &Vec<T>) -> &T {
    &candidates[self::usize(candidates.len())]
}
