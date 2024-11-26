use rs_quant::data::quotes;

#[tokio::test]
async fn get_quotes_without_args() {
    let res = quotes::get_quotes("VUAA.MI", None, None, None, None).await;
    assert!(res.is_ok());
    let quotes = res.unwrap();
    assert!(quotes.len() > 1usize);
}

#[tokio::test]
async fn get_quotes_with_date_range() {
    let res = quotes::get_quotes(
        "VUAA.MI",
        Some("2024-11-11"),
        Some("2024-11-26"),
        None,
        None,
    )
    .await;
    assert!(res.is_ok());
    let quotes = res.unwrap();
    assert!(quotes.len() > 1usize);
}

#[tokio::test]
async fn get_quotes_with_date_range_and_interval() {
    let res = quotes::get_quotes(
        "NVDA",
        Some("2024-01-01"),
        Some("2024-11-01"),
        None,
        Some("1mo"),
    )
    .await;
    assert!(res.is_ok());
    let quotes = res.unwrap();
    assert!(quotes.len() > 1usize);
}

#[tokio::test]
async fn get_quotes_with_range() {
    let res = quotes::get_quotes("AAPL", None, None, Some("5d"), None).await;
    assert!(res.is_ok());
    let quotes = res.unwrap();
    assert_eq!(quotes.len(), 5);
}

#[tokio::test]
async fn get_quotes_with_range_and_interval() {
    let res = quotes::get_quotes("AAPL", None, None, Some("ytd"), Some("1mo")).await;
    assert!(res.is_ok());
    let quotes = res.unwrap();
    assert!(quotes.len() >= 1 && quotes.len() <= 12);
}

#[tokio::test]
async fn get_multiple_quotes() {
    let tickers = vec!["NVDA", "SWDA.MI", "VUAA.L", "IBGX.AS"];
    let quotes = quotes::get_multiple_quotes(tickers, None, None, Some("5d"), None).await;
    assert!(quotes.is_ok());
}
