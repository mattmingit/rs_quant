use crate::yf_error::YahooError;
pub use decimal::*;
use serde::{
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use std::{collections::HashMap, fmt, vec};

/// Made decimal a feature so it can be enabled by user. This handle decimal base on feature enable status.
#[cfg(not(feature = "decimal"))]
pub mod decimal {
    pub type Decimal = f64;
    pub const ZERO: Decimal = 0.0;
}

#[cfg(feature = "decimal")]
pub mod decimal {
    pub type Decimal = rust_decimal::Decimal;
    pub const ZERO: Decimal = Decimal::ZERO;
}

#[derive(Debug, Deserialize)]
pub struct YResponse {
    pub chart: YChart,
}

impl YResponse {
    /// Check data consistency of data fetched
    fn check_consistency(&self) -> Result<(), YahooError> {
        for stock in &self.chart.result {
            let n = stock.timestamp.len();
            if n == 0 {
                return Err(YahooError::EmptyDataSet);
            }
            let quote = &stock.indicators.quote[0];
            if quote.open.len() != n
                || quote.high.len() != n
                || quote.low.len() != n
                || quote.close.len() != n
                || quote.volume.len() != n
            {
                return Err(YahooError::DataInconsistency);
            }
            if let Some(ref adjclose) = stock.indicators.adjclose {
                if adjclose[0].adjclose.len() != n {
                    return Err(YahooError::DataInconsistency);
                }
            }
        }
        Ok(())
    }

    pub fn from_json(json: serde_json::Value) -> Result<YResponse, YahooError> {
        Ok(serde_json::from_value(json)?)
    }

    /// Return the latest valid quote
    pub fn last_quote(&self) -> Result<Quote, YahooError> {
        self.check_consistency()?;
        let stock = &self.chart.result[0];
        let n = stock.timestamp.len();
        for i in (0..n).rev() {
            let quote = stock.indicators.get_ith_quote(stock.timestamp[i], i);
            if quote.is_ok() {
                return quote;
            }
        }
        Err(YahooError::EmptyDataSet)
    }

    pub fn quotes(&self) -> Result<Vec<Quote>, YahooError> {
        self.check_consistency()?;
        let stock = &self.chart.result[0];
        let mut quotes = Vec::new();
        let n = stock.timestamp.len();
        for i in 0..n {
            let timestamp = stock.timestamp[i];
            let quote = stock.indicators.get_ith_quote(timestamp, i);
            if let Ok(q) = quote {
                quotes.push(q);
            }
        }
        Ok(quotes)
    }

    /// This method retrieves information about the metadata of the assets
    pub fn metadata(&self) -> Result<YMetaData, YahooError> {
        self.check_consistency()?;
        let stock = &self.chart.result[0];
        Ok(stock.meta.to_owned())
    }

    /// This method retrieves information about the splits that might have
    /// occurred during considered time period.
    pub fn splits(&self) -> Result<Vec<Split>, YahooError> {
        self.check_consistency()?;
        let stock = &self.chart.result[0];
        if let Some(events) = &stock.events {
            if let Some(splits) = &events.splits {
                let mut data = splits.values().cloned().collect::<Vec<Split>>();
                data.sort_unstable_by_key(|d| d.date);
                return Ok(data);
            }
        }
        Ok(vec![])
    }

    /// This method retrieves information about the dividends that might have
    /// occurred during the considered time period.
    ///
    /// Note: Date is ex-dividend date.
    pub fn dividends(&self) -> Result<Vec<Dividend>, YahooError> {
        self.check_consistency()?;
        let stock = &self.chart.result[0];
        if let Some(events) = &stock.events {
            if let Some(dividends) = &events.dividends {
                let mut data = dividends.values().cloned().collect::<Vec<Dividend>>();
                data.sort_unstable_by_key(|d| d.date);
                return Ok(data);
            }
        }
        Ok(vec![])
    }

    /// This method retrieves information about the capital gains that might have
    /// occurred during the considered time period (available for mutual funds).
    pub fn capital_gains(&self) -> Result<Vec<CapitalGain>, YahooError> {
        self.check_consistency()?;
        let stock = &self.chart.result[0];
        if let Some(events) = &stock.events {
            if let Some(capital_gains) = &events.capital_gains {
                let mut data = capital_gains
                    .values()
                    .cloned()
                    .collect::<Vec<CapitalGain>>();
                data.sort_unstable_by_key(|d| d.date);
                return Ok(data);
            }
        }
        Ok(vec![])
    }
}

/// Struct for single quote
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Quote {
    pub timestamp: u64,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub volume: u64,
    pub close: Decimal,
    pub adjclose: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct YChart {
    pub result: Vec<YQuoteBlock>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct YQuoteBlock {
    pub meta: YMetaData,
    pub timestamp: Vec<u64>,
    pub events: Option<EventsBlock>,
    pub indicators: Quoteblock,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct YMetaData {
    pub currency: Option<String>,
    pub symbol: String,
    pub exchange_name: String,
    pub instrument_type: String,
    #[serde(default)]
    pub first_date_trade: Option<i32>,
    pub regular_market_time: u32,
    pub gmtoffset: i32,
    pub timezone: String,
    pub exchange_timezone_name: String,
    pub regular_market_price: Decimal,
    pub chart_previous_close: Decimal,
    #[serde(default)]
    pub scale: Option<i32>,
    pub price_hint: i32,
    pub current_trading_period: CurrentTradingPeriod,
    #[serde(default)]
    pub trading_periods: TradingPeriods,
    pub data_granularity: String,
    pub range: String,
    pub valid_ranges: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TradingPeriods {
    pub pre: Option<Vec<Vec<PeriodInfo>>>,
    pub regular: Option<Vec<Vec<PeriodInfo>>>,
    pub post: Option<Vec<Vec<PeriodInfo>>>,
}

impl<'de> Deserialize<'de> for TradingPeriods {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Regular,
            Pre,
            Post,
        }

        struct TradingPeriodsVisitor;

        impl<'de> Visitor<'de> for TradingPeriodsVisitor {
            type Value = TradingPeriods;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct (or array) TradingPeriods")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<TradingPeriods, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let regular: Vec<PeriodInfo> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(TradingPeriods {
                    pre: None,
                    regular: Some(vec![regular]),
                    post: None,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<TradingPeriods, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut pre = None;
                let mut post = None;
                let mut regular = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Pre => {
                            if pre.is_some() {
                                return Err(de::Error::duplicate_field("pre"));
                            }
                            pre = Some(map.next_value()?);
                        }
                        Field::Post => {
                            if post.is_some() {
                                return Err(de::Error::duplicate_field("post"));
                            }
                            post = Some(map.next_value()?);
                        }
                        Field::Regular => {
                            if regular.is_some() {
                                return Err(de::Error::duplicate_field("regular"));
                            }
                            regular = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TradingPeriods { pre, post, regular })
            }
        }
        deserializer.deserialize_any(TradingPeriodsVisitor)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CurrentTradingPeriod {
    pub pre: PeriodInfo,
    pub regular: PeriodInfo,
    pub post: PeriodInfo,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct PeriodInfo {
    pub timezone: String,
    pub start: u32,
    pub end: u32,
    pub gmtoffset: i32,
}

#[derive(Debug, Deserialize)]
pub struct Quoteblock {
    quote: Vec<QuoteList>,
    #[serde(default)]
    adjclose: Option<Vec<AdjClose>>,
}

impl Quoteblock {
    fn get_ith_quote(&self, timestamp: u64, i: usize) -> Result<Quote, YahooError> {
        let adjclose = match &self.adjclose {
            Some(adjclose) => adjclose[0].adjclose[i],
            None => None,
        };
        let quote = &self.quote[0];
        // reject if close is not set
        if quote.close[i].is_none() {
            return Err(YahooError::EmptyDataSet);
        }
        Ok(Quote {
            timestamp,
            open: quote.open[i].unwrap_or(ZERO),
            high: quote.high[i].unwrap_or(ZERO),
            low: quote.low[i].unwrap_or(ZERO),
            close: quote.close[i].unwrap_or(ZERO),
            adjclose: adjclose.unwrap_or(ZERO),
            volume: quote.volume[i].unwrap_or(0),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct AdjClose {
    adjclose: Vec<Option<Decimal>>,
}

#[derive(Debug, Deserialize)]
pub struct QuoteList {
    pub volume: Vec<Option<u64>>,
    pub high: Vec<Option<Decimal>>,
    pub close: Vec<Option<Decimal>>,
    pub low: Vec<Option<Decimal>>,
    pub open: Vec<Option<Decimal>>,
}

/// The struct models the type of events that can have affected an asset
#[derive(Debug, Deserialize)]
pub struct EventsBlock {
    pub splits: Option<HashMap<u64, Split>>,
    pub dividends: Option<HashMap<u64, Dividend>>,
    #[serde(rename = "capitalGains")]
    pub capital_gains: Option<HashMap<u64, CapitalGain>>,
}

/// The struct simply models a split that has occurred
#[derive(Debug, Deserialize, Clone)]
pub struct Split {
    /// The date (timestamp) when the split occurred
    pub date: u64,
    /// Numerator of the split. For instance 1:5 split means you get 5 share
    /// wherever you had one before the split. (Here the numerator is 1 and
    /// denominator is 5). A reverse split is considered as nothing but a regular
    /// split with a numerator > denom.
    pub numerator: Decimal,
    /// Denominator of the split. For instance 1:5 split means you get 5 share
    /// wherever you had one before the split. (Here the numerator is 1 and
    /// denominator is 5). A reverse split is considered as nothing but a regular
    /// split with a numerator > denom.
    pub denominator: Decimal,
    /// A textual representation of the split.
    #[serde(rename = "splitRatio")]
    pub split_ratio: String,
}

/// The struct simply models a dividend which has been recorded.
#[derive(Debug, Deserialize, Clone)]
pub struct Dividend {
    /// Price of the dividend
    pub amount: Decimal,
    /// ex-dividend date
    pub date: u64,
}

/// The struct simply models a capital gain which has been recorded.
#[derive(Deserialize, Debug, Clone)]
pub struct CapitalGain {
    /// Amount of capital gain distributed by the fund
    pub amount: f64,
    /// Recorded date of the capital gain
    pub date: u64,
}
