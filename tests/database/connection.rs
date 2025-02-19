use rs_quant::database::connection::DbConnection;

#[tokio::test]
async fn database_connection() {
    let pool = DbConnection::new().await;
    assert!(pool.is_ok());
}
