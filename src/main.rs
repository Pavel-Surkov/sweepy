use clap::Parser;
use sweepy::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
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
}
