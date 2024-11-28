use crate::data::provider::YahooFinance;
use crate::utils::parsers::timestamp_to_localdt;
use std::error::Error;
use yahoofinance::{YOptionContract, YQuote};

pub enum ContractType {
    Call,
    Put,
}

impl YahooFinance {
    /// Retrieve options metadata
    pub async fn get_options_metadata(&self, ticker: &str) -> Result<YQuote, Box<dyn Error>> {
        Ok(self
            .connector
            .search_options(ticker)
            .await?
            .option_chain
            .result[0]
            .quote
            .clone())
    }

    /// Retrieves options expiration dates
    pub async fn get_options_expiration_dates(
        &self,
        ticker: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self
            .connector
            .search_options(ticker)
            .await?
            .option_chain
            .result[0]
            .expiration_dates
            .iter()
            .map(|d| timestamp_to_localdt(*d).unwrap())
            .collect())
    }

    /// Retrieves options strike prices
    pub async fn get_options_strike_prices(
        &self,
        ticker: &str,
    ) -> Result<Vec<f64>, Box<dyn Error>> {
        Ok(self
            .connector
            .search_options(ticker)
            .await?
            .option_chain
            .result[0]
            .strikes
            .clone())
    }

    /// Retrieves options contracts
    pub async fn get_options_contracts(
        &self,
        ticker: &str,
        contract_type: ContractType,
    ) -> Result<Vec<YOptionContract>, Box<dyn Error>> {
        Ok(match contract_type {
            ContractType::Call => self
                .connector
                .search_options(ticker)
                .await?
                .option_chain
                .result[0]
                .options[0]
                .calls
                .clone(),
            ContractType::Put => self
                .connector
                .search_options(ticker)
                .await?
                .option_chain
                .result[0]
                .options[0]
                .puts
                .clone(),
        })
    }
}
