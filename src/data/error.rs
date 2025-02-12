//! custom error returned from methods and functions
use reqwest;
use serde_json;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum YahooErr {
    Fetchfailed(String),
    DeserializedFailed(serde_json::Error),
    RequestFailed(reqwest::Error),
    InvalidJson,
    EmptyDataSet,
    DataInconsistency(String),
    BuilderFailed,
    InvalidDateFormat(String),
}

impl fmt::Display for YahooErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YahooErr::Fetchfailed(e) => {
                write!(f, "Failed to fetch data from yahoo! finance: {}", e)
            }
            YahooErr::DeserializedFailed(e) => {
                write!(f, "Failed to deserialize from yahoo! finance: {}", e)
            }
            YahooErr::RequestFailed(e) => {
                write!(f, "Request to yahoo! finance server failed: {}", e)
            }
            YahooErr::InvalidJson => write!(f, "Yahoo! finance returned invalid JSON format."),
            YahooErr::EmptyDataSet => write!(f, "Yahoo! finance returned an empty data set."),
            YahooErr::DataInconsistency(e) => write!(f, "Yahoo! finance returned inconsistent data: {}", e),
            YahooErr::BuilderFailed => write!(f, "Failed to build yahoo! finance client."),
            YahooErr::InvalidDateFormat(e) => write!(f, "failed to parse yahoo! finance date format. Response returned invalid date format: {}", e)
        }
    }
}

impl Error for YahooErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            YahooErr::DeserializedFailed(e) => Some(e),
            _ => None,
        }
    }
}
