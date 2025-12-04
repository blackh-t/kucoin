use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_millis()
        .to_string()
}
