use crate::server::utilities::deltalake::ValueType;
use utoipa::ToSchema;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, strum_macros::EnumString, ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum OpType {
    #[strum(ascii_case_insensitive)]
    Column,
    #[strum(ascii_case_insensitive)]
    Literal,
    #[strum(ascii_case_insensitive)]
    IsNull,
    #[strum(ascii_case_insensitive)]
    Equal,
    #[strum(ascii_case_insensitive)]
    LessThan,
    #[strum(ascii_case_insensitive)]
    LessThanOrEqual,
    #[strum(ascii_case_insensitive)]
    GreaterThan,
    #[strum(ascii_case_insensitive)]
    GreaterThanOrEqual,
    #[strum(ascii_case_insensitive)]
    And,
    #[strum(ascii_case_insensitive)]
    Or,
    #[strum(ascii_case_insensitive)]
    Not,
}

impl AsRef<str> for OpType {
    fn as_ref(&self) -> &str {
        match self {
            OpType::Column => "column",
            OpType::Literal => "literal",
            OpType::IsNull => "isNull",
            OpType::Equal => "equal",
            OpType::LessThan => "lessThan",
            OpType::LessThanOrEqual => "lessThanOrEqual",
            OpType::GreaterThan => "greaterThan",
            OpType::GreaterThanOrEqual => "greaterThanOrEqual",
            OpType::And => "and",
            OpType::Or => "or",
            OpType::Not => "not",
        }
    }
}

impl std::fmt::Display for OpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Predicate {
    pub op: OpType,
    pub children: Option<Vec<Predicate>>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub value_type: Option<ValueType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("TEST JSON!!!");
    }
}
