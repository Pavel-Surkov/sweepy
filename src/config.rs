use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

use anyhow::{Context, Result, anyhow};
use dirs::config_dir;

use crate::constants::{PROJECT_ROOT_MARKERS, ProjectTemplate};
use crate::utils::is_valid_dir_name;

pub const CLI_DIR_NAME: &str = "sweepy";
pub const CLI_CONFIG_NAME: &str = "config.toml";

fn create_config_language_entry(pt: &ProjectTemplate) -> String {
    format!(
        "[[language]]\nname = \"{}\"\nmark = \"{}\"\ndirs_to_clear = [{}]\n\n",
        pt.name,
        pt.mark,
        pt.dirs_to_clear
            .iter()
            .map(|d| format!("\"{}\"", d))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

pub fn build_default_config() -> String {
    let mut s = String::from("# Sweepy configuration file.\n\n");

    for t in PROJECT_ROOT_MARKERS.iter() {
        s.push_str(&create_config_language_entry(t));
    }

    s
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
        fs::write(&full_config_pb, build_default_config())?;
    }

    Ok(full_config_pb)
}

fn read_language_entries() -> Result<(String, String, String)> {
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    let mut mark = String::new();
    io::stdin().read_line(&mut mark)?;

    let mut dirs_to_clear = String::new();
    io::stdin().read_line(&mut dirs_to_clear)?;

    Ok((
        name.trim().to_string(),
        mark.trim().to_string(),
        dirs_to_clear.trim().to_string(),
    ))
}

fn validate_and_format_dirs_input(input: &String) -> Vec<String> {
    input
        .split(",")
        .filter_map(|s| {
            let trimmed = s.trim();
            if is_valid_dir_name(trimmed) {
                return Some(trimmed.to_string());
            }

            eprintln!(
                "Invalid directory name: {}. Allowed symbols: a-z A-Z 0-9 . _ -",
                trimmed
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

    let new_entry = create_config_language_entry(&new_project_template);
    let mut config = OpenOptions::new().append(true).open(&config_pb)?;

    config
		.write_all(new_entry.as_bytes())
		.with_context(|| format!("Failed to add new language entries into configuration file.\nYou can add them manually at {}", config_pb.display()))?;

    Ok(())
}
