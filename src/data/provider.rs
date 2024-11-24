use crate::data::quotes::YResponse;
use crate::data::search_res::{YOptionChain, YSearchResult, YSearchResultOpt};
use crate::yf_client::YahooConnector;
use crate::yf_error::YahooError;
use crate::{ychart_period_interval_query, ychart_period_query};
use crate::{ychart_range_query, yticker_query};
use reqwest::StatusCode;
use time::OffsetDateTime;

impl YahooConnector {
    /// Retrieve quotes of the last day for the given ticker
    pub async fn get_latest_quote(
        &self,
        ticker: &str,
        interval: &str,
    ) -> Result<YResponse, YahooError> {
        self.get_quote_range(ticker, interval, "1mo").await
    }

    /// Retrieve quotes for the given ticker from a starting date to an ending date (inclusive),
    /// if available
    pub async fn get_quote_history(
        &self,
        ticker: &str,
        start: OffsetDateTime,
        end: OffsetDateTime,
    ) -> Result<YResponse, YahooError> {
        self.get_quote_history_interval(ticker, start, end, "1d")
            .await
    }

    /// Retrieve quotes for the given ticker for an arbitrage range
    pub async fn get_quote_range(
        &self,
        ticker: &str,
        interval: &str,
        range: &str,
    ) -> Result<YResponse, YahooError> {
        let url = format!(
            ychart_range_query!(),
            url = self.url,
            symbol = ticker,
            interval = interval,
            range = range,
        );
        YResponse::from_json(self.send_request(&url).await?)
    }

    ///  Retrieve the quote history for the given ticker form date start to end (inclusive),
    /// if available; specifying the interval of the ticker.
    pub async fn get_quote_history_interval(
        &self,
        ticker: &str,
        start: OffsetDateTime,
        end: OffsetDateTime,
        interval: &str,
    ) -> Result<YResponse, YahooError> {
        let url = format!(
            ychart_period_query!(),
            url = self.url,
            symbol = ticker,
            start = start.unix_timestamp(),
            end = end.unix_timestamp(),
            interval = interval
        );
        YResponse::from_json(self.send_request(&url).await?)
    }

    /// Retrieve the quote history for the given ticker for a given period and ticker interval
    /// and optionally before and after regular trading hours
    pub async fn get_quote_period_interval(
        &self,
        ticker: &str,
        period: &str,
        interval: &str,
        prepost: bool,
    ) -> Result<YResponse, YahooError> {
        let url = format!(
            ychart_period_interval_query!(),
            url = self.url,
            symbol = ticker,
            period = period,
            interval = interval,
            prepost = prepost
        );
        YResponse::from_json(self.send_request(&url).await?)
    }

    /// Retrieve the list of quotes found searching a given name
    pub async fn search_ticker_opt(&self, name: &str) -> Result<YSearchResultOpt, YahooError> {
        let url = format!(yticker_query!(), url = self.search_url, name = name);
        YSearchResultOpt::from_json(self.send_request(&url).await?)
    }

    /// Retrieve the list of quotes found searching a given name
    pub async fn search_ticker_(&self, name: &str) -> Result<YSearchResult, YahooError> {
        let res = self.search_ticker_opt(name).await?;
        Ok(YSearchResult::from_opt(&res))
    }

    /// Get list for options for a given name
    pub async fn search_options(&self, name: &str) -> Result<YOptionChain, YahooError> {
        let url = format!("https://query1.finance.yahoo.com/v7/finance/options/{name}");
        let res = self.client.get(url).send().await?;
        let res = res.json::<YOptionChain>().await?;

        Ok(res)
    }

    /// Send request to yahoo! finance server and transform response to JSON value
    async fn send_request(&self, url: &str) -> Result<serde_json::Value, YahooError> {
        let r = self.client.get(url).send().await?;
        match r.status() {
            StatusCode::OK => Ok(r.json().await?),
            status => Err(YahooError::FetchFailed(format!("{}", status))),
        }
    }
}
