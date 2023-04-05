use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use std::collections::VecDeque;

static ALPHANUMERIC: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '.', '-',
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    EQ,
    GT,
    LT,
    GE,
    LE,
    NE,
    QT,
    Key(String),
    End,
}

type Tokens = VecDeque<Token>;

impl Token {
    fn lex(code: String) -> Result<Tokens> {
        let mut tokens: Tokens = VecDeque::new();
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
            match c {
                ' ' => continue,
                '=' => tokens.push_back(Token::EQ),
                '\'' => tokens.push_back(Token::QT),
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
                c if ALPHANUMERIC.contains(&c) => {
                    let tail: String = iter
                        .by_ref()
                        .take_while(|c| match ALPHANUMERIC.contains(c) {
                            true => true,
                            false => {
                                next = Some(*c);
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
    StrEqual { column: String, value: String },
    StrGreaterThan { column: String, value: String },
    StrLessThan { column: String, value: String },
    StrGreaterEqual { column: String, value: String },
    StrLessEqual { column: String, value: String },
    StrNotEqual { column: String, value: String },
    NumEqual { column: String, value: f64 },
    NumGreaterThan { column: String, value: f64 },
    NumLessThan { column: String, value: f64 },
    NumGreaterEqual { column: String, value: f64 },
    NumLessEqual { column: String, value: f64 },
    NumNotEqual { column: String, value: f64 },
    IsNull { column: String },
    IsNotNull { column: String },
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
            Some(Token::EQ) => {
                return Ok(Operator::Equal);
            }
            Some(Token::GT) => {
                return Ok(Operator::GreaterThan);
            }
            Some(Token::LT) => {
                return Ok(Operator::LessThan);
            }
            Some(Token::GE) => {
                return Ok(Operator::GreaterEqual);
            }
            Some(Token::LE) => {
                return Ok(Operator::LessEqual);
            }
            Some(Token::NE) => {
                return Ok(Operator::NotEqual);
            }
            Some(Token::Key(value)) if value.to_lowercase() == "is" => match tokens.pop_front() {
                Some(Token::Key(value)) if value.to_lowercase() == "null" => {
                    return Ok(Operator::IsNull);
                }
                Some(Token::Key(value)) if value.to_lowercase() == "not" => {
                    match tokens.pop_front() {
                        Some(Token::Key(value)) if value.to_lowercase() == "null" => {
                            return Ok(Operator::IsNotNull);
                        }
                        _ => {
                            return Err(anyhow!("failed to parse SQL operator"));
                        }
                    }
                }
                _ => {
                    return Err(anyhow!("failed to parse SQL operator"));
                }
            },
            _ => {
                return Err(anyhow!("failed to parse SQL operator"));
            }
        }
    }

    fn string(tokens: &mut Tokens) -> Result<String> {
        match tokens.pop_front() {
            Some(Token::QT) => {
                let Some(Token::Key(value)) = tokens.pop_front() else {
		    return Err(anyhow!("failed to parse alphabetic value"));
		};
                let Some(Token::QT) = tokens.pop_front() else {
		    return Err(anyhow!("failed to parse alphabetic value"));
		};
                return Ok(value);
            }
            _ => {
                return Err(anyhow!("failed to parse string"));
            }
        }
    }

    fn number(tokens: &mut Tokens) -> Result<f64> {
        match tokens.pop_front() {
            Some(Token::Key(value)) => {
                return Ok(value
                    .parse::<f64>()
                    .context("failed to parse numeric value")?);
            }
            _ => {
                return Err(anyhow!("failed to parse value"));
            }
        }
    }

    fn end(tokens: &mut Tokens) -> Result<()> {
        let Some(Token::End) = tokens.pop_front() else {
	    return Err(anyhow!("failed to parse end of SQL expression"));
	};
        Ok(())
    }

    pub fn parse(code: String) -> Result<Predicate> {
        let mut tokens = Token::lex(code).context("failed to lex given string")?;
        let column = Self::column(&mut tokens)
            .context("first entry of SQL expression should be column name")?;
        let operator = Self::operator(&mut tokens)
            .context("second entry of SQL expression should be operator")?;
        if operator == Operator::IsNull || operator == Operator::IsNotNull {
            Self::end(&mut tokens).context("invalid SQL expression")?;
            match operator {
                Operator::IsNull => return Ok(Predicate::IsNull { column }),
                Operator::IsNotNull => return Ok(Predicate::IsNotNull { column }),
                _ => {
                    return Err(anyhow!("failed to parse SQL expression"));
                }
            }
        } else {
            if Some(&Token::QT) == tokens.front() {
                let value = Self::string(&mut tokens)
                    .context("third entry of SQL expression should be value")?;
                Self::end(&mut tokens).context("invalid SQL expression")?;
                match operator {
                    Operator::Equal => {
                        return Ok(Predicate::StrEqual { column, value });
                    }
                    Operator::GreaterThan => {
                        return Ok(Predicate::StrGreaterThan { column, value })
                    }
                    Operator::LessThan => return Ok(Predicate::StrLessThan { column, value }),
                    Operator::GreaterEqual => {
                        return Ok(Predicate::StrGreaterEqual { column, value })
                    }
                    Operator::LessEqual => return Ok(Predicate::StrLessEqual { column, value }),
                    Operator::NotEqual => return Ok(Predicate::StrNotEqual { column, value }),
                    _ => {
                        return Err(anyhow!("failed to parse SQL expression"));
                    }
                }
            } else {
                let value = Self::number(&mut tokens)
                    .context("third entry of SQL expression should be value")?;
                Self::end(&mut tokens).context("invalid SQL expression")?;
                match operator {
                    Operator::Equal => return Ok(Predicate::NumEqual { column, value }),
                    Operator::GreaterThan => {
                        return Ok(Predicate::NumGreaterThan { column, value })
                    }
                    Operator::LessThan => return Ok(Predicate::NumLessThan { column, value }),
                    Operator::GreaterEqual => {
                        return Ok(Predicate::NumGreaterEqual { column, value })
                    }
                    Operator::LessEqual => return Ok(Predicate::NumLessEqual { column, value }),
                    Operator::NotEqual => return Ok(Predicate::NumNotEqual { column, value }),
                    _ => {
                        return Err(anyhow!("failed to parse SQL expression"));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5);
        let expr = format!(
            "{}{}{}={}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr.into()).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::EQ);
        assert_eq!(tokens[2], Token::Key(value.to_string()));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::i64(-15, 15);
        let expr = format!(
            "{}{}{}>{}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr.into()).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::GT);
        assert_eq!(tokens[2], Token::QT);
        assert_eq!(tokens[3], Token::Key(value.to_string()));
        assert_eq!(tokens[4], Token::QT);
        assert_eq!(tokens[5], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5);
        let expr = format!(
            "{}{}{}<{}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr.into()).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::LT);
        assert_eq!(tokens[2], Token::Key(value.to_string()));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::i64(-15, 15);
        let expr = format!(
            "{}{}{}>={}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr.into()).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::GE);
        assert_eq!(tokens[2], Token::QT);
        assert_eq!(tokens[3], Token::Key(value.to_string()));
        assert_eq!(tokens[4], Token::QT);
        assert_eq!(tokens[5], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5);
        let expr = format!(
            "{}{}{}<={}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr.into()).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::LE);
        assert_eq!(tokens[2], Token::Key(value.to_string()));
        assert_eq!(tokens[3], Token::End);
        let column = testutils::rand::string(10);
        let value = testutils::rand::i64(-15, 15);
        let expr = format!(
            "{}{}{}<>{}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr.into()).expect("expression should be parsed properly");
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], Token::Key(column));
        assert_eq!(tokens[1], Token::NE);
        assert_eq!(tokens[2], Token::QT);
        assert_eq!(tokens[3], Token::Key(value.to_string()));
        assert_eq!(tokens[4], Token::QT);
        assert_eq!(tokens[5], Token::End);
        let column = testutils::rand::string(10);
        let expr = format!(
            "{}{} IS {} NULL",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
        );
        let tokens = Token::lex(expr.into()).expect("expression should be parsed properly");
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
        let tokens = Token::lex(expr.into()).expect("expression should be parsed properly");
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
        let value = testutils::rand::f64(-1.5, 1.5);
        let expr = format!(
            "{}{}{}={}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr.into()).expect("expression should be parsed properly");
        assert_eq!(predicate, Predicate::NumEqual { column, value });

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
        let predicate = Utility::parse(expr.into()).expect("expression should be parsed properly");
        assert_eq!(predicate, Predicate::StrGreaterThan { column, value });
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5);
        let expr = format!(
            "{}{}{}<{}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr.into()).expect("expression should be parsed properly");
        assert_eq!(predicate, Predicate::NumLessThan { column, value });
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
        let predicate = Utility::parse(expr.into()).expect("expression should be parsed properly");
        assert_eq!(predicate, Predicate::StrGreaterEqual { column, value });
        let column = testutils::rand::string(10);
        let value = testutils::rand::f64(-1.5, 1.5);
        let expr = format!(
            "{}{}{}<={}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            value,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr.into()).expect("expression should be parsed properly");
        assert_eq!(predicate, Predicate::NumLessEqual { column, value });
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
        let predicate = Utility::parse(expr.into()).expect("expression should be parsed properly");
        assert_eq!(predicate, Predicate::StrNotEqual { column, value });
        let column = testutils::rand::string(10);
        let expr = format!(
            "{}{} IS {} NULL",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr.into()).expect("expression should be parsed properly");
        assert_eq!(predicate, Predicate::IsNull { column });
        let column = testutils::rand::string(10);
        let expr = format!(
            "{}{} IS {} NOT {} NULL",
            " ".repeat(testutils::rand::usize(10)),
            column,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
        );
        let predicate = Utility::parse(expr.into()).expect("expression should be parsed properly");
        assert_eq!(predicate, Predicate::IsNotNull { column });
    }
}
