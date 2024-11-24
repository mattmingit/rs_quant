#[allow(dead_code)]
// yahoo! finance api endpoints
pub const YCHART_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart";
pub const YSEARCH_URL: &str = "https://query2.finance.yahoo.com/v1/finance/search";
pub const YQUOTESUMMARY: &str = "https://query1.yahoo.com/v10/finance.quoteSummary";

// Macro rules to set fetch data
#[allow(unused_macros)]
#[macro_export]
macro_rules! yticker_query {
    () => {
        "{url}/q={name}"
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! ychart_period_query {
    () => {
        "{url}/{symbol}?symbol={symbol}&period1={start}&period2={end}&interval={interval}&"
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! ychart_range_query {
    () => {
        "{url}/{symbol}?symbol={symbol}&interval={interval}&range={range}&event=div|split|capitalGains"
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! ychart_period_interval_query {
    () => {
       "{url}/{symbol}?symbol={symbol}&period={period}&interval={interval}&includePrePost={prepost}"
    };
}
