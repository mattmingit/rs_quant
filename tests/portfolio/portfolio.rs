use rs_quant::database::connection::DbConnection;
use rs_quant::database::queries::portfolio_table;
use rs_quant::portfolio::portfolio::Portfolio;

#[test]
fn new_portfolio() {
    assert_eq!(
        Portfolio::new().positions.len(),
        0,
        "New portfolio should have no positions."
    );
}

#[test]
fn default_portfolio() {
    assert_eq!(
        Portfolio::default().positions.len(),
        0,
        "Default method should have no positions."
    )
}

#[tokio::test]
async fn from_db() {
    let pool = DbConnection::new().await.unwrap();
    let data = portfolio_table(&pool.pool).await.unwrap();
    let p = Portfolio::from_database(data);

    // check positions number
    assert_eq!(
        p.positions.len(),
        9,
        "Portfolio should contain 9 positions."
    );

    // check individual position for first and seventh positions
    let first_pos = &p.positions[0];
    assert_eq!(first_pos.symbol, "EIMI.MI");
    assert_eq!(first_pos.quantity, 15);
    assert_eq!(first_pos.buy_value, 455.02);

    let six_pos = &p.positions[4];
    assert_eq!(six_pos.symbol, "NVDA");
    assert_eq!(six_pos.quantity, 13);
    assert_eq!(six_pos.buy_value, 1474.230);
}

#[tokio::test]
async fn total_investment() {
    let pool = DbConnection::new().await.unwrap();
    let data = portfolio_table(&pool.pool).await.unwrap();
    let p = Portfolio::from_database(data);

    assert_eq!(p.total_investments(), 8391.47);
}

#[tokio::test]
async fn total_market_val() {
    let pool = DbConnection::new().await.unwrap();
    let data = portfolio_table(&pool.pool).await.unwrap();
    let p = Portfolio::from_database(data);

    assert_eq!(p.total_mkt_val(), 9552.689999999999);
}

#[tokio::test]
async fn total_pl() {
    let pool = DbConnection::new().await.unwrap();
    let data = portfolio_table(&pool.pool).await.unwrap();
    let p = Portfolio::from_database(data);

    assert_eq!(p.total_pl(), 1161.2199999999998);
}

#[tokio::test]
async fn weights() {
    let pool = DbConnection::new().await.unwrap();
    let data = portfolio_table(&pool.pool).await.unwrap();
    let mut portfolio = Portfolio::from_database(data);
    portfolio.weights();
    assert_ne!(portfolio.positions[0].weight, 0f64);
}
