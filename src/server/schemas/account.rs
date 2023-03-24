use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub name: String,
    pub email: String,
    pub namespace: String,
    pub ttl: i64,
}
