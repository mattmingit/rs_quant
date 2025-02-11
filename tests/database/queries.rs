#[allow(unused_imports)]
use rs_quant::database::connection::DbConnection;
#[allow(unused_imports)]
use rs_quant::database::queries::{portfolio_table, portfolio_tickers};

#[cfg(feature = "db_tests")]
#[tokio::test]
async fn test_portfolio_tickers() {
    let pool = DbConnection::<sqlx::MySql>::new().await.unwrap();
    let t = portfolio_tickers(&pool).await;
    assert!(t.is_ok());
}

#[cfg(feature = "db_tests")]
#[tokio::test]
async fn test_portfolio_table() {
    let pool = DbConnection::<sqlx::MySql>::new().await.unwrap();
    let t = portfolio_table(&pool).await;
    assert!(t.is_ok());
}
