use thiserror::Error as ThisError;

/// Defines Error types.
#[derive(ThisError, Debug)]
pub enum KucoinErrors {
    /// Contents doesn't match the Structure elements  
    #[error("SERDE-JSON-ERROR: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("Account tag is required for {0} ISOLATED account")]
    MissingIsolatedTag(String),

    #[error("REQWEST-ERROR: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
/// Alias Type for Results with Error Handler
pub type KucoinResults<T> = Result<T, KucoinErrors>;

