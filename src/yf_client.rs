use crate::yf_const::{YCHART_URL, YQUOTESUMMARY, YSEARCH_URL};
use crate::yf_error::YahooError;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

// struct for connection parameters to yahoo! finance server
#[allow(dead_code)]
pub struct YahooConnector {
    pub client: Client,
    pub url: &'static str,
    pub search_url: &'static str,
    pub summary_url: &'static str,
}

#[derive(Default)]
pub struct YahooConnectorBuilder {
    pub inner: ClientBuilder,
}

impl YahooConnector {
    // Constructor for new instance of yahoo connector
    pub fn new() -> Result<YahooConnector, YahooError> {
        Self::builder().build()
    }

    pub fn builder() -> YahooConnectorBuilder {
        YahooConnectorBuilder {
            inner: Client::builder(),
        }
    }
}

impl Default for YahooConnector {
    fn default() -> Self {
        YahooConnector {
            client: Client::default(),
            url: YCHART_URL,
            search_url: YSEARCH_URL,
            summary_url: YQUOTESUMMARY,
        }
    }
}

impl YahooConnectorBuilder {
    pub fn build(self) -> Result<YahooConnector, YahooError> {
        self.build_with_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
    }

    pub fn build_with_agent(self, user_agent: &str) -> Result<YahooConnector, YahooError> {
        let client = Client::builder().user_agent(user_agent).build()?;

        Ok(YahooConnector {
            client,
            url: YCHART_URL,
            search_url: YSEARCH_URL,
            summary_url: YQUOTESUMMARY,
        })
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.inner = self.inner.timeout(timeout);
        self
    }
}
