use buoyant_kernel::expressions::{ColumnName, Expression, Predicate};
use buoyant_kernel::schema::{DataType, PrimitiveType};

use crate::models::sharing::v1::JsonPredicate;
use crate::{Error, Result};

/// Convert a `JsonPredicate` to an [Expression](buoyant_kernel::expressions::Expression).
///
/// Boolean-valued operators (comparisons, `not`, `is_null`, `and`, `or`) are returned as
/// [`Expression::Predicate`].
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
        | "greaterthanorequal" => Ok(Expression::from(parse_binary(predicate)?)),
        "not" | "is_null" | "isNull" => Ok(Expression::from(parse_unary(predicate)?)),
        "and" | "or" => Ok(Expression::from(parse_variadics(predicate)?)),
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

fn parse_binary(predicate: &JsonPredicate) -> Result<Predicate> {
    if predicate.children.len() != 2 {
        return Err(Error::invalid_predicate(
            "op: 'binary' requires exactly two children.",
        ));
    }
    let lhs = json_predicate_to_expression(&predicate.children[0])?;
    let rhs = json_predicate_to_expression(&predicate.children[1])?;
    // buoyant_kernel 0.21 has no `<=` / `>=` predicate operators. Under SQL three-valued logic,
    // `a <= b` is equivalent to `NOT (a > b)` (and likewise for `>=`), since both sides are NULL
    // in exactly the same cases.
    match predicate.op.to_ascii_lowercase().as_str() {
        "equal" => Ok(Predicate::eq(lhs, rhs)),
        "less_than" | "lessthan" => Ok(Predicate::lt(lhs, rhs)),
        "less_than_or_equal" | "lessthanorequal" => Ok(Predicate::not(Predicate::gt(lhs, rhs))),
        "greater_than" | "greaterthan" => Ok(Predicate::gt(lhs, rhs)),
        "greater_than_or_equal" | "greaterthanorequal" => {
            Ok(Predicate::not(Predicate::lt(lhs, rhs)))
        }
        _ => Err(Error::invalid_predicate(format!(
            "Invalid binary operator: {}",
            predicate.op
        ))),
    }
}

fn parse_unary(predicate: &JsonPredicate) -> Result<Predicate> {
    if predicate.children.len() != 1 {
        return Err(Error::invalid_predicate(
            "unary operator requires exactly one child.",
        ));
    }
    let child = json_predicate_to_expression(&predicate.children[0])?;
    match predicate.op.to_ascii_lowercase().as_str() {
        "not" => Ok(Predicate::not(Predicate::from_expr(child))),
        "is_null" | "isnull" => Ok(Predicate::is_null(child)),
        _ => Err(Error::invalid_predicate(format!(
            "Invalid unary operator: {}",
            predicate.op
        ))),
    }
}

fn parse_variadics(predicate: &JsonPredicate) -> Result<Predicate> {
    let children = predicate
        .children
        .iter()
        .map(|child| json_predicate_to_expression(child).map(Predicate::from_expr))
        .collect::<Result<Vec<_>>>()?;
    match predicate.op.to_ascii_lowercase().as_str() {
        "and" => Ok(Predicate::and_from(children)),
        "or" => Ok(Predicate::or_from(children)),
        _ => Err(Error::invalid_predicate(format!(
            "Invalid variadic operator: {}",
            predicate.op
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

    fn col(name: &str) -> JsonPredicate {
        JsonPredicate {
            op: "column".to_string(),
            name: Some(name.to_string()),
            children: vec![],
            value: None,
            value_type: None,
        }
    }

    fn lit_bool(value: &str) -> JsonPredicate {
        JsonPredicate {
            op: "literal".to_string(),
            name: None,
            children: vec![],
            value: Some(value.to_string()),
            value_type: Some("boolean".to_string()),
        }
    }

    fn op(name: &str, children: Vec<JsonPredicate>) -> JsonPredicate {
        JsonPredicate {
            op: name.to_string(),
            name: None,
            children,
            value: None,
            value_type: None,
        }
    }

    #[test]
    fn test_json_predicate_to_expression() {
        let expr = json_predicate_to_expression(&col("foo")).unwrap();
        assert_eq!(
            expr,
            Expression::Column(ColumnName::from_naive_str_split("foo"))
        );

        let expr = json_predicate_to_expression(&lit_bool("true")).unwrap();
        assert_eq!(expr, Expression::literal(true));

        let expr =
            json_predicate_to_expression(&op("equal", vec![col("foo"), lit_bool("true")])).unwrap();
        assert_eq!(
            expr,
            Expression::from(Predicate::eq(
                Expression::Column(ColumnName::from_naive_str_split("foo")),
                Expression::literal(true)
            ))
        );

        let expr = json_predicate_to_expression(&op("not", vec![col("foo")])).unwrap();
        assert_eq!(
            expr,
            Expression::from(Predicate::not(Predicate::from_expr(Expression::Column(
                ColumnName::from_naive_str_split("foo")
            ))))
        );

        let expr =
            json_predicate_to_expression(&op("and", vec![col("foo"), lit_bool("true")])).unwrap();
        assert_eq!(
            expr,
            Expression::from(Predicate::and_from(vec![
                Predicate::from_expr(Expression::Column(ColumnName::from_naive_str_split("foo"))),
                Predicate::from_expr(Expression::literal(true)),
            ]))
        );
    }

    #[test]
    fn test_less_than_or_equal_is_negated_greater_than() {
        let expr = json_predicate_to_expression(&op(
            "less_than_or_equal",
            vec![col("foo"), lit_bool("true")],
        ))
        .unwrap();
        assert_eq!(
            expr,
            Expression::from(Predicate::not(Predicate::gt(
                Expression::Column(ColumnName::from_naive_str_split("foo")),
                Expression::literal(true)
            )))
        );
    }
}
