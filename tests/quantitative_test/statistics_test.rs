use rs_quant::quantitative::{returns::Return, statistics::greeks::calculate_alpha};

// #[test]
// fn test_alpha_positive() {
//     let asset_returns = vec![
//         Return {
//             datetime: "2024-01-01".to_string(),
//             asset_return: 0.08,
//         },
//         Return {
//             datetime: "2024-02-01".to_string(),
//             asset_return: 0.07,
//         },
//         Return {
//             datetime: "2024-03-01".to_string(),
//             asset_return: 0.09,
//         },
//     ];
//     let market_returns = vec![
//         Return {
//             datetime: "2024-01-01".to_string(),
//             asset_return: 0.06,
//         },
//         Return {
//             datetime: "2024-02-01".to_string(),
//             asset_return: 0.05,
//         },
//         Return {
//             datetime: "2024-03-01".to_string(),
//             asset_return: 0.07,
//         },
//     ];
//     let risk_free_rate = 0.02;

//     let alpha = calculate_alpha(&asset_returns, &market_returns, risk_free_rate).unwrap();
//     println!("{}", alpha);
//     assert!((alpha - 0.0375).abs() < 1e-10);
// }

#[test]
fn test_alpha_negative() {
    let asset_returns = vec![
        Return {
            datetime: "2024-01-01".to_string(),
            asset_return: 0.02,
        },
        Return {
            datetime: "2024-02-01".to_string(),
            asset_return: 0.01,
        },
        Return {
            datetime: "2024-03-01".to_string(),
            asset_return: 0.03,
        },
    ];
    let market_returns = vec![
        Return {
            datetime: "2024-01-01".to_string(),
            asset_return: 0.05,
        },
        Return {
            datetime: "2024-02-01".to_string(),
            asset_return: 0.06,
        },
        Return {
            datetime: "2024-03-01".to_string(),
            asset_return: 0.07,
        },
    ];
    let risk_free_rate = 0.02;

    let alpha = calculate_alpha(&asset_returns, &market_returns, risk_free_rate).unwrap();
    assert!((alpha + 0.02).abs() < 1e-10);
}

// #[test]
// fn test_alpha_zero_beta() {
//     let asset_returns = vec![
//         Return {
//             datetime: "2024-01-01".to_string(),
//             asset_return: 0.04,
//         },
//         Return {
//             datetime: "2024-02-01".to_string(),
//             asset_return: 0.04,
//         },
//         Return {
//             datetime: "2024-03-01".to_string(),
//             asset_return: 0.04,
//         },
//     ];
//     let market_returns = vec![
//         Return {
//             datetime: "2024-01-01".to_string(),
//             asset_return: 0.04,
//         },
//         Return {
//             datetime: "2024-02-01".to_string(),
//             asset_return: 0.04,
//         },
//         Return {
//             datetime: "2024-03-01".to_string(),
//             asset_return: 0.04,
//         },
//     ];
//     let risk_free_rate = 0.02;

//     let alpha = calculate_alpha(&asset_returns, &market_returns, risk_free_rate).unwrap();
//     assert!((alpha - 0.0).abs() < 1e-10);
// }

#[test]
fn test_alpha_identical_returns() {
    let asset_returns = vec![
        Return {
            datetime: "2024-01-01".to_string(),
            asset_return: 0.05,
        },
        Return {
            datetime: "2024-02-01".to_string(),
            asset_return: 0.05,
        },
        Return {
            datetime: "2024-03-01".to_string(),
            asset_return: 0.05,
        },
    ];
    let market_returns = vec![
        Return {
            datetime: "2024-01-01".to_string(),
            asset_return: 0.05,
        },
        Return {
            datetime: "2024-02-01".to_string(),
            asset_return: 0.05,
        },
        Return {
            datetime: "2024-03-01".to_string(),
            asset_return: 0.05,
        },
    ];
    let risk_free_rate = 0.02;

    let alpha = calculate_alpha(&asset_returns, &market_returns, risk_free_rate).unwrap();
    assert!((alpha - 0.0).abs() < 1e-10);
}

#[test]
fn test_alpha_error_on_empty_inputs() {
    let asset_returns: Vec<Return> = vec![];
    let market_returns: Vec<Return> = vec![];
    let risk_free_rate = 0.02;

    let result = calculate_alpha(&asset_returns, &market_returns, risk_free_rate);
    assert!(result.is_err());
}
