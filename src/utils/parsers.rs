//! # Utility Functions for Parsing and Conversions
//! This module provides various helper functions to handle date/time parsing, timestamp conversion,
//! and data serialization, particularly for use with the `yahoofinance` crate.
//!
//! ## Functions
//!
//! ### `parse_start_date`
//! Converts a string representing a start date into an `OffsetDateTime` object.
//!
//! ###### Arguments
//! - `d`: A `&str` in the format `YYYY-MM-DD`.
//!
//! ##### Returns
//! - `Ok(OffssetDateTime)`: The parsed datetime at midnight UTC.
//! - `Err(Box<dyn Error>)`: Ane error if parsing fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::utils::parsers::parse_start_date;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>>{
//! let start_date = parse_start_date("2023-01-01")?;
//!     println!("Parsed start date: {}", start_date);
//!     Ok(())
//! }
//! ```
//!
//! ### `parse_end_date`
//! Converts a string representing an end date into an `OffsetDateTime` object.
//!
//! ##### Arguments
//! - `d`: A `&str` in the format `YYYY-MM-DD`.
//!
//! ##### Returns
//! - `Ok(OffsetDateTime)`: The parsed datetime at 23:59:59 UTC.
//! - `Err(Box<dyn Error>)`: An error if parsing fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::utils::parsers::parse_end_date;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>>{
//!     let end_date = parse_end_date("2023-01-01")?;
//!     println!("Parsed end date: {}", end_date);
//!     Ok(())
//! }
//! ```
//! ### `timestamp_to_datetime`
//! Converts a UNIX timestamp into an `OffsetDateTime` object.
//!
//! ##### Arguments
//! - `timestamp`: An `i64` representing the UNIX timestamp.
//!
//! ##### Returns
//! - `Ok(OffsetDateTime)`: The corresponding datetime in UTC.
//! - `Err(Box<dyn Error>)`: An error if conversion fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::utils::parsers::timestamp_to_datetime;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let datetime = timestamp_to_datetime(1672531199)?;
//!     println!("Parsed datetime: {}", datetime);
//!     Ok(())
//! }
//! ```
//!
//! ### `timestamp_to_localdt`
//! Converts a UNIX timestamp into a human-readable local datetime string in the format `YYYY-MM-DD HH:MM:SS`.
//!
//! ##### Arguments
//! - `timestamp`: A `u64` representing the UNIX timestamp.
//!
//! ##### Returns
//! - `Ok(String)`: The corresponding local datetime as a string.
//! - `Err(Box<dyn Error>)`: An error if conversion fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::utils::parsers::timestamp_to_localdt;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let local_datetime = timestamp_to_localdt(1672531199)?;
//!     println!("Local datetime: {}", local_datetime);
//!     Ok(())
//! }
//! ```
//!
//! ### `datetime_to_date`
//! Extracts the date part (`YYYY-MM-DD`) from a datetime string in the format `YYYY-MM-DD HH:MM:SS`.
//!
//! ##### Arguments
//! - `d`: A `String` in the format `YYYY-MM-DD HH:MM:SS`.
//!
//! ##### Returns
//! - `Ok(String)`: The extracted date as a string.
//! - `Err(Box<dyn Error>)`: An error if parsing fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::utils::parsers::datetime_to_date;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let date = datetime_to_date("2023-01-01 12:00:00".to_string())?;
//!     println!("Extracted date: {}", date);
//!     Ok(())
//! }
//! ```
//!
//! ### `quotes_to_json`
//! Converts a vector of `Quote` objects (from the `yahoofinance` crate) into a JSON array.
//!
//! ##### Arguments
//! - `data`: A `Vec<Quote>` containing quote data.
//!
//! ##### Returns
//! - `Ok(Value)`: A JSON array representing the quotes.
//! - `Err(Box<dyn Error>)`: An error if conversion fails.
//!
//! ##### Example
//! ```rust
//! use rs_quant::utils::parsers::quotes_to_json;
//! use yahoofinance::Quote;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let quotes = vec![
//!         Quote {
//!             timestamp: 1672531199,
//!             open: 100.0,
//!             high: 110.0,
//!             low: 95.0,
//!             close: 105.0,
//!             adjclose: 104.0,
//!             volume: 100000,
//!         },
//!     ];
//!     let json = quotes_to_json(quotes)?;
//!     println!("JSON data: {}", json);
//!     Ok(())
//! }
//! ```

use serde_json::{json, Value};
use std::error::Error;
use time::macros::format_description;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
use yahoofinance::Quote;

/// Convert str starting date into datetime
pub fn parse_start_date(d: &str) -> Result<OffsetDateTime, Box<dyn Error>> {
    let date_fmt = format_description!("[year]-[month]-[day]");
    let date = Date::parse(d, &date_fmt)?;
    let primitive_datetime = PrimitiveDateTime::new(date, Time::MIDNIGHT);
    let datetime_utc = primitive_datetime.assume_offset(UtcOffset::UTC);

    Ok(datetime_utc)
}

/// Convert str ending date into datetime
pub fn parse_end_date(d: &str) -> Result<OffsetDateTime, Box<dyn Error>> {
    let date_fmt = format_description!("[year]-[month]-[day]");
    let date = Date::parse(d, &date_fmt)?;
    let eod = Time::from_hms_micro(23, 59, 59, 999_999)?;
    let primitive_datetime = PrimitiveDateTime::new(date, eod);
    let datetime_utc = primitive_datetime.assume_offset(UtcOffset::UTC);

    Ok(datetime_utc)
}

/// Convert timestamp into datetime
pub fn timestamp_to_datetime(timestamp: i64) -> Result<OffsetDateTime, Box<dyn Error>> {
    let dt_utc = OffsetDateTime::from_unix_timestamp(timestamp)?;
    Ok(dt_utc.to_offset(UtcOffset::UTC))
}

/// Convert timestamp into local datetime string in the format "Y-m-d H:M:S"
pub fn timestamp_to_localdt(timestamp: u64) -> Result<String, Box<dyn Error>> {
    let t = i64::try_from(timestamp)?;
    let utc_datetime = timestamp_to_datetime(t)?;
    // Used to use this but OffsetDatetime::now_local but generates conflict with CI actions. Keeping commented to future reference.
    //let local_offset = OffsetDateTime::now_local()?.offset();
    // Hardcoding the offset seems to work with CI actions: currently offset set Europe/Rome
    let local_offset = UtcOffset::from_hms(1, 0, 0)?;
    let local_datetime = utc_datetime.to_offset(local_offset);

    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let dt = local_datetime.format(&format)?;

    Ok(dt)
}

pub fn datetime_to_date(d: String) -> Result<String, Box<dyn Error>> {
    match d.as_str().split_once(" ") {
        Some((date, _time)) => Ok(date.to_string()),
        None => Err("Invalid datetime format. Expected %Y-%m-%d %H:%M:%S. {}".into()),
    }
}

pub fn quotes_to_json(data: Vec<Quote>) -> Result<Value, Box<dyn Error>> {
    let d: Vec<Value> = data
        .into_iter()
        .map(|q| {
            let dt = timestamp_to_localdt(q.timestamp)
                .unwrap_or_else(|_| "Invalid timestamp".to_string());
            json!({
                "datetime": dt,
                "open": q.open,
                "high": q.high,
                "low": q.low,
                "close": q.close,
                "adjclose": q.adjclose,
                "volume": q.volume,
            })
        })
        .collect();

    Ok(json!(d))
}
