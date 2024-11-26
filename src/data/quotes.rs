use crate::utils::parsers::{parse_end_date, parse_start_date, timestamp_to_localdt};
use std::collections::HashMap;
use yahoofinance::{Quote, YahooConnector};

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

/// This function retrieves quotes for a single asset query.
pub async fn get_quotes(
    ticker: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
    period: Option<&str>,
    interval: Option<&str>,
) -> Result<Vec<QuoteItem>, Box<dyn std::error::Error>> {
    // Establish yahoo! finance connection
    let provider = YahooConnector::new()?;

    // Set default interval if not passed as function argument
    let interval = interval.unwrap_or("1d");

    // Case 1: get data fro specific date range
    if let (Some(start), Some(end)) = (start_date, end_date) {
        // Convert str date into datetime
        let start_dt = parse_start_date(start)?;
        let end_dt = parse_end_date(end)?;

        let res = provider
            .get_quote_history_interval(ticker, start_dt, end_dt, interval)
            .await?;
        let q = convert_to_datetime_quotes(res.quotes()?)?;
        return Ok(q);
    }

    // Case 2: get data for defined period
    if let Some(period) = period {
        let res = provider.get_quote_range(ticker, interval, period).await?;
        let q = convert_to_datetime_quotes(res.quotes()?)?;
        return Ok(q);
    }

    // Default case:: return quotes for current date
    let res = provider.get_quote_range(ticker, "1m", "1d").await?;
    let q = convert_to_datetime_quotes(res.quotes()?)?;
    Ok(q)
}

#[derive(Debug)]
pub struct MultipleQuoteItems {
    pub data: HashMap<String, Vec<QuoteItem>>,
}

pub async fn get_multiple_quotes(
    tickers: Vec<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
    period: Option<&str>,
    interval: Option<&str>,
) -> Result<MultipleQuoteItems, Box<dyn std::error::Error>> {
    let mut results: HashMap<String, Vec<QuoteItem>> = HashMap::new();

    for ticker in tickers {
        let quotes = get_quotes(ticker, start_date, end_date, period, interval).await?;
        results.insert(ticker.to_string(), quotes);
    }

    Ok(MultipleQuoteItems { data: results })
}

/// This function parse the yahoo! finance response quote item into QuoteItem
pub fn convert_to_datetime_quotes(
    quotes: Vec<Quote>,
) -> Result<Vec<QuoteItem>, Box<dyn std::error::Error>> {
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
