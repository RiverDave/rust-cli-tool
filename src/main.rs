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
use rusty_repo_context_manager::{
    Cli, Config, ContextManager, OutputContext, OutputDestination, OutputFormat,
};

/// Create a Config from parsed CLI arguments
fn create_config_from_cli(cli: Cli) -> Result<Config, Box<dyn std::error::Error>> {
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
    let root_path = current_dir
        .to_str()
        .ok_or("Failed to convert current directory to string")?
        .to_string();

    Ok(Config {
        root_path,
        target_paths: cli.target_paths,
        output_file: cli.output,
        include_patterns: cli.include.unwrap_or_default(),
        exclude_patterns: cli.exclude.unwrap_or_default(),
        is_recursive: cli.recursive,
        recent_only: cli.recent,
        show_line_numbers: cli.line_numbers,
    })
}

/// Determine output destination from config
fn determine_output_destination(config: &Config) -> OutputDestination {
    match &config.output_file {
        Some(path) => OutputDestination::File(path.clone()),
        None => OutputDestination::Stdout,
    }
}

#[allow(deprecated)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = create_config_from_cli(cli)?;

    let mut manager = ContextManager::new(config.clone());
    manager.build_context().unwrap_or_else(|e| {
        eprintln!("Error building context: {}", e);
        std::process::exit(1);
    });

    let output_dest = determine_output_destination(&config);

    OutputContext::new(manager)
        .format(OutputFormat::Markdown)
        .destination(output_dest)
        .generate()?;

    Ok(())
}
