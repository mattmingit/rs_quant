use ndarray::prelude::*;
use ndarray::stack;
use ndarray::Array1;
use ndarray_stats::errors::QuantileError;
use ndarray_stats::CorrelationExt;
use ndarray_stats::{interpolate::Linear, QuantileExt, SummaryStatisticsExt};
use noisy_float::prelude::*;

use super::error::{EmptyInput, MinMaxErr, MultiInputErr, QuantileErr, ShapeMismatch};

// enum to define mean types
pub enum MeanType {
    Arithmetic,
    Harmonic,
    Geometric,
    Weighted,
}

// enum to define variance types
pub enum VarType {
    Sample,
    Population,
}

// trait to define statistics methods
pub trait Statistic {
    fn min_val(&self) -> Result<f64, MinMaxErr>;
    fn max_val(&self) -> Result<f64, MinMaxErr>;
    fn median(&mut self) -> Result<f64, QuantileErr>;
    fn quantile(&mut self, q: f64) -> Result<f64, QuantileErr>;
    fn percentile(&mut self, p: f64) -> Result<f64, QuantileErr>;
    fn interquartile_range(&mut self) -> Result<f64, QuantileErr>;
    fn range(&self) -> Result<f64, MinMaxErr>;
    fn mean_val(&self, m_type: MeanType) -> Result<f64, EmptyInput>;
    fn variance(&self, v_type: VarType) -> Result<f64, EmptyInput>;
    fn std_dev(&self, v_type: VarType) -> Result<f64, EmptyInput>;
    fn covariance(&self, arr: &Self) -> Result<f64, MultiInputErr>;
    fn pearson_corr(&self, arr: &Self) -> Result<f64, MultiInputErr>;
    fn kurt(&self) -> Result<f64, EmptyInput>;
    fn skew(&self) -> Result<f64, EmptyInput>;
}

// statistics methods implementation
impl Statistic for Array1<f64> {
    // compute min value
    fn min_val(&self) -> Result<f64, MinMaxErr> {
        if self.is_empty() {
            return Err(MinMaxErr::EmptyInput);
        }

        Ok(*self.min_skipnan())
    }

    // compute max value
    fn max_val(&self) -> Result<f64, MinMaxErr> {
        if self.is_empty() {
            return Err(MinMaxErr::EmptyInput);
        }
        Ok(*self.max_skipnan())
    }

    // compute median value
    fn median(&mut self) -> Result<f64, QuantileErr> {
        match self.quantile_axis_skipnan_mut(Axis(0), n64(0.5), &Linear) {
            Ok(arr) => Ok(arr[()]),
            Err(QuantileError::EmptyInput) => Err(QuantileErr::EmptyInput),
            Err(QuantileError::InvalidQuantile(q)) => Err(QuantileErr::InvalidQuantile(q)),
        }
    }

    // compute quantile value
    fn quantile(&mut self, q: f64) -> Result<f64, QuantileErr> {
        match self.quantile_axis_skipnan_mut(Axis(0), n64(q), &Linear) {
            Ok(arr) => Ok(arr[()]),
            Err(QuantileError::EmptyInput) => Err(QuantileErr::EmptyInput),
            Err(QuantileError::InvalidQuantile(q)) => Err(QuantileErr::InvalidQuantile(q)),
        }
    }

    // compute percentile value
    fn percentile(&mut self, p: f64) -> Result<f64, QuantileErr> {
        let p = p / 100.0;
        self.quantile(p)
    }

    // compute interquartile range
    fn interquartile_range(&mut self) -> Result<f64, QuantileErr> {
        let q1 = self.quantile(0.25)?;
        let q3 = self.quantile(0.75)?;
        Ok(q3 - q1)
    }

    // compute range
    fn range(&self) -> Result<f64, MinMaxErr> {
        Ok(self.max_val()? - self.min_val()?)
    }

    // compute mean (pass mean type to change computation type)
    fn mean_val(&self, m_type: MeanType) -> Result<f64, EmptyInput> {
        match m_type {
            MeanType::Arithmetic => self.mean().ok_or(EmptyInput),
            MeanType::Geometric => self.geometric_mean().map_err(|_| EmptyInput),
            MeanType::Harmonic => self.harmonic_mean().map_err(|_| EmptyInput),
            MeanType::Weighted => todo!("Weighted mean not yet implemented"), // understand how to implement weights handling and passing as argument (maybe as Option<Vec<f64>> or Option<Array1<f64>>)
        }
    }

    // compute variance (pass var type to handle population bias)
    fn variance(&self, v_type: VarType) -> Result<f64, EmptyInput> {
        if self.is_empty() {
            return Err(EmptyInput);
        }
        match v_type {
            VarType::Sample => Ok(self.var(1.)),
            VarType::Population => Ok(self.var(0.)),
        }
    }

    // compute standard deviation (pass var type to handle population bias)
    fn std_dev(&self, v_type: VarType) -> Result<f64, EmptyInput> {
        if self.is_empty() {
            return Err(EmptyInput);
        }
        match v_type {
            VarType::Sample => Ok(self.std(1.)),
            VarType::Population => Ok(self.std(0.)),
        }
    }

    // compute covariance (convert array arguments into a single bidimensional array, and then convert the resulting covariance matrix into a f64)
    fn covariance(&self, arr: &Self) -> Result<f64, MultiInputErr> {
        if self.is_empty() || arr.is_empty() {
            return Err(MultiInputErr::EmptyInput);
        }
        if self.shape() != arr.shape() {
            return Err(MultiInputErr::ShapeMismatch(ShapeMismatch {
                f_shape: self.shape().to_vec(),
                s_shape: arr.shape().to_vec(),
            }));
        }

        let arr2 = stack![Axis(0), self.view(), arr.view()];
        let cov_m = arr2.cov(1.).map_err(|_| MultiInputErr::EmptyInput)?;
        Ok(cov_m[(0, 1)])
    }

    fn pearson_corr(&self, arr: &Self) -> Result<f64, MultiInputErr> {
        if self.is_empty() || arr.is_empty() {
            return Err(MultiInputErr::EmptyInput);
        }
        if self.shape() != arr.shape() {
            return Err(MultiInputErr::ShapeMismatch(ShapeMismatch {
                f_shape: self.shape().to_vec(),
                s_shape: arr.shape().to_vec(),
            }));
        }

        let arr2 = stack![Axis(0), self.view(), arr.view()];
        let pearson = arr2
            .pearson_correlation()
            .map_err(|_| MultiInputErr::EmptyInput)?;
        Ok(pearson[(0, 1)])
    }

    // compute kurtosis
    fn kurt(&self) -> Result<f64, EmptyInput> {
        Ok(self.kurtosis().map_err(|_| EmptyInput)? - 3.0)
    }

    // compute skewness
    fn skew(&self) -> Result<f64, EmptyInput> {
        self.skewness().map_err(|_| EmptyInput)
    }
}
