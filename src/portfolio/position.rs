//! position

use crate::data::yahoo::{Yahoo, YahooErr};

#[derive(Debug)]
pub struct Position {
    pub symbol: String,
    pub quantity: i32,
    pub currency: String,
    pub buy_date: String,
    pub buy_price: f64,
    pub buy_value: f64,
    pub market_price: f64,
    pub equity: f64,
    pub pl: f64,
    pub pl_pct: f64,
}

impl Position {
    // create new positions instance
    pub fn new(
        symbol: &str,
        quantity: i32,
        currency: &str,
        buy_date: &str,
        buy_price: f64,
        buy_value: f64,
    ) -> Self {
        Position {
            symbol: symbol.to_string(),
            quantity,
            currency: currency.to_string(),
            buy_date: buy_date.to_string(),
            buy_price,
            buy_value,
            market_price: 0.0,
            equity: 0.0,
            pl: 0.0,
            pl_pct: 0.0,
        }
    }

    // update position market price
    pub async fn update_mkt_price(&mut self) -> Result<(), YahooErr> {
        let conn = Yahoo::provider()?;
        match conn.get_latest_quote(&self.symbol).await {
            Ok(p) => {
                self.market_price = p;
                println!("Seuccessfully updated market price for {}", self.symbol);
                Ok(())
            }
            Err(err) => {
                eprintln!("Error updating market price for {}: {}", self.symbol, err);
                Err(YahooErr::FetchFailed(err.to_string()))
            }
        }
    }

    // calculate position buy value (total invested in the position)
    pub fn buy_value(&mut self) {
        self.buy_value = self.buy_price * self.quantity as f64
    }

    // calculate position equity (current market value of the position)
    pub fn equity(&mut self) {
        self.equity = self.market_price * self.quantity as f64
    }

    // calculate profit and loss
    pub fn pl(&mut self) {
        self.pl = self.equity - self.buy_value
    }

    // calculate percentage profit and loss
    pub fn pl_pct(&mut self) {
        self.pl_pct = self.equity / self.buy_value - 1.
    }
}
