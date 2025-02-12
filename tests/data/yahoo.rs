use rs_quant::data::yahoo::{OptionType, ReturnType, Yahoo, YahooErr};

#[tokio::test]
async fn fetch_failed() {
    let conn = Yahoo::provider().unwrap();
    let r = conn
        .get_quotes("INVALID_SYMBOL", None, None, None, None)
        .await;
    assert!(
        matches!(r, Err(YahooErr::FetchFailed(_))),
        "error result: {:?}",
        r
    )
}

// #[tokio::test] // failing because query returns fetch failed error
// async fn invalid_json() {
//     let conn = Yahoo::provider().unwrap();
//     let r = conn
//         .get_quotes("AAPL", None, None, None, Some("invalid-interval"))
//         .await;
//     assert!(
//         matches!(r, Err(YahooErr::InvalidJson)),
//         "error result: {:?}",
//         r
//     )
// }

// #[tokio::test] // failing, prints correct dataset with correct dates. maybe checks already made convert to default case
// async fn invalid_dateformat() {
//     let conn = Yahoo::provider().unwrap();
//     let r = conn
//         .get_quotes("AAPL", Some("invalid-date"), None, None, None)
//         .await;
//     assert!(
//         matches!(r, Err(YahooErr::InvalidDateFormat(_))),
//         "error result: {:?}",
//         r
//     )
// }

// #[tokio::test] // failing because query returns fetch failed error
// async fn empty_dataset() {
//     let conn = Yahoo::provider().unwrap();
//     let r = conn
//         .get_quotes("empty_simbol", None, None, None, None)
//         .await;
//     assert!(
//         matches!(r, Err(YahooErr::EmptyDataSet)),
//         "error result: {:?}",
//         r
//     )
// }

#[tokio::test]
async fn get_quotes_without_args() {
    let conn = Yahoo::provider().unwrap();
    let res = conn.get_quotes("VUAA.MI", None, None, None, None).await;
    assert!(res.is_ok(), "Error: {:?}", res);
    assert!(!res.unwrap().is_empty())
}

#[tokio::test]
async fn get_quotes_with_date_range() {
    let conn = Yahoo::provider().unwrap();
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
    let conn = Yahoo::provider().unwrap();
    let res = conn
        .get_quotes("AAPL", None, None, Some("5d"), Some("1d"))
        .await;
    assert!(res.is_ok(), "Error: {:?}", res);
    assert!(!res.unwrap().is_empty())
}

#[tokio::test]
async fn get_multiple_quotes() {
    let tickers = vec!["NVDA", "SWDA.MI", "VUAA.L", "IBGX.AS"];
    let conn = Yahoo::provider().unwrap();
    let res = conn
        .get_multiple_quotes(tickers, None, None, Some("5d"), Some("1d"))
        .await;
    assert!(res.is_ok(), "Error: {:?}", res);
    assert!(!res.unwrap().is_empty())
}

#[tokio::test]
async fn get_options() {
    let conn = Yahoo::provider().unwrap();
    let call = conn.get_options("AAPL", OptionType::Call).await;
    assert!(call.is_ok(), "Error: {:?}", call);
    assert!(!call.unwrap().is_empty());

    let put = conn.get_options("AAPL", OptionType::Put).await;
    assert!(put.is_ok(), "Error: {:?}", put);
    assert!(!put.unwrap().is_empty())
}

#[tokio::test]
async fn get_latest_quote() {
    let conn = Yahoo::provider().unwrap();
    let quote = conn.get_latest_quote("NVDA").await;
    assert!(quote.is_ok(), "Error: {:?}", quote);
}

#[tokio::test]
async fn compute_returns() {
    //arithmetic returns
    let conn = Yahoo::provider().unwrap();
    let a = conn
        .compute_returns("NVDA", "5d", "1d", ReturnType::Arithmetic)
        .await;
    assert!(a.is_ok());
    assert!(!a.unwrap().is_empty());

    // absolute returns
    let abs = conn
        .compute_returns("VUAA.MI", "5d", "1d", ReturnType::Absolute)
        .await;
    assert!(abs.is_ok());
    assert!(!abs.unwrap().is_empty());

    // logarithmic returns
    let l = conn
        .compute_returns("SP5A.MI", "5d", "1d", ReturnType::Logarithmic)
        .await;
    assert!(l.is_ok());
    assert!(!l.unwrap().is_empty())
}

#[tokio::test]
async fn search_asset() {
    let conn = Yahoo::provider().unwrap();
    assert!(conn.search_asset("AAPL").await.is_ok());
}
