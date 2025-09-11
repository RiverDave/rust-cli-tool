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
// This module provides formatting capabilities for outputting the repository
// context in various formats (e.g., JSON, plain text). -> Only MD is considered for now
//===----------------------------------------------------------------------===//
//

use crate::{Cli, Config, ContextManager, FileEntry};

pub fn generate_output(
    files: Vec<FileEntry>,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!("Generate output NYI")
}
