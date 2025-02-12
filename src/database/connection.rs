use dotenvy::dotenv;
use sqlx::{Database, Error as SqlxError, Pool};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database URL not set in environment variables.")]
    MissingDatabaseUrl,
    #[error("Database connection failed: {0}")]
    DbConnectionError(#[from] SqlxError),
}

#[derive(Debug)]
pub struct DbConnection<DB: Database> {
    pub pool: Pool<DB>,
}

impl<DB: Database> DbConnection<DB> {
    pub async fn new() -> Result<Self, DbError> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").map_err(|_| DbError::MissingDatabaseUrl)?;
        Ok(Self {
            pool: Pool::<DB>::connect(&db_url).await?,
        })
    }

    pub fn get_pool(&self) -> &Pool<DB> {
        &self.pool
    }
}
