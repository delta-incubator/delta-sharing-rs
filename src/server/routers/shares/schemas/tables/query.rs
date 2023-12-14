use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::http::header;
use axum::http::header::HeaderMap;
use axum::http::header::HeaderValue;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_extra::json_lines::JsonLines;
use std::str::FromStr;
use std::time::Duration;
use tame_gcs::signing::ServiceAccount;
use utoipa::IntoParams;
use utoipa::ToSchema;

use crate::config;
use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Name as TableName;
use crate::server::routers::SharedState;
use crate::server::services::deltalake::Service as DeltalakeService;
use crate::server::services::error::Error;
use crate::server::services::table::Service as TableService;
use crate::server::utilities::deltalake::Utility as DeltalakeUtility;
use crate::server::utilities::json::PartitionFilter as JSONPartitionFilter;
use crate::server::utilities::json::PredicateJson;
use crate::server::utilities::json::Utility as JSONUtility;
use crate::server::utilities::signed_url::AwsSigner;
use crate::server::utilities::signed_url::AzureSigner;
use crate::server::utilities::signed_url::GcpSigner;
use crate::server::utilities::signed_url::NoopSigner;
use crate::server::utilities::signed_url::Platform;
use crate::server::utilities::signed_url::Signer;
use crate::server::utilities::signed_url::Utility as SignedUrlUtility;
use crate::server::utilities::sql::PartitionFilter as SQLPartitionFilter;
use crate::server::utilities::sql::Utility as SQLUtility;

const HEADER_NAME: &str = "Delta-Table-Version";

#[derive(Debug, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesQueryPostRequest {
    pub predicate_hints: Option<Vec<String>>,
    pub json_predicate_hints: Option<PredicateJson>,
    pub limit_hint: Option<i32>,
    pub version: Option<i64>,
    pub timestamp: Option<String>,
}

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesQueryPostParams {
    share: String,
    schema: String,
    table: String,
}

#[utoipa::path(
    post,
    path = "/shares/{share}/schemas/{schema}/tables/{table}/query",
    operation_id = "QueryTable",
    tag = "official",
    request_body = SharesSchemasTablesQueryPostRequest,
    params(SharesSchemasTablesQueryPostParams),
    responses(
        (status = 200, description = "The tables were successfully returned.", body = String),
        (status = 400, description = "The request is malformed.", body = ErrorMessage),
        (status = 401, description = "The request is unauthenticated. The bearer token is missing or incorrect.", body = ErrorMessage),
        (status = 403, description = "The request is forbidden from being fulfilled.", body = ErrorMessage),
        (status = 404, description = "The requested resource does not exist.", body = ErrorMessage),
        (status = 500, description = "The request is not handled correctly due to a server error.", body = ErrorMessage),
    )
)]
#[tracing::instrument(skip(state))]
pub async fn post(
    Extension(state): Extension<SharedState>,
    Path(params): Path<SharesSchemasTablesQueryPostParams>,
    Json(payload): Json<SharesSchemasTablesQueryPostRequest>,
) -> Result<Response, Error> {
    let predicate_hints = if let Some(predicate_hints) = payload.predicate_hints {
        let predicate_hints: Result<Vec<SQLPartitionFilter>, _> = predicate_hints
            .into_iter()
            .map(|p| SQLUtility::parse(p.to_owned()))
            .collect();
        if predicate_hints.is_err() {
            tracing::warn!("requested predicate hints are malformed");
        }
        predicate_hints.ok()
    } else {
        None
    };
    let json_predicate_hints = if let Some(json_predicate_hints) = payload.json_predicate_hints {
        let predicate = JSONUtility::parse(json_predicate_hints);
        if predicate.is_err() {
            tracing::warn!("requested predicate hints are malformed");
        }
        predicate.ok()
    } else {
        None
    };
    let json_predicate_hints =
        json_predicate_hints.map(|predicate| JSONPartitionFilter { predicate });
    let timestamp = if let Some(timestamp) = &payload.timestamp {
        let Ok(timestamp) = DeltalakeUtility::datetime_yyyy_mm_dd_hh_mm_ss(timestamp) else {
            tracing::error!("requested timestamp is malformed");
            return Err(Error::ValidationFailed);
        };
        Some(timestamp)
    } else {
        None
    };
    let Ok(share) = ShareName::new(params.share) else {
        tracing::error!("requested share data is malformed");
        return Err(Error::ValidationFailed);
    };
    let Ok(schema) = SchemaName::new(params.schema) else {
        tracing::error!("requested schema data is malformed");
        return Err(Error::ValidationFailed);
    };
    let Ok(table) = TableName::new(params.table) else {
        tracing::error!("requested table data is malformed");
        return Err(Error::ValidationFailed);
    };
    let Ok(table) = TableService::query_by_fqn(&share, &schema, &table, &state.pg_pool).await
    else {
        tracing::error!(
            "request is not handled correctly due to a server error while selecting table"
        );
        return Err(anyhow!("error occured while selecting table(s)").into());
    };
    let Some(table) = table else {
        tracing::error!("requested table does not exist");
        return Err(Error::NotFound);
    };
    let Ok(platform) = Platform::from_str(&table.location) else {
        tracing::error!("requested cloud platform is not supported");
        return Err(anyhow!("error occured while identifying cloud platform").into());
    };
    let Ok(mut table) = DeltalakeUtility::open_table(&table.location).await else {
        tracing::error!(
            "request is not handled correctly due to a server error while loading delta table"
        );
        return Err(anyhow!("error occured while selecting table(s)").into());
    };
    let mut is_time_traveled = false;
    // NOTE: version precedes over timestamp
    if let Some(timestamp) = timestamp {
        let Ok(_) = table.load_with_datetime(timestamp).await else {
            tracing::error!("request is not handled correctly due to a server error while time-traveling delta table");
            return Err(anyhow!("error occured while selecting table(s)").into());
        };
        is_time_traveled = true;
    }
    // NOTE: version precedes over timestamp
    if let Some(version) = &payload.version {
        let Ok(_) = table.load_version(*version).await else {
            tracing::error!("request is not handled correctly due to a server error while time-traveling delta table");
            return Err(anyhow!("error occured while selecting table(s)").into());
        };
        is_time_traveled = true;
    }
    let metadata = {
        let Ok(metadata) = table.get_metadata() else {
            tracing::error!("request is not handled correctly due to a server error while loading delta table metadata");
            return Err(anyhow!("error occured while selecting table(s)").into());
        };
        metadata.to_owned()
    };
    let url_signer: Box<dyn Signer> = match &platform {
        Platform::Aws { .. } => {
            if let Some(creds) = &state.aws_credentials {
                Box::new(AwsSigner {
                    aws: creds.clone(),
                    expiration: Duration::from_secs(config::fetch::<u64>("signed_url_ttl")),
                })
            } else {
                Box::new(NoopSigner {})
            }
        }
        Platform::Azure { .. } => {
            if let Some(creds) = &state.azure_credentials {
                Box::new(AzureSigner {
                    azure: creds.clone(),
                    expiration: Duration::from_secs(config::fetch::<u64>("signed_url_ttl")),
                })
            } else {
                Box::new(NoopSigner {})
            }
        }
        Platform::Gcp { .. } => {
            if let Some(_) = &state.gcp_service_account {
                let creds = ServiceAccount::load_json_file(
                    std::env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap(),
                )
                .unwrap();
                Box::new(GcpSigner {
                    gcp: creds,
                    expiration: Duration::from_secs(config::fetch::<u64>("signed_url_ttl")),
                })
            } else {
                Box::new(NoopSigner {})
            }
        }
        _ => Box::new(NoopSigner {}),
    };

    // let url_signer = |name: String| match &platform {
    //     Platform::Aws { url, bucket, path } => {
    //         if let Some(aws_credentials) = &state.aws_credentials {
    //             let file: String = format!("{}/{}", path, name);
    //             let Ok(signed) = SignedUrlUtility::sign_aws(
    //                 aws_credentials,
    //                 bucket,
    //                 &file,
    //                 &config::fetch::<u64>("signed_url_ttl"),
    //             ) else {
    //                 tracing::error!("failed to sign up AWS S3 url");
    //                 return url.clone();
    //             };
    //             return signed.into();
    //         }
    //         tracing::warn!("AWS credentials were not set");
    //         url.clone()
    //     }
    //     Platform::Gcp { url, bucket, path } => {
    //         if let Some(gcp_service_account) = &state.gcp_service_account {
    //             let file: String = format!("{}/{}", path, name);
    //             let Ok(signed) = SignedUrlUtility::sign_gcp(
    //                 gcp_service_account,
    //                 bucket,
    //                 &file,
    //                 &config::fetch::<u64>("signed_url_ttl"),
    //             ) else {
    //                 tracing::error!("failed to sign up GCP GCS url");
    //                 return url.clone();
    //             };
    //             return signed.into();
    //         }
    //         tracing::warn!("GCP service account was not set");
    //         url.clone()
    //     }
    //     Platform::Azure {
    //         url,
    //         storage_account,
    //         container,
    //         blob_name,
    //     } => {
    //         if let Some(azure_storage_credentials) = &state.azure_storage_credentials {
    //             let Ok(signed) = SignedUrlUtility::sign_azure(
    //                 azure_storage_credentials,
    //                 storage_account,
    //                 container,
    //                 blob_name,
    //                 &config::fetch::<u64>("signed_url_ttl"),
    //             ) else {
    //                 tracing::error!("failed to sign up Azure Storage url");
    //                 return url.clone();
    //             };
    //             return signed.into();
    //         }
    //         tracing::warn!("Azure Storage credentials were not set");
    //         url.clone()
    //     }
    //     Platform::None { url } => {
    //         tracing::warn!("no supported platforms");
    //         url.clone()
    //     }
    // };
    let mut headers = HeaderMap::new();
    headers.insert(HEADER_NAME, table.version().into());
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-ndjson"),
    );
    tracing::info!("delta table was successfully returned");
    Ok((
        StatusCode::OK,
        headers,
        JsonLines::new(
            DeltalakeService::files_from(
                table,
                metadata,
                predicate_hints,
                json_predicate_hints,
                payload.limit_hint,
                is_time_traveled,
                &url_signer,
            )
            .await,
        ),
    )
        .into_response())
}
