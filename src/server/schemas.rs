//pub mod account;
pub mod claims;
pub mod error;
pub mod profile;
pub mod schema;
//pub mod share;
//pub mod table;
use crate::server::routers::admin;
use crate::server::routers::shares;
use crate::server::services::account;
use crate::server::services::share;
use crate::server::services::table;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        admin::login,
        admin::accounts::post,
        admin::accounts::get,
        admin::accounts::list,
        admin::shares::post,
        admin::tables::post,
        admin::tables::get,
        admin::tables::list,
        admin::shares::schemas::tables::post,
        shares::get,
        shares::list,
    ),
    components(
	schemas(
	    claims::Claims,
	    profile::Profile,
	    account::Account,
	    share::Share,
	    table::Table,
	    schema::Schema,
	    error::Error
	),
        schemas(admin::AdminLoginRequest, admin::AdminLoginResponse),
        schemas(admin::accounts::AdminAccountsPostRequest, admin::accounts::AdminAccountsPostResponse),
        schemas(admin::accounts::AdminAccountsGetResponse),
        schemas(admin::accounts::AdminAccountsListResponse),
        schemas(admin::shares::AdminSharesPostRequest, admin::shares::AdminSharesPostResponse),
        schemas(admin::tables::AdminTablesPostRequest, admin::tables::AdminTablesPostResponse),
        schemas(admin::tables::AdminTablesGetResponse),
        schemas(admin::tables::AdminTablesListResponse),
        schemas(admin::shares::schemas::tables::AdminSharesSchemasTablesPostRequest, admin::shares::schemas::tables::AdminSharesSchemasTablesPostResponse),
        schemas(shares::SharesGetResponse),
        schemas(shares::SharesListResponse),
    ),
    tags(
        (name = "Kotosiro Sharing", description = "Kotosiro Deltalake Sharing API")
    )
)]
pub struct ApiDoc;
