use anyhow::{Context, Result, anyhow};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml;
use walkdir::{DirEntry, WalkDir};

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
    match template {
        Some(v) => Some(ProjectInfo {
            path: entry.path().to_path_buf(),
            template: v.clone(),
        }),
        None => None,
    }
}

pub fn find_project_roots(path_buf: &PathBuf) -> Vec<ProjectInfo> {
    let mut iterator: walkdir::IntoIter = WalkDir::new(path_buf).into_iter();
    let mut project_roots: Vec<ProjectInfo> = vec![];

    while let Some(entry) = iterator.next() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_dir() {
            continue;
        }

        let project_template = try_project_info_for(&entry);
        match project_template {
            Some(v) => {
                project_roots.push(v);
                iterator.skip_current_dir();
            }
            None => continue,
        }
    }

    project_roots
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
