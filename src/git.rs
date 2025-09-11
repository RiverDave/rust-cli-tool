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
// This module provides interaction with the libgit2 library to extract
// relevant information from a Git repository, such as the current branch,
// latest commit, and file changes.
//===----------------------------------------------------------------------===//
//

use chrono::{DateTime, Utc};
use git2::Repository;

use crate::types::GitInfo;

/// Extracts Git information from the given repository.
pub fn extract_git_info(repo: &Repository) -> Result<GitInfo, Box<dyn std::error::Error>> {
    let head = repo.head()?;
    let branch_name = head.shorthand().unwrap_or("unknown").to_string();

    // Get the latest commit hash
    let commit = head.peel_to_commit()?;
    let commit_hash = commit.id().to_string();

    // Get author information
    let signature = commit.author();
    let author_name = signature.name().unwrap_or("Unknown").to_string();

    // Get commit date
    let timestamp = signature.when();
    let datetime = DateTime::from_timestamp(timestamp.seconds(), 0).unwrap_or_else(|| Utc::now());
    let date_string = datetime.format("%Y-%m-%d").to_string();

    Ok(GitInfo {
        is_repo: true,
        commit_hash: Some(commit_hash),
        branch: Some(branch_name),
        author: Some(author_name),
        date: Some(date_string),
    })
}
