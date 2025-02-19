use dotenvy::dotenv;
use sqlx::{Error as SqlxError, MySqlPool};
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
pub struct DbConnection {
    pub pool: MySqlPool,
}

impl DbConnection {
    pub async fn new() -> Result<Self, DbError> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").map_err(|_| DbError::MissingDatabaseUrl)?;
        Ok(Self {
            pool: MySqlPool::connect(&db_url).await?,
        })
    }

    pub fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
