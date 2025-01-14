use super::statistics::Statistics;
use super::statistics::StatisticsError;

pub trait StatisticsMatrices {
    fn covariance_matrix(&self) -> Result<Vec<Vec<f64>>, StatisticsError>;

    fn correlation_matrix(&self) -> Result<Vec<Vec<f64>>, StatisticsError>;
}

impl StatisticsMatrices for Vec<Vec<f64>> {
    fn covariance_matrix(&self) -> Result<Vec<Vec<f64>>, StatisticsError> {
        // check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // check if all rows have the same length
        let r_len = self[0].len();
        if !self.iter().all(|r| r.len() == r_len) {
            return Err(StatisticsError::VectorLengthNotEqual);
        }

        // compute covariance matrix
        let mut m = Vec::new();
        for i in 0..self.len() {
            let mut r = Vec::new();
            for j in 0..self.len() {
                let c = self[i].covariance(&self[j])?;
                r.push(c);
            }
            m.push(r);
        }
        Ok(m)
    }

    fn correlation_matrix(&self) -> Result<Vec<Vec<f64>>, StatisticsError> {
        // check if vector is empty
        if self.is_empty() {
            return Err(StatisticsError::EmptyVector);
        }

        // check if all rows have the same length
        let r_len = self[0].len();
        if !self.iter().all(|r| r.len() == r_len) {
            return Err(StatisticsError::VectorLengthNotEqual);
        }

        // compute correlation matrix
        let mut m = Vec::new();
        for i in 0..self.len() {
            let mut r = Vec::new();
            for j in 0..self.len() {
                let c = self[i].correlation(&self[j])?;
                r.push(c);
            }
            m.push(r);
        }
        Ok(m)
    }
}
