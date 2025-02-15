use rs_quant::portfolio::position::Position;

#[test]
fn position_new() {
    let symbol = "AAPL";
    let quantity = 100;
    let currency = "USD";
    let buy_date = "2023-12-01";
    let buy_price = 150.0;
    let buy_value = 15000.0;

    // define portfolio position
    let pos = Position::new(symbol, quantity, currency, buy_date, buy_price, buy_value);
    // check assigned value are correct
    assert_eq!(pos.symbol, "AAPL");
    assert_eq!(pos.quantity, 100);
    assert_eq!(pos.currency, "USD");
    assert_eq!(pos.buy_date, "2023-12-01");
    assert_eq!(pos.buy_price, 150.0);
    assert_eq!(pos.buy_value, 15000.0);

    // check default values are correct
    assert_eq!(pos.market_value, 0.0);
    assert_eq!(pos.equity, 0.0);
    assert_eq!(pos.pl, 0.0);
    assert_eq!(pos.pl_pct, 0.0)
}

#[tokio::test]
async fn update_mkt_price() {
    let mut position = Position {
        symbol: "AAPL".to_string(),
        quantity: 10,
        currency: "USD".to_string(),
        buy_date: "2023-12-01".to_string(),
        buy_price: 150.0,
        buy_value: 1500.0,
        market_value: 0.0,
        equity: 0.0,
        pl: 0.0,
        pl_pct: 0.0,
        weight: 0.0,
    };

    position.update_mkt_price().await.unwrap();
    assert_ne!(position.market_value, 0.0);
}

#[test]
fn pl() {
    let mut position = Position {
        symbol: "AAPL".to_string(),
        quantity: 10,
        currency: "USD".to_string(),
        buy_date: "2023-12-01".to_string(),
        buy_price: 150.0,
        buy_value: 1500.0,
        market_value: 200.0,
        equity: 2000.0,
        pl: 0.0,
        pl_pct: 0.0,
        weight: 0.0,
    };

    let _ = position.pl();
    assert_eq!(position.pl, 500.0);
}

#[test]
fn pl_pct() {
    let mut position = Position {
        symbol: "AAPL".to_string(),
        quantity: 10,
        currency: "USD".to_string(),
        buy_date: "2023-12-01".to_string(),
        buy_price: 150.0,
        buy_value: 1500.0,
        market_value: 200.0,
        equity: 2000.0,
        pl: 500.0,
        pl_pct: 0.0,
        weight: 0.0,
    };

    let _ = position.pl_pct();
    assert_eq!(position.pl_pct, 0.33333333333333326);
}
