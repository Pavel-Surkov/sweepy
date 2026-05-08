use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "sweepy", version, about = "Find and remove stale build artifacts across your projects")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List projects and how much space their build artifacts use
    Scan {
        /// Root directory to search for projects
        path: PathBuf,
    },
    /// Remove build artifacts from projects that haven't been touched recently
    Clean {
        /// Root directory to search for projects
        path: PathBuf,
        /// Only clean projects with no changes older than this (e.g. 90d, 180d)
        #[arg(long, default_value_t = String::from("180d"))]
        older_than: String,
        /// Actually delete files; omit to do a dry run
        #[arg(long)]
        apply: bool,
    },
}
