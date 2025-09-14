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
// Entry point for the repository context packager application.
//
//===----------------------------------------------------------------------===//
//

use clap::Parser;
use cli_rust::{Cli, Config, ContextManager, OutputContext, OutputDestination, OutputFormat};

#[allow(deprecated)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = Config {
        root_path: cli.repo_path,
        output_file: cli.output,
        include_patterns: cli.include.unwrap_or_default(),
        exclude_patterns: cli.exclude.unwrap_or_default(),
        is_recursive: cli.recursive,
    };

    let mut manager = ContextManager::new(config.clone());
    manager.build_context().unwrap_or_else(|e| {
        eprintln!("Error building context: {}", e);
        std::process::exit(1);
    });

    // Parse arguments for output format and destination
    let output_dest = match config.output_file {
        Some(p) => OutputDestination::File(p),
        None => OutputDestination::Stdout,
    };

    OutputContext::new(manager)
        .format(OutputFormat::Markdown)
        .destination(output_dest)
        .generate()?;

    Ok(())
}
