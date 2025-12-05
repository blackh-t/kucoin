use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    /// Transfer amount: The amount is a positive integer multiple of the currency precision.
    pub amount: String,
    /// Unique order ID created by users to identify their orders, e.g. UUID, with a maximum
    /// length of 128 bits
    pub client_oid: String,
    /// currency
    pub currency: String,
    /// Symbol: Required when the account type is ISOLATED or ISOLATED_V2, for example: BTC-USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_account_tag: Option<String>,
    /// Account type: MAIN, TRADE, CONTRACT, MARGIN, ISOLATED, MARGIN_V2, ISOLATED_V2
    pub from_account_type: AccountType,
    /// Transfer out UserId: This is required when transferring from sub-account to
    /// master-account. It is optional for internal transfers.
    pub from_user_id: Option<String>,
    /// Symbol: Required when the account type is ISOLATED or ISOLATED_V2, for example: BTC-USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_tag: Option<String>,
    /// Account type: MAIN, TRADE, CONTRACT, MARGIN, ISOLATED, MARGIN_V2, ISOLATED_V2
    pub to_account_type: AccountType,
    /// Transfer in UserId: This is required when transferring master-account to sub-account. It
    /// is optional for internal transfers.
    pub to_user_id: Option<String>,
    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_request_type: TransferType,
}

/// Account type: MAIN, TRADE, CONTRACT, MARGIN, ISOLATED, MARGIN_V2, ISOLATED_V2
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Contract,
    Isolated,
    #[serde(rename = "ISOLATED_V2")]
    IsolatedV2,
    Main,
    Margin,
    #[serde(rename = "MARGIN_V2")]
    MarginV2,
    Trade,
}

/// Transfer type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferType {
    Internal,
    #[serde(rename = "PARENT_TO_SUB")]
    ParentToSub,
    #[serde(rename = "SUB_TO_PARENT")]
    SubToParent,
}
