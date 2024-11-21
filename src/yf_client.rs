use crate::yf_const::{BASE_URL, SEARCH_URL};
use crate::yf_error::YahooError;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

// struct for connection parameters to yahoo! finance server
pub struct YahooConnector {
    client: Client,
    url: &'static str,
    search_url: &'static str,
}

#[derive(Default)]
pub struct YahooConnectorBuilder {
    inner: ClientBuilder,
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
            url: BASE_URL,
            search_url: SEARCH_URL,
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
            url: BASE_URL,
            search_url: SEARCH_URL,
        })
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.inner = self.inner.timeout(timeout);
        self
    }
}
