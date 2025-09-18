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
// This module provides functionality to build a file tree representation
// of a given repository or directory.
//===----------------------------------------------------------------------===//
//

use crate::Config;
use globset::{Glob, GlobSetBuilder};
use ptree::TreeBuilder;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::fs;

/// Check if a file was modified within the last 7 days
fn is_recently_modified(path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified()?;
    let now = SystemTime::now();
    let seven_days_ago = now - Duration::from_secs(7 * 24 * 60 * 60);

    Ok(modified_time >= seven_days_ago)
}

#[derive(Debug, Clone)]
pub struct TreeContext {
    pub tree_str: String,
    config: Config,
}

impl TreeContext {
    pub fn new(config: Config) -> Self {
        Self {
            tree_str: String::new(),
            config,
        }
    }

    /// Build a complete tree hierarchy from the root directory
    /// Takes into account include/exclude patterns from config
    pub fn build_tree_from_root(&mut self) -> Result<&mut Self, Box<dyn std::error::Error>> {
        let root_path = Path::new(&self.config.root_path);

        // Build globsets for filtering
        let exclude_set = if self.config.exclude_patterns.is_empty() {
            None
        } else {
            Some(self.build_globset(&self.config.exclude_patterns)?)
        };

        let include_set = if self.config.include_patterns.is_empty() {
            None
        } else {
            Some(self.build_globset(&self.config.include_patterns)?)
        };

        // Create tree builder
        let mut tree_builder = TreeBuilder::new(
            root_path
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("root"))
                .to_string_lossy()
                .to_string(),
        );

        // Build the tree recursively
        self.build_tree_recursive(
            root_path,
            root_path,
            &mut tree_builder,
            &exclude_set,
            &include_set,
        )?;

        let tree = tree_builder.build();
        let mut buffer = Vec::new();
        ptree::write_tree_with(&tree, &mut buffer, &ptree::PrintConfig::default())
            .map_err(|e| format!("Failed to write tree: {}", e))?;
        self.tree_str = String::from_utf8(buffer)
            .map_err(|e| format!("Failed to convert tree to string: {}", e))?;

        Ok(self)
    }

    /// Build a tree hierarchy that only includes paths leading to target files/directories
    /// Creates a minimal tree showing only relevant paths
    pub fn build_tree_from_targets(&mut self) -> Result<&mut Self, Box<dyn std::error::Error>> {
        let root_path = Path::new(&self.config.root_path);

        // If no targets specified, build full tree
        if self.config.target_paths.is_empty() {
            return self.build_tree_from_root();
        }

        // FIXME: This is a bit hardcoded IMO, we'll revise this later
        // Special case: if target is "." or the root path itself, build full tree
        for target in &self.config.target_paths {
            let target_path = if Path::new(target).is_absolute() {
                PathBuf::from(target)
            } else {
                root_path.join(target)
            };

            if let (Ok(canonical_target), Ok(canonical_root)) =
                (target_path.canonicalize(), root_path.canonicalize())
                && canonical_target == canonical_root
            {
                // Target is the root directory, build full tree instead
                return self.build_tree_from_root();
            }
        }

        // Collect all target paths and their parent directories
        let mut tree_paths = std::collections::HashSet::new();
        let mut target_directories = std::collections::HashSet::new();

        for target in &self.config.target_paths {
            let target_path = if Path::new(target).is_absolute() {
                PathBuf::from(target)
            } else {
                root_path.join(target)
            };

            // Add the target path and all its parent directories
            if let Ok(canonical_target) = target_path.canonicalize()
                && let Ok(canonical_root) = root_path.canonicalize()
                && canonical_target.starts_with(&canonical_root)
            {
                // If target is a directory, we'll want to show all its contents
                if canonical_target.is_dir() {
                    _ = target_directories.insert(canonical_target.clone());
                }

                let mut current = canonical_target.as_path();
                while current != canonical_root {
                    _ = tree_paths.insert(current.to_path_buf());
                    if let Some(parent) = current.parent() {
                        current = parent;
                    } else {
                        break;
                    }
                }
                _ = tree_paths.insert(canonical_root.clone());
            }
        }

        // Create tree builder
        let mut tree_builder = TreeBuilder::new(
            root_path
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("root"))
                .to_string_lossy()
                .to_string(),
        );

        // Build the tree with only target paths
        self.build_tree_from_target_paths(
            root_path,
            root_path,
            &mut tree_builder,
            &tree_paths,
            &target_directories,
        )?;

        let tree = tree_builder.build();
        let mut buffer = Vec::new();
        ptree::write_tree_with(&tree, &mut buffer, &ptree::PrintConfig::default())
            .map_err(|e| format!("Failed to write tree: {}", e))?;
        self.tree_str = String::from_utf8(buffer)
            .map_err(|e| format!("Failed to convert tree to string: {}", e))?;
        Ok(self)
    }

    /// Helper method to build globset from patterns
    fn build_globset(
        &self,
        patterns: &[String],
    ) -> Result<globset::GlobSet, Box<dyn std::error::Error>> {
        let mut builder = GlobSetBuilder::new();

        for pattern in patterns {
            let glob = Glob::new(pattern)?;
            _ = builder.add(glob);
        }

        Ok(builder.build()?)
    }

    /// Check if a path should be included based on include/exclude patterns
    fn should_include_path(
        &self,
        path: &Path,
        root_path: &Path,
        exclude_set: &Option<globset::GlobSet>,
        include_set: &Option<globset::GlobSet>,
        is_file: bool,
    ) -> bool {
        // Get relative path for pattern matching
        let relative_path = if let Ok(rel_path) = path.strip_prefix(root_path) {
            rel_path
        } else {
            path
        };

        let path_str = relative_path.to_string_lossy().to_string();

        // Check exclude patterns first
        if let Some(exclude) = exclude_set
            && exclude.is_match(&path_str)
        {
            return false;
        }

        // For directories, always include if no specific exclude rule matched
        // This allows traversal into directories that might contain matching files
        if !is_file {
            return true;
        }

        // For files, check include patterns if they exist
        if let Some(include) = include_set {
            include.is_match(&path_str)
        } else {
            true // Include everything if no include patterns specified
        }
    }

    /// Recursively build tree from root directory
    fn build_tree_recursive(
        &self,
        current_path: &Path,
        root_path: &Path,
        tree_builder: &mut TreeBuilder,
        exclude_set: &Option<globset::GlobSet>,
        include_set: &Option<globset::GlobSet>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !current_path.is_dir() {
            return Ok(());
        }

        let mut entries = fs::read_dir(current_path)?
            .filter_map(|entry| entry.ok())
            .collect::<Vec<_>>();

        // Sort entries for consistent output
        entries.sort_by_key(|a| a.file_name());

        for entry in entries {
            let entry_path = entry.path();
            let is_file = entry_path.is_file();

            // Skip if path should be excluded
            if !self.should_include_path(&entry_path, root_path, exclude_set, include_set, is_file)
            {
                continue;
            }

            let name = entry_path
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
                .to_string_lossy()
                .to_string();

            if entry_path.is_dir() {
                _ = tree_builder.begin_child(name);
                if self.config.is_recursive {
                    self.build_tree_recursive(
                        &entry_path,
                        root_path,
                        tree_builder,
                        exclude_set,
                        include_set,
                    )?;
                }
                _ = tree_builder.end_child();
            } else if is_file {
                // Check recent filter if enabled
                if self.config.recent_only {
                    match is_recently_modified(&entry_path) {
                        Ok(false) => continue, // File is not recent, skip
                        Err(_) => continue,    // Error checking modification time, skip
                        Ok(true) => {}         // File is recent, continue processing
                    }
                }

                // Only add files that passed the include filter
                _ = tree_builder.add_empty_child(name);
            }
        }

        Ok(())
    }

    /// Build tree from specific target paths only
    #[allow(clippy::only_used_in_recursion)]
    fn build_tree_from_target_paths(
        &self,
        current_path: &Path,
        root_path: &Path,
        tree_builder: &mut TreeBuilder,
        target_paths: &std::collections::HashSet<PathBuf>,
        target_directories: &std::collections::HashSet<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !current_path.is_dir() {
            return Ok(());
        }

        let mut entries = fs::read_dir(current_path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if let Ok(canonical_path) = entry.path().canonicalize() {
                    // Include if path is in target_paths OR if current directory is a target directory
                    target_paths.contains(&canonical_path)
                        || target_directories
                            .iter()
                            .any(|target_dir| canonical_path.starts_with(target_dir))
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        // Sort entries for consistent output
        entries.sort_by_key(|a| a.file_name());

        for entry in entries {
            let entry_path = entry.path();

            let name = entry_path
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
                .to_string_lossy()
                .to_string();

            if entry_path.is_dir() {
                _ = tree_builder.begin_child(name);
                self.build_tree_from_target_paths(
                    &entry_path,
                    root_path,
                    tree_builder,
                    target_paths,
                    target_directories,
                )?;
                _ = tree_builder.end_child();
            } else {
                // Check recent filter if enabled
                if self.config.recent_only {
                    match is_recently_modified(&entry_path) {
                        Ok(false) => continue, // File is not recent, skip
                        Err(_) => continue,    // Error checking modification time, skip
                        Ok(true) => {}         // File is recent, continue processing
                    }
                }

                _ = tree_builder.add_empty_child(name);
            }
        }

        Ok(())
    }
}
