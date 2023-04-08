use crate::config;
use crate::server::entities::schema::Name as SchemaName;
use crate::server::entities::share::Name as ShareName;
use crate::server::entities::table::Name as TableName;
use crate::server::routers::SharedState;
use crate::server::services::deltalake::Service as DeltalakeService;
use crate::server::services::error::Error;
use crate::server::services::table::Service as TableService;
use crate::server::utilities::deltalake::Utility as DeltalakeUtility;
use crate::server::utilities::signed_url::Platform;
use crate::server::utilities::signed_url::Utility as SignedUrlUtility;
use crate::server::utilities::sql::PartitionFilter as SQLPartitionFilter;
use crate::server::utilities::sql::Utility as SQLUtility;
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
use utoipa::IntoParams;
use utoipa::ToSchema;

const HEADER_NAME: &str = "Delta-Table-Version";

#[derive(Debug, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SharesSchemasTablesQueryPostRequest {
    pub predicate_hints: Option<Vec<String>>,
    pub json_predicate_hints: Option<String>,
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
    request_body = SharesSchemasTablesQueryPostRequest,
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
    let predicate_hints = if let Some(predicate_hints) = &payload.predicate_hints {
        let predicate_hints: Result<Vec<SQLPartitionFilter>, _> = predicate_hints
            .into_iter()
            .map(|p| SQLUtility::parse(p.to_owned()))
            .collect();
        if let Err(_) = predicate_hints {
            tracing::warn!("requested predicate hints are malformed");
        }
        predicate_hints.ok()
    } else {
        None
    };
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
    let Ok(table) = TableService::query_by_fqn(
        &share,
        &schema,
        &table,
        &state.pg_pool,
    ).await else {
        tracing::error!("request is not handled correctly due to a server error while selecting table");
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
        tracing::error!("request is not handled correctly due to a server error while loading delta table");
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
    let url_signer = |name: String| match &platform {
        Platform::AWS { bucket, path } => {
            let file: String = format!("{}/{}", path, name);
            let Ok(url) = SignedUrlUtility::sign_aws(
		    &state.aws_credentials,
		    &bucket,
		    &file,
		    &config::fetch::<u64>("signed_url_ttl")
		) else {
                    tracing::error!("failed to sign up AWS S3 url");
		    return String::from("");
		};
            return url.into();
        }
        Platform::GCP { bucket, path } => {
            let file: String = format!("{}/{}", path, name);
            let Ok(url) = SignedUrlUtility::sign_gcp(
		    &state.gcp_service_account,
		    &bucket,
		    &file,
		    &config::fetch::<u64>("signed_url_ttl")
		) else {
                    tracing::error!("failed to sign up GCP GCS url");
		    return String::from("");
		};
            return url.into();
        }
    };
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
        JsonLines::new(DeltalakeService::files_from(
            table,
            metadata,
            predicate_hints,
            is_time_traveled,
            &url_signer,
        )),
    )
        .into_response())
}
