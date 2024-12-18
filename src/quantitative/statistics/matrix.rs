use crate::quantitative::returns::Return;
use crate::quantitative::statistics::descriptive::compute_covariance;
use ndarray::Array2;
use std::error::Error;

pub fn covariance_matrix(asset_returns: &[Vec<Return>]) -> Result<Array2<f64>, Box<dyn Error>> {
    let n_assets = asset_returns.len();
    let mut cov_matrix = Array2::<f64>::zeros((n_assets, n_assets));

    for i in 0..n_assets {
        for j in 0..n_assets {
            cov_matrix[(i, j)] = compute_covariance(&asset_returns[i], &asset_returns[j])?;
        }
    }

    Ok(cov_matrix)
}
