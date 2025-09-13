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
// This module provides functionality to build the repository context.
//
//===----------------------------------------------------------------------===//
//

use std::io::Write;

use crate::git;
use crate::types::*;
use git2::Repository;

pub struct ContextManager {
    config: Config,
    context: Option<RepositoryContext>,
}

impl ContextManager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            context: None,
        }
    }

    /// This is the heart of our implementation.
    /// Build the repository context by gathering information from git and the filesystem.
    /// This function initializes the context and populates it with relevant data.
    pub fn build_context(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // We need to initiate the interaction with libgit2 in the root path
        let repo: Repository = match Repository::open(&self.config.root_path) {
            Ok(repo) => repo,
            Err(e) => return Err(format!("Failed to open repository: {}", e).into()),
        };

        // Utilize all modules to build the context
        self.context = Some(RepositoryContext {
            root_path: repo.path().to_str().unwrap_or("").to_string(),
            git_info: git::extract_git_info(&repo)?,
            file_ctx: FileContext::from_root(self.config.clone(), &self.config.root_path)?,
        });

        assert!(self.context.is_some());

        Ok(())
    }

    /// Generate output based on the built context (This is to be replaced with proper output handling)
    /// Pretty printing perhaps?
    pub fn generate_output(&self, config: Config) -> Result<(), Box<dyn std::error::Error>> {
        // Get the files from the context
        // (This looks horrible, might need to refactor)
        let files = self
            .context
            .as_ref()
            .map(|ctx| &ctx.file_ctx.file_entries)
            .into_iter()
            .flatten();

        // If an output file is provided, write to it
        if let Some(output_file) = &config.output_file {
            let mut file_buffer = std::fs::File::create(output_file).unwrap_or_else(|err| {
                eprintln!("Error creating output file: {}", err);
                std::process::exit(1);
            });
            for file in files {
                _ = writeln!(file_buffer, "File Discovered: {}", file.path);
            }
        } else {
            // Otherwise, print to stdout
            for file in files {
                println!("File Discovered: {}", file.path);
            }
        }

        Ok(())
    }
}
