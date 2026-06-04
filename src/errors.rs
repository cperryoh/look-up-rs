#[derive(thiserror::Error, Debug)]
pub enum LookUpErrors {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Json error: {0}")]
    JsonError(#[from] serde_json::error::Error),
}
pub type Result<T> = std::result::Result<T, LookUpErrors>;
