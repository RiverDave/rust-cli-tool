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
// This module provides functionality to discover files in a given directory,
// including support for filtering and excluding files based on various criteria.
//===----------------------------------------------------------------------===//
//

use globset::{Glob, GlobSetBuilder};
use std::fs;
use std::path::{Path, PathBuf};

use crate::types::{Config, FileContext, FileEntry};

impl FileContext {
    pub fn new(config: Config) -> Self {
        Self {
            file_entries: Vec::new(),
            config,
        }
    }

    /// Create a new FileContext with files discovered from the given root path
    pub fn from_root(config: Config, root_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let files = Self::discover_files(root_path, &config)?;
        Ok(Self {
            file_entries: files,
            config,
        })
    }

    /// Discover files in the given root path
    pub fn discover_files(
        root_path: &str,
        config: &Config,
    ) -> Result<Vec<FileEntry>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();

        // Build globsets for include and exclude patterns
        let exclude_set = if config.exclude_patterns.is_empty() {
            None
        } else {
            Some(build_globset(&config.exclude_patterns)?)
        };

        let include_set = if config.include_patterns.is_empty() {
            None
        } else {
            Some(build_globset(&config.include_patterns)?)
        };

        // Start traversal
        Self::traverse_directory(
            root_path,
            Path::new(root_path),
            config,
            &mut files,
            &exclude_set,
            &include_set,
        )?;

        Ok(files)
    }

    /// Recursively traverse directories to find files consider glob patterns (include/exclude)
    fn traverse_directory(
        current_path_str: &str,
        root_path: &Path,
        config: &Config,
        files: &mut Vec<FileEntry>,
        exclude_set: &Option<globset::GlobSet>,
        include_set: &Option<globset::GlobSet>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let current_path = Path::new(current_path_str);

        if !current_path.exists() || !current_path.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let entry_path = entry.path();

            // Skip hidden files and directories (starting with .)
            if let Some(name) = entry_path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }

            // Compute relative path (fallback to absolute if cannot strip)
            let rel_path: PathBuf = match entry_path.strip_prefix(root_path) {
                Ok(p) => p.to_path_buf(),
                Err(_) => entry_path.clone(),
            };
            let rel_str = rel_path.to_string_lossy();

            // Exclude patterns: if any match, skip
            if let Some(exclude) = exclude_set {
                if exclude.is_match(rel_str.as_ref()) {
                    continue;
                }
            }

            if entry_path.is_file() {
                // Include patterns: if provided and none match, skip
                if let Some(include) = include_set {
                    if !include.is_match(rel_str.as_ref()) {
                        continue;
                    }
                }

                match create_file_entry(&entry_path) {
                    Ok(mut file_entry) => {
                        // Store relative path for consistency
                        file_entry.path = rel_str.to_string();
                        files.push(file_entry)
                    }
                    Err(e) => eprintln!(
                        "Warning: Could not process file {}: {}",
                        entry_path.to_string_lossy(),
                        e
                    ),
                }
            } else if entry_path.is_dir() && config.is_recursive {
                Self::traverse_directory(
                    &entry_path.to_string_lossy(),
                    root_path,
                    config,
                    files,
                    exclude_set,
                    include_set,
                )?;
            }
        }

        Ok(())
    }
}

fn build_globset(patterns: &[String]) -> Result<globset::GlobSet, Box<dyn std::error::Error>> {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let glob = Glob::new(pattern)?;
        builder.add(glob);
    }

    Ok(builder.build()?)
}

fn create_file_entry(path: &Path) -> Result<FileEntry, Box<dyn std::error::Error>> {
    let metadata = fs::metadata(path)?;
    let size = metadata.len();

    // Determine if file is binary by reading first few bytes
    let is_binary = is_binary_file(path)?;

    // Read content if it's not binary and not too large (e.g., < 1MB)
    // It'd be fun if the user could configure this limit, too complex for now
    let content = if !is_binary && size < 1_000_000 {
        match fs::read_to_string(path) {
            Ok(content) => Some(content),
            Err(_) => None, // Treat as binary if we can't read as UTF-8
        }
    } else {
        None
    };

    Ok(FileEntry {
        path: path.to_string_lossy().to_string(),
        content,
        size,
        is_binary,
    })
}

/// Simple heuristic to determine if a file is binary
/// Source: https://post.bytes.com/forum/topic/python/18010-determine-file-type-binary-or-text
fn is_binary_file(path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    // Read first 512 bytes to check for binary content
    let mut buffer = [0; 512];

    match fs::File::open(path) {
        Ok(mut file) => {
            use std::io::Read;
            let bytes_read = file.read(&mut buffer)?;

            // Check for null bytes (common indicator of binary files)
            let is_binary = buffer[..bytes_read].contains(&0);
            Ok(is_binary)
        }
        Err(_) => Ok(true), // Assume binary if we can't read
    }
}
