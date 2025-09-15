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

#[derive(Debug, Clone)]
pub struct ContextManager {
    pub config: Config,
    pub context: Option<RepositoryContext>,
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
    /// Now discovers repo from current working directory and processes specific target paths.
    pub fn build_context(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Discover git repository from the current working directory (root_path)
        let repo: Repository = match Repository::discover(&self.config.root_path) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(format!(
                    "Failed to discover repository from {}: {}",
                    self.config.root_path, e
                )
                .into());
            }
        };

        // Get the actual repository root path
        let actual_repo_root = get_repo_root_path(&repo)?;

        // Build file context with target paths (for CLI) or from root (for tests/empty paths)
        let file_ctx = if self.config.target_paths.is_empty() {
            // If no target paths specified, process the entire repo (for tests and compatibility)
            FileContext::from_root(self.config.clone(), &actual_repo_root)?
        } else {
            // Process only the specified target paths (new CLI behavior)
            FileContext::from_target_paths(self.config.clone(), &actual_repo_root)?
        };

        // Utilize all modules to build the context
        self.context = Some(RepositoryContext {
            root_path: actual_repo_root,
            git_info: git::extract_git_info(&repo)?,
            file_ctx,
        });

        assert!(self.context.is_some());

        Ok(())
    }
}

/// The root path read from git2 links the .git folder. While this is useful for git operations,
/// for our purposes we need the actual root path of the repository. So It's convenient for the user.
fn get_repo_root_path(repo: &Repository) -> Result<String, Box<dyn std::error::Error>> {
    let workdir = repo.workdir().ok_or("Failed to get workdir")?;
    Ok(workdir.to_str().unwrap_or("").to_string())
}
