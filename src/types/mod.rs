pub mod deposit;
pub mod spot;
pub mod sup_account;
pub mod transfer;
pub mod withdraw;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KuCoinResponse<T> {
    pub code: String,
    pub msg: Option<String>, // Error message
    pub data: Option<T>,
}
