use serde::Deserialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not url encode params: {0}")]
    UrlEncode(#[from] serde_urlencoded::ser::Error),
    #[error("could not process request: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("could not parse json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("the api returned an error: {0}")]
    Api(ApiError),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiError {
    pub error: bool,
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
