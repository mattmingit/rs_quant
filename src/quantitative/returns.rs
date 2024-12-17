//! # Asset Return Calculations
//! This module provides functionality to compute the arithmetic and logarithmic
//! returns for financial assets and their cumulative returns over time.
//!
//! ## Types and Enums
//!
//! ### `Return Type`
//! An enum representing the type of returns to compute:
//! - `Logarithmic`: Computes the natural logarithmic of the ratio of consecutive prices.
//! - `Arithmetic`: Computes the arithmetic percentage change between consecutive prices.
//!
//! ### Structs
//!
//! ### `Return`
//! Represents a single asset return with:
//! - `datetime`: The date and time of the return (as `String`).
//! - `asset_return`: The computed return value as a `f64`.
//!
//! ### `CumulativeReturn`
//! Represents a single cumulative return value with:
//! - `datetime`: The date and time (as `String`).
//! - `asset_cumulative_return`: The cumulative return value as a `f64`.
//! ## Functions
//! ### `compute_returns`
//! Computes the returns (arithmetic or logarithmic) for a given dataset of quotes.
//!
//! #### Arguments
//! - `data`: A `Vec<QuoteItem>` containing the data points (quote items) for calculation.
//! Each `QuoteItem` must have an `adjclose` (adjusted close price) and a `datetime`.
//! - `return_type`: A `ReturnType` enum indicating whether to compute arithmetic or logarithmic returns.
//!
//! #### Returns
//! - `Ok(Vec<Return>)`: A vector of computed `Return` structs if the computation is successful.
//! - `Err(Box<dyn Error>)`: An error if the data is insufficient or invalid.
//!
//! #### Errors
//! - Returns an error if the data has fewer than two items.
//! - Returns an error if any `adjclose` value is zero, which would cause a division error.
//!
//! #### Example
//! ```rust
//! use rs_quant::data::quotes::QuoteItem;
//! use rs_quant::quantitative::returns::{compute_returns, ReturnType};
//!
//! let quotes = vec![
//!     QuoteItem {
//!         datetime: "2023-12-01".to_string(),
//!         open: 90.0,
//!         high: 130.0,
//!         low: 90.0,
//!         close: 120.0,
//!         adjclose: 120.0,
//!         volume: 10000
//!     },
//!     QuoteItem{
//!         datetime: "2023-12-02".to_string(),
//!         open: 80.0,
//!         high: 150.0,
//!         low: 80.0,
//!         close: 140.0,
//!         adjclose: 140.0,
//!         volume: 15000
//!     }
//! ];
//!
//! let result = compute_returns(quotes, ReturnType::Arithmetic);
//! assert!(result.is_ok());
//! ```
//!
//! ### `compute_cumulative_returns`
//! Computes the cumulative returns from a given vector of returns.
//!
//! #### Arguments
//! - `returns`: A `Vec<Return>` obtained from the `compute_returns` function.
//!
//! #### Returns
//! - `Ok(Vec<CumulativeReturns>)`: A vector of cumulative returns.
//! - `Err(Box<dyn Error>)`: An error if computation fails.
//!
//! ### Example
//! ```rust
//! use rs_quant::quantitative::returns::{compute_cumulative_returns, Return};
//!
//! let returns = vec![
//!     Return {
//!         datetime: "2023-12-02".to_string(),
//!         asset_return: 0.1
//!     },
//!     Return {
//!         datetime: "2023-12-03".to_string(),
//!         asset_return: 0.05
//!     }
//! ];
//!
//! let result = compute_cumulative_returns(returns);
//! assert!(result.is_ok());
//! ```
//!
//! ## Notes
//! - Ensure `adjclose` values in `QuoteItem` are non-zero to avoid errors.
//! - This module assumes `QuoteItem` is defined elsewhere in your project, and includes at least `adjclose` and `datetime` fields.

use crate::data::quotes::QuoteItem;
use std::error::Error;

pub enum ReturnType {
    Logarithmic,
    Arithmetic,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Return {
    pub datetime: String,
    pub asset_return: f64,
}

impl Return {
    pub fn to_f64(&self) -> f64 {
        self.asset_return
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CumulativeReturn {
    pub datetime: String,
    pub asset_cumulative_returns: f64,
}

/// Compute returns function
pub fn compute_returns(
    data: Vec<QuoteItem>,
    return_type: ReturnType,
) -> Result<Vec<Return>, Box<dyn Error>> {
    if data.len() < 2 {
        return Err("Not enough data to calculate returns.".into());
    }

    let mut ret = Vec::new();
    for pair in data.windows(2) {
        let prev = &pair[0];
        let curr = &pair[1];

        if prev.adjclose.abs() < f64::EPSILON || curr.adjclose.abs() < f64::EPSILON {
            return Err("Error: adjclose value is zero.".into());
        }

        let ret_value = match return_type {
            ReturnType::Arithmetic => (curr.adjclose / prev.adjclose) - 1.0,
            ReturnType::Logarithmic => (curr.adjclose / prev.adjclose).ln(),
        };

        ret.push(Return {
            datetime: curr.datetime.clone(),
            asset_return: ret_value,
        })
    }

    Ok(ret)
}

/// Compute cumulative returns function
pub fn compute_cumulative_returns(
    returns: Vec<Return>,
) -> Result<Vec<CumulativeReturn>, Box<dyn Error>> {
    let mut cumulative = Vec::with_capacity(returns.len());
    let mut cum_ret = 1.0;

    for r in returns {
        cum_ret *= 1.0 + r.asset_return;
        cumulative.push(CumulativeReturn {
            datetime: r.datetime.clone(),
            asset_cumulative_returns: cum_ret - 1.0,
        });
    }
    Ok(cumulative)
}

/// Compute expected market return
pub fn expected_market_return(market_returns: &Vec<Return>) -> Result<f64, &'static str> {
    if market_returns.is_empty() {
        return Err("Market returns data is empty.");
    }

    let total: f64 = market_returns.iter().map(|r| r.asset_return).sum();
    let mean: f64 = total / market_returns.len() as f64;
    Ok(((1.0 + mean).powf(12f64)) - 1.0)
}
