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


use crate::types::{FileEntry, Config};

// We might want to include as a dependency `ignore` crate for better file discovery

pub fn discover_files(
    root_path: &str, 
    config: &Config
) -> Result<Vec<FileEntry>, Box<dyn std::error::Error>> {
    todo!()
}