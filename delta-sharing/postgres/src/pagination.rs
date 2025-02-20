// adopted from https://github.com/lakekeeper/lakekeeper/blob/main/crates/iceberg-catalog/src/implementations/postgres/pagination.rs

use base64::Engine;
use chrono::{DateTime, Utc};

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub(crate) enum PaginateToken<T> {
    V1(V1PaginateToken<T>),
}

#[derive(Debug, PartialEq)]
pub(crate) struct V1PaginateToken<T> {
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) id: T,
}

impl<T> std::fmt::Display for PaginateToken<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_string = match self {
            PaginateToken::V1(V1PaginateToken { created_at, id }) => {
                format!("1&{}&{}", created_at.timestamp_micros(), id)
            }
        };
        write!(
            f,
            "{}",
            base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(&token_string)
        )
    }
}

impl<T, Z> TryFrom<&str> for PaginateToken<T>
where
    T: for<'a> TryFrom<&'a str, Error = Z> + std::fmt::Display,
    Z: std::error::Error + Send + Sync + 'static,
{
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = String::from_utf8(base64::prelude::BASE64_URL_SAFE_NO_PAD.decode(s)?)
            .map_err(|_| Error::generic("Decoded b64 contained an invalid utf8-sequence."))?;

        let parts = s.splitn(3, '&').collect::<Vec<_>>();

        match *parts
            .first()
            .ok_or_else(|| Error::generic("empty page token."))?
        {
            "1" => match &parts[1..] {
                &[ts, id] => {
                    let created_at = chrono::DateTime::from_timestamp_micros(
                        ts.parse()
                            .map_err(|_| Error::generic("invalid timestamp"))?,
                    )
                    .ok_or_else(|| Error::generic("invalid timestamp"))?;
                    let id = id
                        .try_into()
                        .map_err(|e| Error::generic(format!("invalid id: {e:?}")))?;
                    Ok(PaginateToken::V1(V1PaginateToken { created_at, id }))
                }
                _ => Err(Error::generic("unexpected segment count")),
            },
            _ => Err(Error::generic("invalid page token version")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_paginate_token() {
        let created_at = Utc::now();
        let token = PaginateToken::V1(V1PaginateToken {
            created_at,
            id: Uuid::nil(),
        });

        let token_str = token.to_string();
        let token: PaginateToken<Uuid> = PaginateToken::try_from(token_str.as_str()).unwrap();
        // we lose some precision while serializing the timestamp making tests flaky
        let created_at =
            chrono::DateTime::from_timestamp_micros(created_at.timestamp_micros()).unwrap();
        assert_eq!(
            token,
            PaginateToken::V1(V1PaginateToken {
                created_at,
                id: Uuid::nil(),
            })
        );
    }

    #[test]
    fn test_paginate_token_with_ampersand() {
        let created_at = Utc::now();
        let token = PaginateToken::V1(V1PaginateToken {
            created_at,
            id: "kubernetes/some-name&with&ampersand".to_string(),
        });

        let token_str = token.to_string();
        let token: PaginateToken<String> = PaginateToken::try_from(token_str.as_str()).unwrap();
        // we lose some precision while serializing the timestamp making tests flaky
        let created_at =
            chrono::DateTime::from_timestamp_micros(created_at.timestamp_micros()).unwrap();
        assert_eq!(
            token,
            PaginateToken::V1(V1PaginateToken {
                created_at,
                id: "kubernetes/some-name&with&ampersand".to_string(),
            })
        );
    }

    #[test]
    fn test_paginate_token_with_user_id() {
        let created_at = Utc::now();
        let token = PaginateToken::V1(V1PaginateToken {
            created_at,
            id: "kubernetes/some-name",
        });

        let token_str = token.to_string();
        let token: PaginateToken<String> = PaginateToken::try_from(token_str.as_str()).unwrap();
        // we lose some precision while serializing the timestamp making tests flaky
        let created_at =
            chrono::DateTime::from_timestamp_micros(created_at.timestamp_micros()).unwrap();
        assert_eq!(
            token,
            PaginateToken::V1(V1PaginateToken {
                created_at,
                id: "kubernetes/some-name".to_string(),
            })
        );
    }
}
