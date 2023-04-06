use crate::server::utilities::sql::Predicate as SQLPredicate;
use anyhow::Context;
use anyhow::Result;
use axum::BoxError;
use chrono::DateTime;
use chrono::Utc;
use deltalake::delta::DeltaTable;
use deltalake::delta::DeltaTableMetaData;
use futures_util::stream::Stream;
use serde_json::json;
use std::collections::HashMap;
use utoipa::ToSchema;

pub const VERSION: i32 = 1;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolDetail {
    pub min_reader_version: i32,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    pub protocol: ProtocolDetail,
}

impl Protocol {
    fn new() -> Self {
        Self {
            protocol: ProtocolDetail {
                min_reader_version: VERSION,
            },
        }
    }
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub provider: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MetadataDetail {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub format: Format,
    pub schema_string: String,
    pub partition_columns: Vec<String>,
    pub configuration: HashMap<String, Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_files: Option<i64>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub meta_data: MetadataDetail,
}

impl Metadata {
    fn from(metadata: DeltaTableMetaData) -> Self {
        Self {
            meta_data: MetadataDetail {
                id: metadata.id,
                name: metadata.name,
                description: metadata.description,
                format: Format {
                    provider: metadata.format.get_provider(),
                },
                schema_string: json!(metadata.schema).to_string(),
                partition_columns: metadata.partition_columns,
                configuration: metadata.configuration,
                version: None,
                size: None,
                num_files: None,
            },
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stats {
    pub num_records: i64,
    pub min_values: HashMap<String, serde_json::Value>,
    pub max_values: HashMap<String, serde_json::Value>,
    pub null_count: HashMap<String, i64>,
}

pub struct Service;

impl Service {
    fn check_hints(predicate: &SQLPredicate, stats: &Stats) -> bool {
        match predicate {
            SQLPredicate::IsNull { column } => {
                let Some(count) = stats.null_count.get(column) else {
		    return true;
		};
                return count > &0;
            }
            SQLPredicate::IsNotNull { column } => {
                let Some(count) = stats.null_count.get(column) else {
		    return false;
		};
                return count == &0;
            }
            SQLPredicate::Equal { column, value } => {
                let min = stats.min_values.get(column);
                let max = stats.max_values.get(column);
                match (min, max) {
                    (
                        Some(serde_json::Value::String(min)),
                        Some(serde_json::Value::String(max)),
                    ) => {
                        return min <= value && value <= max;
                    }
                    (
                        Some(serde_json::Value::Number(min)),
                        Some(serde_json::Value::Number(max)),
                    ) => {
                        let Ok(value) = value.parse::<f64>() else {
			    return false;
			};
                        let Some(min) = min.as_f64() else {
			    return false;
			};
                        let Some(max) = max.as_f64() else {
			    return false;
			};
                        return min <= value && value <= max;
                    }
                    _ => false,
                }
            }
            SQLPredicate::GreaterThan { column, value } => {
                let max = stats.max_values.get(column);
                match max {
                    Some(serde_json::Value::String(max)) => {
                        return value < max;
                    }
                    Some(serde_json::Value::Number(max)) => {
                        let Ok(value) = value.parse::<f64>() else {
			    return false;
			};
                        let Some(max) = max.as_f64() else {
			    return false;
			};
                        return value < max;
                    }
                    _ => false,
                }
            }
            SQLPredicate::LessThan { column, value } => {
                let min = stats.min_values.get(column);
                match min {
                    Some(serde_json::Value::String(min)) => {
                        return min < value;
                    }
                    Some(serde_json::Value::Number(min)) => {
                        let Ok(value) = value.parse::<f64>() else {
			    return false;
			};
                        let Some(min) = min.as_f64() else {
			    return false;
			};
                        return min < value;
                    }
                    _ => false,
                }
            }
            SQLPredicate::GreaterEqual { column, value } => {
                let max = stats.max_values.get(column);
                match max {
                    Some(serde_json::Value::String(max)) => {
                        return value <= max;
                    }
                    Some(serde_json::Value::Number(max)) => {
                        let Ok(value) = value.parse::<f64>() else {
			    return false;
			};
                        let Some(max) = max.as_f64() else {
			    return false;
			};
                        return value <= max;
                    }
                    _ => false,
                }
            }
            SQLPredicate::LessEqual { column, value } => {
                let min = stats.min_values.get(column);
                match min {
                    Some(serde_json::Value::String(min)) => {
                        return min <= value;
                    }
                    Some(serde_json::Value::Number(min)) => {
                        let Ok(value) = value.parse::<f64>() else {
			    return false;
			};
                        let Some(min) = min.as_f64() else {
			    return false;
			};
                        return min <= value;
                    }
                    _ => false,
                }
            }
            SQLPredicate::NotEqual { column, value: _ } => {
                let min = stats.min_values.get(column);
                let max = stats.max_values.get(column);
                match (min, max) {
                    (Some(serde_json::Value::String(_)), Some(serde_json::Value::String(_))) => {
                        return true;
                    }
                    (Some(serde_json::Value::Number(_)), Some(serde_json::Value::Number(_))) => {
                        return true;
                    }
                    _ => false,
                }
            }
        }
    }

    pub fn load_files(
        table: DeltaTable,
        predicate_hints: Option<Vec<SQLPredicate>>,
    ) -> impl Stream<Item = Result<serde_json::Value, BoxError>> {
        let files = table.get_state().files();
        if let Some(predicates) = predicate_hints {
            if predicates.len() > 0 {
                let filtered: Vec<_> = files
                    .iter()
                    .filter(|&f| {
                        predicates.iter().all(|p| {
                            let Some(stats) = &f.stats else {
                        	return false;
                            };
                            let Ok(stats): Result<Stats, _> = serde_json::from_str(stats) else {
                        	return false;
                            };
                            Self::check_hints(&p, &stats)
                        })
                    })
                    .collect();
                println!("FILTERED");
                println!("{:?}", filtered);
                todo!()
            }
        }
        for file in files {
            let stats: Stats = serde_json::from_str(file.stats.as_ref().unwrap()).unwrap();
            println!("{:?}", stats);
        }
        futures_util::stream::iter(vec![])
    }

    pub fn load_metadata(
        metadata: DeltaTableMetaData,
    ) -> impl Stream<Item = Result<serde_json::Value, BoxError>> {
        futures_util::stream::iter(vec![
            Ok(json!(Protocol::new())),
            Ok(json!(Metadata::from(metadata))),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        println!("TEST DELTALAKE!!!");
    }
}
