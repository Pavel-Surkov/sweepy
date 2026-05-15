use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use dirs::config_dir;

use crate::constants::{PROJECT_ROOT_MARKERS, ProjectTemplate, SweepyConfig};
use crate::utils::{is_valid_dir_name, prompt};

pub const CLI_DIR_NAME: &str = "sweepy";
pub const CLI_CONFIG_NAME: &str = "config.toml";

const CONFIG_HEADER: &str = "# Sweepy configuration file.\n\n";

/// Serialize one or more language templates into a TOML fragment of `[[language]]` tables.
/// Using the TOML serializer escapes special characters correctly, so a name
/// or marker containing `"`, `\`, or a newline can't corrupt the file.
fn serialize_languages(language: Vec<ProjectTemplate>) -> Result<String> {
    toml::to_string_pretty(&SweepyConfig { language }).context("Failed to serialize config to TOML")
}

pub fn build_default_config() -> Result<String> {
    let body = serialize_languages(PROJECT_ROOT_MARKERS.clone())?;
    Ok(format!("{CONFIG_HEADER}{body}"))
}

pub fn find_or_create_config() -> Result<PathBuf> {
    let system_config_pb = config_dir()
        .ok_or_else(|| anyhow!("Could not determine config directory. Is $HOME set?"))?;

    let config_dir_pb = system_config_pb.join(CLI_DIR_NAME);
    let full_config_pb = config_dir_pb.join(CLI_CONFIG_NAME);

    if !full_config_pb.exists() {
        if !config_dir_pb.exists() {
            fs::create_dir(config_dir_pb)?;
        }
        fs::write(&full_config_pb, build_default_config()?)?;
    }

    Ok(full_config_pb)
}

fn read_language_entries() -> Result<(String, String, String)> {
    let name = prompt("Language name (e.g. Rust): ")?;
    let mark = prompt("Marker file (e.g. Cargo.toml): ")?;
    let dirs_to_clear = prompt("Dirs to clear (separate with comma e.g. target, dist, ...): ")?;

    Ok((name, mark, dirs_to_clear))
}

fn validate_and_format_dirs_input(input: &str) -> Vec<String> {
    input
        .split(",")
        .filter_map(|s| {
            let trimmed = s.trim();
            if is_valid_dir_name(trimmed) {
                return Some(trimmed.to_string());
            }

            eprintln!(
                "{}",
                format!(
                    "Invalid directory name: '{}'. Allowed symbols: a-z A-Z 0-9 . _ -",
                    trimmed
                )
                .red()
            );
            None
        })
        .collect::<Vec<_>>()
}

pub fn add_new_language(config_pb: &PathBuf) -> Result<()> {
    let (name, mark, dirs_to_clear) = read_language_entries()?;
    let dirs_to_clear = validate_and_format_dirs_input(&dirs_to_clear);

    let new_project_template = ProjectTemplate {
        name,
        mark,
        dirs_to_clear,
    };

    let new_entry = serialize_languages(vec![new_project_template])?;
    let mut config = OpenOptions::new().append(true).open(config_pb)?;

    config
		.write_all(format!("\n{new_entry}").as_bytes())
		.with_context(|| format!("Failed to add new language entries into configuration file.\nYou can add them manually at {}", config_pb.display()))?;

    println!("{}", "New language added successfully".green());

    Ok(())
}
