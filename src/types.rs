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
// This file defines the core data structures used across the repository 
// context packager library.
//===----------------------------------------------------------------------===//
//


#[derive(Debug, Clone)]
pub struct Config {
    pub paths: Vec<String>,
    pub output_file: Option<String>,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub content: Option<String>,  // None for binary files
    pub size: u64,
    pub is_binary: bool,
}

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub is_repo: bool,
    pub commit_hash: Option<String>,
    pub branch: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
}

// TODO: This should be linked with libgit2::Repository for more details?
#[derive(Debug)]
pub struct RepositoryContext {
    pub root_path: String,
    pub git_info: GitInfo,
    pub files: Vec<FileEntry>,
    pub tree_structure: String,
    pub total_files: usize,
    pub total_lines: usize,
}