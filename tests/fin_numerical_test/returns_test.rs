use rs_quant::data::quotes;
use rs_quant::data::quotes::QuoteItem;
use rs_quant::fin_numerical::math::returns::{compute_returns, ReturnType};

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
    let res = quotes::get_quotes("NVDA", None, None, Some("5d"), None)
        .await
        .unwrap();

    let returns = compute_returns(res, ReturnType::Arithmetic).unwrap();
    assert!(returns.len() > 1usize);
}
