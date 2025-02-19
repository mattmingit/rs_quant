use chrono::NaiveDate;
use serde::Serialize;
use sqlx::{types::Decimal, Error as SqlxError, FromRow, MySql};
use thiserror::Error;

use super::connection::DbConnection;

#[derive(Debug, Error)]
pub enum DbQueryError {
    #[error("Query execution failed: {0}")]
    QueryError(#[from] SqlxError),
    #[error("Query returned empty result.")]
    Notfound,
}

#[derive(FromRow, Serialize, Debug)]
pub struct VWPortfolio {
    pub ticker: String,
    pub quantity: Decimal,
    pub currency: String,
    pub buy_date: NaiveDate,
    pub buy_price: Decimal,
    pub buy_value: Decimal,
    pub market_price: Decimal,
    pub market_value: Decimal,
    #[sqlx(rename = "PL")]
    pub pl: Decimal,
    #[sqlx(rename = "PL_pct")]
    pub pl_pct: Decimal,
}

pub async fn portfolio_tickers(pool: &DbConnection<MySql>) -> Result<Vec<String>, DbQueryError> {
    let r = sqlx::query!("SELECT ticker FROM vw_portfolio")
        .fetch_all(&pool.pool)
        .await?;

    if r.is_empty() {
        return Err(DbQueryError::Notfound);
    }
    Ok(r.iter().map(|r| r.ticker.clone()).collect())
}

pub async fn portfolio_table(pool: &DbConnection<MySql>) -> Result<Vec<VWPortfolio>, DbQueryError> {
    Ok(sqlx::query_as::<_, VWPortfolio>("SELECT ticker, quantity, currency, buy_date, buy_price, buy_value, market_price, market_value, PL, PL_pct FROM vw_portfolio").fetch_all(&pool.pool).await?)
}
