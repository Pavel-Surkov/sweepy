use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

use sweepy::cleaner;

use sweepy::cli::{Cli, Commands};
use sweepy::scanner::{
    find_project_roots, get_last_modification_timestamp, get_removable_space_bytes,
};
use sweepy::units;
use sweepy::validation::validate_workspace_path;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            validate_workspace_path(&path)
                .with_context(|| format!("Invalid workspace path: {}", path.display()))?;

            let project_roots = find_project_roots(&path);
            let mut total_removable_space_bytes: u64 = 0;

            println!("{}", "—".repeat(70));
            println!(
                "| {:<35} | {:>10} | {:>15} |",
                "Project".white(),
                "Size".white(),
                "Last modified".white()
            );
            println!("{}", "—".repeat(70));

            for root_buf in project_roots {
                let Some(project_name) = root_buf.file_name() else {
                    continue;
                };

                let removable_space_bytes = get_removable_space_bytes(&root_buf);
                total_removable_space_bytes += removable_space_bytes;

                let Some(last_mtime) = get_last_modification_timestamp(&root_buf) else {
                    continue;
                };

                let days_since_last_modification = units::get_days_since(last_mtime);
                // TODO: Remove hardcoded 180d and add CLI option --older-than
                let days_since_last_modification = if days_since_last_modification > 180 {
                    days_since_last_modification.to_string().red()
                } else {
                    days_since_last_modification.to_string().white()
                };

                println!(
                    "| {:<35} | {:>6} MiB | {:>6} days ago |",
                    project_name.to_string_lossy().white(),
                    units::bytes_to_mb(removable_space_bytes)
                        .to_string()
                        .white(),
                    days_since_last_modification
                );
            }

            let total = format!("{:.2}", units::bytes_to_gb(total_removable_space_bytes)).red();

            println!("{}", "—".repeat(70));
            println!("\n▶ Total removable space: ~ {} GiB\n", total);
        }
        Commands::Clean {
            path,
            older_than,
            apply,
        } => {
            validate_workspace_path(&path)
                .with_context(|| format!("Invalid workspace path: {}", path.display()))?;

            let project_roots = find_project_roots(&path);
            let older_than_unix_ts = cleaner::get_older_than_unix(&older_than)?;

            let projects_to_clear = project_roots
                .iter()
                .filter_map(|pb| {
                    let last_mtime = get_last_modification_timestamp(pb)?;
                    if older_than_unix_ts - last_mtime > 0 {
                        return Some(pb);
                    }

                    None
                })
                .collect();

            cleaner::remove_all_removable_dirs(projects_to_clear, apply);
        }
    }

    Ok(())
}
