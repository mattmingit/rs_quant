use rs_quant::database::connection::DbConnection;
use rs_quant::database::queries::{portfolio_table, portfolio_tickers};

#[tokio::test]
async fn test_portfolio_tickers() {
    let pool = DbConnection::new().await.unwrap();
    let t = portfolio_tickers(&pool.pool).await;
    assert!(t.is_ok());
}

#[tokio::test]
async fn test_portfolio_table() {
    let pool = DbConnection::new().await.unwrap();
    let t = portfolio_table(&pool.pool).await;
    assert!(t.is_ok());
}
