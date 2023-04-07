use crate::server::utilities::deltalake::ColumnType;
use crate::server::utilities::deltalake::Stats;
use crate::server::utilities::deltalake::Utility as DeltalakeUtility;
use crate::server::utilities::sql::ColumnFilter as SQLColumnFilter;
use crate::server::utilities::sql::Utility as SQLUtility;
use anyhow::Context;
use anyhow::Result;
use axum::BoxError;
use chrono::DateTime;
use chrono::Utc;
use deltalake::delta::DeltaTable;
use deltalake::delta::DeltaTableMetaData;
use deltalake::schema::Schema;
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
    fn check_sql_hints(filter: &SQLColumnFilter, stats: &Stats, schema: &Schema) -> bool {
        let min = stats.min_values.get(&filter.name);
        let max = stats.max_values.get(&filter.name);
        let null_count = stats.null_count.get(&filter.name);
        let field = schema.get_field_with_name(&filter.name);
        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
        let Some(null_count) = null_count else {
	    return true;
	};
        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
        let Ok(field) = field else {
	    return true;
	};
        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
        let Ok(column_type) = ColumnType::try_from(field.get_type()) else {
	    return true;
	};
        match (min, max) {
            (Some(serde_json::Value::String(min)), Some(serde_json::Value::String(max))) => {
                match column_type {
                    ColumnType::Boolean => {
                        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                        let Ok(ref min) = min.parse::<bool>() else {
			    return true;
			};
                        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                        let Ok(ref max) = max.parse::<bool>() else {
			    return true;
			};
                        return SQLUtility::check(&filter.predicate, min, max, null_count);
                    }
                    ColumnType::String => {
                        return SQLUtility::check(&filter.predicate, min, max, null_count);
                    }
                    ColumnType::Date => {
                        return SQLUtility::check(&filter.predicate, min, max, null_count);
                    }
                    // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                    _ => {
                        return true;
                    }
                }
            }
            (Some(serde_json::Value::Number(min)), Some(serde_json::Value::Number(max))) => {
                match column_type {
                    ColumnType::Int => {
                        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                        let Some(ref min) = min.as_i64() else {
			    return true;
			};
                        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                        let Some(ref max) = max.as_i64() else {
			    return true;
			};
                        return SQLUtility::check(&filter.predicate, min, max, null_count);
                    }
                    ColumnType::Long => {
                        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                        let Some(ref min) = min.as_i64() else {
			    return true;
			};
                        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                        let Some(ref max) = max.as_i64() else {
			    return true;
			};
                        return SQLUtility::check(&filter.predicate, min, max, null_count);
                    }
                    // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                    _ => {
                        return true;
                    }
                }
            }
            // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
            _ => return true,
        };
    }

    fn filter_with_sql_hints(
        files: Vec<File>,
        schema: Option<Schema>,
        predicate_hints: Option<Vec<SQLColumnFilter>>,
    ) -> Vec<File> {
        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
        let Some(schema) = schema else {
	    return files;
	};
        if let Some(predicates) = predicate_hints {
            if predicates.len() > 0 {
                return files
                    .into_iter()
                    .filter(|f| {
                        predicates.iter().all(|p| {
                            // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                            let Ok(stats) = DeltalakeUtility::get_stats(f) else {
				return true;
			    };
                            Self::check_sql_hints(&p, &stats, &schema)
                        })
                    })
                    .collect::<Vec<File>>();
            }
        }
        files
    }

    pub fn load_files(
        table: DeltaTable,
        predicate_hints: Option<Vec<SQLColumnFilter>>,
    ) -> impl Stream<Item = Result<serde_json::Value, BoxError>> {
        let files = Self::filter_with_sql_hints(
            table.get_state().files().to_owned(),
            table.schema().cloned(),
            predicate_hints,
        );
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
