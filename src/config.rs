use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use dirs::config_dir;

pub const CLI_DIR_NAME: &str = "sweepy";
pub const CLI_CONFIG_NAME: &str = "config.toml";

const DEFAULT_CONFIG: &str = "\
# Sweepy configuration file.
# Built-in support for Rust, Node.js, and PHP is always active.

# Disable built-in languages by name.
# disabled_languages = [\"Node.js\"]

# Add custom languages or override disabled ones.
# [[languages]]
# name = \"Python\"
# mark = \"requirements.txt\"
# dirs_to_clear = [\"__pycache__\", \".venv\", \"venv\", \"dist\", \"build\"]
";

pub fn find_or_create_config() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let system_config_pb = config_dir()
        .ok_or_else(|| anyhow!("Could not determine config directory. Is $HOME set?"))?;

    let config_dir_pb = system_config_pb.join(CLI_DIR_NAME);
    let full_config_pb = config_dir_pb.join(CLI_CONFIG_NAME);

    if !full_config_pb.exists() {
        if !config_dir_pb.exists() {
            fs::create_dir(config_dir_pb)?;
        }
        fs::write(&full_config_pb, DEFAULT_CONFIG)?;
    }

    Ok(full_config_pb)
}
