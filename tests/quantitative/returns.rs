use std::collections::HashMap;

use ndarray::array;
use rs_quant::data::yahoo::{MultiQuoteItem, QuoteItem, Yahoo};
use rs_quant::quantitative::returns::{ReturnType, Returns};

#[test]
fn returns() {
    let arr = array![
        QuoteItem {
            datetime: "2024-02-01".to_string(),
            adjclose: 100.0,
            open: 0.0,
            high: 0.0,
            close: 0.0,
            low: 0.0,
            volume: 0
        },
        QuoteItem {
            datetime: "2024-02-02".to_string(),
            adjclose: 105.0,
            open: 0.0,
            high: 0.0,
            close: 0.0,
            low: 0.0,
            volume: 0
        },
        QuoteItem {
            datetime: "2024-02-02".to_string(),
            adjclose: 120.0,
            open: 0.0,
            high: 0.0,
            close: 0.0,
            low: 0.0,
            volume: 0
        },
        QuoteItem {
            datetime: "2024-02-02".to_string(),
            adjclose: 98.0,
            open: 0.0,
            high: 0.0,
            close: 0.0,
            low: 0.0,
            volume: 0
        },
    ];
    assert!(arr.returns(ReturnType::Arithmetic).is_ok());
    assert_eq!(
        arr.returns(ReturnType::Arithmetic).unwrap(),
        array![
            ("2024-02-02".to_string(), 0.05,),
            ("2024-02-02".to_string(), 0.1429,),
            ("2024-02-02".to_string(), -0.1833,)
        ]
    )
}

#[tokio::test]
async fn returns_from_yahoo() {
    let conn = Yahoo::provider().unwrap();
    let data = conn
        .get_quotes("AAPL", None, None, Some("5d"), Some("1d"))
        .await
        .unwrap();
    println!("{:#?}", data.returns(ReturnType::Arithmetic).unwrap());
    assert!(data.returns(ReturnType::Arithmetic).is_ok());
}

#[test]
fn cumulative_returns() {
    let data = array![
        QuoteItem {
            datetime: "2024-02-01".to_string(),
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            adjclose: 102.0,
            volume: 1000,
        },
        QuoteItem {
            datetime: "2024-02-02".to_string(),
            open: 102.0,
            high: 106.0,
            low: 98.0,
            close: 104.0,
            adjclose: 104.0,
            volume: 1100,
        },
        QuoteItem {
            datetime: "2024-02-03".to_string(),
            open: 104.0,
            high: 108.0,
            low: 100.0,
            close: 107.0,
            adjclose: 107.0,
            volume: 1200,
        }
    ];

    let cumulative_returns = data.cumulative_returns(ReturnType::Arithmetic).unwrap();

    let expected_returns = array![
        ("2024-02-02".to_string(), 0.0196), // (104/102) - 1
        ("2024-02-03".to_string(), 0.049),  // cumulative sum: 0.0196 + (107/104 - 1)
    ];

    assert_eq!(cumulative_returns, expected_returns);
}

#[test]
fn test_cumulative_returns_multi_asset() {
    let prices = array![
        MultiQuoteItem {
            date: "2024-02-01".to_string(),
            prices: HashMap::from([("AAPL".to_string(), 150.0), ("GOOG".to_string(), 2800.0),]),
        },
        MultiQuoteItem {
            date: "2024-02-02".to_string(),
            prices: HashMap::from([("AAPL".to_string(), 155.0), ("GOOG".to_string(), 2856.0),]),
        },
        MultiQuoteItem {
            date: "2024-02-03".to_string(),
            prices: HashMap::from([("AAPL".to_string(), 158.0), ("GOOG".to_string(), 2900.0),]),
        }
    ];

    let result = prices
        .cumulative_returns_multiquote(ReturnType::Arithmetic)
        .unwrap();

    let expected = array![
        (
            "2024-02-02".to_string(),
            HashMap::from([
                ("AAPL".to_string(), 0.0333), // (1.0 + 0.0333) - 1.0 = 0.0333
                ("GOOG".to_string(), 0.02),   // (1.0 + 0.02) - 1.0 = 0.02
            ])
        ),
        (
            "2024-02-03".to_string(),
            HashMap::from([
                ("AAPL".to_string(), 0.0533), // (1.0 + 0.0333) * (1.0 + 0.0129) - 1.0 = 0.0533
                ("GOOG".to_string(), 0.0357), // (1.0 + 0.02) * (1.0 + 0.0154) - 1.0 = 0.0357
            ])
        )
    ];

    assert_eq!(result, expected);
}
