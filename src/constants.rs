use std::path::PathBuf;

pub enum SupportedLanguage {
    Rust,
    NodeJS,
    Php,
    Unknown,
}

impl SupportedLanguage {
    pub fn as_str(&self) -> &str {
        match self {
            SupportedLanguage::Unknown => "unknown",
            SupportedLanguage::Rust => "Rust",
            SupportedLanguage::Php => "PHP",
            SupportedLanguage::NodeJS => "NodeJS",
        }
    }
}
pub enum Mark {
    CargoToml,
    PackageJson,
    Composer,
    Git,
}

impl Mark {
    pub fn as_str(&self) -> &str {
        match self {
            Mark::Git => ".git",
            Mark::CargoToml => "Cargo.toml",
            Mark::Composer => "composer.json",
            Mark::PackageJson => "package.json",
        }
    }
}

pub struct ProjectTemplate {
    pub mark: Mark,
    pub lang: SupportedLanguage,
    pub dirs_to_clear: &'static [&'static str],
}

pub struct ProjectInfo {
    pub template: &'static ProjectTemplate,
    pub path: PathBuf,
}

// TODO: Think of refactoring this part
pub static PROJECT_ROOT_MARKERS: &[ProjectTemplate] = &[
    ProjectTemplate {
        lang: SupportedLanguage::Rust,
        mark: Mark::CargoToml,
        dirs_to_clear: &["target"],
    },
    ProjectTemplate {
        lang: SupportedLanguage::NodeJS,
        mark: Mark::PackageJson,
        dirs_to_clear: &[
            "node_modules",
            "dist",
            "build",
            ".next",
            ".nuxt",
            ".cache",
            ".vite",
            "coverage",
            "out",
        ],
    },
    ProjectTemplate {
        lang: SupportedLanguage::Php,
        mark: Mark::Composer,
        dirs_to_clear: &["vendor"],
    },
    // Logically, ".git" IS a project marker, BUT no need to include it
    // because none of directories will be removed, so it shouldn't be counted as a project
    // ProjectTemplate {
    //     lang: SupportedLanguage::Unknown,
    //     mark: Mark::Git,
    //     dirs_to_clear: &[],
    // },
];

pub const ALLOWED_TIME_UNITS: [char; 3] = ['d', 'm', 'y'];
