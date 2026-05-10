use std::time::SystemTime;

use anyhow::{Ok, Result, anyhow, bail};

use crate::units::system_time_to_unix_secs;

const ALLOWED_TIME_UNITS: [char; 3] = ['d', 'm', 'y'];

// Returns unix timestamp for older_than relatively to SystemTime::now()
pub fn get_older_than_unix(older_than: &String) -> Result<i64> {
    let unit = older_than
        .chars()
        .last()
        .ok_or_else(|| anyhow!("older_than value is empty"))?;

    if !ALLOWED_TIME_UNITS.contains(&unit) {
        if unit.is_ascii_digit() {
            bail!(
                "No time unit provided in '{}': expected d, m or y",
                older_than
            );
        } else {
            bail!(
                "Unexpected time unit '{}' in '{}': expected d, m or y",
                unit,
                older_than
            );
        }
    } else {
        let count = older_than[..older_than.len() - 1]
            .parse::<i64>()
            .map_err(|_| {
                anyhow!(
                    "Invalid number in '{}': expected format like 180d, 6m, 2y",
                    older_than
                )
            })?;

        let unix_secs = match unit {
            'd' => count * 60 * 60 * 24,
            'm' => count * 60 * 60 * 24 * 30,
            'y' => count * 60 * 60 * 24 * 365,
            _ => unreachable!(),
        };

        let now = system_time_to_unix_secs(SystemTime::now()).unwrap_or(0);
        Ok(now - unix_secs)
    }
}
