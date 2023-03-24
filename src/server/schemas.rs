use crate::server::interactors::admin;
use crate::server::interactors::shares;
use utoipa::OpenApi;
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub share_credentials_version: i64,
    pub endpoint: String,
    pub bearer_token: String,
    pub expiration_time: String,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub name: String,
    pub email: String,
    pub namespace: String,
    pub ttl: i64,
}

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub id: String,
    pub name: String,
}

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
	schemas(Profile, Account, Share),
        schemas(admin::AdminLoginRequest, admin::AdminLoginResponse, crate::error::ErrorResponse),
        schemas(admin::accounts::AdminAccountsPostRequest, admin::accounts::AdminAccountsPostResponse, crate::error::ErrorResponse),
        schemas(admin::accounts::AdminAccountsGetResponse, crate::error::ErrorResponse),
        schemas(admin::accounts::AdminAccountsListResponse, crate::error::ErrorResponse),
        schemas(admin::shares::AdminSharesPostRequest, admin::shares::AdminSharesPostResponse, crate::error::ErrorResponse),
        schemas(shares::SharesGetResponse, crate::error::ErrorResponse),
        schemas(shares::SharesListResponse, crate::error::ErrorResponse),
    ),
    tags(
        (name = "Kotosiro Sharing", description = "Kotosiro Deltalake Sharing API")
    )
)]
pub struct ApiDoc;
