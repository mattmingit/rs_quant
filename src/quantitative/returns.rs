use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReturnsError {
    #[error("Empty Input.")]
    EmptyInput,
    #[error("Inconsistent data: vector length must be more than one.")]
    LengthError,
}

// compute asset returns

// calculate expected returns based on historical data (which is wrong, implement better algorithm)
pub fn expected_returns(v: &[(String, f64)]) -> Result<f64, ReturnsError> {
    if v.is_empty() {
        return Err(ReturnsError::EmptyInput);
    }
    if v.len() < 2 {
        return Err(ReturnsError::LengthError);
    }

    Ok(v.iter().map(|(_, val)| val).sum::<f64>() / v.len() as f64)
}
