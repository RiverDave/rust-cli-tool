# Simple CI/CD Pipeline

This document explains the basic CI/CD setup for the cli-rust project.

## What It Does

The CI/CD pipeline automatically:
- **Tests your code** when you push changes
- **Checks code quality** with formatting and linting
- **Builds your project** to make sure it compiles
- **Creates releases** when you tag a version

## How It Works

### 1. CI Workflow (`.github/workflows/ci.yml`)

**When it runs:** Every time you push to the `main` branch or create a pull request

**What it does:**
1. Installs Rust with formatting and linting tools
2. Checks if your code is properly formatted
3. Runs lints to catch potential issues
4. Runs all tests
5. Builds the project

### 2. Release Workflow (`.github/workflows/release.yml`)

**When it runs:** When you create a version tag (like `v1.0.0`)

**What it does:**
1. Installs Rust
2. Builds the release version
3. Creates a GitHub release with the built file

## How to Use

### Running Checks Locally

```bash
# Check everything
make check

# Or run individual commands
cargo fmt -- --check    # Check formatting
cargo clippy           # Run lints
cargo test             # Run tests
cargo build --release  # Build project
```

### Creating a Release

```bash
# Tag a new version
git tag v1.0.0
git push origin v1.0.0

# GitHub will automatically create a release
```

## Files Created

- `.github/workflows/ci.yml` - Main CI workflow
- `.github/workflows/release.yml` - Release workflow
- `.github/dependabot.yml` - Dependency updates
- `Makefile` - Simple commands for development
- `clippy.toml` - Basic linting rules
- `.pre-commit-config.yaml` - Pre-commit hooks (optional)

That's it! Simple and easy to understand. 🚀