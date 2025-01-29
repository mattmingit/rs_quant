use ndarray::Array1;
use ndarray_stats::{interpolate::Linear, Quantile1dExt, QuantileExt, SummaryStatisticsExt};
use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum StatErr {
    #[error("Error: empty array, must contain at least one element.")]
    EmptyArr,
}

pub trait Stat {
    fn min(&self) -> Result<f64, StatErr>;
    fn max(&self) -> Result<f64, StatErr>;
    fn median(&self) -> Result<f64, StatErr>;
}

impl Stat for Array1<f64> {
    fn min(&self) -> Result<f64, StatErr> {
        if self.is_empty() {
            return Err(StatErr::EmptyArr);
        }

        Ok(*self.min_skipnan())
    }

    fn max(&self) -> Result<f64, StatErr> {
        if self.is_empty() {
            return Err(StatErr::EmptyArr);
        }
        Ok(*self.max_skipnan())
    }

    fn median(&self) -> Result<f64, StatErr> {
        if self.is_empty() {
            return Err(StatErr::EmptyArr);
        }

        // Ok(self.quantile_mut(0.5, &Linear)?)
    }
}
