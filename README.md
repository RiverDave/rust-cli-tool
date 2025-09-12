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

## License

MIT License - see [LICENSE](LICENSE) file.

## Author

[David Rivera](https://github.com/RiverDave)