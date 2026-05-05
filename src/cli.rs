use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "sweepy", version, about = "Safe stale workspace cleaner")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Scan {
        path: PathBuf,
    },
    Clean {
        path: PathBuf,
        #[arg(long, default_value_t = String::from("180d"))]
        older_than: String,
        #[arg(long, default_value_t = false)]
        apply: bool,
    },
}
