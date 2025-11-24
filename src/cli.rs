//===----------------------------------------------------------------------===//
//
// Copyright (c) 2025 David Rivera
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
// SPDX-License-Identifier: MIT
//
//===----------------------------------------------------------------------===//
//
// This module defines the command-line interface for the repository context packager.
//===----------------------------------------------------------------------===//
//

use clap::Parser;

#[derive(Parser)]
#[command(name = "repo-context")]
#[command(about = "Package repository context for LLMs")]
#[command(version = "Repository Context Packager v0.1.0\nBuilt with Rust")]
/// Main CLI structure for the application.
pub struct Cli {
    /// Target paths/files to process (required)
    #[arg(help = "Files or directories to process", required = true)]
    pub target_paths: Vec<String>,

    /// Toggle Recursive file traversal
    #[arg(short, long, default_value_t = true)] // NOTE: Haven't tested this yet
    pub recursive: bool,

    /// Output file (default: stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Exclude dir/file patterns
    #[arg(short = 'e', long = "exclude")]
    pub exclude: Option<Vec<String>>,

    /// Include dir/file patterns
    #[arg(short = 'i', long = "include")]
    pub include: Option<Vec<String>>,

    /// Only include files modified within the last 7 days
    #[arg(long = "recent")]
    pub recent: bool,
    /// Show line numbers in file content output
    #[arg(short = 'l', long = "line-numbers")]
    pub line_numbers: bool,
}
