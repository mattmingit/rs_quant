use crate::quantitative::returns::Return;
use crate::quantitative::statistics::descriptive::{compute_covariance, compute_variance};

pub fn calculate_beta(
    asset_returns: &[Return],
    market_returns: &[Return],
) -> Result<f64, Box<dyn std::error::Error>> {
    // Step 1: Compute asset-market covariance
    let covariance = compute_covariance(asset_returns, market_returns)?;

    // Step 2: Compute market variance
    let market_variance = compute_variance(market_returns)?;

    // Step 3: Check whether variance is equal to zero to avoid a division by zero
    if market_variance == 0.0 {
        return Err("Error: market variance is zero, can not divide by zero.".into());
    }

    // Step 5: Compute Beta: Beta is covariance divided by market variance
    Ok(covariance / market_variance)
}
