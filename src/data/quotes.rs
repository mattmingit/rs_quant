//! # Yahoo! Finance QuoteRetrieval and Parsing
//! This module provides functionality to retrieve and process financial quotes from Yahoo! Finance
//! using the `yahoofinance` crate (a fork of `yahoo_finance_api`, maintained by its original developers).
//! It defines structures for handling quote data and methods for querying single and multiple assets.
//! ## Structs
//!
//! ### QuoteItem
//! Represents a financial quote with a human-readable datetime string instead of a timestamp.
//! Fields included:
//! - `datetime`: A `String` representing the datetime of the quote.
//! - `open`: Opening price as `f64`.
//! -  `high`: Highest price during the interval as `f64`.
//! - `low`: Lowest price during the interval as `f64`.
//! - `close`: Closing price as `f64`.
//! - `adjclose`: Adjusted closing price as `f64`.
//! - `volume`: Trading volume as `f64`.
//!
//! ### `MultipleQuoteItems`
//! Represents quotes for multiple assets, where each entry includes:
//! - `data`: A `Vec<(String, Vec<QuoteItem>)>` where the `String` is the ticker, and
//!    `Vec<QuoteItem>` contains the quotes for that ticker.
//!
//! ## Implementation
//!
//! ### Methods for `YahooFinance`
//!
//! #### `get_quotes`
//! Retrieves historical or periodic quotes for a single asset.
//!
//! ##### Arguments
//! - `ticker`: A `&str` specifying the asset ticker symbol (e.g., "AAPL").
//! - `start_date`: An optional `&str` specifying the start date (e.g, "2023-01-01").
//! - `end_date`: An optional `&str` specifying the end date (e.g, "2023-12-31").
//! - `period`: An optional `&str` for predefined periods (e.g, "1d", "5d", "1mo").
//! - `interval`: An optional `&str` for data intervals (e.g, "1d", "1h"). Defaults to `"1d"`.
//!
//! ##### Returns
//! - `Ok(Vec<QuoteItem>)`: A vector of quotes for the given asset.
//! - `Err(Box<dyn Error>)`: An error if retrieval or parsing fails.
//!
//! ##### Behavior
//! - **Date Range**: Fetches quotes for the specified start and end dates.
//! - **Defined Period**: Fetches quotes for the specified period if no date range is provided.
//! - **Default**: Fetches quotes for the last 1 day at a 1-minute interval if neither a date range nor period is provided.
//!
//! ##### Example
//! ```rust
//! use rs_quant::data::provider::YahooFinance;
//!
//! #[tokio::main]
//! async fn main() {
//!     let connector = YahooFinance::connector(); // Assuming `connector` is synchronous
//!     match connector
//!         .get_quotes("AAPL", Some("2023-01-01"), Some("2023-01-11"), None, Some("1d"))
//!         .await
//!     {
//!         Ok(quotes) => {
//!             for quote in quotes {
//!                 println!("{:?}", quote);
//!             }
//!         }
//!         Err(err) => eprintln!("Error fetching quotes: {}", err),
//!     }
//! }
//! ```
//!
//! #### `get_multiple_quotes`
//! Retrieves historical or periodic quotes for multiple assets.
//!
//! ##### Arguments
//! - `tickers`: A `Vec<&str>` of asset ticker symbols (e.g., `vec!["AAPL", "GOOG"]`).
//! - `start_date`: An optional `&str` specifying the start date.
//! - `end_date`: An optional `&str` specifying the end date.
//! - `period`: An optional `&str` for predefined periods.
//! - `interval`: An optional `&str` for data intervals. Defaults to `"1d"`.
//!
//! ##### Returns
//! - `Ok(MultipleQuoteItems)`: A collection of quotes for all specified tickers.
//! - `Err(Box<dyn Error>)`: An error if retrieval or parsing fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::data::provider::YahooFinance;
//!
//! #[tokio::main]
//! async fn main() {
//!     let connector = YahooFinance::connector(); // Assuming `connector` is synchronous
//!     match connector
//!         .get_multiple_quotes(
//!             vec!["AAPL", "GOOG"],
//!             Some("2023-01-01"),
//!             Some("2023-01-11"),
//!             None,
//!             Some("1d"),
//!         )
//!         .await
//!     {
//!         Ok(multiple_quotes) => {
//!             for (ticker, quotes) in multiple_quotes.data {
//!                 println!("Quotes for {}: {:?}", ticker, quotes);
//!             }
//!         }
//!         Err(err) => eprintln!("Error fetching multiple quotes: {}", err),
//!     }
//! }
//! ```
//!
//! ## Helper Function
//!
//! ### `convert_to_datetime_quotes`
//! Converts a vector of `Quote` (from Yahoo Finance API) into a vector of `QuoteItem`.
//!
//! #### Arguments
//! - `quotes`: A `Vec<Quote>` returned by the Yahoo Finance API.
//!
//! #### Returns
//! - `Ok(Vec<QuoteItem>)`: A vector of `QuoteItem` with human-readable datetimes.
//! - `Err(Box<dyn Error>)`: An error if datetime conversion fails.
//!
//! ## Notes
//! - The `yahoofinance` crate must be included in your project as a dependency in `Cargo.toml`.
//! - The helper functions (`parse_start_date`, `parse_end_date`, `timestamp_to_localdt`, etc.) are
//!   assumed to be defined in the `utils::parsers` module.
//! - Datetime conversion ensures the timestamps from Yahoo Finance are user-friendly.

use crate::data::provider::YahooFinance;
use crate::utils::parsers::{
    datetime_to_date, parse_end_date, parse_start_date, timestamp_to_localdt,
};
use std::error::Error;
use yahoofinance::Quote;

/// This struct simply models yahoo! finance quotes with datetime string instead of timestamp
#[derive(Debug)]
pub struct QuoteItem {
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub adjclose: f64,
    pub volume: u64,
}

/// This struct simply models the result of multiple assets quotes querying from yahoo! finance
#[derive(Debug)]
pub struct MultipleQuoteItems {
    pub data: Vec<(String, Vec<QuoteItem>)>,
}

impl YahooFinance {
    /// Retrieve quotes for a single asset query
    pub async fn get_quotes(
        &self,
        ticker: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
        period: Option<&str>,
        interval: Option<&str>,
    ) -> Result<Vec<QuoteItem>, Box<dyn Error>> {
        // Set default interval if not passed as function argument
        let interval = interval.unwrap_or("1d");

        // Case 1: get data for specific date range
        if let (Some(start_date), Some(end_date)) = (start_date, end_date) {
            let start_date = parse_start_date(start_date)?;
            let end_date = parse_end_date(end_date)?;

            return Ok(convert_to_datetime_quotes(
                self.connector
                    .get_quote_history_interval(ticker, start_date, end_date, interval)
                    .await?
                    .quotes()?,
            )?);
        }

        // Case 2: get data for defined period
        if let Some(period) = period {
            return Ok(convert_to_datetime_quotes(
                self.connector
                    .get_quote_range(ticker, interval, period)
                    .await?
                    .quotes()?,
            )?);
        }

        // Default case: return quotes for current date
        Ok(convert_to_datetime_quotes(
            self.connector
                .get_quote_range(ticker, "1m", "1d")
                .await?
                .quotes()?,
        )?)
    }

    /// This function retrieves quote for multiple assets query
    pub async fn get_multiple_quotes(
        &self,
        tickers: Vec<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        period: Option<&str>,
        interval: Option<&str>,
    ) -> Result<MultipleQuoteItems, Box<dyn Error>> {
        // Initialize results hashmap container
        let mut r: Vec<(String, Vec<QuoteItem>)> = Vec::new();

        // Loop through tickers and fetch quotes data
        for t in tickers {
            r.push((
                t.to_string(),
                self.get_quotes(t, start_date, end_date, period, interval)
                    .await?
                    .into_iter()
                    .map(|mut q| {
                        if let Ok(date) = datetime_to_date(q.datetime.clone()) {
                            q.datetime = date;
                        }
                        q
                    })
                    .collect(),
            ));
        }
        Ok(MultipleQuoteItems { data: r })
    }
}

/// This function parse the yahoo! finance response quote item into QuoteItem
fn convert_to_datetime_quotes(quotes: Vec<Quote>) -> Result<Vec<QuoteItem>, Box<dyn Error>> {
    quotes
        .into_iter()
        .map(|q| {
            let datetime = timestamp_to_localdt(q.timestamp)?;
            Ok(QuoteItem {
                datetime,
                open: q.open,
                high: q.high,
                low: q.low,
                close: q.close,
                adjclose: q.adjclose,
                volume: q.volume,
            })
        })
        .collect()
}
