use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositList {
    /// current page
    pub current_page: i64,
    pub items: Vec<Deposit>,
    /// page size
    pub page_size: i64,
    /// total number
    pub total_num: i64,
    /// total pages
    pub total_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    /// Deposit address
    pub address: Option<String>,
    /// Deposit amount
    pub amount: Option<String>,
    /// Whether there is any debt.A quick rollback will cause the deposit to fail. If the deposit
    /// fails, you will need to repay the balance.
    pub arrears: Option<bool>,
    /// The chainName of currency
    pub chain: Option<String>,
    /// Database record creation time
    pub created_at: Option<i64>,
    /// Currency
    pub currency: Option<String>,
    /// Fees charged for deposit
    pub fee: Option<String>,
    /// Internal deposit or not
    pub is_inner: Option<bool>,
    /// Address remark. If there’s no remark, it is empty.
    pub memo: Option<String>,
    /// remark
    pub remark: Option<String>,
    /// Status
    pub status: Option<DepositStatus>,
    /// Update time of the database record
    pub updated_at: Option<i64>,
    /// Wallet Txid
    pub wallet_tx_id: Option<String>,
}

/// Status
#[derive(Debug, Clone, Serialize, Deserialize)]
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
