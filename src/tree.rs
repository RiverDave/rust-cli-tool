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

use crate::FileEntry;

pub fn build_tree(files: Vec<FileEntry>) -> Result<(), Box<dyn std::error::Error>> {
    files.iter().for_each(|f| {
        println!("File: {:?}", f.path);
    });

    Ok(())
}
