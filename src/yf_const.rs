// yahoo! finance api endpoints
pub const YCHART_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart";
pub const YSEARCH_URL: &str = "https://query2.finance.yahoo.com/v1/finance/search";
pub const YQUOTESUMMARY: &str = "https://query1.yahoo.com/v10/finance.quoteSummary";

// Macro rules to set fetch data
macro_rules! YTCIKER_QUERY {
    () => {
        "{url}/q={name}"
    };
}

macro_rules! YCHART_PERIOD_QUERY {
    () => {
        "{url}/{symbol}?symbol={symbol}&period1={start}&period2={end}&interval={interval}&"
    };
}

macro_rules! YCHART_RANGE_QUERY {
    () => {
        "{url}/{symbol}?symbol={symbol}&interval={interval}&range={range}&event=div|split|captialGains"
    };
}

macro_rules! YCHART_PERIOD_INTERVAL_QUERY {
    () => {
       "{url}/{symbol}?symbol={symbol}&period={period}&interval={interval}&includePrePost={prepost}"
    };
}
