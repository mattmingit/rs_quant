#[allow(unused_imports)]
use rs_quant::database::connection::DbConnection;
#[allow(unused_imports)]
use sqlx::MySql;

#[cfg(feature = "db_tests")]
#[tokio::test]
async fn database_connection() {
    let pool = DbConnection::<MySql>::new().await;
    assert!(pool.is_ok());
}
