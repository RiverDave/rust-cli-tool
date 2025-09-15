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
// Simple test for summary calculation functionality
//===----------------------------------------------------------------------===//

use cli_rust::types::{Config, FileContext};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_summary_calculation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create test files
    let file1 = temp_dir.path().join("test.txt");
    fs::write(&file1, "Line 1\nLine 2\n").expect("Failed to write test.txt"); // 2 lines

    let file2 = temp_dir.path().join("data.bin");
    fs::write(&file2, vec![0u8, 255u8]).expect("Failed to write binary file"); // 0 lines

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    // Test summary calculations
    assert_eq!(file_context.file_entries.len(), 2);

    let total_lines: u64 = file_context.file_entries.iter().map(|f| f.lines).sum();
    assert_eq!(total_lines, 2); // Only text file contributes

    let total_size: u64 = file_context.file_entries.iter().map(|f| f.size).sum();
    assert!(total_size > 0);
}
