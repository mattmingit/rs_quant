use rs_quant::data::provider::YahooFinance;
use rs_quant::data::quotes::QuoteItem;
use rs_quant::quantitative::returns::{
    compute_cumulative_returns, compute_returns, expected_market_return, CumulativeReturn, Return,
    ReturnType,
};

#[test]
fn test_compute_returns_arithmetic() {
    let data = vec![
        QuoteItem {
            datetime: "2024-01-01".to_string(),
            open: 10.0,
            high: 10.1,
            low: 10.0,
            close: 10.05,
            adjclose: 10.05,
            volume: 219,
        },
        QuoteItem {
            datetime: "2024-01-02".to_string(),
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 10.1,
            adjclose: 10.1,
            volume: 759,
        },
        QuoteItem {
            datetime: "2024-01-03".to_string(),
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 20.1,
            adjclose: 20.1,
            volume: 759,
        },
        QuoteItem {
            datetime: "2024-01-04".to_string(),
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 10.1,
            adjclose: 100.1,
            volume: 759,
        },
    ];

    let result = compute_returns(data, ReturnType::Arithmetic).unwrap();
    assert!(result.len() > 1usize);
}

#[test]
fn test_compute_returns_logarithmic() {
    let data = vec![
        QuoteItem {
            datetime: "2024-01-01".to_string(),
            open: 10.0,
            high: 10.1,
            low: 10.0,
            close: 10.05,
            adjclose: 10.05,
            volume: 219,
        },
        QuoteItem {
            datetime: "2024-01-02".to_string(),
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 10.1,
            adjclose: 10.1,
            volume: 759,
        },
        QuoteItem {
            datetime: "2024-01-03".to_string(),
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 20.1,
            adjclose: 20.1,
            volume: 759,
        },
        QuoteItem {
            datetime: "2024-01-04".to_string(),
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 10.1,
            adjclose: 100.1,
            volume: 759,
        },
    ];

    let result = compute_returns(data, ReturnType::Logarithmic).unwrap();
    assert!(result.len() > 1usize);
}

#[tokio::test]
async fn test_with_yahoo_data() {
    let conn = YahooFinance::connector();
    let res = conn
        .get_quotes("NVDA", None, None, Some("5d"), None)
        .await
        .unwrap();

    let returns = compute_returns(res, ReturnType::Arithmetic).unwrap();
    assert!(returns.len() > 1usize);
}

#[tokio::test]
async fn test_cumulative_returns() {
    let returns = vec![
        Return {
            datetime: "2023-01-01".to_string(),
            asset_return: 0.01,
        },
        Return {
            datetime: "2023-01-02".to_string(),
            asset_return: 0.02,
        },
        Return {
            datetime: "2023-01-03".to_string(),
            asset_return: -0.01,
        },
    ];

    let expected = vec![
        CumulativeReturn {
            datetime: "2023-01-01".to_string(),
            asset_cumulative_returns: 0.01,
        },
        CumulativeReturn {
            datetime: "2023-01-02".to_string(),
            asset_cumulative_returns: 0.0302,
        },
        CumulativeReturn {
            datetime: "2023-01-03".to_string(),
            asset_cumulative_returns: 0.019898,
        },
    ];

    let result = compute_cumulative_returns(returns).unwrap();

    // Compare each result with the expected output
    for (res, exp) in result.iter().zip(expected.iter()) {
        assert_eq!(res.datetime, exp.datetime);
    }
}

#[tokio::test]
async fn test_expected_market_return() {
    let conn = YahooFinance::connector();
    let res = conn
        .get_quotes("^GSPC", None, None, Some("30y"), Some("1mo"))
        .await
        .unwrap();

    let returns = compute_returns(res, ReturnType::Arithmetic).unwrap();
    let expected_market_return = expected_market_return(&returns);
    assert!(expected_market_return.is_ok());
}
