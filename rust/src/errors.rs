use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API Error: status={status}, message={message}")]
    Api {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
