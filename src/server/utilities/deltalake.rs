use crate::config;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use deltalake::delta::open_table_with_storage_options;
use deltalake::delta::DeltaTable;
use deltalake::schema::SchemaDataType;
use std::cmp::max;
use std::cmp::min;
use std::collections::hash_map::HashMap;
use std::fmt;
use utoipa::ToSchema;

pub type File = deltalake::action::Add;

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
        Interval { min, max }
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

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, strum_macros::EnumString, ToSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum ValueType {
    #[strum(ascii_case_insensitive)]
    Boolean,
    #[strum(ascii_case_insensitive)]
    Int,
    #[strum(ascii_case_insensitive)]
    Long,
    #[strum(ascii_case_insensitive)]
    String,
    #[strum(ascii_case_insensitive)]
    Date,
}

impl AsRef<str> for ValueType {
    fn as_ref(&self) -> &str {
        match self {
            ValueType::Boolean => "boolean",
            ValueType::Int => "int",
            ValueType::Long => "long",
            ValueType::String => "string",
            ValueType::Date => "date",
        }
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<SchemaDataType> for ValueType {
    type Error = anyhow::Error;

    fn try_from(schema_data_type: SchemaDataType) -> std::result::Result<Self, Self::Error> {
        match schema_data_type {
            SchemaDataType::primitive(name) if name.to_lowercase() == "boolean" => {
                Ok(ValueType::Boolean)
            }
            SchemaDataType::primitive(name) if name.to_lowercase() == "integer" => {
                Ok(ValueType::Int)
            }
            SchemaDataType::primitive(name) if name.to_lowercase() == "long" => Ok(ValueType::Long),
            SchemaDataType::primitive(name) if name.to_lowercase() == "string" => {
                Ok(ValueType::String)
            }
            SchemaDataType::primitive(name) if name.to_lowercase() == "date" => Ok(ValueType::Date),
            _ => Err(anyhow!("failed to parse column type")),
        }
    }
}

impl TryFrom<&SchemaDataType> for ValueType {
    type Error = anyhow::Error;

    fn try_from(schema_data_type: &SchemaDataType) -> std::result::Result<Self, Self::Error> {
        match schema_data_type {
            SchemaDataType::primitive(name) if name.to_lowercase() == "boolean" => {
                Ok(ValueType::Boolean)
            }
            SchemaDataType::primitive(name) if name.to_lowercase() == "integer" => {
                Ok(ValueType::Int)
            }
            SchemaDataType::primitive(name) if name.to_lowercase() == "long" => Ok(ValueType::Long),
            SchemaDataType::primitive(name) if name.to_lowercase() == "string" => {
                Ok(ValueType::String)
            }
            SchemaDataType::primitive(name) if name.to_lowercase() == "date" => Ok(ValueType::Date),
            _ => Err(anyhow!("failed to parse column type")),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub num_records: i64,
    pub min_values: HashMap<String, serde_json::Value>,
    pub max_values: HashMap<String, serde_json::Value>,
    pub null_count: HashMap<String, i64>,
}

pub struct Utility;

impl Utility {
    pub async fn open_table(location: &str) -> Result<DeltaTable> {
        let google_service_account_path = format!(
            "{}",
            shellexpand::tilde(
                std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
                    .ok()
                    .unwrap_or("~/.gcp/service-account-file.json".into())
                    .as_str()
            )
        );
        let aws_profile = std::env::var("AWS_PROFILE").unwrap_or(config::AWS_PROFILE.into());
        let aws_region = std::env::var("AWS_REGION").unwrap_or(config::AWS_REGION.into());
        open_table_with_storage_options(
            location,
            HashMap::from([
                (
                    String::from("google_service_account_path"),
                    google_service_account_path,
                ),
                (String::from("profile"), aws_profile),
                (String::from("region"), aws_region),
            ]),
        )
        .await
        .context("failed to open delta table")
    }

    pub fn get_stats(file: &File) -> Result<Stats> {
        let Some(stats) = &file.stats else {
	    return Err(anyhow!("failed to acquire statistics json"));
	};
        serde_json::from_str(stats).context("failed to serialize statistics")
    }

    pub fn datetime_yyyy_mm_dd(datetime: &str) -> Result<DateTime<Utc>> {
        Utc.datetime_from_str(datetime, "%Y-%m-%d")
            .context("failed to parse deltalake datetime")
    }

    pub fn datetime_yyyy_mm_dd_hh_mm_ss(datetime: &str) -> Result<DateTime<Utc>> {
        Utc.datetime_from_str(datetime, "%Y/%m/%d %H:%M:%S")
            .context("failed to parse deltalake datetime")
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
