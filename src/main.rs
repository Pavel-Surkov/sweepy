use anyhow::{Context, Result};
use clap::Parser;

use sweepy::cli::{Cli, Commands};
use sweepy::scanner::{
    bytes_to_gb, bytes_to_mb, find_project_roots, get_last_modification_timestamp,
    get_removable_space_bytes,
};
use sweepy::validation::validate_workspace_path;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            validate_workspace_path(&path)
                .with_context(|| format!("Invalid workspace path: {}", path.display()))?;

            let project_roots = find_project_roots(&path);
            let mut total_removable_space_bytes: u64 = 0;

            for root_buf in project_roots {
                let Some(project_name) = root_buf.file_name() else {
                    continue;
                };

                let removable_space_bytes = get_removable_space_bytes(&root_buf);
                total_removable_space_bytes += removable_space_bytes;

                let Ok(Some(last_mtime)) = get_last_modification_timestamp(&root_buf) else {
                    eprintln!(
                        "failed to get last modification timestamp for {}",
                        project_name.to_string_lossy()
                    );
                    continue;
                };

                println!(
                    "dir_name: {}; removable_space: {} MiB, last_modified: {}",
                    project_name.to_string_lossy(),
                    bytes_to_mb(removable_space_bytes),
                    last_mtime
                );
            }

            println!(
                "\nTotal removable space: around {:.2} GiB",
                bytes_to_gb(total_removable_space_bytes)
            );
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
