# Repository Context Packager

A simple Rust CLI tool that packages your codebase for LLMs. It scans directories, shows file contents, and displays a nice tree view of your project structure.

## Features

- 📁 Recursive file scanning with include/exclude patterns
- 🌳 Pretty ASCII tree visualization
- 📝 Extracts text file contents (skips binary files)
- 🔄 Git repository information
- 🎯 Glob pattern filtering

## Installation

### Prerequisites

- [Install Rust](https://rustup.rs/) (1.70+ required)
- Git (optional, for repository features)

### Build and Run

```bash
git clone https://github.com/RiverDave/rust-cli-tool.git
cd rust-cli-tool
cargo build --release
./target/release/cli-rust
```

## Quick Start

```bash
# Scan current directory
./cli-rust .

# Include only source files
./cli-rust . -i "src/*"

# Exclude build artifacts
./cli-rust . -e "target/*" -e "*.log"

# Save to file
./cli-rust . -o output.txt
```

## Command Options

| Option | Description |
|--------|-------------|
| `-i, --include` | Include file patterns (e.g., "src/*") |
| `-e, --exclude` | Exclude file patterns (e.g., "target/*") |
| `-o, --output` | Save to file instead of stdout |
| `-r, --recursive` | Recursive scanning (default: true) |

## Dependencies

| Crate | Purpose |
|-------|---------|
| [clap](https://crates.io/crates/clap) | Command-line parsing |
| [git2](https://crates.io/crates/git2) | Git repository operations |
| [globset](https://crates.io/crates/globset) | Pattern matching |
| [ptree](https://crates.io/crates/ptree) | Tree visualization |
| [chrono](https://crates.io/crates/chrono) | Date/time handling |

## Pattern Matching Semantics

The tool uses `globset` to match patterns against file paths relative to the root you pass.

Rules:

1. Hidden files/directories (starting with `.`) are skipped automatically.
2. Exclude patterns: if any pattern matches a relative path, that file (or directory contents) is skipped.
3. Include patterns: if provided, only files matching at least one include pattern are kept (after exclusion filtering).
4. If no include patterns are supplied, all non-excluded, non-hidden files are considered.
5. Patterns follow standard glob rules: `**` matches across directory boundaries.

Examples:

```bash
# Include only Rust and Markdown sources
./cli-rust . --include 'src/**/*.rs' '**/*.md'

# Exclude build artifacts and logs
./cli-rust . --exclude 'target/**' '**/*.log'

# Combine include + exclude
./cli-rust . --include 'src/**/*.rs' --exclude 'src/generated/**'
```

Gotchas:

- To exclude an entire directory tree, prefer `dir/**` (not just `dir/*`).
- Include patterns use OR logic: any match keeps the file.
- Exclude wins over include (a file matching both is excluded).
- Binary detection is heuristic (null byte scan of first 512 bytes) and such files have no inlined content.

If patterns don’t behave as expected, run with no patterns first to view relative paths, then refine patterns.

## Development

### CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

- **Automated testing** on multiple Rust versions (stable, beta, nightly)
- **Cross-platform builds** for Linux, Windows, and macOS
- **Code quality checks** with rustfmt and clippy
- **Security auditing** with cargo audit
- **Automated releases** when version tags are pushed
- **Dependency updates** via Dependabot

See [CI/CD Documentation](docs/CI_CD.md) for detailed information.

### Local Development

```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run lints
cargo clippy

# Build release binary
cargo build --release
```

Or use the provided Makefile:

```bash
make help    # Show all available commands
make check   # Run all checks
make all     # Run all checks and build
```

## License

MIT License - see [LICENSE](LICENSE) file.

## Author

[David Rivera](https://github.com/RiverDave)