use rs_quant::utils::parsers::str_to_datetime;
use std::error::Error;
use time::macros::datetime;

#[test]
fn test_parser_str_to_datetime() -> Result<(), Box<dyn Error>> {
    let d = "2024-10-23";
    let osdt = str_to_datetime(d)?;
    assert_eq!(datetime!(2024-10-23 0:00:00 +00:00), osdt);
    Ok(())
}
