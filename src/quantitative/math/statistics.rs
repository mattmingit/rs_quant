use core::f64;

#[derive(Debug, thiserror::Error)]
pub enum StatisticsError {
    #[error("Error: empty vector, it must contain at least one element.")]
    EmptyVector,
    #[error("Error: MeanType not implemented.")]
    MeanTypeNotImplemented,
    #[error("Error: VarianceType not implemented.")]
    VarianceTypenotImplented,
    #[error("Error: StandardDeviationType not implemented.")]
    StandardDeviationTypeNotImplemented,
    #[error("Error: vectors length is not equal, variable vectors must have equal length.")]
    VectorLengthNotEqual,
    #[error("Error: vector must have at least two elements.")]
    VectorLengthLessThanTwo,
    #[error("Error: vector must have at least three elements.")]
    VectorLengthLessThanThree,
    #[error("Error: vector must have at least four elements.")]
    VectorLengthLessThanFour,
    #[error("Error: percentile must be between 0 and 1.")]
    PercentileError,
    #[error("Error: quantile must be between 0 and 1.")]
    QuantileError,
    #[error("Error: standard deviation should not be zero.")]
    StandardDeviationZero,
}

// Enum mean type variation to compute different types of mean
pub enum MeanType {
    Arithmetic,
    Harmonic,
    Geometric,
    Quadratic,
    Cubic,
    //Weighted,
}

// Enum variance type variation to compute different types of variance
pub enum VarianceType {
    CorrectedSample,   // uses N-1 in the denominator
    UncorrectedSample, // uses N in the denominator
}

// Enum standard deviation type variation to compute different types of standard deviation
pub enum StandardDeviationType {
    CorrectedSample,   // uses corrected sample variance
    UncorrectedSample, // uses uncorrected sample variance
}

/// StatisticDescriptive trait for computing descriptive statistics of a vector of f64

pub trait Statistics {
    /// Calculate the minimum value of a vector
    fn min(&self) -> Result<f64, StatisticsError>;

    /// Calculate the maximum value of a vector
    fn max(&self) -> Result<f64, StatisticsError>;

    /// Calculate the median of a vector
    fn median(&self) -> Result<f64, StatisticsError>;

    /// Calculate the percentile of a vector
    fn percentile(&self, percentile: f64) -> Result<f64, StatisticsError>;

    /// Calculate the quantile of a vector
    fn quantile(&self, quantile: f64) -> Result<f64, StatisticsError>;

    /// Calculate the interquantile range of a vector
    fn interquantile_range(&self) -> Result<f64, StatisticsError>;

    /// Calculate the range of a vector
    fn range(&self) -> Result<f64, StatisticsError>;

    // MEAN FUNCTION
    /// Compute the mean of a vector of f64
    fn mean(&self, mean_type: MeanType) -> Result<f64, StatisticsError>;

    // VARIANCE FUNCTION
    /// Compute the variance of a vector of f64
    fn variance(&self, variance_type: VarianceType) -> Result<f64, StatisticsError>;

    // STANDARD DEVIATION FUNCTION
    /// Compute the standard deviation of a vector of f64
    fn standard_deviation(
        &self,
        std_deviation_type: StandardDeviationType,
    ) -> Result<f64, StatisticsError>;

    /// Calculate the covariance between two vectors
    fn covariance(&self, v: &Self) -> Result<f64, StatisticsError>;

    /// Calculate the correlation between two vectors
    fn correlation(&self, v: &Self) -> Result<f64, StatisticsError>;

    /// Calculate the skewness of a vector
    fn skewness(&self) -> Result<f64, StatisticsError>;

    /// Calculate the kurtosis of a vector
    fn kurtosis(&self) -> Result<f64, StatisticsError>;
}

impl Statistics for Vec<f64> {
    fn min(&self) -> Result<f64, StatisticsError> {
        // Check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // Find minimum value
        self.iter().try_fold(f64::INFINITY, |a, &b| Ok(a.min(b)))
    }

    fn max(&self) -> Result<f64, StatisticsError> {
        // Check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // Find maxium value
        self.iter()
            .try_fold(f64::NEG_INFINITY, |a, &b| Ok(a.max(b)))
    }

    fn median(&self) -> Result<f64, StatisticsError> {
        // Check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // Sort vector
        let mut sorted = self.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Find median
        let mid = sorted.len() / 2;
        match sorted.len() % 2 == 0 {
            true => Ok((sorted[mid - 1] + sorted[mid]) / 2.0),
            false => Ok(sorted[mid]),
        }
    }

    fn percentile(&self, percentile: f64) -> Result<f64, StatisticsError> {
        // Check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // Check if percentile is in bounds
        if !(0.0..=1.0).contains(&percentile) {
            return Err(StatisticsError::PercentileError);
        }

        // Sort vector
        let mut sorted = self.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Find percentile
        let index = percentile * (sorted.len() - 1) as f64;
        let lower = sorted[index.floor() as usize];
        let upper = sorted[index.ceil() as usize];

        Ok(lower + (upper - lower) * (index - index.floor()))
    }

    fn quantile(&self, quantile: f64) -> Result<f64, StatisticsError> {
        // Check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // Check if quantile is in bounds
        if !(0.0..=1.0).contains(&quantile) {
            return Err(StatisticsError::QuantileError);
        }

        // Sort vector
        let mut sorted = self.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Find quantile
        let index = quantile * (sorted.len() - 1) as f64;
        let lower = sorted[index.floor() as usize];
        let upper = sorted[index.ceil() as usize];

        Ok(lower + (upper - lower) * (index - index.floor()))
    }

    fn interquantile_range(&self) -> Result<f64, StatisticsError> {
        // Check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // Sort vector
        let mut sorted = self.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Find interquantile range
        let q1 = sorted[sorted.len() / 4];
        let q3 = sorted[3 * sorted.len() / 4];

        Ok(q3 - q1)
    }

    fn range(&self) -> Result<f64, StatisticsError> {
        // Check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // Find range
        Ok(self.max()? - self.min()?)
    }

    fn mean(&self, mean_type: MeanType) -> Result<f64, StatisticsError> {
        // Check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // Match mean type and compute mean
        match mean_type {
            MeanType::Arithmetic => Ok(self.iter().sum::<f64>() / self.len() as f64),
            MeanType::Harmonic => {
                Ok(self.len() as f64 / self.iter().map(|&x| 1.0 / x).sum::<f64>())
            }
            MeanType::Geometric => Ok(self.iter().product::<f64>().powf(1.0 / self.len() as f64)),
            MeanType::Quadratic => {
                Ok((self.iter().map(|&x| x.powi(2)).sum::<f64>() / self.len() as f64).sqrt())
            }
            MeanType::Cubic => {
                Ok((self.iter().map(|&x| x.powi(3)).sum::<f64>() / self.len() as f64).cbrt())
            }
        }
    }

    fn variance(&self, variance_type: VarianceType) -> Result<f64, StatisticsError> {
        // Check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // Match variance type and compute variance
        match variance_type {
            VarianceType::CorrectedSample => {
                let m = self.mean(MeanType::Arithmetic)?;
                Ok(self.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (self.len() - 1) as f64)
            }
            VarianceType::UncorrectedSample => {
                let m = self.mean(MeanType::Arithmetic)?;
                Ok(self.iter().map(|x| (x - m).powi(2)).sum::<f64>() / self.len() as f64)
            }
        }
    }

    fn standard_deviation(
        &self,
        std_deviation_type: StandardDeviationType,
    ) -> Result<f64, StatisticsError> {
        // Check if vector has consistent number of elements
        if self.len() < 2 {
            return Err(StatisticsError::VectorLengthLessThanTwo);
        }

        // Match standard deviation type and compute standard deviation
        match std_deviation_type {
            StandardDeviationType::CorrectedSample => {
                Ok(self.variance(VarianceType::CorrectedSample)?.sqrt())
            }
            StandardDeviationType::UncorrectedSample => {
                Ok(self.variance(VarianceType::UncorrectedSample)?.sqrt())
            }
        }
    }

    fn covariance(&self, v: &Self) -> Result<f64, StatisticsError> {
        // Check if both variable vectors have equal length
        if self.len() != v.len() {
            return Err(StatisticsError::VectorLengthNotEqual);
        }

        // Check if vector has consistent number of elements
        if self.len() < 2 {
            return Err(StatisticsError::VectorLengthLessThanTwo);
        }

        // Compute means
        let n = self.len() as f64;
        let mean_x = self.iter().sum::<f64>() / n;
        let mean_y = v.iter().sum::<f64>() / n;

        // Compute covariance
        let cov = self
            .iter()
            .zip(v.iter())
            .map(|(x, y)| (x - mean_x) * (y - mean_y))
            .sum::<f64>();

        Ok(cov / (n - 1.0))
    }

    fn correlation(&self, v: &Self) -> Result<f64, StatisticsError> {
        // Compute covariance and standard deviations
        let cov = self.covariance(v)?;
        let std_x = self.standard_deviation(StandardDeviationType::CorrectedSample)?;
        let std_y = v.standard_deviation(StandardDeviationType::CorrectedSample)?;

        // Check if standard deviations are equal to zero
        if std_x == 0.0 || std_y == 0.0 {
            return Err(StatisticsError::StandardDeviationZero);
        }

        Ok(cov / (std_x * std_y))
    }

    fn skewness(&self) -> Result<f64, StatisticsError> {
        // Check if vector has less than three elements
        if self.len() < 3 {
            return Err(StatisticsError::VectorLengthLessThanThree);
        }

        // Compute mean and standard deviation
        let n = self.len() as f64;
        let mean = self.mean(MeanType::Arithmetic)?;
        let std = self.standard_deviation(StandardDeviationType::CorrectedSample)?;

        // Compute skewness
        let skew = self.iter().map(|x| ((x - mean) / std).powi(3)).sum::<f64>();

        Ok(skew * n / ((n - 1.0) * (n - 2.0)))
    }

    fn kurtosis(&self) -> Result<f64, StatisticsError> {
        // Check if vector has less than four elements
        if self.len() < 4 {
            return Err(StatisticsError::VectorLengthLessThanFour);
        }

        // Compute mean and standard deviation
        let n = self.len() as f64;
        let mean = self.mean(MeanType::Arithmetic)?;
        let std = self.standard_deviation(StandardDeviationType::CorrectedSample)?;

        // Check for zero standard deviation
        if std == 0.0 {
            return Err(StatisticsError::StandardDeviationZero);
        }

        // Compute kurtosis
        let kurt = self.iter().map(|x| ((x - mean) / std).powi(4)).sum::<f64>();

        Ok(kurt * n * (n + 1.0) / ((n - 1.0) * (n - 2.0) * (n - 3.0))
            - 3.0 * (n - 1.0).powi(2) / ((n - 2.0) * (n - 3.0)))
    }
}
