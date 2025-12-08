pub mod deposit;
pub mod spot;
pub mod transfer;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KuCoinResponse<T> {
    pub code: String,
    pub msg: Option<String>, // Error message
    pub data: Option<T>,
}
