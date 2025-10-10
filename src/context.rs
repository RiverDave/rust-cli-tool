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

use crate::TreeContext;
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

    /// Discover the git repository from the configured root path
    fn discover_repository(&self) -> Result<Repository, Box<dyn std::error::Error>> {
        Repository::discover(&self.config.root_path).map_err(|e| {
            format!(
                "Failed to discover repository from {}: {}",
                self.config.root_path, e
            )
            .into()
        })
    }

    /// Build tree representation based on configuration
    /// Returns tree string for either full repo or specific target paths
    fn build_tree_representation(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut tree_ctx = TreeContext::new(self.config.clone());

        let tree_str = if self.config.target_paths.is_empty() {
            tree_ctx.build_tree_from_root()?.tree_str.clone()
        } else {
            tree_ctx.build_tree_from_targets()?.tree_str.clone()
        };

        Ok(tree_str)
    }

    /// Build the file context based on configuration
    /// Returns FileContext for either full repo or specific target paths
    fn build_file_context(
        &self,
        repo_root: &str,
    ) -> Result<FileContext, Box<dyn std::error::Error>> {
        if self.config.target_paths.is_empty() {
            // If no target paths specified, process the entire repo (for tests and compatibility)
            FileContext::from_root(self.config.clone(), repo_root)
        } else {
            // Process only the specified target paths (new CLI behavior)
            FileContext::from_target_paths(self.config.clone(), repo_root)
        }
    }

    /// This is the heart of our implementation.
    /// Build the repository context by gathering information from git and the filesystem.
    /// This function initializes the context and populates it with relevant data.
    /// Now discovers repo from current working directory and processes specific target paths.
    pub fn build_context(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let repo = self.discover_repository()?;
        let actual_repo_root = get_repo_root_path(&repo)?;

        let file_ctx = self.build_file_context(&actual_repo_root)?;
        let tree_repr = self.build_tree_representation()?;

        self.context = Some(RepositoryContext {
            root_path: actual_repo_root,
            git_info: git::extract_git_info(&repo)?,
            file_ctx,
            tree_repr,
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
