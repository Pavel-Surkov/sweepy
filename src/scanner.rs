use anyhow::{Context, Result, anyhow, bail};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use toml;
use walkdir::{DirEntry, WalkDir};

use crate::cleaner::ALLOWED_TIME_UNITS;
use crate::config::find_or_create_config;
use crate::constants::{self, ProjectInfo, ProjectTemplate, SweepyConfig};
use crate::utils::system_time_to_unix_secs;

fn try_project_info_for(entry: &DirEntry) -> Option<ProjectInfo> {
    let config_path = find_or_create_config();

    // Tries to create template from config file
    // If error, create from default PROJECT_ROOT_MARKERS
    let template: Option<ProjectTemplate> = match config_path {
        Ok(pb) => {
            let content = fs::read_to_string(&pb).ok()?;
            let config = toml::from_str::<SweepyConfig>(&content).ok()?;

            config
                .language
                .into_iter()
                .find(|m| entry.path().join(&m.mark).exists())
        }
        Err(_) => constants::PROJECT_ROOT_MARKERS
            .iter()
            .find(|m| entry.path().join(&m.mark).exists())
            .cloned(),
    };

    // Transform ProjectTemplate into ProjectInfo
    template.map(|v| ProjectInfo {
        path: entry.path().to_path_buf(),
        template: v.clone(),
    })
}

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

pub fn find_project_roots(path_buf: &PathBuf, older_than: &String) -> Result<Vec<ProjectInfo>> {
    let cutoff = get_older_than_unix(older_than)?;
    let mut iterator: walkdir::IntoIter = WalkDir::new(path_buf).into_iter();
    let mut project_roots: Vec<ProjectInfo> = vec![];

    while let Some(entry) = iterator.next() {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_dir() {
            continue;
        }

        let Some(project_info) = try_project_info_for(&entry) else {
            continue;
        };

        // Do not propagate into folders like node_modules, target etc.
        iterator.skip_current_dir();

        let Some(last_mtime) = get_last_modification_timestamp(entry.path()) else {
            continue;
        };

        if last_mtime <= cutoff {
            project_roots.push(project_info);
        }
    }

    Ok(project_roots)
}

pub fn get_last_modification_timestamp(path: &Path) -> Option<i64> {
    // if .git is available, get last commit timestamp via git cli
    let ts: Option<i64> = if path.join(".git").is_dir() {
        let Ok(output) = Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("log")
            .arg("-1")
            .arg("--format=%ct")
            .output()
            .with_context(|| format!("failed to run git in {}", path.display()))
        else {
            return None;
        };

        if !output.status.success() {
            return None; // not a git repo or no commits
        }

        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if raw.is_empty() {
            return None;
        }

        raw.parse::<i64>()
            .context("failed to parse git commit timestamp")
            .ok()
    } else {
        // else get metadata last modified timestamp
        let Ok(metadata) = path.metadata() else {
            return None;
        };

        metadata
            .modified()
            .ok()
            .and_then(system_time_to_unix_secs)
            .ok_or_else(|| anyhow!("failed to get filesystem timestamp for {}", path.display()))
            .ok()
    };

    ts
}

fn get_dir_size_bytes(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok().map(|m| m.len()))
        .sum()
}

pub fn get_removable_space_bytes(pi: &ProjectInfo) -> u64 {
    let mut total = 0u64;
    for d in &pi.template.dirs_to_clear {
        let dir_path = pi.path.join(d);
        if let Ok(md) = dir_path.symlink_metadata()
            && md.is_dir()
        {
            total += get_dir_size_bytes(&dir_path);
        }
    }

    total
}
