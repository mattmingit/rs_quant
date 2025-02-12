use rs_quant::commons::date::{
    datetime_to_date, parse_date, timestamp_to_datetime, timestamp_to_localdt, DateType,
};
use time::macros::{datetime, format_description};
use time::{OffsetDateTime, UtcOffset};

#[test]
fn test_parse_date() {
    let start = parse_date("2021-06-17", DateType::Start).unwrap();
    assert_eq!(start, datetime!(2021-06-17 0:00:00 UTC));

    let end = parse_date("2021-06-17", DateType::End).unwrap();
    assert_eq!(end, datetime!(2021-06-17 23:59:59.999999 UTC));
}

#[test]
fn test_timestamp_to_datetime() {
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
fn invalid_timestamp() {
    let invalid_timestamp = i64::MAX;
    let r = timestamp_to_datetime(invalid_timestamp);
    assert!(r.is_err());
}

#[test]
fn test_timestamp_to_localdt() {
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
fn test_datetime_to_date() {
    let datetime = "2024-11-30 12:45:30".to_string();
    let date = datetime_to_date(datetime).unwrap();
    assert_eq!(date, "2024-11-30");
}
