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
    #[deprecated(
        note = "This is a placeholder method and will be replaced with proper output handling."
    )]
    pub fn generate_output(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.context
            .as_ref()
            .unwrap()
            .file_ctx
            .file_entries
            .iter()
            .for_each(|f| {
                println!("Discovered File: {:?}", f.path);
            });

        Ok(())
    }
}
