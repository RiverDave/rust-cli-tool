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
use std::path::Path;

use crate::types::{Config, FileContext, FileEntry};

impl FileContext {
    pub fn new(config: Config) -> Self {
        Self {
            file_entries: Vec::new(),
            config,
        }
    }

    pub fn build_file_context(&self, root_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            file_entries: self.discover_files(root_path, &self.config)?,
            config: self.config.clone(),
        })
    }

    /// Uses Context to discover files
    pub fn discover_files(
        &self,
        root_path: &str,
        config: &Config,
    ) -> Result<Vec<FileEntry>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();

        // Build globsets for include and exclude patterns
        let exclude_set = build_globset(&config.exclude_patterns)?;
        let include_set = if config.include_patterns.is_empty() {
            None
        } else {
            Some(build_globset(&config.include_patterns)?)
        };

        // Start traversal
        Self::traverse_directory(
            self,
            root_path,
            config,
            &mut files,
            &exclude_set,
            &include_set,
        )?;

        Ok(files)
    }

    fn traverse_directory(
        &self,
        dir_path: &str,
        config: &Config,
        files: &mut Vec<FileEntry>,
        exclude_set: &globset::GlobSet,
        include_set: &Option<globset::GlobSet>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(dir_path);

        if !path.exists() || !path.is_dir() {
            return Ok(());
        }

        let entries = fs::read_dir(path)?;

        // TODO: I'd certainly like this to be more 'idiomatic' Rust (avoid mut, for)
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry_path.to_string_lossy().to_string();

            // Skip hidden files and directories (starting with .)
            if let Some(name) = entry_path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }

            // Check exclude patterns
            if exclude_set.is_match(&file_name) {
                continue;
            }

            if entry_path.is_file() {
                // Check include patterns (if any)
                if let Some(include) = include_set {
                    if !include.is_match(&file_name) {
                        continue;
                    }
                }

                // Create FileEntry
                match create_file_entry(&entry_path) {
                    Ok(file_entry) => files.push(file_entry),
                    Err(e) => eprintln!("Warning: Could not process file {}: {}", file_name, e),
                }
            } else if entry_path.is_dir() && config.is_recursive {
                // Recursively traverse subdirectories
                Self::traverse_directory(
                    self,
                    &file_name,
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
