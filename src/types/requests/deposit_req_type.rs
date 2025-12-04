use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositQuery {
    /// currency
    pub currency: String,
    /// Current request page.
    pub current_page: Option<i64>,
    /// End time (milliseconds)
    pub end_at: Option<i64>,
    /// Number of results per request. Minimum is 10, maximum is 500.
    pub page_size: Option<i64>,
    /// Start time (milliseconds)
    pub start_at: Option<i64>,
    /// Status. Available value: PROCESSING, SUCCESS, and FAILURE
    pub status: Option<DepositStatus>,
}

/// Status. Available value: PROCESSING, SUCCESS, and FAILURE
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DepositStatus {
    Failure,
    Processing,
    Success,
    #[serde(rename = "TRM_MGT_REJECTED")]
    TrmMgtRejected,
    #[serde(rename = "WAIT_TRM_MGT")]
    WaitTrmMgt,
}
