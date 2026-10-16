use std::collections::HashMap;

use buoyant_kernel::actions::{Metadata, Protocol};

use crate::error::Result;
use crate::models::sharing::v1::{
    parquet_log_message::Entry as ParquetEntry, query_response::Response as QueryResponseType,
    Format as FormatMessage, MetadataParquet, ParquetLogMessage, ParquetResponse, ProtocolParquet,
    QueryResponse,
};

impl From<&Metadata> for MetadataParquet {
    fn from(value: &Metadata) -> Self {
        MetadataParquet {
            id: value.id().to_string(),
            name: value.name().map(str::to_string),
            description: value.description().map(str::to_string),
            format: Some(FormatMessage {
                provider: value.format_provider().to_string(),
                options: format_options(value),
            }),
            schema_string: value.schema_string().clone(),
            partition_columns: value.partition_columns().to_vec(),
        }
    }
}

/// The `format.options` map of a [`Metadata`] action.
///
/// buoyant_kernel 0.21 exposes no accessor for the format options, only for the provider. The
/// `Metadata` type serializes its `format` as `{"provider", "options"}`, so read the map from that
/// representation. Serializing a `Metadata` (strings and maps of strings) cannot fail, so a
/// failure here would indicate a kernel change and is treated as empty options.
fn format_options(value: &Metadata) -> HashMap<String, String> {
    serde_json::to_value(value)
        .ok()
        .and_then(|v| v.get("format")?.get("options").cloned())
        .and_then(|options| serde_json::from_value(options).ok())
        .unwrap_or_default()
}

impl From<&Metadata> for ParquetLogMessage {
    fn from(value: &Metadata) -> Self {
        ParquetLogMessage {
            entry: Some(ParquetEntry::Metadata(value.into())),
        }
    }
}

impl From<&Protocol> for ProtocolParquet {
    fn from(value: &Protocol) -> Self {
        ProtocolParquet {
            min_reader_version: value.min_reader_version(),
        }
    }
}

impl From<&Protocol> for ParquetLogMessage {
    fn from(value: &Protocol) -> Self {
        ParquetLogMessage {
            entry: Some(ParquetEntry::Protocol(value.into())),
        }
    }
}

impl From<ProtocolParquet> for ParquetLogMessage {
    fn from(value: ProtocolParquet) -> Self {
        ParquetLogMessage {
            entry: Some(ParquetEntry::Protocol(value)),
        }
    }
}

impl From<MetadataParquet> for ParquetLogMessage {
    fn from(value: MetadataParquet) -> Self {
        ParquetLogMessage {
            entry: Some(ParquetEntry::Metadata(value)),
        }
    }
}

impl<T: IntoIterator<Item = ParquetLogMessage>> From<T> for QueryResponse {
    fn from(value: T) -> Self {
        QueryResponse {
            response: Some(QueryResponseType::Parquet(ParquetResponse {
                entries: value.into_iter().collect(),
            })),
        }
    }
}

impl IntoIterator for QueryResponse {
    type Item = Result<String>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let res = self.response.unwrap();
        match res {
            QueryResponseType::Parquet(msg) => msg
                .entries
                .iter()
                .map(|it| Ok(serde_json::to_string(it)?))
                .collect::<Vec<_>>()
                .into_iter(),
            QueryResponseType::Delta(_msg) => todo!(),
        }
    }
}
