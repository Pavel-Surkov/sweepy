pub fn bytes_to_mb(bytes: u64) -> u64 {
    bytes / 1024 / 1024
}

pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}
