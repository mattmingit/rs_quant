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
