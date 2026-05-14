use serde::Deserialize;
use std::path::PathBuf;
use std::sync::LazyLock;

// pub enum SupportedLanguage {
//     Rust,
//     NodeJS,
//     Php,
//     Unknown,
// }

// impl SupportedLanguage {
//     pub fn as_str(&self) -> &str {
//         match self {
//             SupportedLanguage::Unknown => "unknown",
//             SupportedLanguage::Rust => "Rust",
//             SupportedLanguage::Php => "PHP",
//             SupportedLanguage::NodeJS => "Node.js",
//         }
//     }
// }
// pub enum Mark {
//     CargoToml,
//     PackageJson,
//     Composer,
//     Git,
// }

// impl Mark {
//     pub fn as_str(&self) -> &str {
//         match self {
//             Mark::Git => ".git",
//             Mark::CargoToml => "Cargo.toml",
//             Mark::Composer => "composer.json",
//             Mark::PackageJson => "package.json",
//         }
//     }
// }

#[derive(Deserialize, Debug)]
pub struct SweepyConfig {
    pub language: Vec<ProjectTemplate>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectTemplate {
    pub mark: String,
    pub name: String,
    pub dirs_to_clear: Vec<String>,
}

pub struct ProjectInfo {
    pub template: ProjectTemplate,
    pub path: PathBuf,
}

// TODO: Think of refactoring this part
pub static PROJECT_ROOT_MARKERS: LazyLock<Vec<ProjectTemplate>> = LazyLock::new(|| {
    vec![
        ProjectTemplate {
            name: "Rust".to_string(),
            mark: "Cargo.toml".to_string(),
            dirs_to_clear: vec!["target".to_string()],
        },
        ProjectTemplate {
            name: "Node.js".to_string(),
            mark: "package.json".to_string(),
            dirs_to_clear: vec![
                "node_modules".to_string(),
                "dist".to_string(),
                "build".to_string(),
                ".next".to_string(),
                ".nuxt".to_string(),
                ".cache".to_string(),
                ".vite".to_string(),
                "coverage".to_string(),
                "out".to_string(),
            ],
        },
        ProjectTemplate {
            name: "PHP".to_string(),
            mark: "composer.json".to_string(),
            dirs_to_clear: vec!["vendor".to_string()],
        },
        // Logically, ".git" IS a project marker, BUT no need to include it
        // because none of directories will be removed, so it shouldn't be counted as a project
        // ProjectTemplate {
        //     lang: SupportedLanguage::Unknown,
        //     mark: Mark::Git,
        //     dirs_to_clear: &[],
        // },
    ]
});
