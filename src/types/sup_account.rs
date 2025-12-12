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
