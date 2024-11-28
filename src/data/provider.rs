use yahoofinance::YahooConnector;

/// Define the connection using yahoo_finance_api crate
pub struct YahooFinance {
    pub connector: YahooConnector,
}

impl YahooFinance {
    pub fn connector() -> Self {
        let c = YahooConnector::new().expect("Failed to build connection with yahoo! finance.");
        Self { connector: c }
    }
}
