use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub id: String,
    pub name: String,
}
