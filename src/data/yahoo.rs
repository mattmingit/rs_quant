use std::{collections::HashMap, error::Error};

use crate::commons::{
    date::{datetime_to_date, parse_date, timestamp_to_localdt, DateType},
    parser::round_to_three,
};
use yahoofinance::{Quote, YOptionContract, YahooConnector};

// struct to define connection
pub struct Yahoo {
    pub provider: YahooConnector,
}

// struct to model yahoo! finance quotes with datetime string instead of timestamp
#[derive(Debug, Clone)]
pub struct QuoteItem {
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub adjclose: f64,
    pub volume: u64,
}

// struct to model yahoo! finance quotes into mutiplte quotes
#[derive(Debug, Clone)]
pub struct MultiQuoteItem {
    pub date: String,
    pub prices: HashMap<String, f64>,
}

// struct to model yahoo! finance options
#[derive(Debug, Clone)]
pub struct OptionContract {
    pub contract_symbol: Option<String>,
    pub strike: Option<f64>,
    pub currency: Option<String>,
    pub last_price: Option<f64>,
    pub change: Option<f64>,
    pub percent_change: Option<f64>,
    pub volume: Option<u64>,
    pub open_interest: Option<u64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub contract_size: Option<String>,
    pub expiration: Option<u64>, // refactor to convert this to date string
    pub last_trade_date: Option<u64>, // refactor to convert this to date string
    pub implied_volatility: Option<f64>,
    pub in_the_money: Option<bool>,
}

// option type enumerator
pub enum OptionType {
    Call,
    Put,
}

impl Yahoo {
    pub fn provider() -> Self {
        let provider =
            YahooConnector::new().expect("Failed to build connection with yahoo! finance api.");
        Self { provider }
    }

    // get quotation data for single asset query
    pub async fn get_quotes(
        &self,
        symbol: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
        period: Option<&str>,
        interval: Option<&str>,
    ) -> Result<Vec<QuoteItem>, Box<dyn Error>> {
        // set default interval if not passed as argument
        let interval = interval.unwrap_or("1m");

        // case 1: get data for specific date range
        if let (Some(start_date), Some(end_date)) = (start_date, end_date) {
            let start_dt = parse_date(start_date, DateType::Start)?;
            let end_dt = parse_date(end_date, DateType::End)?;
            return convert_to_quoteitem(
                self.provider
                    .get_quote_history_interval(symbol, start_dt, end_dt, interval)
                    .await?
                    .quotes()?,
            );
        }

        // case 2: get data for defined period
        if let Some(period) = period {
            return convert_to_quoteitem(
                self.provider
                    .get_quote_range(symbol, interval, period)
                    .await?
                    .quotes()?,
            );
        }

        // default case: get data for current trading day
        convert_to_quoteitem(
            self.provider
                .get_quote_range(symbol, interval, "1d")
                .await?
                .quotes()?,
        )
    }

    // get quotation data for multiple assets query
    pub async fn get_multiple_quotes(
        &self,
        tickers: Vec<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        period: Option<&str>,
        interval: Option<&str>,
    ) -> Result<Vec<MultiQuoteItem>, Box<dyn Error>> {
        // initialize results hashmap container
        let mut r: Vec<(String, Vec<QuoteItem>)> = Vec::new();

        // loop through tickers and fetch quotes data
        for t in tickers {
            r.push((
                t.to_string(),
                self.get_quotes(t, start_date, end_date, period, interval)
                    .await?
                    .into_iter()
                    .map(|mut q| {
                        if let Ok(date) = datetime_to_date(q.datetime.clone()) {
                            q.datetime = date;
                        }
                        q
                    })
                    .collect(),
            ));
        }

        // transform the data into format
        let mut m_quotes: Vec<MultiQuoteItem> = Vec::new();

        // create a map to group quotes by date
        let mut date_map: HashMap<String, HashMap<String, f64>> = HashMap::new();

        for (ticker, quotes) in r {
            for quote in quotes {
                let date = quote.datetime.clone();
                let adjclose = quote.adjclose;

                let prices = date_map.entry(date).or_default();
                prices.insert(ticker.clone(), adjclose);
            }
        }

        // convert the date_map into a vector of DynamicQuoteItem
        for (date, prices) in date_map {
            m_quotes.push(MultiQuoteItem { date, prices });
        }

        // sort the dynamic_quotes by date (if needed)
        m_quotes.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(m_quotes)
    }

    // get latest quotation for an asset
    pub async fn get_latest_quote(&self, ticker: &str) -> Result<f64, Box<dyn Error>> {
        Ok(round_to_three(
            self.provider
                .get_latest_quotes(ticker, "1d")
                .await?
                .last_quote()?
                .close,
        ))
    }

    // get asset options data
    pub async fn get_options(
        &self,
        ticker: &str,
        option_type: OptionType,
    ) -> Result<Vec<OptionContract>, Box<dyn Error>> {
        let r = self.provider.search_options(ticker).await?;
        let options = &r.option_chain.result[0].options[0];
        let options = match option_type {
            OptionType::Call => &options.calls,
            OptionType::Put => &options.puts,
        };
        convert_to_optioncontract(options)
    }
}

// helper function to convert yaho! finance response into vector
pub fn convert_to_quoteitem(quotes: Vec<Quote>) -> Result<Vec<QuoteItem>, Box<dyn Error>> {
    quotes
        .into_iter()
        .map(|q| {
            let datetime = timestamp_to_localdt(q.timestamp)?;

            Ok(QuoteItem {
                datetime,
                open: round_to_three(q.open),
                high: round_to_three(q.high),
                low: round_to_three(q.low),
                close: round_to_three(q.close),
                adjclose: round_to_three(q.adjclose),
                volume: q.volume,
            })
        })
        .collect()
}

// helper function to convert yahoo! finance response into vector
fn convert_to_optioncontract(
    options: &[YOptionContract],
) -> Result<Vec<OptionContract>, Box<dyn Error>> {
    Ok(options
        .iter()
        .map(|option| OptionContract {
            contract_symbol: option.contract_symbol.clone(),
            strike: option.strike,
            currency: option.currency.clone(),
            last_price: option.last_price,
            change: option.change,
            percent_change: option.percent_change,
            volume: option.volume,
            open_interest: option.open_interest,
            bid: option.bid,
            ask: option.ask,
            contract_size: option.contract_size.clone(),
            expiration: option.expiration,
            last_trade_date: option.last_trade_date,
            implied_volatility: option.implied_volatility,
            in_the_money: option.in_the_money,
        })
        .collect())
}
