use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub share_credentials_version: i64,
    pub endpoint: String,
    pub bearer_token: String,
    pub expiration_time: String,
}
