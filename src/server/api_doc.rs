use crate::server::routers::catalog;
use crate::server::routers::sharing;
use crate::server::services::account;
use crate::server::services::error;
use crate::server::services::profile;
use crate::server::services::schema;
use crate::server::services::share;
use crate::server::services::table;
use crate::server::utilities::deltalake;
use crate::server::utilities::json;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        catalog::login,
        catalog::profile,
        catalog::accounts::post,
        catalog::accounts::get,
        catalog::accounts::list,
        catalog::shares::post,
        catalog::shares::schemas::post,
        catalog::shares::schemas::tables::post,
        sharing::shares::get,
        sharing::shares::list,
        sharing::shares::all_tables::list,
        sharing::shares::schemas::list,
        sharing::shares::schemas::tables::list,
        sharing::shares::schemas::tables::version::get,
        sharing::shares::schemas::tables::metadata::get,
        sharing::shares::schemas::tables::query::post,
    ),
    components(
	schemas(
	    profile::Profile,
	    account::Account,
	    share::Share,
	    table::Table,
	    table::TableDetail,
	    schema::Schema,
	    schema::SchemaDetail,
	    error::ErrorMessage,
	    deltalake::ValueType,
	    json::OpType,
	    json::PredicateJson
	),
        schemas(catalog::CatalogLoginRequest, catalog::CatalogLoginResponse, catalog::CatalogProfileResponse),
        schemas(catalog::accounts::CatalogAccountsPostRequest, catalog::accounts::CatalogAccountsPostResponse),
        schemas(catalog::accounts::CatalogAccountsGetResponse),
        schemas(catalog::accounts::CatalogAccountsListResponse),
        schemas(catalog::shares::CatalogSharesPostRequest, catalog::shares::CatalogSharesPostResponse),
        schemas(catalog::shares::schemas::CatalogSharesSchemasPostRequest, catalog::shares::schemas::CatalogSharesSchemasPostResponse),
        schemas(catalog::shares::schemas::tables::CatalogSharesSchemasTablesPostRequest, catalog::shares::schemas::tables::CatalogSharesSchemasTablesPostResponse),
        schemas(sharing::shares::SharingSharesGetResponse),
        schemas(sharing::shares::SharingSharesListResponse),
        schemas(sharing::shares::all_tables::SharingSharesAllTablesListResponse),
        schemas(sharing::shares::schemas::SharingSharesSchemasListResponse),
        schemas(sharing::shares::schemas::tables::SharingSharesSchemasTablesListResponse),
        schemas(sharing::shares::schemas::tables::query::SharingSharesSchemasTablesQueryPostRequest),
    ),
    tags(
        (name = "Delta Sharing", description = "Delta Sharing API")
    )
)]
pub struct ApiDoc;
