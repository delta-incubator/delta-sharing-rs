use anyhow::anyhow;
use anyhow::Result;
use sqlx::postgres::PgDatabaseError;
use sqlx::Acquire;
use sqlx::Postgres;

const INTEGRITY_ERROR: &str = "23";

pub trait PgAcquire<'c>: Acquire<'c, Database = Postgres> + Send {}

impl<'c, T> PgAcquire<'c> for T where T: Acquire<'c, Database = Postgres> + Send {}

pub struct Utility;

impl Utility {
    pub fn error<T>(
        response: anyhow::Result<T>,
    ) -> Result<std::result::Result<T, Box<PgDatabaseError>>> {
        match response {
            Ok(v) => Ok(Ok(v)),
            Err(e) => match e.downcast::<sqlx::Error>() {
                Ok(sqlx::Error::Database(e)) => Ok(Err(e.downcast::<PgDatabaseError>())),
                _ => Err(anyhow!("unknow database error")),
            },
        }
    }

    pub fn is_conflict(error: &PgDatabaseError) -> bool {
        &error.code()[..2] == INTEGRITY_ERROR
    }
}
