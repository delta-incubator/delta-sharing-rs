use delta_kernel::expressions::{
    BinaryOperator, ColumnName, Expression, UnaryOperator, VariadicOperator,
};
use delta_kernel::schema::{DataType, PrimitiveType};

use crate::models::sharing::v1::JsonPredicate;
use crate::{Error, Result};

/// Convert a `JsonPredicate` to an [Expression](delta_kernel::expressions::Expression).
pub fn json_predicate_to_expression(predicate: &JsonPredicate) -> Result<Expression> {
    match predicate.op.to_ascii_lowercase().as_str() {
        "column" => parse_column(predicate),
        "literal" => parse_literal(predicate),
        "equal"
        | "less_than"
        | "lessThan"
        | "less_than_or_equal"
        | "lessthanorequal"
        | "greater_than"
        | "greaterthan"
        | "greater_than_or_equal"
        | "greaterthanorequal" => parse_binary(predicate),
        "not" | "is_null" | "isNull" => parse_unary(predicate),
        "and" | "or" => parse_variadics(predicate),
        _ => Err(Error::invalid_predicate(format!(
            "Invalid operator: {}",
            predicate.op
        ))),
    }
}

fn parse_column(predicate: &JsonPredicate) -> Result<Expression> {
    if let Some(name) = &predicate.name {
        Ok(Expression::Column(ColumnName::from_naive_str_split(name)))
    } else {
        Err(Error::invalid_predicate(
            "op: 'column' requires field 'name' to be specified.",
        ))
    }
}

fn parse_literal(predicate: &JsonPredicate) -> Result<Expression> {
    if let Some(data_type) = &predicate.value_type {
        if let Some(value) = &predicate.value {
            match parse_data_type(data_type)? {
                DataType::Primitive(primitive) => match primitive {
                    PrimitiveType::Boolean
                    | PrimitiveType::Integer
                    | PrimitiveType::Long
                    | PrimitiveType::String
                    | PrimitiveType::Date
                    | PrimitiveType::Float
                    | PrimitiveType::Double
                    | PrimitiveType::Timestamp
                    | PrimitiveType::TimestampNtz => {
                        Ok(Expression::literal(primitive.parse_scalar(value)?))
                    }
                    _ => Err(Error::invalid_predicate(format!(
                        "unsupported primitive type: {}",
                        primitive
                    ))),
                },
                _ => Err(Error::invalid_predicate(
                    "op: 'literal' requires field 'valueType' to be a primitive type.",
                )),
            }
        } else {
            Err(Error::invalid_predicate(
                "op: 'literal' requires field 'value' to be specified.",
            ))
        }
    } else {
        Err(Error::invalid_predicate(
            "op: 'literal' requires field 'valueType' to be specified.",
        ))
    }
}

fn parse_binary(predicate: &JsonPredicate) -> Result<Expression> {
    if predicate.children.len() != 2 {
        return Err(Error::invalid_predicate(
            "op: 'binary' requires exactly two children.",
        ));
    }
    let lhs = json_predicate_to_expression(&predicate.children[0])?;
    let rhs = json_predicate_to_expression(&predicate.children[1])?;
    let op = parse_binary_operator(&predicate.op)?;
    Ok(Expression::binary(op, lhs, rhs))
}

fn parse_binary_operator(val: impl AsRef<str>) -> Result<BinaryOperator> {
    match val.as_ref().to_ascii_lowercase().as_str() {
        "equal" => Ok(BinaryOperator::Equal),
        "less_than" | "lessthan" => Ok(BinaryOperator::LessThan),
        "less_than_or_equal" | "lessthanorequal" => Ok(BinaryOperator::LessThanOrEqual),
        "greater_than" | "greaterthan" => Ok(BinaryOperator::GreaterThan),
        "greater_than_or_equal" | "greaterthanorequal" => Ok(BinaryOperator::GreaterThanOrEqual),
        _ => Err(Error::invalid_predicate(format!(
            "Invalid binary operator: {}",
            val.as_ref()
        ))),
    }
}

fn parse_unary(predicate: &JsonPredicate) -> Result<Expression> {
    if predicate.children.len() != 1 {
        return Err(Error::invalid_predicate(
            "unary operator requires exactly one child.",
        ));
    }
    let expr = json_predicate_to_expression(&predicate.children[0])?;
    let op = parse_unary_operator(&predicate.op)?;
    Ok(Expression::unary(op, expr))
}

fn parse_unary_operator(val: impl AsRef<str>) -> Result<UnaryOperator> {
    match val.as_ref().to_ascii_lowercase().as_str() {
        "not" => Ok(UnaryOperator::Not),
        "is_null" | "isnull" => Ok(UnaryOperator::IsNull),
        _ => Err(Error::invalid_predicate(format!(
            "Invalid unary operator: {}",
            val.as_ref()
        ))),
    }
}

fn parse_variadics(predicate: &JsonPredicate) -> Result<Expression> {
    let children = predicate
        .children
        .iter()
        .map(json_predicate_to_expression)
        .collect::<Result<Vec<_>>>()?;
    let op = parse_variadic_operator(&predicate.op)?;
    Ok(Expression::variadic(op, children))
}

fn parse_variadic_operator(val: impl AsRef<str>) -> Result<VariadicOperator> {
    match val.as_ref().to_ascii_lowercase().as_str() {
        "and" => Ok(VariadicOperator::And),
        "or" => Ok(VariadicOperator::Or),
        _ => Err(Error::invalid_predicate(format!(
            "Invalid variadic operator: {}",
            val.as_ref()
        ))),
    }
}

fn parse_data_type(val: impl AsRef<str>) -> Result<DataType> {
    match val.as_ref().to_ascii_lowercase().as_str() {
        "bool" | "boolean" => Ok(PrimitiveType::Boolean.into()),
        "int" => Ok(PrimitiveType::Integer.into()),
        "long" => Ok(PrimitiveType::Long.into()),
        "string" => Ok(PrimitiveType::String.into()),
        "date" => Ok(PrimitiveType::Date.into()),
        "float" => Ok(PrimitiveType::Float.into()),
        "double" => Ok(PrimitiveType::Double.into()),
        "timestamp" => Ok(PrimitiveType::Timestamp.into()),
        "timestamp_ntz" | "timestampntz" => Ok(PrimitiveType::TimestampNtz.into()),
        _ => Err(Error::invalid_predicate(format!(
            "Invalid data type: {}",
            val.as_ref()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_predicate_to_expression() {
        let predicate = JsonPredicate {
            op: "column".to_string(),
            name: Some("foo".to_string()),
            children: vec![],
            value: None,
            value_type: None,
        };
        let expr = json_predicate_to_expression(&predicate).unwrap();
        assert_eq!(
            expr,
            Expression::Column(ColumnName::from_naive_str_split("foo"))
        );

        let predicate = JsonPredicate {
            op: "literal".to_string(),
            name: None,
            children: vec![],
            value: Some("true".to_string()),
            value_type: Some("boolean".to_string()),
        };
        let expr = json_predicate_to_expression(&predicate).unwrap();
        assert_eq!(expr, Expression::literal(true));

        let predicate = JsonPredicate {
            op: "equal".to_string(),
            name: None,
            children: vec![
                JsonPredicate {
                    op: "column".to_string(),
                    name: Some("foo".to_string()),
                    children: vec![],
                    value: None,
                    value_type: None,
                },
                JsonPredicate {
                    op: "literal".to_string(),
                    name: None,
                    children: vec![],
                    value: Some("true".to_string()),
                    value_type: Some("boolean".to_string()),
                },
            ],
            value: None,
            value_type: None,
        };
        let expr = json_predicate_to_expression(&predicate).unwrap();
        assert_eq!(
            expr,
            Expression::binary(
                BinaryOperator::Equal,
                Expression::Column(ColumnName::from_naive_str_split("foo")),
                Expression::literal(true)
            )
        );

        let predicate = JsonPredicate {
            op: "not".to_string(),
            name: None,
            children: vec![JsonPredicate {
                op: "column".to_string(),
                name: Some("foo".to_string()),
                children: vec![],
                value: None,
                value_type: None,
            }],
            value: None,
            value_type: None,
        };
        let expr = json_predicate_to_expression(&predicate).unwrap();
        assert_eq!(
            expr,
            Expression::unary(
                UnaryOperator::Not,
                Expression::Column(ColumnName::from_naive_str_split("foo"))
            )
        );

        let predicate = JsonPredicate {
            op: "and".to_string(),
            name: None,
            children: vec![
                JsonPredicate {
                    op: "column".to_string(),
                    name: Some("foo".to_string()),
                    children: vec![],
                    value: None,
                    value_type: None,
                },
                JsonPredicate {
                    op: "literal".to_string(),
                    name: None,
                    children: vec![],
                    value: Some("true".to_string()),
                    value_type: Some("bool".to_string()),
                },
            ],
            value: None,
            value_type: None,
        };
        let expr = json_predicate_to_expression(&predicate).unwrap();
        assert_eq!(
            expr,
            Expression::variadic(
                VariadicOperator::And,
                vec![
                    Expression::Column(ColumnName::from_naive_str_split("foo")),
                    Expression::literal(true)
                ]
            )
        );
    }
}
