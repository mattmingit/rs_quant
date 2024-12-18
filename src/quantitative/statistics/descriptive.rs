use crate::quantitative::returns::Return;
use crate::quantitative::traits::AssetReturn;
use crate::quantitative::utils::align_returns;
use ndarray::{Array1, ArrayBase, Ix1, OwnedRepr};
use std::error::Error;

/// Compute covariance
pub fn compute_covariance(
    returns_x: &[Return],
    returns_y: &[Return],
) -> Result<f64, Box<dyn Error>> {
    // Step 1: Align assets returns
    let (ret_x, ret_y) = align_returns(returns_x, returns_y)?;

    // Step 2: convert Returns to ndarray Array1<f64>
    let arr_x: ArrayBase<OwnedRepr<f64>, Ix1> = Array1::from_iter(ret_x.iter().cloned());
    let arr_y: ArrayBase<OwnedRepr<f64>, Ix1> = Array1::from_iter(ret_y.iter().cloned());

    // Step 3: Compute means
    let mean_x = arr_x.mean().unwrap_or(0.0);
    let mean_y = arr_y.mean().unwrap_or(0.0);

    // Step 4: Compute covariance
    Ok((&arr_x - mean_x).dot(&(arr_y - mean_y)) / arr_x.len() as f64)
}

/// Compute variance
pub fn compute_variance<T>(returns: &[T]) -> Result<f64, Box<dyn Error>>
where
    T: AssetReturn,
{
    // Step 1: Convert Returns to ndarray Array1<f64>
    let arr: ArrayBase<OwnedRepr<f64>, Ix1> =
        Array1::from_iter(returns.iter().map(|x| x.asset_return()));

    // Step 2: Compute means
    let mean = arr.mean().unwrap_or(0.0);

    // Step 3: Compute variance
    Ok((&arr - mean).mapv(|x| x * x).sum() / arr.len() as f64)
}
