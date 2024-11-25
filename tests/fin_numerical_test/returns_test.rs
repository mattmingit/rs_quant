use rs_quant::fin_numerical::math::returns::{compute_returns, ReturnType};
use yahoo_finance_api::Quote;

#[test]
fn test_compute_returns_arithmetic() {
    let data = vec![
        Quote {
            timestamp: 1729839600,
            open: 10.0,
            high: 10.1,
            low: 10.0,
            close: 10.05,
            adjclose: 10.05,
            volume: 219,
        },
        Quote {
            timestamp: 1730102400,
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 10.1,
            adjclose: 10.1,
            volume: 759,
        },
        Quote {
            timestamp: 1730102400,
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 20.1,
            adjclose: 20.1,
            volume: 759,
        },
        Quote {
            timestamp: 1730102400,
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 10.1,
            adjclose: 100.1,
            volume: 759,
        },
    ];

    let result = compute_returns(data.clone(), ReturnType::Arithmetic).unwrap();
    assert!(result.len() > 1usize);
}

#[test]
fn test_compute_returns_logarithmic() {
    let data = vec![
        Quote {
            timestamp: 1729839600,
            open: 10.0,
            high: 10.1,
            low: 10.0,
            close: 10.05,
            adjclose: 10.05,
            volume: 219,
        },
        Quote {
            timestamp: 1730102400,
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 10.1,
            adjclose: 10.1,
            volume: 759,
        },
        Quote {
            timestamp: 1730102400,
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 20.1,
            adjclose: 20.1,
            volume: 759,
        },
        Quote {
            timestamp: 1730102400,
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 10.1,
            adjclose: 100.1,
            volume: 759,
        },
    ];

    let result = compute_returns(data.clone(), ReturnType::Logarithmic).unwrap();
    assert!(result.len() > 1usize);
}
