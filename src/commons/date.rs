use thiserror::Error;
use time::macros::format_description;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

// datetime parsing errors
#[derive(Debug, Error)]
pub enum DateTimeError {
    #[error("Failed to parse date '{0}': {1}")]
    InvalidDateFormat(String, String),
    #[error("Failed to convert timestamp '{0}' into datetime: {1}")]
    InvalidTimestamp(i64, String),
    #[error("Invalid datetime format. Expected %Y-%m-%d %H:%M:%S, but got '{0}'")]
    InvalidDateTimeFormat(String),
    #[error("Invalid time component: {0}")]
    InvalidTimeComponent(String),
}

pub enum DateType {
    Start,
    End,
}

// convert start or end date into datetime
pub fn parse_date(date: &str, date_type: DateType) -> Result<OffsetDateTime, DateTimeError> {
    match date_type {
        DateType::Start => {
            let date_fmt = format_description!("[year]-[month]-[day]");
            let date = Date::parse(date, date_fmt).map_err(|err| {
                DateTimeError::InvalidDateFormat(date.to_string(), err.to_string())
            })?;
            let primitive_dt = PrimitiveDateTime::new(date, Time::MIDNIGHT);
            let dt_utc = primitive_dt.assume_offset(UtcOffset::UTC);
            Ok(dt_utc)
        }
        DateType::End => {
            let date_fmt = format_description!("[year]-[month]-[day]");
            let date = Date::parse(date, date_fmt).map_err(|err| {
                DateTimeError::InvalidDateFormat(date.to_string(), err.to_string())
            })?;
            let eod = Time::from_hms_micro(23, 59, 59, 999_999)
                .map_err(|err| DateTimeError::InvalidTimeComponent(err.to_string()))?;
            let primitive_dt = PrimitiveDateTime::new(date, eod);
            let dt_utc = primitive_dt.assume_offset(UtcOffset::UTC);
            Ok(dt_utc)
        }
    }
}

// convert timestamp into datetime
pub fn timestamp_to_datetime(timestamp: i64) -> Result<OffsetDateTime, DateTimeError> {
    Ok(OffsetDateTime::from_unix_timestamp(timestamp)
        .map_err(|err| DateTimeError::InvalidTimestamp(timestamp, err.to_string()))?
        .to_offset(UtcOffset::UTC))
}

// convert timestamp into local datetime string in the format "Y-m-d H:M:S"
pub fn timestamp_to_localdt(timestamp: u64) -> Result<String, DateTimeError> {
    let utc_datetime = timestamp_to_datetime(timestamp as i64)?;
    let local_datetime = utc_datetime.to_offset(
        UtcOffset::from_hms(1, 0, 0)
            .map_err(|err| DateTimeError::InvalidTimestamp(timestamp as i64, err.to_string()))?,
    );
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let dt = local_datetime
        .format(&format)
        .map_err(|err| DateTimeError::InvalidTimestamp(timestamp as i64, err.to_string()))?;
    Ok(dt)
}

// convert datetime into simple date
pub fn datetime_to_date(d: String) -> Result<String, DateTimeError> {
    match d.as_str().split_once(" ") {
        Some((date, _time)) => Ok(date.to_string()),
        None => Err(DateTimeError::InvalidDateTimeFormat(d)),
    }
}
