use std::os::unix::fs::MetadataExt;

use anyhow::{Context, Result};
use clap::Parser;

use sweepy::cli::{Cli, Commands};
use sweepy::scanner::{find_project_roots, get_last_modification_ts};
use sweepy::validation::validate_workspace_path;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            validate_workspace_path(&path)
                .with_context(|| format!("Invalid workspace path: {}", path.display()))?;

            let project_roots = find_project_roots(&path);
            for root_buf in project_roots {
                let Some(project_name) = root_buf.file_name() else {
                    continue;
                };

                // FIXME: Now returns folder size itself, but not the size of all files inside
                let project_size_mb = match root_buf.metadata() {
                    Ok(m) => m.size() as f64 / 1024.0 / 1024.0,
                    Err(err) => {
                        eprintln!(
                            "failed to read metadata for {}: {err}",
                            project_name.to_string_lossy()
                        );
                        continue;
                    }
                };

                let Ok(Some(last_mtime)) = get_last_modification_ts(&root_buf) else {
                    eprintln!(
                        "failed to get last modification timestamp for {}",
                        project_name.to_string_lossy()
                    );
                    continue;
                };

                println!(
                    "dir_name: {}; dir_size: {project_size_mb:.3} MiB, last_modified: {}",
                    project_name.to_string_lossy(),
                    last_mtime
                );
            }
        }
        Commands::Clean {
            path,
            older_than,
            apply,
        } => {
            println!(
                "clean={}, older_than={}, apply={}",
                path.display(),
                older_than,
                apply
            );
        }
    }

    Ok(())
}
