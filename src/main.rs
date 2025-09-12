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
use cli_rust::{Cli, Config, ContextManager};

#[allow(deprecated)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = Config {
        root_path: cli.path,
        output_file: cli.output,
        include_patterns: cli.include.unwrap_or_default(),
        exclude_patterns: cli.exclude.unwrap_or_default(),
        is_recursive: cli.recursive,
    };

    println!("Config: {:?}", config);

    let mut manager = ContextManager::new(config);
    manager.build_context()?;

    manager.generate_output()?;

    Ok(())
}
