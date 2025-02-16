//use super::error::YahooErr;
use crate::commons::{
    date::{datetime_to_date, parse_date, timestamp_to_localdt, DateType},
    parser::round_to_three,
};
use ndarray::Array1;
use sqlx::Result;
use std::{collections::HashMap, error::Error};
use thiserror::Error;
use yahoofinance::{Quote, YOptionContract, YSearchResult, YahooConnector};

#[derive(Debug, Error)]
pub enum YahooErr {
    #[error("Failed to fetch data from yahoo! finance: {0}")]
    FetchFailed(String),
    #[error("Failed to deserialize from yahoo! finance: {0}")]
    DeserializationFailed(#[from] serde_json::Error),
    #[error("Request to yahoo! finance servers failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Yahoo! finance returned invalid JSON format.")]
    InvalidJson,
    #[error("Yahoo! finance returned an empty data set.")]
    EmptyDataSet,
    #[error("Yahoo! Finance returned inconsistent data: {0}")]
    DataInconsistency(String),
    #[error("Failed to build Yahoo! Finance client.")]
    BuilderFailed,
    #[error(
        "Failed to parse Yahoo! Finance date format. Response returned invalid date format: {0}"
    )]
    InvalidDateFormat(String),
}

// struct to model connection with yahoo! finance
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

// implementation of Yahoo struct
impl Yahoo {
    pub fn provider() -> Result<Self, YahooErr> {
        let provider = YahooConnector::new().map_err(|_| YahooErr::BuilderFailed)?;
        Ok(Self { provider })
    }

    // get quotation data for single asset query
    pub async fn get_quotes(
        &self,
        symbol: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
        period: Option<&str>,
        interval: Option<&str>,
    ) -> Result<Array1<QuoteItem>, YahooErr> {
        // set default interval if not passed as argument
        let interval = interval.unwrap_or("1m");

        // case 1: get data for specific date range
        if let (Some(start_date), Some(end_date)) = (start_date, end_date) {
            let start_dt = parse_date(start_date, DateType::Start)
                .map_err(|_| YahooErr::InvalidDateFormat(start_date.to_string()))?;
            let end_dt = parse_date(end_date, DateType::End)
                .map_err(|_| YahooErr::InvalidDateFormat(end_date.to_string()))?;
            return convert_to_quoteitem(
                self.provider
                    .get_quote_history_interval(symbol, start_dt, end_dt, interval)
                    .await
                    .map_err(|err| YahooErr::FetchFailed(err.to_string()))?
                    .quotes()
                    .map_err(|err| {
                        if err.to_string().contains("EOF") || err.to_string().contains("empty") {
                            YahooErr::EmptyDataSet
                        } else {
                            YahooErr::InvalidJson
                        }
                    })?,
            )
            .map_err(|e| YahooErr::DataInconsistency(e.to_string()));
        }

        // case 2: get data for defined period
        if let Some(period) = period {
            return convert_to_quoteitem(
                self.provider
                    .get_quote_range(symbol, interval, period)
                    .await
                    .map_err(|err| YahooErr::FetchFailed(err.to_string()))?
                    .quotes()
                    .map_err(|err| {
                        if err.to_string().contains("EOF") || err.to_string().contains("empty") {
                            YahooErr::EmptyDataSet
                        } else {
                            YahooErr::InvalidJson
                        }
                    })?,
            )
            .map_err(|e| YahooErr::DataInconsistency(e.to_string()));
        }

        // default case: get data for current trading day
        convert_to_quoteitem(
            self.provider
                .get_quote_range(symbol, interval, "1d")
                .await
                .map_err(|err| YahooErr::FetchFailed(err.to_string()))?
                .quotes()
                .map_err(|err| {
                    if err.to_string().contains("EOF") || err.to_string().contains("empty") {
                        YahooErr::EmptyDataSet
                    } else {
                        YahooErr::InvalidJson
                    }
                })?,
        )
        .map_err(|e| YahooErr::DataInconsistency(e.to_string()))
    }

    // get quotation data for multiple assets query
    pub async fn get_multiple_quotes(
        &self,
        tickers: Vec<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        period: Option<&str>,
        interval: Option<&str>,
    ) -> Result<Array1<MultiQuoteItem>, YahooErr> {
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

        Ok(Array1::from_vec(m_quotes))
    }

    // get latest quotation for an asset
    pub async fn get_latest_quote(&self, ticker: &str) -> Result<f64, YahooErr> {
        Ok(round_to_three(
            self.provider
                .get_latest_quotes(ticker, "1d")
                .await
                .map_err(|err| YahooErr::FetchFailed(err.to_string()))?
                .last_quote()
                .map_err(|e| YahooErr::DataInconsistency(e.to_string()))?
                .close,
        ))
    }

    // get asset options data
    pub async fn get_options(
        &self,
        ticker: &str,
        option_type: OptionType,
    ) -> Result<Array1<OptionContract>, YahooErr> {
        let r = self
            .provider
            .search_options(ticker)
            .await
            .map_err(|e| YahooErr::FetchFailed(e.to_string()))?;

        if r.option_chain.result.is_empty() {
            return Err(YahooErr::EmptyDataSet);
        }

        let options = &r.option_chain.result[0].options[0];
        let options = match option_type {
            OptionType::Call => &options.calls,
            OptionType::Put => &options.puts,
        };
        convert_to_optioncontract(options)
    }

    // search asset
    pub async fn search_asset(self, name: &str) -> Result<YSearchResult, YahooErr> {
        self.provider
            .search_ticker(name)
            .await
            .map_err(|e| YahooErr::FetchFailed(e.to_string()))
    }
}

// helper function to convert yaho! finance response into vector
fn convert_to_quoteitem(quotes: Vec<Quote>) -> Result<Array1<QuoteItem>, Box<dyn Error>> {
    Ok(Array1::from_iter(
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
            .collect::<Result<Vec<QuoteItem>, Box<dyn Error>>>()?,
    ))
}

fn convert_to_optioncontract(
    options: &[YOptionContract],
) -> Result<Array1<OptionContract>, YahooErr> {
    Ok(Array1::from_iter(options.iter().map(|option| {
        OptionContract {
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
        }
    })))
}
