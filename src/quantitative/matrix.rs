use ndarray::Array2;
use ndarray_stats::CorrelationExt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MatrixError {
    #[error("Empty input")]
    EmptyInput,
    #[error("Failed to invert matrix.")]
    NonInvertible,
    #[error("Invalid dimensions, arraya must have same length.")]
    InvalidDimensions,
}

pub trait Matrix {
    fn covariance_matrix(&self) -> Result<Array2<f64>, MatrixError>;
}

impl Matrix for Array2<f64> {
    fn covariance_matrix(&self) -> Result<Array2<f64>, MatrixError> {
        self.cov(1.).map_err(|_| MatrixError::EmptyInput)
    }
}
