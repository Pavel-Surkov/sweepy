use serde::Deserialize;
use std::path::PathBuf;
use std::sync::LazyLock;

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
        ProjectTemplate {
            name: "Elixir".to_string(),
            mark: "mix.exs".to_string(),
            dirs_to_clear: vec!["_build".to_string(), "deps".to_string()],
        },
        ProjectTemplate {
            name: "Zig".to_string(),
            mark: "build.zig".to_string(),
            dirs_to_clear: vec![".zig-cache".to_string(), "zig-out".to_string()],
        },
        ProjectTemplate {
            name: "Maven".to_string(),
            mark: "pom.xml".to_string(),
            dirs_to_clear: vec!["target".to_string()],
        },
        ProjectTemplate {
            name: "Gradle".to_string(),
            mark: "build.gradle".to_string(),
            dirs_to_clear: vec!["build".to_string(), ".gradle".to_string()],
        },
        ProjectTemplate {
            name: "Swift".to_string(),
            mark: "Package.swift".to_string(),
            dirs_to_clear: vec![".build".to_string()],
        },
        // Logically, ".git" IS a project marker, BUT no need to include it
        // because none of directories will be removed, so it shouldn't be counted as a project
    ]
});
