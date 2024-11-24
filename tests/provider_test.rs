use rs_quant::yf_client::YahooConnector;
use time::macros::datetime;

#[tokio::test]
async fn test_single_quote() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.get_latest_quote("NVDA", "1d").await.unwrap();
    assert_eq!(&res.chart.result[0].meta.symbol, "NVDA");
    assert_eq!(&res.chart.result[0].meta.range, "1mo");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn test_strange_api_response() {
    let provider = YahooConnector::new().unwrap();

    let start = datetime!(2019-07-03 0:00:00.0 UTC);
    let end = datetime!(2019-07-04 23:59:59.99 UTC);

    let res = provider.get_quote_history("IBM", start, end).await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "IBM");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    assert_eq!(
        &res.chart.result[0].meta.first_date_trade,
        &Some(-252322200)
    );
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
#[should_panic(expected = "DeserializeFailed")]
async fn test_api_responses_missing_fileds() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.get_latest_quote("BF.B", "1m").await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "BF");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1m");
    assert_eq!(&res.chart.result[0].meta.range, "1d");
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn test_get_quote_history() {
    let provider = YahooConnector::new().unwrap();

    let start = datetime!(2020-01-01 0:00:00.0 UTC);
    let end = datetime!(2020-01-31 23:59:59.99 UTC);

    let res = provider.get_quote_history("AAPL", start, end).await;
    if res.is_ok() {
        let res = res.unwrap();
        assert_eq!(res.chart.result[0].timestamp.len(), 21);
        let quotes = res.quotes().unwrap();
        assert_eq!(quotes.len(), 21);
    }
}

#[tokio::test]
async fn test_quote_range() {
    let provider = YahooConnector::new().unwrap();
    let res = provider
        .get_quote_range("HNL.DE", "1d", "1mo")
        .await
        .unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "HNL.DE");
    assert_eq!(&res.chart.result[0].meta.range, "1mo");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn test_get_metadata() {
    let provider = YahooConnector::new().unwrap();
    let res = provider
        .get_quote_range("HLN.DE", "1d", "1mo")
        .await
        .unwrap();
    let meta = res.metadata().unwrap();
    assert_eq!(meta.symbol, "HLN.DE");
}

#[tokio::test]
async fn test_get() {
    let provider = YahooConnector::new().unwrap();
    let start = datetime!(2019-01-01 0:00:00.00 UTC);
    let end = datetime!(2020-01-31 23:59:59.99 UTC);

    let response = provider
        .get_quote_history_interval("AAPL", start, end, "1mo")
        .await
        .unwrap();
    assert_eq!(&response.chart.result[0].timestamp.len(), &13);
    assert_eq!(&response.chart.result[0].meta.data_granularity, "1mo");
    let quotes = response.quotes().unwrap();
    assert_eq!(quotes.len(), 13usize);
}

#[tokio::test]
async fn test_large_volume() {
    let provider = YahooConnector::new().unwrap();
    let res = provider
        .get_quote_range("BTC-USD", "1d", "5d")
        .await
        .unwrap();
    let quotes = res.quotes().unwrap();
    assert!(quotes.len() > 0usize);
}

#[tokio::test]
async fn test_search_ticker() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.search_ticker_("Apple").await.unwrap();

    assert_eq!(res.count, 15);
    let mut apple_found = false;
    for i in res.quotes {
        if i.exchange == "NMS" && i.symbol == "AAPL" && i.short_name == "Apple Inc." {
            apple_found = true;
            break;
        }
    }
    assert!(apple_found);
}

#[tokio::test]
async fn search_options() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.search_options("AAPL");

    assert!(res.await.is_ok());
}

#[tokio::test]
async fn test_mutual_fund_history() {
    let provider = YahooConnector::new().unwrap();

    let start = datetime!(2020-01-01 0:00:00.00 UTC);
    let end = datetime!(2020-01-31 23:59:59.99 UTC);

    let res = provider.get_quote_history("VTSAX", start, end).await;
    if res.is_ok() {
        let res = res.unwrap();
        assert_eq!(res.chart.result[0].timestamp.len(), 21);
        let quotes = res.quotes().unwrap();
        assert_eq!(quotes.len(), 21);
    }
}

#[tokio::test]
async fn test_mutual_fund_latest() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.get_latest_quote("VTSAX", "1d").await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "VTSAX");
    assert_eq!(&res.chart.result[0].meta.range, "1mo");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn test_mutual_fund_latest_with_null_first_trade_date() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.get_latest_quote("SIWA.F", "1d").await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "SIWA.F");
    assert_eq!(&res.chart.result[0].meta.range, "1mo");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn test_mutual_fund_range() {
    let provider = YahooConnector::new().unwrap();
    let res = provider
        .get_quote_range("VTSAX", "1d", "1mo")
        .await
        .unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "VTSAX");
    assert_eq!(&res.chart.result[0].meta.range, "1mo");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
}

#[tokio::test]
async fn test_mutual_fund_capital() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.get_quote_range("AMAGX", "1d", "5y").await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "AMAGX");
    assert_eq!(&res.chart.result[0].meta.range, "5y");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    let capital_gains = res.capital_gains().unwrap();
    assert!(capital_gains.len() > 0usize);
}
