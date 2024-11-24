use std::error::Error;
use time::macros::format_description;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

pub fn str_to_datetime(d: &str) -> Result<OffsetDateTime, Box<dyn Error>> {
    let date_fmt = format_description!("[year]-[month]-[day]");
    let date = Date::parse(d, &date_fmt)?;
    let primitive_datetime = PrimitiveDateTime::new(date, Time::MIDNIGHT);
    let datetime_utc = primitive_datetime.assume_offset(UtcOffset::UTC);

    Ok(datetime_utc)
}
