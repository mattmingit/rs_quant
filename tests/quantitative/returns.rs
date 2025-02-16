use ndarray::array;
use rs_quant::data::yahoo::{QuoteItem, Yahoo};
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
        ("2024-02-03".to_string(), 0.0484), // cumulative sum: 0.0196 + (107/104 - 1)
    ];

    assert_eq!(cumulative_returns, expected_returns);
}

#[tokio::test]
async fn returns_multiquote_from_yahoo() {
    let conn = Yahoo::provider().unwrap();
    let data = conn
        .get_multiple_quotes(
            vec!["AAPL", "NVDA", "GOOG"],
            None,
            None,
            Some("5d"),
            Some("1d"),
        )
        .await
        .unwrap();
    println!(
        "{:#?}",
        data.returns_multiquote(ReturnType::Arithmetic).unwrap()
    )
}
