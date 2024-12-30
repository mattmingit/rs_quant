use rs_quant::data::provider::YahooFinance;
use rs_quant::quantitative::returns::{compute_returns, ReturnType};
use rs_quant::quantitative::statistics::greeks::{calculate_alpha, calculate_beta};

#[tokio::test]
async fn test_beta() {
    let conn = YahooFinance::connector();
    let asset = conn
        .get_quotes("NVDA", None, None, Some("5y"), Some("1mo"))
        .await
        .unwrap();
    let asset_returns = compute_returns(asset, ReturnType::Arithmetic).unwrap();

    let market = conn
        .get_quotes("^GSPC", None, None, Some("5y"), Some("1mo"))
        .await
        .unwrap();
    let market_returns = compute_returns(market, ReturnType::Arithmetic).unwrap();

    let beta = calculate_beta(&asset_returns, &market_returns);
    assert!(beta.is_ok());
    assert_ne!(beta.unwrap(), 0.0)
}

#[tokio::test]
async fn test_alpha_positive() {
    let asset_return = 0.08;
    let market_return = 0.06;
    let risk_free_rate = 0.02;
    let beta = 1.2;

    let r = calculate_alpha(asset_return, market_return, risk_free_rate, beta).unwrap();
    let exp_alpha = 0.08 - (0.02 + (0.06 - 0.02) * 1.2);
    assert!((r - exp_alpha).abs() < 1e-10);
}

#[tokio::test]
async fn test_alpha_negative() {
    let asset_return = 0.03;
    let market_return = 0.05;
    let risk_free_rate = 0.01;
    let beta = 1.5;

    let r = calculate_alpha(asset_return, market_return, risk_free_rate, beta).unwrap();
    let exp_alpha = 0.03 - (0.01 + (0.05 - 0.01) * 1.5);
    assert!((r - exp_alpha).abs() < 1e-10);
}

#[test]
fn test_alpha_zero_beta() {
    let asset_return = 0.04; // 4%
    let market_return = 0.07; // 7%
    let risk_free_rate = 0.03; // 3%
    let beta = 0.0; // No market sensitivity

    let result = calculate_alpha(asset_return, market_return, risk_free_rate, beta).unwrap();
    let expected_alpha = 0.04 - (0.03 + (0.07 - 0.03) * 0.0);
    assert!((result - expected_alpha).abs() < 1e-10);
}

#[test]
fn test_alpha_negative_beta() {
    let asset_return = 0.06; // 6%
    let market_return = 0.05; // 5%
    let risk_free_rate = 0.02; // 2%
    let beta = -0.5; // Negative market sensitivity

    let result = calculate_alpha(asset_return, market_return, risk_free_rate, beta).unwrap();
    let expected_alpha = 0.06 - (0.02 + (0.05 - 0.02) * -0.5);
    assert!((result - expected_alpha).abs() < 1e-10);
}

#[test]
fn test_alpha_error_handling() {
    // Simulate a case with NaN values, which may occur in certain edge scenarios
    let asset_return = f64::NAN;
    let market_return = 0.05;
    let risk_free_rate = 0.02;
    let beta = 1.0;

    let result = calculate_alpha(asset_return, market_return, risk_free_rate, beta);
    assert!(result.is_err());
}
