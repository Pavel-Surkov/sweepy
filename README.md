# sweepy

> Find and free disk space by removing stale build artifacts across your projects.

`sweepy` walks a directory of projects, identifies which ones haven't been touched in a while, and removes their generated build directories — safely. By default it always does a dry run; you have to explicitly pass `--apply` to delete anything.

Supports Rust and Node.js projects.

## Features

- **Smart activity detection** — uses last git commit timestamp when available, falls back to filesystem mtime
- **Multi-ecosystem** — detects Rust (`target/`), Node.js (`node_modules/`, `dist/`, `.next/`, `.vite/`, `.cache/`, `coverage/`) projects
- **Dry-run by default** — prints exactly what would be removed before touching anything
- **Colored table output** — projects inactive for 180+ days are highlighted in red
- **Flexible time syntax** — `90d`, `6m`, `2y`

## Supported languages

| Language | Detected by | Directories removed |
|----------|-------------|---------------------|
| 🦀 Rust | `Cargo.toml` | `target` |
| 🟩 Node.js | `package.json` | `node_modules`, `dist`, `build`, `.next`, `.nuxt`, `.cache`, `.vite`, `coverage`, `out` |

Nested projects are not double-counted — traversal stops at the first project root found in each subtree.

## Installation

```bash
cargo install sweepy
```

Or build from source:

```bash
git clone https://github.com/Pavel-Surkov/sweepy
cd sweepy
cargo build --release
cp target/release/sweepy ~/.local/bin/
```

## Usage

### Scan a workspace

List all projects and how much reclaimable space their build artifacts occupy:

```
sweepy scan ~/projects
```

```
——————————————————————————————————————————————————————————————————————
| Project                             |       Size |   Last modified |
——————————————————————————————————————————————————————————————————————
| my-api                              |    342 MiB |     12 days ago |
| old-side-project                    |    891 MiB |    203 days ago |
| sweepy                              |     64 MiB |      0 days ago |
——————————————————————————————————————————————————————————————————————

▶ Total removable space: ~ 1.27 GiB
```

Projects not modified in over 180 days are shown in red.

### Clean stale projects (dry run)

Preview what would be removed for projects inactive for 90 days or more:

```bash
sweepy clean ~/projects --older-than 90d
```

### Apply the cleanup

Once you're satisfied with the dry-run output, add `--apply` to actually delete the directories:

```bash
sweepy clean ~/projects --older-than 90d --apply
```

> [!CAUTION]
> `--apply` permanently deletes build directories. Only known generated directories are ever removed — source files are never touched.

### Time format

The `--older-than` flag accepts values in days (`d`), months (`m`), or years (`y`):

| Value  | Meaning       |
|--------|---------------|
| `90d`  | 90 days       |
| `6m`   | 6 months      |
| `2y`   | 2 years       |

Default is `180d`.
