pub enum SupportedLanguage {
    Rust,
    NodeJS,
    Unknown,
}

impl SupportedLanguage {
    pub fn as_str(&self) -> &str {
        match self {
            SupportedLanguage::Unknown => "unknown",
            SupportedLanguage::Rust => "Rust",
            SupportedLanguage::NodeJS => "NodeJS",
        }
    }
}
pub enum Mark {
    Git,
    CargoToml,
    PackageJson,
}

impl Mark {
    pub fn as_str(&self) -> &str {
        match self {
            Mark::Git => ".git",
            Mark::CargoToml => "Cargo.toml",
            Mark::PackageJson => "package.json",
        }
    }
}

pub struct ProjectTemplate {
    pub mark: Mark,
    pub lang: SupportedLanguage,
    pub dirs_to_clear: &'static [&'static str],
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
    // TODO: Think about, maybe no need in Unknown language since it's not clearing anything
    // But anyway, .git IS a project marker
    ProjectTemplate {
        lang: SupportedLanguage::Unknown,
        mark: Mark::Git,
        dirs_to_clear: &[],
    },
];

// TODO: remove
pub const DIRS_TO_CLEAR: &[&str] = &[
    "node_modules",
    ".next",
    "dist",
    ".vite",
    ".cache",
    "coverage",
    "target",
];

pub const ALLOWED_TIME_UNITS: [char; 3] = ['d', 'm', 'y'];
