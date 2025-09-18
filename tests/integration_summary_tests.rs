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
// Simple test to demonstrate line counting capabilities
//===----------------------------------------------------------------------===//

use cli_rust::types::{Config, FileContext};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_line_counting() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create test files with known line counts
    let file1 = temp_dir.path().join("test.txt");
    fs::write(&file1, "Line 1\nLine 2\nLine 3\n").expect("Failed to write test.txt"); // 3 lines

    let file2 = temp_dir.path().join("empty.txt");
    fs::write(&file2, "").expect("Failed to write empty.txt"); // 0 lines

    let file3 = temp_dir.path().join("binary.bin");
    fs::write(&file3, vec![0u8, 255u8]).expect("Failed to write binary.bin"); // 0 lines (binary)

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        recent_only: false,
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    assert_eq!(file_context.file_entries.len(), 3);

    // Verify line counts
    for file_entry in &file_context.file_entries {
        if file_entry.path.ends_with("test.txt") {
            assert_eq!(file_entry.lines, 3);
            assert!(!file_entry.is_binary);
        } else if file_entry.path.ends_with("empty.txt") {
            assert_eq!(file_entry.lines, 0);
            assert!(!file_entry.is_binary);
        } else if file_entry.path.ends_with("binary.bin") {
            assert_eq!(file_entry.lines, 0);
            assert!(file_entry.is_binary);
        }
    }
}
