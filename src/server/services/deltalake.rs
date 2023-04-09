use crate::server::utilities::deltalake::Utility as DeltalakeUtility;
use crate::server::utilities::json::PartitionFilter as JSONPartitionFilter;
use crate::server::utilities::json::Utility as JSONUtility;
use crate::server::utilities::sql::PartitionFilter as SQLPartitionFilter;
use crate::server::utilities::sql::Utility as SQLUtility;
use anyhow::Result;
use axum::BoxError;
use deltalake::action::Add;
use deltalake::delta::DeltaTable;
use deltalake::delta::DeltaTableMetaData;
use deltalake::schema::Schema;
use futures_util::stream::Stream;
use md5;
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

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    pub id: String,
    pub url: String,
    pub partition_values: HashMap<String, String>,
    pub size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub file: FileDetail,
}

impl File {
    fn from(
        add: Add,
        version: Option<i64>,
        timestamp: Option<i64>,
        url_signer: &dyn Fn(String) -> String,
    ) -> Self {
        let mut partition_values: HashMap<String, String> = HashMap::new();
        for (k, v) in add.partition_values.into_iter() {
            if let Some(v) = v {
                partition_values.insert(k, v);
            }
        }
        Self {
            file: FileDetail {
                id: String::from(format!("{:x}", md5::compute(add.path.as_bytes()))),
                url: url_signer(add.path),
                partition_values: partition_values,
                size: add.size,
                stats: add.stats,
                version: version,
                timestamp: timestamp,
            },
        }
    }
}

pub struct Service;

impl Service {
    fn filter_with_limit_hint(files: Vec<Add>, limit_hint: Option<i32>) -> Vec<Add> {
        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
        let Some(limit_hint) = limit_hint else {
	    return files;
	};
        let mut records_so_far = 0;
        return files
            .into_iter()
            .filter(|f| {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(stats) = DeltalakeUtility::get_stats(f) else {
		    return true;
		};
                if records_so_far > limit_hint.into() {
                    return false;
                } else {
                    records_so_far += stats.num_records;
                    return true;
                }
            })
            .collect::<Vec<Add>>();
    }

    fn filter_with_sql_hints(
        files: Vec<Add>,
        schema: Option<Schema>,
        predicate_hints: Option<Vec<SQLPartitionFilter>>,
    ) -> Vec<Add> {
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
                            SQLUtility::filter(&p, &stats, &schema)
                        })
                    })
                    .collect::<Vec<Add>>();
            }
        }
        files
    }

    fn filter_with_json_hints(
        files: Vec<Add>,
        schema: Option<Schema>,
        json_predicate_hints: Option<JSONPartitionFilter>,
    ) -> Vec<Add> {
        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
        let Some(schema) = schema else {
	    return files;
	};
        if let Some(JSONPartitionFilter { predicate }) = json_predicate_hints {
            return files
                .into_iter()
                .filter(|f| {
                    // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                    let Ok(stats) = DeltalakeUtility::get_stats(f) else {
			return true;
		    };
                    JSONUtility::filter(&predicate, &stats, &schema)
                })
                .collect::<Vec<Add>>();
        }
        files
    }

    pub fn files_from(
        table: DeltaTable,
        metadata: DeltaTableMetaData,
        predicate_hints: Option<Vec<SQLPartitionFilter>>,
        json_predicate_hints: Option<JSONPartitionFilter>,
        limit_hint: Option<i32>,
        is_time_traveled: bool,
        url_signer: &dyn Fn(String) -> String,
    ) -> impl Stream<Item = Result<serde_json::Value, BoxError>> {
        let version = if is_time_traveled {
            Some(table.version())
        } else {
            None
        };
        let timestamp = if is_time_traveled {
            metadata.created_time
        } else {
            None
        };
        let files = Self::filter_with_limit_hint(table.get_state().files().to_owned(), limit_hint);
        let files = Self::filter_with_sql_hints(files, table.schema().cloned(), predicate_hints);
        let files =
            Self::filter_with_json_hints(files, table.schema().cloned(), json_predicate_hints);
        let mut files = files
            .into_iter()
            .map(|f| {
                Ok::<serde_json::Value, BoxError>(json!(File::from(
                    f, version, timestamp, url_signer
                )))
            })
            .collect::<Vec<Result<serde_json::Value, BoxError>>>();
        let mut ret = vec![
            Ok(json!(Protocol::new())),
            Ok(json!(Metadata::from(metadata))),
        ];
        ret.append(&mut files);
        futures_util::stream::iter(ret)
    }

    pub fn metadata_from(
        metadata: DeltaTableMetaData,
    ) -> impl Stream<Item = Result<serde_json::Value, BoxError>> {
        let ret = vec![
            Ok(json!(Protocol::new())),
            Ok(json!(Metadata::from(metadata))),
        ];
        futures_util::stream::iter(ret)
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
