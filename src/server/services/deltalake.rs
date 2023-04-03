use anyhow::Result;
use axum::BoxError;
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

pub struct Service;

impl Service {
    pub fn load_metadata(
        metadata: DeltaTableMetaData,
    ) -> impl Stream<Item = Result<serde_json::Value, BoxError>> {
        futures_util::stream::iter(vec![
            Ok::<serde_json::Value, BoxError>(json!(Protocol::new())),
            Ok::<serde_json::Value, BoxError>(json!(Metadata::from(metadata))),
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
