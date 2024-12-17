use rs_quant::data::provider::YahooFinance;
use rs_quant::quantitative::returns::{compute_returns, ReturnType};
use rs_quant::quantitative::statistics::greeks::calculate_beta;

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

    let beta = calculate_beta(asset_returns, market_returns);
    assert!(beta.is_ok());
}
