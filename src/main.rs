use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

use sweepy::cleaner::{get_projects_to_clear, remove_all_removable_dirs};
use sweepy::cli::{Cli, Commands};
use sweepy::config::{add_new_language, build_default_config, find_or_create_config};
use sweepy::scanner::{
    find_project_roots, get_last_modification_timestamp, get_removable_space_bytes,
};
use sweepy::utils::{self, validate_workspace_path};

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
                let Some(project_name) = root_buf.path.file_name() else {
                    continue;
                };

                let removable_space_bytes = get_removable_space_bytes(&root_buf);
                total_removable_space_bytes += removable_space_bytes;

                let Some(last_mtime) = get_last_modification_timestamp(&root_buf.path) else {
                    continue;
                };

                let days_since_last_modification = utils::get_days_since(last_mtime);
                // TODO: Remove hardcoded 180d and add CLI option --older-than
                let days_since_last_modification = if days_since_last_modification > 180 {
                    days_since_last_modification.to_string().red()
                } else {
                    days_since_last_modification.to_string().white()
                };

                println!(
                    "| {:<35} | {:>6} MiB | {:>6} days ago |",
                    project_name.to_string_lossy().white(),
                    utils::bytes_to_mb(removable_space_bytes)
                        .to_string()
                        .white(),
                    days_since_last_modification
                );
            }

            let total = format!("{:.2}", utils::bytes_to_gb(total_removable_space_bytes)).red();

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
            let projects_to_clear = get_projects_to_clear(&project_roots, &older_than);

            remove_all_removable_dirs(projects_to_clear, apply);
        }
        Commands::Config {
            add_language,
            reset,
            print_path,
        } => {
            let config_pb = find_or_create_config().expect("Failed to find the config");

            if print_path {
                println!("\nPATH TO THE CONFIGURATION FILE: {}", config_pb.display());
            }

            if reset && config_pb.exists() {
                fs::write(&config_pb, build_default_config()?).with_context(|| {
                    format!("Failed to reset config at {}", config_pb.display())
                })?;
                println!("{}", "Config is successfully reset to defaults".green());
            }

            if add_language {
                add_new_language(&config_pb)?;
            }
        }
    }

    Ok(())
}
