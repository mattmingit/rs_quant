use ndarray::Array1;
use thiserror::Error;

use super::position::Position;
use crate::{database::queries::VWPortfolio, quantitative::returns::ReturnsError};

#[derive(Debug, Error)]
pub enum PortfolioError {
    #[error("Failed to calculate returns: {0}")]
    ReturnsError(String),
}

#[derive(Debug)]
pub struct Portfolio {
    pub positions: Array1<Position>,
}

// default method implementation
impl Default for Portfolio {
    fn default() -> Self {
        Self::new()
    }
}

impl Portfolio {
    // create new portfolio instance
    pub fn new() -> Self {
        Self {
            positions: Array1::from_vec(vec![]),
        }
    }

    // create portfolio instance from database (currently personal MySql db)
    pub fn from_database(data: Vec<VWPortfolio>) -> Self {
        let p = data
            .into_iter()
            .map(|e| Position {
                symbol: e.ticker,
                quantity: i32::try_from(e.quantity).unwrap(),
                currency: e.currency,
                buy_date: e.buy_date.to_string(),
                buy_price: f64::try_from(e.buy_price).unwrap(),
                buy_value: f64::try_from(e.buy_value).unwrap(),
                market_value: f64::try_from(e.market_price).unwrap(),
                equity: f64::try_from(e.equity).unwrap(),
                pl: f64::try_from(e.pl).unwrap(),
                pl_pct: f64::try_from(e.pl_pct).unwrap(),
                weight: 0.0,
            })
            .collect::<Vec<Position>>();
        Portfolio {
            positions: p.into(),
        }
    }

    // calculate total investments across all positions
    pub fn total_investments(&self) -> f64 {
        self.positions.iter().map(|p| p.buy_value).sum::<f64>()
    }

    // calculate total market value
    pub fn total_mkt_val(&self) -> f64 {
        self.positions.iter().map(|p| p.equity).sum::<f64>()
    }

    // calculate overall profit and loss
    pub fn total_pl(&self) -> f64 {
        self.positions.iter().map(|p| p.pl).sum::<f64>()
    }

    // calculate positions weights
    pub fn weights(&mut self) {
        let total = self.total_mkt_val();

        // Avoid dividing by zero
        if total == 0.0 {
            for p in &mut self.positions {
                p.weight = 0.0;
            }
            return;
        }

        // calculate and set weights
        for p in &mut self.positions {
            p.weight = p.market_value / total;
        }
    }

    // calculate portfolio return
    pub fn portfolio_return(&self) -> f64 {
        self.positions
            .iter()
            .map(|p| p.pl_pct * p.weight)
            .sum::<f64>()
    }

    // calculate portfolio expected return using CAPM:w
    pub async fn portfolio_expected_return(&mut self) -> Result<f64, ReturnsError> {
        // step 1: ensure weights
        self.weights();

        // step 2: fetch risk-free avg rate and mkt returns
        todo!()
    }
}
