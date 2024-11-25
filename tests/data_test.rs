use rs_quant::utils::parsers::{parse_end_date, parse_start_date};
use yahoo_finance_api::YahooConnector;

#[tokio::test]
async fn test_single_single_quote() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.get_latest_quotes("HNL.DE", "1d").await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "HNL.DE");
    assert_eq!(&res.chart.result[0].meta.range, "1mo");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn test_strange_api_response() {
    let provider = YahooConnector::new().unwrap();

    let start = parse_start_date("2019-01-01").unwrap();
    let end = parse_end_date("2020-07-04").unwrap();

    let res = provider.get_quote_history("IBM", start, end).await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "IBM");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    assert_eq!(
        &res.chart.result[0].meta.first_trade_date,
        &Some(-252322200)
    );

    let _ = res.last_quote().unwrap();
}

#[tokio::test]
#[should_panic(expected = "DeserializeFailed")]
async fn test_api_responses_missing_fields() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.get_latest_quotes("BF.B", "1m").await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "BF.B");
    assert_eq!(&res.chart.result[0].meta.range, "1d");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1m");

    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn test_get_quote_history() {
    let provider = YahooConnector::new().unwrap();

    let start = parse_start_date("2020-01-01").unwrap();
    let end = parse_end_date("2020-01-31").unwrap();

    let res = provider.get_quote_history("AAPL", start, end).await;
    if res.is_ok() {
        let res = res.unwrap();
        assert_eq!(res.chart.result[0].timestamp.len(), 21);
        let quotes = res.quotes().unwrap();
        assert_eq!(quotes.len(), 21);
    }
}

#[tokio::test]
async fn test_get_quote_range() {
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
    let res = provider.get_quote_range("SP5A.MI", "1d", "1mo").await;
    let metadata = res.unwrap().metadata().unwrap();
    assert_eq!(&metadata.symbol, "SP5A.MI");
    assert_eq!(&metadata.exchange_name, "MIL");
    assert_eq!(&metadata.instrument_type, "ETF");
}

#[tokio::test]
async fn test_get() {
    let provider = YahooConnector::new().unwrap();
    let start = parse_start_date("2019-01-01").unwrap();
    let end = parse_end_date("2020-01-31").unwrap();
    let res = provider
        .get_quote_history_interval("AAPL", start, end, "1mo")
        .await
        .unwrap();
    assert_eq!(&res.chart.result[0].timestamp.len(), &13);
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1mo");
    let quotes = res.quotes().unwrap();
    assert_eq!(quotes.len(), 13usize);
}

#[tokio::test]
async fn get_large_volume() {
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
    let res = provider.search_ticker("Apple").await.unwrap();

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
    let res = provider.search_options("AAPL").await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_mutual_fund_history() {
    let provider = YahooConnector::new().unwrap();

    let start = parse_start_date("2020-01-01").unwrap();
    let end = parse_end_date("2020-01-31").unwrap();

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
    let res = provider.get_latest_quotes("VTSAX", "1d").await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "VTSAX");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    assert_eq!(&res.chart.result[0].meta.range, "1mo");
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn test_mutual_fund_latest_with_null_first_trade() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.get_latest_quotes("SIWA.F", "1d").await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "SIWA.F");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    assert_eq!(&res.chart.result[0].meta.range, "1mo");
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn tes_mutual_fund_range() {
    let provider = YahooConnector::new().unwrap();
    let res = provider
        .get_quote_range("VTSAX", "1d", "1mo")
        .await
        .unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "VTSAX");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    assert_eq!(&res.chart.result[0].meta.range, "1mo");
    let _ = res.last_quote().unwrap();
}

#[tokio::test]
async fn test_mutual_fund_capital_gains() {
    let provider = YahooConnector::new().unwrap();
    let res = provider.get_quote_range("AMAGX", "1d", "5y").await.unwrap();

    assert_eq!(&res.chart.result[0].meta.symbol, "AMAGX");
    assert_eq!(&res.chart.result[0].meta.data_granularity, "1d");
    assert_eq!(&res.chart.result[0].meta.range, "5y");
    let capital_gains = res.capital_gains().unwrap();
    assert!(capital_gains.len() > 0usize);
}
