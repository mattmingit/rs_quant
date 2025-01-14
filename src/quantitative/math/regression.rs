use super::statistics::{Statistics, StatisticsError, VarianceType};

pub trait Regression {
    fn beta(&self, v: &Self) -> Result<f64, StatisticsError>;
}

impl Regression for Vec<f64> {
    fn beta(&self, v: &Self) -> Result<f64, StatisticsError> {
        Ok(self.covariance(v)? / self.variance(VarianceType::CorrectedSample)?)
    }
}
