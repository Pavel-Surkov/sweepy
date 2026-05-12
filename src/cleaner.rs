use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use anyhow::{Ok, Result, anyhow, bail};
use clap::builder::OsStr;
use colored::Colorize;

use crate::constants;
use crate::units::system_time_to_unix_secs;

// Returns unix timestamp for older_than relatively to SystemTime::now()
pub fn get_older_than_unix(older_than: &String) -> Result<i64> {
    let unit = older_than
        .chars()
        .last()
        .ok_or_else(|| anyhow!("older_than value is empty"))?;

    if !constants::ALLOWED_TIME_UNITS.contains(&unit) {
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

fn remove_removable_dirs(path: &PathBuf, is_apply: bool) {
    constants::DIRS_TO_CLEAR
        .iter()
        .filter(|rd| path.join(rd).is_dir())
        .map(|rd| path.join(rd))
        .for_each(|pb| {
            if is_apply {
                if let Err(e) = fs::remove_dir_all(&pb) {
                    eprintln!("Warning: failed to remove {}: {e}", pb.display());
                }
            } else {
                // Dry run
                println!("[dry-run] would remove: {}", pb.display());
            }
        });
}

pub fn remove_all_removable_dirs(paths: Vec<&PathBuf>, is_apply: bool) {
    if !is_apply {
        println!();
        println!("{}", "Directories to be removed:".red());
    }

    paths.iter().for_each(|pb| {
        let fallback = &OsStr::from("unknown");
        let project_name = pb.file_name().unwrap_or(fallback).to_string_lossy();

        if is_apply {
            println!("Cleaning project: {}", project_name.white());
        } else {
            println!("[dry-run] Cleaning project: {}", project_name.white());
        }

        remove_removable_dirs(pb, is_apply);
    });

    if !is_apply {
        println!("{}", "—".repeat(30).red());
    }
}
