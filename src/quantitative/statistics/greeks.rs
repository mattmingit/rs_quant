use std::error::Error;

use crate::quantitative::returns::Return;
use crate::quantitative::statistics::descriptive::{compute_covariance, compute_variance};

// Calculate beta of an asset
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

// Calculate alpha of an asset
pub fn calculate_alpha(
    asset_return: f64,
    market_return: f64,
    risk_free_rate: f64,
    beta: f64,
) -> Result<f64, Box<dyn Error>> {
    if asset_return.is_nan() || market_return.is_nan() || risk_free_rate.is_nan() || beta.is_nan() {
        return Err("Input values must not be NaN".into());
    }

    if asset_return.is_infinite()
        || market_return.is_infinite()
        || risk_free_rate.is_infinite()
        || beta.is_infinite()
    {
        return Err("Input values must not be infinite".into());
    }

    Ok(asset_return - (risk_free_rate + (market_return - risk_free_rate) * beta))
}
