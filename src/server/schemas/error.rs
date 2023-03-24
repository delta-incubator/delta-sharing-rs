use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub error_code: String,
    pub message: String,
}
