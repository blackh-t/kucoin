use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotCanceledData {
    /// The size you canceled
    pub cancel_size: String,
    /// order id
    pub order_id: String,
}
