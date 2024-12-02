//! # Options Data Retrieval with Yahoo Finance
//! This module extends the functionality of the `YahooFinance` struct to support options data retrieval using the `yahoofinance` crate (a fork of `yahoo_finance_api` maintained by its original developers).
//!
//! ## Enums
//!
//! ### `ContractType`
//! Specifies the type of options contract to retrieve:
//! - `Call`: For call options.
//! - `Put`: For put options.
//!
//! ## Implementation
//!
//! ### Methods for `YahooFinance`
//!
//! #### `get_options_metadata`
//! Retrieves metadata for options associated with a given ticker.
//!
//! ##### Arguments
//! - `ticker`: A `&str` specifying the asset ticker symbol (e.g., "AAPL").
//!
//! ##### Returns
//! - `Ok(YQuote)`: The metadata as a `YQuote` object.
//! - `Err(Box<dyn Error>)`: An error if retrieval fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::data::provider::YahooFinance;
//!
//! #[tokio::main]
//! async fn main() {
//!     let connector = YahooFinance::connector();
//!     match connector.get_options_metadata("AAPL").await {
//!         Ok(metadata) => println!("{:?}", metadata),
//!         Err(err) => eprintln!("Error fetching options metadata: {}", err),
//!     }
//! }
//! ```
//!
//! #### `get_options_expiration_dates`
//! Retrieves a list of expiration dates for options associated with a given ticker.
//!
//! ##### Arguments
//! - `ticker`: A `&str` specifying the asset ticker symbol (e.g., "AAPL").
//!
//! ##### Returns
//! - `Ok(Vec<String>)`: A vector of expiration dates as human-readable strings.
//! - `Err(Box<dyn Error>)`: An error if retrieval or parsing fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::data::provider::YahooFinance;
//!
//! #[tokio::main]
//! async fn main() {
//!     let connector = YahooFinance::connector();
//!     match connector.get_options_expiration_dates("AAPL").await {
//!         Ok(expiration_dates) => {
//!             for date in expiration_dates {
//!                 println!("Expiration date: {}", date);
//!             }
//!         }
//!         Err(err) => eprintln!("Error fetching expiration dates: {}", err),
//!     }
//! }
//! ```
//!
//! #### `get_options_strike_prices`
//! Retrieves a list of strike prices for options associated with a given ticker.
//!
//! ##### Arguments
//! - `ticker`: A `&str` specifying the asset ticker symbol (e.g., "AAPL").
//!
//! ##### Returns
//! - `Ok(Vec<f64>)`: A vector of strike prices.
//! - `Err(Box<dyn Error>)`: An error if retrieval fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::data::provider::YahooFinance;
//!
//! #[tokio::main]
//! async fn main() {
//!     let connector = YahooFinance::connector();
//!     match connector.get_options_strike_prices("AAPL").await {
//!         Ok(strike_prices) => {
//!             for price in strike_prices {
//!                 println!("Strike price: {}", price);
//!             }
//!         }
//!         Err(err) => eprintln!("Error fetching strike prices: {}", err),
//!     }
//! }
//! ```
//!
//! #### `get_options_contracts`
//! Retrieves a list of options contracts for a given ticker and contract type (Call or Put).
//!
//! ##### Arguments
//! - `ticker`: A `&str` specifying the asset ticker symbol (e.g., "AAPL").
//! - `contract_type`: A `ContractType` enum value (`ContractType::Call` or `ContractType::Put`).
//!
//! ##### Returns
//! - `Ok(Vec<YOptionContract>)`: A vector of options contracts matching the specified type.
//! - `Err(Box<dyn Error>)`: An error if retrieval fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::data::provider::{YahooFinance};
//! use rs_quant::data::options::ContractType;
//!
//! #[tokio::main]
//! async fn main() {
//!     let connector = YahooFinance::connector();
//!     match connector.get_options_contracts("AAPL", ContractType::Call).await {
//!         Ok(contracts) => {
//!             for contract in contracts {
//!                 println!("{:?}", contract);
//!             }
//!         }
//!         Err(err) => eprintln!("Error fetching options contracts: {}", err),
//!     }
//! }
//! ```
//!
//! ## Notes
//! - The `yahoofinance` crate must be included in your project as a dependency in `Cargo.toml`.
//! - The helper function `timestamp_to_localdt` is assumed to convert timestamps to human-readable datetime strings.
//! - The `search_options` method from the `yahoofinance` crate fetches the underlying data required for all options-related methods.

use crate::data::provider::YahooFinance;
use crate::utils::parsers::timestamp_to_localdt;
use std::error::Error;
use yahoofinance::{YOptionContract, YQuote};

pub enum ContractType {
    Call,
    Put,
}

impl YahooFinance {
    /// Retrieve options metadata
    pub async fn get_options_metadata(&self, ticker: &str) -> Result<YQuote, Box<dyn Error>> {
        Ok(self
            .connector
            .search_options(ticker)
            .await?
            .option_chain
            .result[0]
            .quote
            .clone())
    }

    /// Retrieves options expiration dates
    pub async fn get_options_expiration_dates(
        &self,
        ticker: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self
            .connector
            .search_options(ticker)
            .await?
            .option_chain
            .result[0]
            .expiration_dates
            .iter()
            .map(|d| timestamp_to_localdt(*d).unwrap())
            .collect())
    }

    /// Retrieves options strike prices
    pub async fn get_options_strike_prices(
        &self,
        ticker: &str,
    ) -> Result<Vec<f64>, Box<dyn Error>> {
        Ok(self
            .connector
            .search_options(ticker)
            .await?
            .option_chain
            .result[0]
            .strikes
            .clone())
    }

    /// Retrieves options contracts
    pub async fn get_options_contracts(
        &self,
        ticker: &str,
        contract_type: ContractType,
    ) -> Result<Vec<YOptionContract>, Box<dyn Error>> {
        Ok(match contract_type {
            ContractType::Call => self
                .connector
                .search_options(ticker)
                .await?
                .option_chain
                .result[0]
                .options[0]
                .calls
                .clone(),
            ContractType::Put => self
                .connector
                .search_options(ticker)
                .await?
                .option_chain
                .result[0]
                .options[0]
                .puts
                .clone(),
        })
    }
}
