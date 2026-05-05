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
