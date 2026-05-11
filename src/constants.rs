pub const PROJECT_ROOT_MARKERS: &[&str] = &[
    ".git",
    "package.json",
    "Cargo.toml",
    "pnpm-lock.yaml",
    "yarn.lock",
    "package-lock.json",
    "bun.lockb",
];

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
