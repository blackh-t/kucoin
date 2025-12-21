// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::SubAccRequest;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: SubAccRequest = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccRequest {
    /// API expiration time
    pub expire: Option<Expire>,
    /// IP whitelist (You may add up to 20 IPs. Use a halfwidth comma to each IP)
    pub ip_whitelist: Option<String>,
    /// Password (Must contain 7–32 characters. Cannot contain any spaces.)
    pub passphrase: String,
    /// [Permissions](/docs-new/introduction)(Only General, Spot, Futures, Margin, Unified,
    /// InnerTransfer (Flex Transfer) permissions can be set
    pub permission: Option<String>,
    /// Remarks (1–24 characters)
    pub remark: String,
    /// Sub-account name, create sub account name of API Key.
    pub sub_name: String,
}

/// API expiration time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expire {
    #[serde(rename = "-1")]
    NeverExpire,
    #[serde(rename = "180")]
    T180day,
    #[serde(rename = "30")]
    T30day,
    #[serde(rename = "360")]
    T360day,
    #[serde(rename = "90")]
    T90day,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccData {
    /// API Key
    pub api_key: String,
    /// API Secret Key
    pub api_secret: String,
    /// API Version
    pub api_version: i64,
    /// Time of event
    pub created_at: i64,
    /// IP whitelist
    pub ip_whitelist: Option<String>,
    /// Password
    pub passphrase: String,
    /// [Permissions](/docs-new/introduction)
    pub permission: String,
    /// Remarks
    pub remark: String,
    /// Sub-account name
    pub sub_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccListData {
    /// Current request page
    pub current_page: i64,
    pub items: Vec<SubAccItem>,
    /// Number of results per request. Minimum is 1, maximum is 100
    pub page_size: i64,
    /// Total number of messages
    pub total_num: i64,
    /// Total number of pages
    pub total_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccItem {
    /// Sub-account Permission
    pub access: String,
    /// Time of event
    pub created_at: i64,
    pub hosted_status: String,
    /// Sub-account active permissions: If you do not have the corresponding permissions, you
    /// must log in to the sub-account and go to the corresponding web page to activate.
    pub opened_trade_types: Vec<String>,
    /// Remarks
    pub remarks: String,
    /// Sub-account; 2:Enable, 3:Frozen
    pub status: i64,
    /// Sub-account name
    pub sub_name: String,
    /// Sub-account Permissions
    pub trade_types: Vec<String>,
    /// Sub-account type
    #[serde(rename = "type")]
    pub item_type: i64,
    /// Sub-account UID
    pub uid: i64,
    /// Sub-account User ID
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccBalance {
    /// Funding Account
    pub main_accounts: Vec<MainAccount>,
    /// Margin Account
    pub margin_accounts: Vec<MarginAccount>,
    /// The username of a sub-user.
    pub sub_name: String,
    /// The user ID of a sub-user.
    pub sub_user_id: String,
    /// Spot Account
    pub trade_accounts: Vec<TradeAccount>,
    /// This param is deprecated and only valid for some old users
    #[serde(rename = "tradeHFAccounts")]
    pub trade_hf_accounts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainAccount {
    /// Funds available to withdraw or trade.
    pub available: Option<String>,
    /// Total funds in an account.
    pub balance: Option<String>,
    /// The base currency amount.
    pub base_amount: Option<String>,
    /// Calculated on this currency.
    pub base_currency: Option<String>,
    /// The base currency price.
    pub base_currency_price: Option<String>,
    /// Currency
    pub currency: Option<String>,
    /// Funds on hold (not available for use).
    pub holds: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAccount {
    /// Funds available to withdraw or trade.
    pub available: Option<String>,
    /// Total funds in an account.
    pub balance: Option<String>,
    /// The base currency amount.
    pub base_amount: Option<String>,
    /// Calculated on this currency.
    pub base_currency: Option<String>,
    /// The base currency price.
    pub base_currency_price: Option<String>,
    /// Currency
    pub currency: Option<String>,
    /// Funds on hold (not available for use).
    pub holds: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeAccount {
    /// Funds available to withdraw or trade.
    pub available: Option<String>,
    /// Total funds in an account.
    pub balance: Option<String>,
    /// The base currency amount.
    pub base_amount: Option<String>,
    /// Calculated on this currency.
    pub base_currency: Option<String>,
    /// The base currency price.
    pub base_currency_price: Option<String>,
    /// Currency
    pub currency: Option<String>,
    /// Funds on hold (not available for use).
    pub holds: Option<String>,
    pub tag: Option<String>,
}
