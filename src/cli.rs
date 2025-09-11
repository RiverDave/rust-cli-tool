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


pub struct Cli {
    /// Paths to analyze
    pub paths: Vec<String>,
    
    /// Output file (default: stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Exclude dir/file patterns
    #[arg(short = 'e', long = "exclude")]
    pub exclude: Option<Vec<String>>,

    /// Include dir/file patterns
    #[arg(short = 'i', long = "include")]
    pub include: Option<Vec<String>>,
}