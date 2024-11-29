use anyhow::{anyhow, Result};
use deltalake::kernel::Schema;
use utoipa::ToSchema;

use crate::server::utilities::deltalake::Stats;
use crate::server::utilities::deltalake::ValueType;

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
pub struct PredicateJson {
    pub op: OpType,
    pub children: Option<Vec<PredicateJson>>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub value_type: Option<ValueType>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Predicate {
    Equal {
        column: String,
        value: String,
        value_type: ValueType,
    },
    GreaterThan {
        column: String,
        value: String,
        value_type: ValueType,
    },
    LessThan {
        column: String,
        value: String,
        value_type: ValueType,
    },
    GreaterEqual {
        column: String,
        value: String,
        value_type: ValueType,
    },
    LessEqual {
        column: String,
        value: String,
        value_type: ValueType,
    },
    IsNull {
        column: String,
        value_type: ValueType,
    },
    And(Vec<Predicate>),
    Or(Vec<Predicate>),
    Not(Box<Predicate>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct PartitionFilter {
    pub predicate: Predicate,
}

pub struct Utility;

impl Utility {
    fn check<T: PartialOrd + std::str::FromStr>(
        predicate: &Predicate,
        min: &T,
        max: &T,
        null_count: &i64,
    ) -> bool {
        match predicate {
            Predicate::IsNull { .. } => null_count > &0,
            Predicate::Equal { value, .. } => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                min <= value && value <= max
            }
            Predicate::GreaterThan { value, .. } => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                value < max
            }
            Predicate::LessThan { value, .. } => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                min < value
            }
            Predicate::GreaterEqual { value, .. } => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                value <= max
            }
            Predicate::LessEqual { value, .. } => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                min <= value
            }
            _ => unreachable!(),
        }
    }

    pub fn parse(json: PredicateJson) -> Result<Predicate> {
        match json.op {
            OpType::And => {
                let Some(children) = json.children else {
                    return Err(anyhow!("JSON AND predicate must have child predicates"));
                };
                let children: Result<Vec<Predicate>, _> =
                    children.into_iter().map(Self::parse).collect();
                let Ok(children) = children else {
                    return Err(anyhow!(
                        "failed to parse JSON AND predicate while parsing child predicates"
                    ));
                };
                Ok(Predicate::And(children))
            }
            OpType::Or => {
                let Some(children) = json.children else {
                    return Err(anyhow!("JSON OR predicate must have child predicates"));
                };
                let children: Result<Vec<Predicate>, _> =
                    children.into_iter().map(Self::parse).collect();
                let Ok(children) = children else {
                    return Err(anyhow!(
                        "failed to parse JSON OR predicate while parsing child predicates"
                    ));
                };
                Ok(Predicate::Or(children))
            }
            OpType::Not => {
                let Some(mut children) = json.children else {
                    return Err(anyhow!("JSON NOT predicate must have child predicate"));
                };
                let child = if children.len() != 1 {
                    None
                } else {
                    Some(children.swap_remove(0))
                };
                let Some(child) = child else {
                    return Err(anyhow!("JSON NOT predicate must have child predicate"));
                };
                let Ok(child) = Self::parse(child) else {
                    return Err(anyhow!(
                        "failed to parse JSON NOT predicate while parsing child predicate"
                    ));
                };
                Ok(Predicate::Not(Box::new(child)))
            }
            OpType::IsNull => {
                let Some(mut children) = json.children else {
                    return Err(anyhow!("JSON IS NULL predicate must have child predicate"));
                };
                if children.len() != 1 {
                    return Err(anyhow!(
                        "wrong number of arguments for JSON IS NULL predicate"
                    ));
                }
                let Some(column) = children.iter().position(|c| c.op == OpType::Column) else {
                    return Err(anyhow!("JSON IS NULL predicate must have COLUMN predicate"));
                };
                let column = children.swap_remove(column);
                let Some(value_type) = column.value_type else {
                    return Err(anyhow!(
                        "missing column value type for JSON EQUAL predicate"
                    ));
                };
                let Some(column) = column.name else {
                    return Err(anyhow!("missing column name for JSON IS NULL predicate"));
                };
                Ok(Predicate::IsNull { column, value_type })
            }
            OpType::Equal => {
                let Some(mut children) = json.children else {
                    return Err(anyhow!("JSON EQUAL predicate must have child predicates"));
                };
                if children.len() != 2 {
                    return Err(anyhow!(
                        "wrong number of arguments for JSON EQUAL predicate"
                    ));
                }
                let Some(column) = children.iter().position(|c| c.op == OpType::Column) else {
                    return Err(anyhow!("JSON EQUAL predicate must have COLUMN predicate"));
                };
                let column = children.swap_remove(column);
                let Some(column_type) = column.value_type else {
                    return Err(anyhow!(
                        "missing column value type for JSON EQUAL predicate"
                    ));
                };
                let Some(column) = column.name else {
                    return Err(anyhow!("missing column name for JSON EQUAL predicate"));
                };
                let Some(value) = children.iter().position(|c| c.op == OpType::Literal) else {
                    return Err(anyhow!("JSON EQUAL predicate must have LITERAL predicate"));
                };
                let value = children.swap_remove(value);
                let Some(value_type) = value.value_type else {
                    return Err(anyhow!(
                        "missing literal value type for JSON EQUAL predicate"
                    ));
                };
                let Some(value) = value.value else {
                    return Err(anyhow!("missing literal value for JSON EQUAL predicate"));
                };
                if column_type != value_type {
                    return Err(anyhow!("inconsistent value type for JSON EQUAL predicate"));
                }
                Ok(Predicate::Equal {
                    column,
                    value,
                    value_type,
                })
            }
            OpType::GreaterThan => {
                let Some(mut children) = json.children else {
                    return Err(anyhow!(
                        "JSON GREATER THAN predicate must have child predicates"
                    ));
                };
                if children.len() != 2 {
                    return Err(anyhow!(
                        "wrong number of arguments for JSON GREATER THAN predicate"
                    ));
                }
                let Some(column) = children.iter().position(|c| c.op == OpType::Column) else {
                    return Err(anyhow!(
                        "JSON GREATER THAN predicate must have COLUMN predicate"
                    ));
                };
                let column = children.swap_remove(column);
                let Some(column_type) = column.value_type else {
                    return Err(anyhow!(
                        "missing column value type for JSON GREATER THAN predicate"
                    ));
                };
                let Some(column) = column.name else {
                    return Err(anyhow!(
                        "missing column name for JSON GREATER THAN predicate"
                    ));
                };
                let Some(value) = children.iter().position(|c| c.op == OpType::Literal) else {
                    return Err(anyhow!(
                        "JSON GREATER THAN predicate must have LITERAL predicate"
                    ));
                };
                let value = children.swap_remove(value);
                let Some(value_type) = value.value_type else {
                    return Err(anyhow!(
                        "missing literal value type for JSON GREATER THAN predicate"
                    ));
                };
                let Some(value) = value.value else {
                    return Err(anyhow!(
                        "missing literal value for JSON GREATER THAN predicate"
                    ));
                };
                if column_type != value_type {
                    return Err(anyhow!(
                        "inconsistent value type for JSON GREATER THAN predicate"
                    ));
                }
                Ok(Predicate::GreaterThan {
                    column,
                    value,
                    value_type,
                })
            }
            OpType::LessThan => {
                let Some(mut children) = json.children else {
                    return Err(anyhow!(
                        "JSON LESS THAN predicate must have child predicates"
                    ));
                };
                if children.len() != 2 {
                    return Err(anyhow!(
                        "wrong number of arguments for JSON LESS THAN predicate"
                    ));
                }
                let Some(column) = children.iter().position(|c| c.op == OpType::Column) else {
                    return Err(anyhow!(
                        "JSON LESS THAN predicate must have COLUMN predicate"
                    ));
                };
                let column = children.swap_remove(column);
                let Some(column_type) = column.value_type else {
                    return Err(anyhow!(
                        "missing column value type for JSON LESS THAN predicate"
                    ));
                };
                let Some(column) = column.name else {
                    return Err(anyhow!("missing column name for JSON LESS THAN predicate"));
                };
                let Some(value) = children.iter().position(|c| c.op == OpType::Literal) else {
                    return Err(anyhow!(
                        "JSON LESS THAN predicate must have LITERAL predicate"
                    ));
                };
                let value = children.swap_remove(value);
                let Some(value_type) = value.value_type else {
                    return Err(anyhow!(
                        "missing literal value type for JSON LESS THAN predicate"
                    ));
                };
                let Some(value) = value.value else {
                    return Err(anyhow!(
                        "missing literal value for JSON LESS THAN predicate"
                    ));
                };
                if column_type != value_type {
                    return Err(anyhow!(
                        "inconsistent value type for JSON LESS THAN predicate"
                    ));
                }
                Ok(Predicate::LessThan {
                    column,
                    value,
                    value_type,
                })
            }
            OpType::GreaterThanOrEqual => {
                let Some(mut children) = json.children else {
                    return Err(anyhow!(
                        "JSON GREATER THAN OR EQUAL predicate must have child predicates"
                    ));
                };
                if children.len() != 2 {
                    return Err(anyhow!(
                        "wrong number of arguments for JSON GREATER THAN OR EQUAL predicate"
                    ));
                }
                let Some(column) = children.iter().position(|c| c.op == OpType::Column) else {
                    return Err(anyhow!(
                        "JSON GREATER THAN OR EQUAL predicate must have COLUMN predicate"
                    ));
                };
                let column = children.swap_remove(column);
                let Some(column_type) = column.value_type else {
                    return Err(anyhow!(
                        "missing column value type for JSON GREATER THAN OR EQUAL predicate"
                    ));
                };
                let Some(column) = column.name else {
                    return Err(anyhow!(
                        "missing column name for JSON GREATER THAN OR EQUAL predicate"
                    ));
                };
                let Some(value) = children.iter().position(|c| c.op == OpType::Literal) else {
                    return Err(anyhow!(
                        "JSON GREATER THAN OR EQUAL predicate must have LITERAL predicate"
                    ));
                };
                let value = children.swap_remove(value);
                let Some(value_type) = value.value_type else {
                    return Err(anyhow!(
                        "missing literal value type for JSON GREATER THAN OR EQUAL predicate"
                    ));
                };
                let Some(value) = value.value else {
                    return Err(anyhow!(
                        "missing literal value for JSON GREATER THAN OR EQUAL predicate"
                    ));
                };
                if column_type != value_type {
                    return Err(anyhow!(
                        "inconsistent value type for JSON GREATER THAN OR EQUAL predicate"
                    ));
                }
                Ok(Predicate::GreaterEqual {
                    column,
                    value,
                    value_type,
                })
            }
            OpType::LessThanOrEqual => {
                let Some(mut children) = json.children else {
                    return Err(anyhow!(
                        "JSON LESS THAN OR EQUAL predicate must have child predicates"
                    ));
                };
                if children.len() != 2 {
                    return Err(anyhow!(
                        "wrong number of arguments for JSON LESS THAN OR EQUAL predicate"
                    ));
                }
                let Some(column) = children.iter().position(|c| c.op == OpType::Column) else {
                    return Err(anyhow!(
                        "JSON LESS THAN OR EQUAL predicate must have COLUMN predicate"
                    ));
                };
                let column = children.swap_remove(column);
                let Some(column_type) = column.value_type else {
                    return Err(anyhow!(
                        "missing column value type for JSON LESS THAN OR EQUAL predicate"
                    ));
                };
                let Some(column) = column.name else {
                    return Err(anyhow!(
                        "missing column name for JSON LESS THAN OR EQUAL predicate"
                    ));
                };
                let Some(value) = children.iter().position(|c| c.op == OpType::Literal) else {
                    return Err(anyhow!(
                        "JSON LESS THAN OR EQUAL predicate must have LITERAL predicate"
                    ));
                };
                let value = children.swap_remove(value);
                let Some(value_type) = value.value_type else {
                    return Err(anyhow!(
                        "missing literal value type for JSON LESS THAN OR EQUAL predicate"
                    ));
                };
                let Some(value) = value.value else {
                    return Err(anyhow!(
                        "missing literal value for JSON LESS THAN OR EQUAL predicate"
                    ));
                };
                if column_type != value_type {
                    return Err(anyhow!(
                        "inconsistent value type for JSON LESS THAN OR EQUAL predicate"
                    ));
                }
                Ok(Predicate::LessEqual {
                    column,
                    value,
                    value_type,
                })
            }
            OpType::Column => Err(anyhow!("invalid JSON predicate")),
            OpType::Literal => Err(anyhow!("invalid JSON predicate")),
        }
    }

    pub fn filter(predicate: &Predicate, stats: &Stats, schema: &Schema) -> bool {
        match predicate {
            Predicate::And(children) => children.iter().all(|c| Self::filter(c, stats, schema)),
            Predicate::Or(children) => children.iter().any(|c| Self::filter(c, stats, schema)),
            Predicate::Not(child) => !Self::filter(child, stats, schema),
            Predicate::IsNull { column, value_type }
            | Predicate::Equal {
                column, value_type, ..
            }
            | Predicate::GreaterThan {
                column, value_type, ..
            }
            | Predicate::LessThan {
                column, value_type, ..
            }
            | Predicate::GreaterEqual {
                column, value_type, ..
            }
            | Predicate::LessEqual {
                column, value_type, ..
            } => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Some(null_count) = stats.null_count.get(column) else {
                    return true;
                };
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Some(field) = schema.field(column) else {
                    return true;
                };
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(column_type) = ValueType::try_from(field.data_type()) else {
                    return true;
                };
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                if column_type != *value_type {
                    return true;
                }
                match (stats.min_values.get(column), stats.max_values.get(column)) {
                    (Some(serde_json::Value::Bool(min)), Some(serde_json::Value::Bool(max))) => {
                        match column_type {
                            ValueType::Boolean => Self::check(predicate, min, max, null_count),
                            // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                            _ => true,
                        }
                    }
                    (
                        Some(serde_json::Value::String(min)),
                        Some(serde_json::Value::String(max)),
                    ) => {
                        match column_type {
                            ValueType::String => Self::check(predicate, min, max, null_count),
                            ValueType::Date => Self::check(predicate, min, max, null_count),
                            // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                            _ => true,
                        }
                    }
                    (
                        Some(serde_json::Value::Number(min)),
                        Some(serde_json::Value::Number(max)),
                    ) => {
                        match column_type {
                            ValueType::Int => {
                                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                                let Some(ref min) = min.as_i64() else {
                                    return true;
                                };
                                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                                let Some(ref max) = max.as_i64() else {
                                    return true;
                                };
                                Self::check(predicate, min, max, null_count)
                            }
                            ValueType::Long => {
                                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                                let Some(ref min) = min.as_i64() else {
                                    return true;
                                };
                                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                                let Some(ref max) = max.as_i64() else {
                                    return true;
                                };
                                Self::check(predicate, min, max, null_count)
                            }
                            // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                            _ => true,
                        }
                    }
                    // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                    _ => true,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse() {
        let op = OpType::IsNull;
        let column = testutils::rand::string(10);
        let value_types = vec!["boolean", "int", "long", "string", "date"];
        let value_type = testutils::rand::choose(&value_types);
        let value_type =
            ValueType::from_str(value_type).expect("value type should be parsed properly");
        let json = PredicateJson {
            op,
            children: Some(vec![PredicateJson {
                op: OpType::Column,
                children: None,
                name: Some(column.clone()),
                value: None,
                value_type: Some(value_type),
            }]),
            name: None,
            value: None,
            value_type: None,
        };
        let predicate = Utility::parse(json).expect("json should be parsed properly");
        assert_eq!(predicate, Predicate::IsNull { column, value_type });
        let op = OpType::Equal;
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let value_types = vec!["boolean", "int", "long", "string", "date"];
        let value_type = testutils::rand::choose(&value_types);
        let value_type =
            ValueType::from_str(value_type).expect("value type should be parsed properly");
        let json = PredicateJson {
            op,
            children: Some(vec![
                PredicateJson {
                    op: OpType::Column,
                    children: None,
                    name: Some(column.clone()),
                    value: None,
                    value_type: Some(value_type),
                },
                PredicateJson {
                    op: OpType::Literal,
                    children: None,
                    name: None,
                    value: Some(value.clone()),
                    value_type: Some(value_type),
                },
            ]),
            name: None,
            value: None,
            value_type: None,
        };
        let predicate = Utility::parse(json).expect("json should be parsed properly");
        assert_eq!(
            predicate,
            Predicate::Equal {
                column,
                value,
                value_type
            }
        );
        let op = OpType::LessThan;
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let value_types = vec!["boolean", "int", "long", "string", "date"];
        let value_type = testutils::rand::choose(&value_types);
        let value_type =
            ValueType::from_str(value_type).expect("value type should be parsed properly");
        let json = PredicateJson {
            op,
            children: Some(vec![
                PredicateJson {
                    op: OpType::Column,
                    children: None,
                    name: Some(column.clone()),
                    value: None,
                    value_type: Some(value_type),
                },
                PredicateJson {
                    op: OpType::Literal,
                    children: None,
                    name: None,
                    value: Some(value.clone()),
                    value_type: Some(value_type),
                },
            ]),
            name: None,
            value: None,
            value_type: None,
        };
        let predicate = Utility::parse(json).expect("json should be parsed properly");
        assert_eq!(
            predicate,
            Predicate::LessThan {
                column,
                value,
                value_type
            }
        );
        let op = OpType::LessThanOrEqual;
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let value_types = vec!["boolean", "int", "long", "string", "date"];
        let value_type = testutils::rand::choose(&value_types);
        let value_type =
            ValueType::from_str(value_type).expect("value type should be parsed properly");
        let json = PredicateJson {
            op,
            children: Some(vec![
                PredicateJson {
                    op: OpType::Column,
                    children: None,
                    name: Some(column.clone()),
                    value: None,
                    value_type: Some(value_type),
                },
                PredicateJson {
                    op: OpType::Literal,
                    children: None,
                    name: None,
                    value: Some(value.clone()),
                    value_type: Some(value_type),
                },
            ]),
            name: None,
            value: None,
            value_type: None,
        };
        let predicate = Utility::parse(json).expect("json should be parsed properly");
        assert_eq!(
            predicate,
            Predicate::LessEqual {
                column,
                value,
                value_type
            }
        );
        let op = OpType::GreaterThan;
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let value_types = vec!["boolean", "int", "long", "string", "date"];
        let value_type = testutils::rand::choose(&value_types);
        let value_type =
            ValueType::from_str(value_type).expect("value type should be parsed properly");
        let json = PredicateJson {
            op,
            children: Some(vec![
                PredicateJson {
                    op: OpType::Column,
                    children: None,
                    name: Some(column.clone()),
                    value: None,
                    value_type: Some(value_type),
                },
                PredicateJson {
                    op: OpType::Literal,
                    children: None,
                    name: None,
                    value: Some(value.clone()),
                    value_type: Some(value_type),
                },
            ]),
            name: None,
            value: None,
            value_type: None,
        };
        let predicate = Utility::parse(json).expect("json should be parsed properly");
        assert_eq!(
            predicate,
            Predicate::GreaterThan {
                column,
                value,
                value_type
            }
        );
        let op = OpType::GreaterThanOrEqual;
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let value_types = vec!["boolean", "int", "long", "string", "date"];
        let value_type = testutils::rand::choose(&value_types);
        let value_type =
            ValueType::from_str(value_type).expect("value type should be parsed properly");
        let json = PredicateJson {
            op,
            children: Some(vec![
                PredicateJson {
                    op: OpType::Column,
                    children: None,
                    name: Some(column.clone()),
                    value: None,
                    value_type: Some(value_type),
                },
                PredicateJson {
                    op: OpType::Literal,
                    children: None,
                    name: None,
                    value: Some(value.clone()),
                    value_type: Some(value_type),
                },
            ]),
            name: None,
            value: None,
            value_type: None,
        };
        let predicate = Utility::parse(json).expect("json should be parsed properly");
        assert_eq!(
            predicate,
            Predicate::GreaterEqual {
                column,
                value,
                value_type
            }
        );
        let op_1 = OpType::IsNull;
        let column_1 = testutils::rand::string(10);
        let value_types_1 = vec!["boolean", "int", "long", "string", "date"];
        let value_type_1 = testutils::rand::choose(&value_types_1);
        let value_type_1 =
            ValueType::from_str(value_type_1).expect("value type should be parsed properly");
        let json_1 = PredicateJson {
            op: op_1,
            children: Some(vec![PredicateJson {
                op: OpType::Column,
                children: None,
                name: Some(column_1.clone()),
                value: None,
                value_type: Some(value_type_1),
            }]),
            name: None,
            value: None,
            value_type: None,
        };
        let op_2 = OpType::IsNull;
        let column_2 = testutils::rand::string(10);
        let value_types_2 = vec!["boolean", "int", "long", "string", "date"];
        let value_type_2 = testutils::rand::choose(&value_types_2);
        let value_type_2 =
            ValueType::from_str(value_type_2).expect("value type should be parsed properly");
        let json_2 = PredicateJson {
            op: op_2,
            children: Some(vec![PredicateJson {
                op: OpType::Column,
                children: None,
                name: Some(column_2.clone()),
                value: None,
                value_type: Some(value_type_2),
            }]),
            name: None,
            value: None,
            value_type: None,
        };
        let op = OpType::And;
        let json = PredicateJson {
            op,
            children: Some(vec![json_1, json_2]),
            name: None,
            value: None,
            value_type: None,
        };
        let predicate = Utility::parse(json).expect("json should be parsed properly");
        assert_eq!(
            predicate,
            Predicate::And(vec![
                Predicate::IsNull {
                    column: column_1,
                    value_type: value_type_1
                },
                Predicate::IsNull {
                    column: column_2,
                    value_type: value_type_2
                }
            ])
        );
        let op_1 = OpType::IsNull;
        let column_1 = testutils::rand::string(10);
        let value_types_1 = vec!["boolean", "int", "long", "string", "date"];
        let value_type_1 = testutils::rand::choose(&value_types_1);
        let value_type_1 =
            ValueType::from_str(value_type_1).expect("value type should be parsed properly");
        let json_1 = PredicateJson {
            op: op_1,
            children: Some(vec![PredicateJson {
                op: OpType::Column,
                children: None,
                name: Some(column_1.clone()),
                value: None,
                value_type: Some(value_type_1),
            }]),
            name: None,
            value: None,
            value_type: None,
        };
        let op_2 = OpType::IsNull;
        let column_2 = testutils::rand::string(10);
        let value_types_2 = vec!["boolean", "int", "long", "string", "date"];
        let value_type_2 = testutils::rand::choose(&value_types_2);
        let value_type_2 =
            ValueType::from_str(value_type_2).expect("value type should be parsed properly");
        let json_2 = PredicateJson {
            op: op_2,
            children: Some(vec![PredicateJson {
                op: OpType::Column,
                children: None,
                name: Some(column_2.clone()),
                value: None,
                value_type: Some(value_type_2),
            }]),
            name: None,
            value: None,
            value_type: None,
        };
        let op = OpType::Or;
        let json = PredicateJson {
            op,
            children: Some(vec![json_1, json_2]),
            name: None,
            value: None,
            value_type: None,
        };
        let predicate = Utility::parse(json).expect("json should be parsed properly");
        assert_eq!(
            predicate,
            Predicate::Or(vec![
                Predicate::IsNull {
                    column: column_1,
                    value_type: value_type_1
                },
                Predicate::IsNull {
                    column: column_2,
                    value_type: value_type_2
                }
            ])
        );

        let op_1 = OpType::IsNull;
        let column_1 = testutils::rand::string(10);
        let value_types_1 = vec!["boolean", "int", "long", "string", "date"];
        let value_type_1 = testutils::rand::choose(&value_types_1);
        let value_type_1 =
            ValueType::from_str(value_type_1).expect("value type should be parsed properly");
        let json_1 = PredicateJson {
            op: op_1,
            children: Some(vec![PredicateJson {
                op: OpType::Column,
                children: None,
                name: Some(column_1.clone()),
                value: None,
                value_type: Some(value_type_1),
            }]),
            name: None,
            value: None,
            value_type: None,
        };
        let op = OpType::Not;
        let json = PredicateJson {
            op,
            children: Some(vec![json_1]),
            name: None,
            value: None,
            value_type: None,
        };
        let predicate = Utility::parse(json).expect("json should be parsed properly");
        assert_eq!(
            predicate,
            Predicate::Not(Box::new(Predicate::IsNull {
                column: column_1,
                value_type: value_type_1
            }))
        );
    }
}
