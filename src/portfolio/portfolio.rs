use ndarray::Array1;

use super::position::Position;
use crate::database::queries::VWPortfolio;

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
                market_price: f64::try_from(e.market_price).unwrap(),
                equity: f64::try_from(e.equity).unwrap(),
                pl: f64::try_from(e.pl).unwrap(),
                pl_pct: f64::try_from(e.pl_pct).unwrap(),
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
}
