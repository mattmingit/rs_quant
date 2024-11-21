use thiserror::Error;

#[derive(Debug, Error)]
pub enum YahooError {
    #[error("fetching data from yahoo! finance failed")]
    FetchFailed(String),
    #[error("deserializing response from yahoo! finance failed")]
    DeserializeFailed(#[from] serde_json::Error),
    #[error("connection to yahoo! finance server failed")]
    ConnectionFailed(#[from] reqwest::Error),
    #[error("yahoo! finance returned invalid JSON format")]
    InvalidJson,
    #[error("yahoo! finance returned an empty dataset")]
    EmptyDataSet,
    #[error("yahoo! finance returned inconsistent data")]
    DataInconsistency,
    #[error("constructing yahoo! finance client failed")]
    BuildFailed,
}
