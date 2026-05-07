use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::{DirEntry, WalkDir};

const PROJECT_ROOT_MARKERS: &[&str] = &[
    ".git",
    "package.json",
    "Cargo.toml",
    "pnpm-lock.yaml",
    "yarn.lock",
    "package-lock.json",
    "bun.lockb",
];

const DIRS_TO_CLEAR: &[&str] = &[
    "node_modules",
    ".next",
    "dist",
    ".vite",
    ".cache",
    "coverage",
    "target",
];

// resolves an entry path is a git/cargo/npm project
fn is_project_root(entry: &DirEntry) -> bool {
    PROJECT_ROOT_MARKERS
        .iter()
        .any(|m| entry.path().join(m).exists())
}

pub fn find_project_roots(path: &PathBuf) -> Vec<std::path::PathBuf> {
    let mut iterator: walkdir::IntoIter = WalkDir::new(path).into_iter();
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

    return project_roots;
}

fn system_time_to_unix_secs(t: SystemTime) -> Option<i64> {
    let secs = t.duration_since(UNIX_EPOCH).ok()?.as_secs();
    i64::try_from(secs).ok()
}

pub fn get_last_modification_timestamp(path_buf: &PathBuf) -> Result<Option<i64>> {
    let ts: i64;

    // if .git is available, get last commit timestamp via git cli
    if path_buf.join(".git").is_dir() {
        let output = Command::new("git")
            .arg("-C")
            .arg(path_buf.as_path())
            .arg("log")
            .arg("-1")
            .arg("--format=%ct")
            .output()
            .with_context(|| format!("failed to run git in {}", path_buf.display()))?;

        if !output.status.success() {
            return Ok(None); // not a git repo or no commits
        }

        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if raw.is_empty() {
            return Ok(None);
        }

        ts = raw
            .parse::<i64>()
            .context("failed to parse git commit timestamp")?;
    } else {
        // else get metadata last modified timestamp
        let metadata = path_buf.metadata()?;

        ts = metadata
            .modified()
            .ok()
            .and_then(system_time_to_unix_secs)
            .ok_or_else(|| {
                anyhow!(
                    "failed to get filesystem timestamp for {}",
                    path_buf.display()
                )
            })?;
    }

    Ok(Some(ts))
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
    for d in DIRS_TO_CLEAR {
        let dir_path = path.join(d);
        if let Ok(md) = dir_path.metadata()
            && md.is_dir()
        {
            total += get_dir_size_bytes(&dir_path);
        }
    }

    return total;
}

pub fn bytes_to_mb(bytes: u64) -> u64 {
    bytes / 1024 / 1024
}

pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}
