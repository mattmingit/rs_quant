use rs_quant::data::yahoo::{OptionType, Yahoo};

#[tokio::test]
async fn get_quotes_without_args() {
    let conn = Yahoo::provider();
    let res = conn.get_quotes("VUAA.MI", None, None, None, None).await;
    assert!(res.is_ok(), "Error: {:?}", res);
    assert!(!res.unwrap().is_empty())
}

#[tokio::test]
async fn get_quotes_with_date_range() {
    let conn = Yahoo::provider();
    let res = conn
        .get_quotes(
            "VUAA.MI",
            Some("2024-01-01"),
            Some("2025-01-01"),
            None,
            Some("1d"),
        )
        .await;
    assert!(res.is_ok(), "Error: {:?}", res);
    assert!(!res.unwrap().is_empty())
}

#[tokio::test]
async fn get_quotes_with_period() {
    let conn = Yahoo::provider();
    let res = conn
        .get_quotes("AAPL", None, None, Some("5d"), Some("1d"))
        .await;
    assert!(res.is_ok(), "Error: {:?}", res);
    assert!(!res.unwrap().is_empty())
}

#[tokio::test]
async fn get_multiple_quotes() {
    let tickers = vec!["NVDA", "SWDA.MI", "VUAA.L", "IBGX.AS"];
    let conn = Yahoo::provider();
    let res = conn
        .get_multiple_quotes(tickers, None, None, Some("5d"), Some("1d"))
        .await;
    assert!(res.is_ok(), "Error: {:?}", res);
    assert!(!res.unwrap().is_empty())
}

#[tokio::test]
async fn get_options() {
    let conn = Yahoo::provider();
    let call = conn.get_options("AAPL", OptionType::Call).await;
    assert!(call.is_ok(), "Error: {:?}", call);
    assert!(!call.unwrap().is_empty());

    let put = conn.get_options("AAPL", OptionType::Put).await;
    assert!(put.is_ok(), "Error: {:?}", put);
    assert!(!put.unwrap().is_empty())
}

#[tokio::test]
async fn get_latest_quote() {
    let conn = Yahoo::provider();
    let quote = conn.get_latest_quote("NVDA").await;
    assert!(quote.is_ok(), "Error: {:?}", quote);
}
