use utoipa::ToSchema;

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub name: String,
    pub email: String,
    pub namespace: String,
    pub role: Role,
    pub exp: i64,
}

#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize, strum_macros::EnumString)]
pub enum Role {
    #[strum(ascii_case_insensitive)]
    #[serde(rename = "admin")]
    Admin,
    #[strum(ascii_case_insensitive)]
    #[serde(rename = "guest")]
    Guest,
}
