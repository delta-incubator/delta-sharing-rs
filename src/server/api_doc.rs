use crate::server::routers::admin;
use crate::server::routers::shares;
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
        admin::login,
        admin::profile,
        admin::accounts::post,
        admin::accounts::get,
        admin::accounts::list,
        admin::shares::post,
        admin::shares::schemas::post,
        admin::shares::schemas::tables::post,
        shares::get,
        shares::list,
        shares::all_tables::list,
        shares::schemas::list,
        shares::schemas::tables::list,
        shares::schemas::tables::version::get,
        shares::schemas::tables::metadata::get,
        shares::schemas::tables::query::post,
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
        schemas(admin::AdminLoginRequest, admin::AdminLoginResponse, admin::AdminProfileResponse),
        schemas(admin::accounts::AdminAccountsPostRequest, admin::accounts::AdminAccountsPostResponse),
        schemas(admin::accounts::AdminAccountsGetResponse),
        schemas(admin::accounts::AdminAccountsListResponse),
        schemas(admin::shares::AdminSharesPostRequest, admin::shares::AdminSharesPostResponse),
        schemas(admin::shares::schemas::AdminSharesSchemasPostRequest, admin::shares::schemas::AdminSharesSchemasPostResponse),
        schemas(admin::shares::schemas::tables::AdminSharesSchemasTablesPostRequest, admin::shares::schemas::tables::AdminSharesSchemasTablesPostResponse),
        schemas(shares::SharesGetResponse),
        schemas(shares::SharesListResponse),
        schemas(shares::all_tables::SharesAllTablesListResponse),
        schemas(shares::schemas::SharesSchemasListResponse),
        schemas(shares::schemas::tables::SharesSchemasTablesListResponse),
        schemas(shares::schemas::tables::query::SharesSchemasTablesQueryPostRequest),
    ),
    tags(
        (name = "Delta Sharing", description = "Delta Sharing API")
    )
)]
pub struct ApiDoc;
