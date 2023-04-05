use crate::config;
use anyhow::Context;
use anyhow::Result;
use deltalake::delta::open_table_with_storage_options;
use deltalake::delta::DeltaTable;
use std::cmp::max;
use std::cmp::min;
use std::collections::hash_map::HashMap;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Interval<T>
where
    T: Copy + PartialOrd + Ord,
{
    min: T,
    max: T,
}

impl<T> fmt::Display for Interval<T>
where
    T: Copy + PartialOrd + Ord + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

impl<T> fmt::Debug for Interval<T>
where
    T: Copy + PartialOrd + Ord + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<T> Interval<T>
where
    T: Copy + PartialOrd + Ord,
{
    pub fn new(min: T, max: T) -> Self {
        Interval { min: min, max: max }
    }

    pub fn is_before(&self, value: T) -> bool {
        self.max < value
    }

    pub fn is_on_or_before(&self, value: T) -> bool {
        self.max <= value
    }

    pub fn contains(&self, value: T) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn is_after(&self, value: T) -> bool {
        self.min > value
    }

    pub fn is_on_or_after(&self, value: T) -> bool {
        self.min >= value
    }

    pub fn is_empty(&self) -> bool {
        self.max < self.min
    }

    pub fn intersect(&self, other: Self) -> Self {
        let new_min = max(self.min, other.min);
        let new_max = min(self.max, other.max);
        Interval {
            min: new_min,
            max: max(new_min, new_max),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64_interval() {
        let min = testutils::rand::i64(-10, 10);
        let max = testutils::rand::i64(100, 1000);
        let interval = Interval::new(min, max);
        let val = testutils::rand::i64(10, 100);
        assert!(interval.contains(val));
        let val = testutils::rand::i64(1001, 2000);
        assert!(interval.is_before(val));
        let val = testutils::rand::i64(1000, 2000);
        assert!(interval.is_on_or_before(val));
        let val = testutils::rand::i64(-100, -11);
        assert!(interval.is_after(val));
        let val = testutils::rand::i64(-100, -10);
        assert!(interval.is_on_or_after(val));
        let min = testutils::rand::i64(100, 1000);
        let max = testutils::rand::i64(-10, 10);
        let interval = Interval::new(min, max);
        assert!(interval.is_empty());
    }

    #[test]
    fn test_string_interval() {
        let min = testutils::rand::i64(2000, 3000).to_string();
        let max = testutils::rand::i64(4000, 5000).to_string();
        let interval = Interval::new(&min, &max);
        let val = testutils::rand::i64(3000, 4000).to_string();
        assert!(interval.contains(&val));
        let val = testutils::rand::i64(5001, 6000).to_string();
        assert!(interval.is_before(&val));
        let val = testutils::rand::i64(5000, 6000).to_string();
        assert!(interval.is_on_or_before(&val));
        let val = testutils::rand::i64(1000, 1999).to_string();
        assert!(interval.is_after(&val));
        let val = testutils::rand::i64(1000, 2000).to_string();
        assert!(interval.is_on_or_after(&val));
        let min = testutils::rand::i64(4000, 5000).to_string();
        let max = testutils::rand::i64(2000, 3000).to_string();
        let interval = Interval::new(&min, &max);
        assert!(interval.is_empty());
    }
}
