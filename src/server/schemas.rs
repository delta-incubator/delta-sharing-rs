pub mod account;
pub mod claims;
pub mod error;
pub mod profile;
pub mod share;
use crate::server::routers::admin;
use crate::server::routers::shares;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        admin::login,
        admin::accounts::post,
        admin::accounts::get,
        admin::accounts::list,
        admin::shares::post,
        shares::get,
        shares::list,
    ),
    components(
	schemas(claims::Claims, profile::Profile, account::Account, share::Share, error::Error),
        schemas(admin::AdminLoginRequest, admin::AdminLoginResponse),
        schemas(admin::accounts::AdminAccountsPostRequest, admin::accounts::AdminAccountsPostResponse),
        schemas(admin::accounts::AdminAccountsGetResponse),
        schemas(admin::accounts::AdminAccountsListResponse),
        schemas(admin::shares::AdminSharesPostRequest, admin::shares::AdminSharesPostResponse),
        schemas(shares::SharesGetResponse),
        schemas(shares::SharesListResponse),
    ),
    tags(
        (name = "Kotosiro Sharing", description = "Kotosiro Deltalake Sharing API")
    )
)]
pub struct ApiDoc;
