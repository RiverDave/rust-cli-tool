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
// This library provides a set of tools for managing and manipulating
// repository contexts, including file trees, git information, and more.
//===----------------------------------------------------------------------===//
//

/// Re-export main types for easy access
pub use types::*;

pub mod cli;
pub mod context;
pub mod files;
pub mod git;
pub mod output;
pub mod tree;
/// Internal modules
pub mod types;

// Re-export key functionality
pub use cli::Cli;
pub use context::ContextManager;
