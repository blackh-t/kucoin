use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferData {
    /// Transfer order ID
    pub order_id: String,
}
