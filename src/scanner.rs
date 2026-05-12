use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

use crate::constants;
use crate::units::system_time_to_unix_secs;

// resolves an entry path is a git/cargo/npm project
fn is_project_root(entry: &DirEntry) -> bool {
    constants::PROJECT_ROOT_MARKERS
        .iter()
        .any(|m| entry.path().join(m).exists())
}

pub fn find_project_roots(path_buf: &PathBuf) -> Vec<std::path::PathBuf> {
    let mut iterator: walkdir::IntoIter = WalkDir::new(path_buf).into_iter();
    let mut project_roots: Vec<std::path::PathBuf> = vec![];

    while let Some(entry) = iterator.next() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_dir() {
            continue;
        }

        if is_project_root(&entry) {
            project_roots.push(entry.path().to_path_buf());
            iterator.skip_current_dir();
            continue;
        }
    }

    project_roots
}

pub fn get_last_modification_timestamp(path_buf: &PathBuf) -> Option<i64> {
    let ts: Option<i64>;

    // if .git is available, get last commit timestamp via git cli
    if path_buf.join(".git").is_dir() {
        let Ok(output) = Command::new("git")
            .arg("-C")
            .arg(path_buf.as_path())
            .arg("log")
            .arg("-1")
            .arg("--format=%ct")
            .output()
            .with_context(|| format!("failed to run git in {}", path_buf.display()))
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

        ts = raw
            .parse::<i64>()
            .context("failed to parse git commit timestamp")
            .ok();
    } else {
        // else get metadata last modified timestamp
        let Ok(metadata) = path_buf.metadata() else {
            return None;
        };

        ts = metadata
            .modified()
            .ok()
            .and_then(system_time_to_unix_secs)
            .ok_or_else(|| {
                anyhow!(
                    "failed to get filesystem timestamp for {}",
                    path_buf.display()
                )
            })
            .ok();
    }

    ts
}

fn get_dir_size_bytes(path: &PathBuf) -> u64 {
    WalkDir::new(path.as_path())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok().map(|m| m.len()))
        .sum()
}

pub fn get_removable_space_bytes(path: &PathBuf) -> u64 {
    let mut total = 0u64;
    for d in constants::DIRS_TO_CLEAR {
        let dir_path = path.join(d);
        if let Ok(md) = dir_path.metadata()
            && md.is_dir()
        {
            total += get_dir_size_bytes(&dir_path);
        }
    }

    total
}
