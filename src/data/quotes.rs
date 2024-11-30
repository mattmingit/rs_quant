use crate::data::provider::YahooFinance;
use crate::utils::parsers::{parse_end_date, parse_start_date, timestamp_to_localdt};
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
                    .await?,
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
