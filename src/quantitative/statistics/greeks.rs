use crate::quantitative::returns::Return;
use ndarray::Array1;

pub fn calculate_beta(
    asset_returns: Vec<Return>,
    market_returns: Vec<Return>,
) -> Result<f64, &'static str> {
    if asset_returns.len() != market_returns.len() {
        return Err("Error: asset_returns and market_returns must have same non-zero length.");
    }

    // Convert Returns to ndarray Array1<f64>
    let asset_array: Array1<f64> = Array1::from(
        asset_returns
            .iter()
            .map(|r| r.to_f64())
            .collect::<Vec<f64>>(),
    );
    let market_array: Array1<f64> = Array1::from(
        market_returns
            .iter()
            .map(|r| r.to_f64())
            .collect::<Vec<f64>>(),
    );

    // Compute means
    let asset_mean = asset_array.mean().unwrap_or(0.0);
    let market_mean = market_array.mean().unwrap_or(0.0);

    // Compute covariance
    let covariance =
        (&asset_array - asset_mean).dot(&(&market_array - market_mean)) / asset_array.len() as f64;

    // Compute variance
    let market_variance =
        (&market_array - market_mean).mapv(|x| x * x).sum() / market_array.len() as f64;

    if market_variance == 0.0 {
        return Err("Error: market variance is zero, can not divide by zero.");
    }

    // Beta is covariance divided by market variance
    Ok(covariance / market_variance)
}
