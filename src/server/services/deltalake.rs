use anyhow::Result;
use deltalake::delta::DeltaTableMetaData;
use serde_json::json;
use std::collections::HashMap;
use utoipa::ToSchema;

pub const VERSION: i32 = 1;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    pub min_reader_version: i32,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub provider: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
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

impl TryFrom<DeltaTableMetaData> for Metadata {
    type Error = anyhow::Error;

    fn try_from(metadata: DeltaTableMetaData) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
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
        })
    }
}

pub struct Service;

impl Service {
    pub fn new_protocol() -> Result<Protocol> {
        Ok(Protocol {
            min_reader_version: VERSION,
        })
    }

    pub fn metadata_from(metadata: DeltaTableMetaData) -> Result<Metadata> {
        Metadata::try_from(metadata)
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
