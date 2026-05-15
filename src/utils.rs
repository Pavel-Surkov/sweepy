use std::io::{self, Write};
use std::time::SystemTime;

use anyhow::{Result, bail};
use std::path::Path;

pub fn validate_workspace_path(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("Path does not exists: {}", path.display());
    }

    if !path.is_dir() {
        bail!("Path is not a directory: {}", path.display());
    }

    Ok(())
}

pub fn prompt(label: &str) -> Result<String> {
    print!("{label}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

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

pub fn is_valid_dir_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}
