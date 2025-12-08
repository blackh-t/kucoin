pub mod deposit_res_type;
pub mod spot_contract_res_type;
pub mod transfer_res_type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KuCoinResponse<T> {
    pub code: String,
    pub msg: Option<String>, // Error message
    pub data: Option<T>,
}
