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
