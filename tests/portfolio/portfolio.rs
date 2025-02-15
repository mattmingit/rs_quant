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
    let pool = DbConnection::<sqlx::MySql>::new().await.unwrap();
    let data = portfolio_table(&pool).await.unwrap();
    let p = Portfolio::from_database(data);

    // check positions number
    assert_eq!(
        p.positions.len(),
        9,
        "Portfolio should contain 9 positions."
    );

    // check individual position for first and seventh positions
    let first_pos = &p.positions[0];
    assert_eq!(first_pos.symbol, "SWDA.MI");
    assert_eq!(first_pos.quantity, 16);
    assert_eq!(first_pos.buy_value, 1354.900);

    let six_pos = &p.positions[6];
    assert_eq!(six_pos.symbol, "NVDA");
    assert_eq!(six_pos.quantity, 13);
    assert_eq!(six_pos.buy_value, 1474.230);
}

#[tokio::test]
async fn total_investment() {
    let pool = DbConnection::<sqlx::MySql>::new().await.unwrap();
    let data = portfolio_table(&pool).await.unwrap();
    let p = Portfolio::from_database(data);

    assert_eq!(p.total_investments(), 8391.47);
}

#[tokio::test]
async fn total_market_val() {
    let pool = DbConnection::<sqlx::MySql>::new().await.unwrap();
    let data = portfolio_table(&pool).await.unwrap();
    let p = Portfolio::from_database(data);

    assert_eq!(p.total_mkt_val(), 9483.07);
}

#[tokio::test]
async fn total_pl() {
    let pool = DbConnection::<sqlx::MySql>::new().await.unwrap();
    let data = portfolio_table(&pool).await.unwrap();
    let p = Portfolio::from_database(data);

    assert_eq!(p.total_pl(), 1091.6000000000001);
}

#[tokio::test]
async fn weights() {
    let pool = DbConnection::<sqlx::MySql>::new().await.unwrap();
    let data = portfolio_table(&pool).await.unwrap();
    let mut portfolio = Portfolio::from_database(data);
    portfolio.weights();
    assert_ne!(portfolio.positions[0].weight, 0f64);
}
