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
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::types::{Config, FileContext, FileEntry};

/// Count lines in a file efficiently without loading entire content into memory
// NOTE: I wonder how expensive would this be?
fn get_file_lines(path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count() as u64)
}

/// Filter fn: Check if a file was modified within the last 7 days
fn is_recently_modified(path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified()?;
    let now = SystemTime::now();
    let seven_days_ago = now - Duration::from_secs(7 * 24 * 60 * 60);

    Ok(modified_time >= seven_days_ago)
}

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

    /// Create a new FileContext with files discovered from specific target paths
    pub fn from_target_paths(
        config: Config,
        repo_root: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut all_files = Vec::new();

        for target_path in &config.target_paths {
            // Convert target path to absolute path if it's relative
            let abs_target_path = if Path::new(target_path).is_absolute() {
                target_path.clone()
            } else {
                // Resolve relative to current working directory (config.root_path)
                Path::new(&config.root_path)
                    .join(target_path)
                    .to_string_lossy()
                    .to_string()
            };

            let target_path_obj = Path::new(&abs_target_path);

            if target_path_obj.is_file() {
                // Single file - check recent filter if enabled
                if config.recent_only {
                    match is_recently_modified(target_path_obj) {
                        Ok(false) => continue, // File is not recent, skip
                        Err(e) => {
                            eprintln!(
                                "Warning: Could not check modification time for {}: {}",
                                abs_target_path, e
                            );
                            continue;
                        }
                        Ok(true) => {} // File is recent, continue processing
                    }
                }

                // Single file - create file entry directly
                match create_file_entry(target_path_obj) {
                    Ok(mut file_entry) => {
                        // Make path relative to repo root for consistency
                        if let Ok(rel_path) = target_path_obj.strip_prefix(repo_root) {
                            file_entry.path = rel_path.to_string_lossy().to_string();
                        }
                        all_files.push(file_entry);
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not process file {}: {}", abs_target_path, e)
                    }
                }
            } else if target_path_obj.is_dir() {
                // Directory - discover files within it
                let files = Self::discover_files(&abs_target_path, &config)?;
                all_files.extend(files);
            } else {
                eprintln!("Warning: Target path does not exist: {}", abs_target_path);
            }
        }

        Ok(Self {
            file_entries: all_files,
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
            if let Some(name) = entry_path.file_name()
                && name.to_string_lossy().starts_with('.')
            {
                continue;
            }

            // Compute relative path (fallback to absolute if cannot strip)
            let rel_path: PathBuf = match entry_path.strip_prefix(root_path) {
                Ok(p) => p.to_path_buf(),
                Err(_) => entry_path.clone(),
            };
            let rel_str = rel_path.to_string_lossy();

            // Exclude patterns: if any match, skip
            if let Some(exclude) = exclude_set
                && exclude.is_match(rel_str.as_ref())
            {
                continue;
            }

            if entry_path.is_file() {
                // Include patterns: if provided and none match, skip
                if let Some(include) = include_set
                    && !include.is_match(rel_str.as_ref())
                {
                    continue;
                }

                // Recent filter: if enabled and file is not recently modified, skip
                if config.recent_only {
                    match is_recently_modified(&entry_path) {
                        Ok(false) => continue,
                        Err(e) => {
                            eprintln!(
                                "Warning: Could not check modification time for {}: {}",
                                entry_path.to_string_lossy(),
                                e
                            );
                            continue;
                        }
                        Ok(true) => {} // File is recent, continue processing
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
        _ = builder.add(glob);
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
        fs::read_to_string(path).ok()
    } else {
        None
    };

    let lines = if !is_binary { get_file_lines(path)? } else { 0 };

    Ok(FileEntry {
        path: path.to_string_lossy().to_string(),
        content,
        size,
        lines,
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
