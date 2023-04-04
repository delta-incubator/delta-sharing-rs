use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

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
        let col = testutils::rand::string(10);
        let val = testutils::rand::f64(-1.5, 1.5);
        let expr = format!(
            "{}{}{}={}{}{}",
            " ".repeat(testutils::rand::usize(10)),
            col,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            val,
            " ".repeat(testutils::rand::usize(10)),
        );
        let toks = lex(expr.into()).expect("expression should be parsed properly");
        assert_eq!(toks.len(), 4);
        assert_eq!(toks[0], Token::Key(col));
        assert_eq!(toks[1], Token::EQ);
        assert_eq!(toks[2], Token::Key(val.to_string()));
        assert_eq!(toks[3], Token::End);

        let col = testutils::rand::string(10);
        let val = testutils::rand::i64(-15, 15);
        let expr = format!(
            "{}{}{}={}'{}'{}",
            " ".repeat(testutils::rand::usize(10)),
            col,
            " ".repeat(testutils::rand::usize(10)),
            " ".repeat(testutils::rand::usize(10)),
            val,
            " ".repeat(testutils::rand::usize(10)),
        );
        let toks = lex(expr.into()).expect("expression should be parsed properly");
        assert_eq!(toks.len(), 6);
        assert_eq!(toks[0], Token::Key(col));
        assert_eq!(toks[1], Token::EQ);
        assert_eq!(toks[2], Token::Key(val.to_string()));
        let expr = "col >= '10'";
        let ret = lex(expr.into());
        println!("{:?}", ret);
    }
}
