use rs_quant::database::connection::DbConnection;
use sqlx::MySql;

#[tokio::test]
async fn database_connection() {
    let pool = DbConnection::<MySql>::new().await;
    assert!(pool.is_ok());
}
