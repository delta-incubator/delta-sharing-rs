use std::collections::VecDeque;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use deltalake::schema::Schema;

use crate::server::utilities::deltalake::Stats;
use crate::server::utilities::deltalake::ValueType;

static KEYWORDS: &[char] = &[' ', '=', '\'', '\"', '>', '<'];

#[derive(Debug, PartialEq, Eq)]
enum Token {
    EQ,
    GT,
    LT,
    GE,
    LE,
    NE,
    Key(String),
    End,
}

type Tokens = VecDeque<Token>;

type Stack = Vec<char>;

impl Token {
    fn lex(code: String) -> Result<Tokens> {
        let mut tokens: Tokens = VecDeque::new();
        let mut stack: Stack = Vec::new();
        let mut iter = code.chars().peekable();
        let mut next: Option<char> = None;
        loop {
            let c = match next {
                Some(c) => c,
                None => match iter.next() {
                    None => break,
                    Some(c) => c,
                },
            };
            next = None;
            if !stack.is_empty() {
                let tail: String = iter
                    .by_ref()
                    .take_while(|c| c != stack.last().unwrap())
                    .collect();
                let key: String = format!("{}{}", c, tail)
                    .parse()
                    .context("failed to lex key")?;
                tokens.push_back(Token::Key(key));
                iter.next();
                stack.pop();
                continue;
            }
            match c {
                ' ' => continue,
                '=' => tokens.push_back(Token::EQ),
                '\'' => stack.push('\''),
                '\"' => stack.push('\"'),
                '>' => {
                    if iter.peek() == Some(&'=') {
                        iter.next();
                        tokens.push_back(Token::GE);
                    } else {
                        tokens.push_back(Token::GT);
                    }
                }
                '<' => {
                    if iter.peek() == Some(&'=') {
                        iter.next();
                        tokens.push_back(Token::LE);
                    } else if iter.peek() == Some(&'>') {
                        iter.next();
                        tokens.push_back(Token::NE);
                    } else {
                        tokens.push_back(Token::LT);
                    }
                }
                c if !KEYWORDS.contains(&c) => {
                    let tail: String = iter
                        .by_ref()
                        .take_while(|&c| match !KEYWORDS.contains(&c) {
                            true => true,
                            false => {
                                next = Some(c);
                                false
                            }
                        })
                        .collect();
                    let key: String = format!("{}{}", c, tail)
                        .parse()
                        .context("failed to lex key")?;
                    tokens.push_back(Token::Key(key));
                }
                _ => return Err(anyhow!("could not recognize a character: {}", c)),
            }
        }
        tokens.push_back(Token::End);
        Ok(tokens)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Equal,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    NotEqual,
    IsNull,
    IsNotNull,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Predicate {
    Equal(String),
    GreaterThan(String),
    LessThan(String),
    GreaterEqual(String),
    LessEqual(String),
    NotEqual(String),
    IsNull,
    IsNotNull,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PartitionFilter {
    pub column: String,
    pub predicate: Predicate,
}

pub struct Utility;

impl Utility {
    fn column(tokens: &mut Tokens) -> Result<String> {
        let Some(Token::Key(column)) = tokens.pop_front() else {
            return Err(anyhow!("failed to parse column name"));
        };
        Ok(column)
    }

    fn operator(tokens: &mut Tokens) -> Result<Operator> {
        match tokens.pop_front() {
            Some(Token::EQ) => Ok(Operator::Equal),
            Some(Token::GT) => Ok(Operator::GreaterThan),
            Some(Token::LT) => Ok(Operator::LessThan),
            Some(Token::GE) => Ok(Operator::GreaterEqual),
            Some(Token::LE) => Ok(Operator::LessEqual),
            Some(Token::NE) => Ok(Operator::NotEqual),
            Some(Token::Key(value)) if value.to_lowercase() == "is" => match tokens.pop_front() {
                Some(Token::Key(value)) if value.to_lowercase() == "null" => Ok(Operator::IsNull),
                Some(Token::Key(value)) if value.to_lowercase() == "not" => {
                    match tokens.pop_front() {
                        Some(Token::Key(value)) if value.to_lowercase() == "null" => {
                            Ok(Operator::IsNotNull)
                        }
                        _ => Err(anyhow!("failed to parse SQL operator")),
                    }
                }
                _ => Err(anyhow!("failed to parse SQL operator")),
            },
            _ => Err(anyhow!("failed to parse SQL operator")),
        }
    }

    fn value(tokens: &mut Tokens) -> Result<String> {
        match tokens.pop_front() {
            Some(Token::Key(value)) => Ok(value),
            _ => Err(anyhow!("failed to parse string")),
        }
    }

    fn end(tokens: &mut Tokens) -> Result<()> {
        let Some(Token::End) = tokens.pop_front() else {
            return Err(anyhow!("failed to parse end of SQL expression"));
        };
        Ok(())
    }

    fn check<T: PartialOrd + std::str::FromStr>(
        predicate: &Predicate,
        min: &T,
        max: &T,
        null_count: &i64,
    ) -> bool {
        match predicate {
            Predicate::IsNull => null_count > &0,
            Predicate::IsNotNull => null_count == &0,
            Predicate::Equal(value) => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                min <= value && value <= max
            }
            Predicate::GreaterThan(value) => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                value < max
            }
            Predicate::LessThan(value) => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                min < value
            }
            Predicate::GreaterEqual(value) => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                value <= max
            }
            Predicate::LessEqual(value) => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                min <= value
            }
            Predicate::NotEqual(value) => {
                // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                let Ok(ref value) = value.parse::<T>() else {
                    return true;
                };
                min != value && value != max
            }
        }
    }

    pub fn parse(code: String) -> Result<PartitionFilter> {
        let mut tokens = Token::lex(code).context("failed to lex given string")?;
        let column = Self::column(&mut tokens)
            .context("first entry of SQL expression should be column name")?;
        let operator = Self::operator(&mut tokens)
            .context("second entry of SQL expression should be operator")?;
        if operator == Operator::IsNull || operator == Operator::IsNotNull {
            Self::end(&mut tokens).context("invalid SQL expression")?;
            match operator {
                Operator::IsNull => {
                    return Ok(PartitionFilter {
                        column,
                        predicate: Predicate::IsNull,
                    })
                }
                Operator::IsNotNull => {
                    return Ok(PartitionFilter {
                        column,
                        predicate: Predicate::IsNotNull,
                    })
                }
                _ => {
                    return Err(anyhow!("failed to parse SQL expression"));
                }
            }
        }
        let value =
            Self::value(&mut tokens).context("third entry of SQL expression should be value")?;
        Self::end(&mut tokens).context("invalid SQL expression")?;
        match operator {
            Operator::Equal => Ok(PartitionFilter {
                column,
                predicate: Predicate::Equal(value),
            }),
            Operator::GreaterThan => Ok(PartitionFilter {
                column,
                predicate: Predicate::GreaterThan(value),
            }),
            Operator::LessThan => Ok(PartitionFilter {
                column,
                predicate: Predicate::LessThan(value),
            }),
            Operator::GreaterEqual => Ok(PartitionFilter {
                column,
                predicate: Predicate::GreaterEqual(value),
            }),
            Operator::LessEqual => Ok(PartitionFilter {
                column,
                predicate: Predicate::LessEqual(value),
            }),
            Operator::NotEqual => Ok(PartitionFilter {
                column,
                predicate: Predicate::NotEqual(value),
            }),
            _ => Err(anyhow!("failed to parse SQL expression")),
        }
    }

    pub fn filter(filter: &PartitionFilter, stats: &Stats, schema: &Schema) -> bool {
        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
        let Some(null_count) = stats.null_count.get(&filter.column) else {
            return true;
        };
        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
        let Ok(field) = schema.get_field_with_name(&filter.column) else {
            return true;
        };
        // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
        let Ok(column_type) = ValueType::try_from(field.get_type()) else {
            return true;
        };
        match (
            stats.min_values.get(&filter.column),
            stats.max_values.get(&filter.column),
        ) {
            (Some(serde_json::Value::Bool(min)), Some(serde_json::Value::Bool(max))) => {
                match column_type {
                    ValueType::Boolean => Self::check(&filter.predicate, min, max, null_count),
                    // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                    _ => true,
                }
            }
            (Some(serde_json::Value::String(min)), Some(serde_json::Value::String(max))) => {
                match column_type {
                    ValueType::String => Self::check(&filter.predicate, min, max, null_count),
                    ValueType::Date => Self::check(&filter.predicate, min, max, null_count),
                    // NOTE: The server may try its best to filter files in a BEST EFFORT mode.
                    _ => true,
                }
            }
            (Some(serde_json::Value::Number(min)), Some(serde_json::Value::Number(max))) => {
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
                        Self::check(&filter.predicate, min, max, null_count)
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
                        Self::check(&filter.predicate, min, max, null_count)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let expr = format!(
            "{}{}{}={}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::EQ);
        assert_eq!(tokens[2], Token::Key(value));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::i64(-15, 15).to_string();
        let expr = format!(
            "{}{}{}>{}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::GT);
        assert_eq!(tokens[2], Token::Key(value));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let expr = format!(
            "{}{}{}<{}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::LT);
        assert_eq!(tokens[2], Token::Key(value));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::i64(-15, 15).to_string();
        let expr = format!(
            "{}{}{}>={}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::GE);
        assert_eq!(tokens[2], Token::Key(value));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let expr = format!(
            "{}{}{}<={}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::LE);
        assert_eq!(tokens[2], Token::Key(value));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::i64(-15, 15).to_string();
        let expr = format!(
            "{}{}{}<>{}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::NE);
        assert_eq!(tokens[2], Token::Key(value));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let expr = format!(
            "{}{} IS {} NULL",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::Key("IS".into()));
        assert_eq!(tokens[2], Token::Key("NULL".into()));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let expr = format!(
            "{}{} IS {} NOT {} NULL",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::Key("IS".into()));
        assert_eq!(tokens[2], Token::Key("NOT".into()));
        assert_eq!(tokens[3], Token::Key("NULL".into()));
        assert_eq!(tokens[4], Token::End);
    }

    #[test]
    fn test_parse() {
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let expr = format!(
            "{}{}{}={}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr).expect("expression should be parsed properly");
        assert_eq!(
            predicate,
            PartitionFilter {
                column,
                predicate: Predicate::Equal(value)
            }
        );
        let column = testutils::rand::string(10);
        let value = testutils::rand::string(10);
        let expr = format!(
            "{}{}{}>{}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr).expect("expression should be parsed properly");
        assert_eq!(
            predicate,
            PartitionFilter {
                column,
                predicate: Predicate::GreaterThan(value)
            }
        );
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let expr = format!(
            "{}{}{}<{}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr).expect("expression should be parsed properly");
        assert_eq!(
            predicate,
            PartitionFilter {
                column,
                predicate: Predicate::LessThan(value)
            }
        );
        let column = testutils::rand::string(10);
        let value = testutils::rand::string(10);
        let expr = format!(
            "{}{}{}>={}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr).expect("expression should be parsed properly");
        assert_eq!(
            predicate,
            PartitionFilter {
                column,
                predicate: Predicate::GreaterEqual(value)
            }
        );
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5).to_string();
        let expr = format!(
            "{}{}{}<={}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr).expect("expression should be parsed properly");
        assert_eq!(
            predicate,
            PartitionFilter {
                column,
                predicate: Predicate::LessEqual(value)
            }
        );
        let column = testutils::rand::string(10);
        let value = testutils::rand::string(10);
        let expr = format!(
            "{}{}{}<>{}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr).expect("expression should be parsed properly");
        assert_eq!(
            predicate,
            PartitionFilter {
                column,
                predicate: Predicate::NotEqual(value)
            }
        );
        let column = testutils::rand::string(10);
        let expr = format!(
            "{}{} IS {} NULL",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr).expect("expression should be parsed properly");
        assert_eq!(
            predicate,
            PartitionFilter {
                column,
                predicate: Predicate::IsNull
            }
        );
        let column = testutils::rand::string(10);
        let expr = format!(
            "{}{} IS {} NOT {} NULL",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr).expect("expression should be parsed properly");
        assert_eq!(
            predicate,
            PartitionFilter {
                column,
                predicate: Predicate::IsNotNull
            }
        );
    }
}
