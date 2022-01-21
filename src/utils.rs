use std::time;
pub fn now_sec() -> u64 {
    time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_secs()
}
