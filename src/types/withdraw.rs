// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Model;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Model = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRequest {
    /// Withdrawal amount, a positive number which is a multiple of the amount precision
    pub amount: String,
    /// The chainId of currency, For a currency with multiple chains, it is recommended to
    /// specify the chain parameter instead of using the default chain; you can query the chainId
    /// through the response of the GET /api/v3/currencies/{currency} interface.
    pub chain: Option<String>,
    /// currency
    pub currency: String,
    /// Withdrawal fee deduction type: INTERNAL, EXTERNAL, or not specified
    ///
    /// 1. INTERNAL: Deduct the transaction fees from your withdrawal amount
    /// 2. EXTERNAL: Deduct the transaction fees from your main account
    /// 3. If you don't specify the feeDeductType parameter, when the balance in your main
    /// account is sufficient to support the withdrawal, the system will initially deduct the
    /// transaction fees from your main account. But if the balance in your main account is not
    /// sufficient to support the withdrawal, the system will deduct the fees from your
    /// withdrawal amount. For example: Suppose you are going to withdraw 1 BTC from the KuCoin
    /// platform (transaction fee: 0.0001BTC), if the balance in your main account is
    /// insufficient, the system will deduct the transaction fees from your withdrawal amount. In
    /// this case, you will be receiving 0.9999BTC.
    pub fee_deduct_type: Option<String>,
    /// Internal withdrawal or not. Default: False
    pub is_inner: Option<bool>,
    /// Address remark. If there’s no remark, it is empty. When you withdraw from other platforms
    /// to KuCoin, you need to fill in memo(tag). Be careful: If you do not fill in memo(tag),
    /// your deposit may not be available.
    pub memo: Option<String>,
    /// Remark
    pub remark: Option<String>,
    /// Withdrawal address
    pub to_address: String,
    /// Withdrawal type, ADDRESS (withdrawal address), UID, MAIL (email), PHONE (mobile phone
    /// number). Note: If you withdraw by uid/mail/phone, there will be rate limits: 3 times/10
    /// seconds, 50 times/24 hours (calculated on a rolling basis based on the first request time)
    pub withdraw_type: WithdrawType,
}

/// Withdrawal type, ADDRESS (withdrawal address), UID, MAIL (email), PHONE (mobile phone
/// number). Note: If you withdraw by uid/mail/phone, there will be rate limits: 3 times/10
/// seconds, 50 times/24 hours (calculated on a rolling basis based on the first request time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WithdrawType {
    #[serde(rename = "ADDRESS")]
    Address,
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "PHONE")]
    Phone,
    #[serde(rename = "UID")]
    Uid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawResponse {
    /// Withdrawal id, a unique ID for a withdrawal
    pub withdrawal_id: String,
}
