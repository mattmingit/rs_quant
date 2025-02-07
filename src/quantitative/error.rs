//! custom errors returned from methods and functions
use noisy_float::types::N64;
use std::error::Error;
use std::fmt;

// input arr empty error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyInput;

impl fmt::Display for EmptyInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty input.")
    }
}

impl Error for EmptyInput {}

// computing min/max value error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MinMaxErr {
    // empty input
    EmptyInput,
    // undefined order between tested pair of values
    UndefinedOrder,
}

impl fmt::Display for MinMaxErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MinMaxErr::EmptyInput => write!(f, "Empty input."),
            MinMaxErr::UndefinedOrder => {
                write!(f, "Undefined ordering between a tested pair of values.")
            }
        }
    }
}

impl Error for MinMaxErr {}

impl From<EmptyInput> for MinMaxErr {
    fn from(_: EmptyInput) -> Self {
        MinMaxErr::EmptyInput
    }
}

// error invoked by methods and functions that expect two arr with same shape (e.g shapemismatch error is raised when arr1.shape() == arr2.shape() is equal to false)
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeMismatch {
    pub f_shape: Vec<usize>,
    pub s_shape: Vec<usize>,
}

impl fmt::Display for ShapeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Array shapes do not match: {:?} and {:?}",
            self.f_shape, self.s_shape
        )
    }
}

impl Error for ShapeMismatch {}

// error invoked for methods that takes multiple non-empty arr
#[derive(Debug, Clone, PartialEq)]
pub enum MultiInputErr {
    // one or more arr are empty
    EmptyInput,
    // arr with different shapes
    ShapeMismatch(ShapeMismatch),
}

impl MultiInputErr {
    // returns whether `self` is the `EmptyInput` variant
    pub fn is_empty_input(&self) -> bool {
        match self {
            MultiInputErr::EmptyInput => true,
            _ => false,
        }
    }

    // returns whether `self` is the `ShapeMismatch` variant
    pub fn is_shapemismatch(&self) -> bool {
        match self {
            MultiInputErr::ShapeMismatch(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for MultiInputErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultiInputErr::EmptyInput => write!(f, "Empty input."),
            MultiInputErr::ShapeMismatch(e) => write!(f, "Shape Mismatch: {}", e),
        }
    }
}

impl Error for MultiInputErr {}

impl From<EmptyInput> for MultiInputErr {
    fn from(_: EmptyInput) -> Self {
        MultiInputErr::EmptyInput
    }
}

impl From<ShapeMismatch> for MultiInputErr {
    fn from(e: ShapeMismatch) -> Self {
        MultiInputErr::ShapeMismatch(e)
    }
}

// quantile computing error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantileErr {
    // empty input
    EmptyInput,
    // quantile `q` was not in 0. and 1. range
    InvalidQuantile(N64),
}

impl fmt::Display for QuantileErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantileErr::EmptyInput => write!(f, "Empty input."),
            QuantileErr::InvalidQuantile(q) => write!(
                f,
                "{:} is not a valid quantile. it must be between 0. and 1. range (inclusive)",
                q
            ),
        }
    }
}

impl Error for QuantileErr {}

impl From<EmptyInput> for QuantileErr {
    fn from(_: EmptyInput) -> Self {
        QuantileErr::EmptyInput
    }
}
