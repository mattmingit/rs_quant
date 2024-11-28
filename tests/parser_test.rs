use rs_quant::utils::parsers::{
    parse_end_date, parse_start_date, quotes_to_json, timestamp_to_datetime, timestamp_to_localdt,
};
use serde_json::Value;
use time::macros::{datetime, format_description};
use time::{OffsetDateTime, UtcOffset};
use yahoofinance::Quote;

#[test]
fn start_date_parser() {
    let start = parse_start_date("2021-06-17").unwrap();
    assert_eq!(start, datetime!(2021-06-17 0:00:00 UTC));
}

#[test]
fn end_date_parser() {
    let end = parse_end_date("2021-06-17").unwrap();
    assert_eq!(end, datetime!(2021-06-17 23:59:59.999999 UTC));
}

#[test]
fn timestamp_to_datetime_parser() {
    let timestamp = 1732440896;
    let r = timestamp_to_datetime(timestamp).unwrap();

    assert_eq!(r.year(), 2024);
    assert_eq!(r.month() as u8, 11);
    assert_eq!(r.day(), 24);
    assert_eq!(r.hour(), 9);
    assert_eq!(r.minute(), 34);
    assert_eq!(r.second(), 56);
    assert_eq!(r.offset().whole_hours(), 0);
}

#[test]
fn test_invalid_timestamp() {
    let invalid_timestamp = i64::MAX;
    let r = timestamp_to_datetime(invalid_timestamp);
    assert!(r.is_err());
}

#[test]
fn timestamp_to_local_datetime() {
    let timestamp = OffsetDateTime::now_utc().unix_timestamp() as u64;
    let r = timestamp_to_localdt(timestamp).unwrap();

    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let local_offset = UtcOffset::from_hms(1, 0, 0).unwrap();
    let expected_datetime = OffsetDateTime::now_utc()
        .to_offset(local_offset)
        .format(&format)
        .unwrap();

    assert_eq!(r, expected_datetime);
}

#[test]
fn test_quotes_to_json() {
    let sample = vec![
        Quote {
            timestamp: 1729839600, // Corresponds to a valid timestamp
            open: 10.0,
            high: 10.1,
            low: 10.0,
            close: 10.05,
            adjclose: 10.05,
            volume: 219,
        },
        Quote {
            timestamp: 1730102400, // Another valid timestamp
            open: 10.05,
            high: 10.1,
            low: 10.0,
            close: 10.1,
            adjclose: 10.1,
            volume: 759,
        },
    ];

    let r = quotes_to_json(sample.clone()).unwrap();
    let parsed: Vec<Value> = serde_json::from_value(r).unwrap();
    assert_eq!(parsed.len(), sample.len());

    let fq = &parsed[0];
    assert_eq!(fq["open"], 10.0);
    assert_eq!(fq["high"], 10.1);
    assert_eq!(fq["low"], 10.0);
    assert_eq!(fq["close"], 10.05);
    assert_eq!(fq["adjclose"], 10.05);
    assert_eq!(fq["volume"], 219);

    let dt = fq["datetime"].as_str().unwrap();
    assert!(dt.contains("-"));
}
