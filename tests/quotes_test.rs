use rs_quant::data::quotes::*;
#[test]
fn test_deserialize_period_info() {
    let period_info_json = r#"
        {
            "timezone": "EST",
            "start": 1705501800,
            "end": 1705525200,
            "gmtoffset": -18000
        }
        "#;
    let period_info_expected = PeriodInfo {
        timezone: "EST".to_string(),
        start: 1705501800,
        end: 1705525200,
        gmtoffset: -18000,
    };
    let period_info_deserialized: PeriodInfo = serde_json::from_str(period_info_json).unwrap();
    assert_eq!(&period_info_deserialized, &period_info_expected);
}

#[test]
fn test_deserialize_trading_periods_simple() {
    let trading_periods_json = r#"
        [
            [
                {
                    "timezone": "EST",
                    "start": 1705501800,
                    "end": 1705525200,
                    "gmtoffset": -18000
                }

            ]
        ]
        "#;
    let trading_periods_expected = TradingPeriods {
        pre: None,
        regular: Some(vec![vec![PeriodInfo {
            timezone: "EST".to_string(),
            start: 1705501800,
            end: 1705525200,
            gmtoffset: -18000,
        }]]),
        post: None,
    };
    let trading_periods_deserialized: TradingPeriods =
        serde_json::from_str(trading_periods_json).unwrap();
    assert_eq!(&trading_periods_expected, &trading_periods_deserialized);
}

#[test]
fn test_deserialize_trading_periods_complex_regular_only() {
    let trading_periods_json = r#"
        {
            "regular": [
              [
                {
                  "timezone": "EST",
                  "start": 1705501800,
                  "end": 1705525200,
                  "gmtoffset": -18000
                }
              ]
            ]
        }
       "#;
    let trading_periods_expected = TradingPeriods {
        pre: None,
        regular: Some(vec![vec![PeriodInfo {
            timezone: "EST".to_string(),
            start: 1705501800,
            end: 1705525200,
            gmtoffset: -18000,
        }]]),
        post: None,
    };
    let trading_periods_deserialized: TradingPeriods =
        serde_json::from_str(trading_periods_json).unwrap();
    assert_eq!(&trading_periods_expected, &trading_periods_deserialized);
}

#[test]
fn test_deserialize_trading_periods_complex() {
    let trading_periods_json = r#"
        {
            "pre": [
              [
                {
                  "timezone": "EST",
                  "start": 1705482000,
                  "end": 1705501800,
                  "gmtoffset": -18000
                }
              ]
            ],
            "post": [
              [
                {
                  "timezone": "EST",
                  "start": 1705525200,
                  "end": 1705539600,
                  "gmtoffset": -18000
                }
              ]
            ],
            "regular": [
              [
                {
                  "timezone": "EST",
                  "start": 1705501800,
                  "end": 1705525200,
                  "gmtoffset": -18000
                }
              ]
            ]
        }
       "#;
    let trading_periods_expected = TradingPeriods {
        pre: Some(vec![vec![PeriodInfo {
            timezone: "EST".to_string(),
            start: 1705482000,
            end: 1705501800,
            gmtoffset: -18000,
        }]]),
        regular: Some(vec![vec![PeriodInfo {
            timezone: "EST".to_string(),
            start: 1705501800,
            end: 1705525200,
            gmtoffset: -18000,
        }]]),
        post: Some(vec![vec![PeriodInfo {
            timezone: "EST".to_string(),
            start: 1705525200,
            end: 1705539600,
            gmtoffset: -18000,
        }]]),
    };
    let trading_periods_deserialized: TradingPeriods =
        serde_json::from_str(trading_periods_json).unwrap();
    assert_eq!(&trading_periods_expected, &trading_periods_deserialized);
}
