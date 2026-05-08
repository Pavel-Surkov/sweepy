use std::time::SystemTime;

pub fn bytes_to_mb(bytes: u64) -> u64 {
    bytes / (1024 * 1024)
}

pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024 * 1024 * 1024) as f64
}

pub fn system_time_to_unix_secs(t: SystemTime) -> Option<i64> {
    let secs = t.duration_since(SystemTime::UNIX_EPOCH).ok()?.as_secs();
    i64::try_from(secs).ok()
}

fn get_days_from_secs(secs: u64) -> u64 {
    secs / (60 * 60 * 24)
}

pub fn get_days_since(ts: i64) -> u64 {
    let now_ts = system_time_to_unix_secs(SystemTime::now()).unwrap_or(0);
    let elapsed = (now_ts - ts).max(0) as u64;
    get_days_from_secs(elapsed)
}
