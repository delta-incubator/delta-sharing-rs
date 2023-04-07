use crate::server::utilities::deltalake::Stats;
use crate::server::utilities::deltalake::Utility as DeltalakeUtility;
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

type File = deltalake::action::Add;

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

pub struct Service;

impl Service {
    fn check_sql_hints(predicate: &SQLPredicate, stats: &Stats) -> bool {
        let min = stats.min_values.get(predicate.column());
        let max = stats.max_values.get(predicate.column());
        let null_count = stats.null_count.get(predicate.column());
        let (min, max) = match (min, max) {
            (Some(serde_json::Value::String(min)), Some(serde_json::Value::String(max))) => {
                (min, max)
            }
            (Some(serde_json::Value::Number(min)), Some(serde_json::Value::Number(max))) => {
                (min, max)
            }
            _ => return false,
        };
        let Some(count) = null_count else {
	    return false;
	};
        predicate.hold(min, max, null_count)
    }

    fn filter_with_sql_hints(
        files: Vec<File>,
        predicate_hints: Option<Vec<SQLPredicate>>,
    ) -> Vec<File> {
        if let Some(predicates) = predicate_hints {
            if predicates.len() > 0 {
                return files
                    .into_iter()
                    .filter(|f| {
                        predicates.iter().all(|p| {
                            let Ok(stats) = DeltalakeUtility::get_stats(f) else {
				return false;
			    };
                            Self::check_sql_hints(&p, &stats)
                        })
                    })
                    .collect::<Vec<File>>();
            }
        }
        files
    }

    pub fn load_files(
        table: DeltaTable,
        predicate_hints: Option<Vec<SQLPredicate>>,
    ) -> impl Stream<Item = Result<serde_json::Value, BoxError>> {
        let files =
            Self::filter_with_sql_hints(table.get_state().files().to_owned(), predicate_hints);
        for file in files {
            let stats_raw = file.stats.as_ref().unwrap();
            let stats: Stats = serde_json::from_str(stats_raw).unwrap();
            println!("{:?}", file);
            println!("{:?}", stats_raw);
            println!("{:?}", stats);
        }
        let metadata = table.get_metadata().unwrap();
        //        let schema = DeltalakeUtility::get_schema(metadata).unwrap();
        //        println!("{:?}", schema);
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
