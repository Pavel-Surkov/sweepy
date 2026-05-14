use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use dirs::config_dir;

use crate::constants::PROJECT_ROOT_MARKERS;

pub const CLI_DIR_NAME: &str = "sweepy";
pub const CLI_CONFIG_NAME: &str = "config.toml";

fn build_default_config() -> String {
    let mut s = String::from("# Sweepy configuration file.\n\n");

    for t in PROJECT_ROOT_MARKERS.iter() {
        s.push_str(&format!(
            "[[language]]\nname = \"{}\"\nmark = \"{}\"\ndirs_to_clear = [{}]\n\n",
            t.name,
            t.mark,
            t.dirs_to_clear
                .iter()
                .map(|d| format!("\"{}\"", d))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    s
}

pub fn find_or_create_config() -> Result<PathBuf, Box<dyn std::error::Error>> {
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
