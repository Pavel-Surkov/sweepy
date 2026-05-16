# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2026-05-16

### Features
- **scan:** Add --older-than flag to scan only the necessary projects

## [0.3.0] - 2026-05-15

### Features
- **config:** Add config module with find_or_create_config and Config command scaffold

- **config:** Add --reset flag scaffold to Config command

- **config:** Add --print-path flag to Config command

- **config:** Implement --reset to restore default config

- **constants:** Add Elixir, Zig, Maven, Gradle, Swift as built-in defaults

- **config:** Implement interactive --add-language

- **config:** Add prompts and colored feedback to --add-language


### Internal
- Internal code improvements

## [0.2.1] - 2026-05-13

### Features
- **languages:** Add PHP support via composer.json marker


### Internal
- Internal code improvements

## [0.2.0] - 2026-05-12

### Features
- **clean:** Target dirs to remove are now per-language instead of a global list


### Internal
- Internal code improvements

## [0.1.1] - 2026-05-12

### Internal
- Internal code improvements

## [0.1.0] - 2026-05-11

### Bug Fixes
- **scan:** Widen table columns to fit longer project names

- **cliff:** Guard unreleased footer link against missing previous tag


### Features
- **cli:** Add initial scan and clean command skeleton

- **core:** Add workspace path validation in scan flow

- **scan:** Add project root discovery with walkdir

- **scan:** Report removable space per project and total

- **units:** Add days_since helper and consolidate time utils

- **scan:** Pretty-print scan output as colored table

- **clean:** Scaffold clean command with older_than parsing

- **clean:** Implement clean command with dry-run support

- **release:** Enable crates.io publish step


### Internal
- Internal code improvements


### Performance
- **scanner:** Tighten git detection and removable dir checks

[0.3.1]: https://github.com/Pavel-Surkov/sweepy/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Pavel-Surkov/sweepy/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/Pavel-Surkov/sweepy/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Pavel-Surkov/sweepy/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Pavel-Surkov/sweepy/compare/v0.1.0...v0.1.1

