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

pub fn get_last_modification_ts(path_buf: &PathBuf) -> Result<Option<i64>> {
    let ts: i64;

    // if .git is available, get last commit timestamp via git cli
    if path_buf.join(".git").exists() {
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
