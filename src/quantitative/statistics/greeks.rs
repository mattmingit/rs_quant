use crate::quantitative::returns::Return;
use chrono::NaiveDate;
use ndarray::Array1;
use std::collections::HashMap;

pub fn calculate_beta(
    asset_returns: &[Return],
    market_returns: &[Return],
) -> Result<f64, &'static str> {
    // Step 1: Align asset and market returns based on date
    let mut market_map: HashMap<String, f64> = HashMap::new();
    // Parse the market dates and return values
    for ret in market_returns {
        // Strip the time portion and parse only the date
        let date_str = ret.datetime.split_whitespace().next().unwrap_or("");
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => {
                market_map.insert(date.to_string(), ret.to_f64());
            }
            Err(e) => {
                // Print the error to understand where the issue occurs
                eprintln!("Error parsing market date '{}': {:?}", date_str, e);
                return Err("Error: Invalid market date format.");
            }
        }
    }

    let mut aligned_asset_returns = Vec::new();
    let mut aligned_market_returns = Vec::new();

    // Parse the asset dates and check for alignment with market dates
    for asset in asset_returns {
        // Strip the time portion and parse only the date
        let date_str = asset.datetime.split_whitespace().next().unwrap_or(""); // Remove any extra spaces or invisible characters
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => {
                // Check if the same date exists in the market map
                if let Some(&market_return) = market_map.get(&date.to_string()) {
                    aligned_asset_returns.push(asset.to_f64());
                    aligned_market_returns.push(market_return);
                }
            }
            Err(e) => {
                // Print the error to understand where the issue occurs
                eprintln!("Error parsing asset date '{}': {:?}", date_str, e);
                return Err("Error: Invalid asset date format.");
            }
        }
    }

    if aligned_asset_returns.is_empty() || aligned_market_returns.is_empty() {
        return Err("Error: No overlapping dates between asset and market returns.");
    }

    // Step 2: Convert Returns to ndarray Array1<f64>
    let asset_array: Array1<f64> = Array1::from_iter(aligned_asset_returns);
    let market_array: Array1<f64> = Array1::from_iter(aligned_market_returns);

    // Step 3: Compute means
    let asset_mean = asset_array.mean().unwrap_or(0.0);
    let market_mean = market_array.mean().unwrap_or(0.0);

    // Step 4: Compute covariance
    let covariance =
        (&asset_array - asset_mean).dot(&(&market_array - market_mean)) / asset_array.len() as f64;

    // Step 5: Compute variance
    let market_variance =
        (&market_array - market_mean).mapv(|x| x * x).sum() / market_array.len() as f64;

    if market_variance == 0.0 {
        return Err("Error: market variance is zero, can not divide by zero.");
    }

    // Beta is covariance divided by market variance
    Ok(covariance / market_variance)
}
