use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

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

fn lex(code: String) -> Result<Vec<Token>> {
    let mut iter = code.chars().peekable();
    let mut toks: Vec<Token> = Vec::new();
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
            '=' => toks.push(Token::EQ),
            '\'' => toks.push(Token::QT),
            '>' => {
                if iter.peek() == Some(&'=') {
                    let _ = iter.by_ref().take(1);
                    toks.push(Token::GE);
                } else {
                    toks.push(Token::GT);
                }
            }
            '<' => {
                if iter.peek() == Some(&'=') {
                    let _ = iter.by_ref().take(1);
                    toks.push(Token::LE);
                } else if iter.peek() == Some(&'>') {
                    let _ = iter.by_ref().take(1);
                    toks.push(Token::NE);
                } else {
                    toks.push(Token::LT);
                }
            }
            c if c.is_ascii_alphanumeric() => {
                let tail: String = iter
                    .by_ref()
                    .take_while(|c| match c.is_ascii_alphanumeric() {
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
                toks.push(Token::Key(key));
            }
            _ => return Err(anyhow!("could not recognize a character: {}", c)),
        }
    }
    toks.push(Token::End);
    Ok(toks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let expr = "col = 10";
        let ret = lex(expr.into());
        println!("{:?}", ret);
        let expr = "col >= '10'";
        let ret = lex(expr.into());
        println!("{:?}", ret);
    }
}
