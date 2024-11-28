use rs_quant::data::options::ContractType;
use rs_quant::data::provider::YahooFinance;

#[tokio::test]
async fn test_option_metadata() {
    let conn = YahooFinance::connector();
    let meta = conn.get_options_metadata("AAPL").await.unwrap();

    assert_eq!(meta.region, "US");
    assert_eq!(meta.symbol, "AAPL");
    assert_eq!(meta.exchange, "NMS");
    assert_eq!(meta.full_exchange_name, "NasdaqGS");
    assert_eq!(meta.currency, "USD");
    assert_eq!(meta.long_name, "Apple Inc.");
}

#[tokio::test]
async fn test_options_expiration_dates() {
    let conn = YahooFinance::connector();
    let dates = conn.get_options_expiration_dates("META").await;
    assert!(dates.is_ok());

    let dates_n = dates.unwrap();
    assert_ne!(dates_n.len(), 0);
}

#[tokio::test]
async fn test_options_strike_prices() {
    let conn = YahooFinance::connector();
    let strikes = conn.get_options_strike_prices("AVGO").await;
    assert!(strikes.is_ok());

    let strikes = strikes.unwrap();
    assert_ne!(strikes.len(), 0);
}

#[tokio::test]
async fn test_options_contracts() {
    let conn = YahooFinance::connector();
    let calls = conn
        .get_options_contracts("GS", ContractType::Call)
        .await
        .unwrap();
    assert_ne!(calls.len(), 0);

    let puts = conn
        .get_options_contracts("NVDA", ContractType::Put)
        .await
        .unwrap();
    assert_ne!(puts.len(), 0);
}
