use anyhow::{Context, Result};
use clap::Parser;
use sweepy::cli::{Cli, Commands};
use sweepy::validation::validate_workspace_path;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            validate_workspace_path(&path)
                .with_context(|| format!("Invalid workspace path: {}", path.display()))?;

            println!("scan: {}", path.display());
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
