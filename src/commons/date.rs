use std::error::Error;
use time::macros::format_description;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

pub enum DateType {
    Start,
    End,
}

// convert start or end date into datetime
pub fn parse_date(date: &str, date_type: DateType) -> Result<OffsetDateTime, Box<dyn Error>> {
    match date_type {
        DateType::Start => {
            let date_fmt = format_description!("[year]-[month]-[day]");
            let date = Date::parse(date, date_fmt)?;
            let primitive_dt = PrimitiveDateTime::new(date, Time::MIDNIGHT);
            let dt_utc = primitive_dt.assume_offset(UtcOffset::UTC);
            Ok(dt_utc)
        }
        DateType::End => {
            let date_fmt = format_description!("[year]-[month]-[day]");
            let date = Date::parse(date, date_fmt)?;
            let eod = Time::from_hms_micro(23, 59, 59, 999_999)?;
            let primitive_dt = PrimitiveDateTime::new(date, eod);
            let dt_utc = primitive_dt.assume_offset(UtcOffset::UTC);
            Ok(dt_utc)
        }
    }
}

// convert timestamp into datetime
pub fn timestamp_to_datetime(timestamp: i64) -> Result<OffsetDateTime, Box<dyn Error>> {
    Ok(OffsetDateTime::from_unix_timestamp(timestamp)?.to_offset(UtcOffset::UTC))
}

// convert timestamp into local datetime string in the format "Y-m-d H:M:S"
pub fn timestamp_to_localdt(timestamp: u64) -> Result<String, Box<dyn Error>> {
    let utc_datetime = timestamp_to_datetime(timestamp as i64)?;
    let local_datetime = utc_datetime.to_offset(UtcOffset::from_hms(1, 0, 0)?);
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
