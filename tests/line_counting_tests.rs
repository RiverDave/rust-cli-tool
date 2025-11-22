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
// Tests for line counting functionality and summary generation
//===----------------------------------------------------------------------===//

use rusty_repo_context_manager::types::{Config, FileContext};
use std::fs;
use tempfile::TempDir;

/// Create a temporary test file with specified content
fn create_test_file(dir: &TempDir, filename: &str, content: &str) {
    let file_path = dir.path().join(filename);
    fs::write(&file_path, content).expect("Failed to write test file");
}

/// Create a temporary binary file
fn create_binary_file(dir: &TempDir, filename: &str) {
    let file_path = dir.path().join(filename);
    let binary_data = vec![0u8, 1u8, 2u8, 255u8, 0u8]; // Contains null bytes
    fs::write(&file_path, binary_data).expect("Failed to write binary file");
}

#[test]
fn test_single_line_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let content = "Hello, world!";
    create_test_file(&temp_dir, "single_line.txt", content);

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        recent_only: false,
        ..Default::default()
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    assert_eq!(file_context.file_entries.len(), 1);
    let file_entry = &file_context.file_entries[0];
    assert_eq!(file_entry.lines, 1);
    assert!(!file_entry.is_binary);
    assert!(file_entry.content.is_some());
}

#[test]
fn test_multi_line_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let content = "Line 1\nLine 2\nLine 3\nLine 4\n";
    create_test_file(&temp_dir, "multi_line.txt", content);

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        recent_only: false,
        ..Default::default()
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    assert_eq!(file_context.file_entries.len(), 1);
    let file_entry = &file_context.file_entries[0];
    assert_eq!(file_entry.lines, 4);
    assert!(!file_entry.is_binary);
}

#[test]
fn test_empty_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_file(&temp_dir, "empty.txt", "");

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        recent_only: false,
        ..Default::default()
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    assert_eq!(file_context.file_entries.len(), 1);
    let file_entry = &file_context.file_entries[0];
    assert_eq!(file_entry.lines, 0);
    assert!(!file_entry.is_binary);
}

#[test]
fn test_file_without_trailing_newline() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let content = "Line 1\nLine 2\nLine 3"; // No trailing newline
    create_test_file(&temp_dir, "no_trailing_newline.txt", content);

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        recent_only: false,
        ..Default::default()
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    assert_eq!(file_context.file_entries.len(), 1);
    let file_entry = &file_context.file_entries[0];
    assert_eq!(file_entry.lines, 3);
    assert!(!file_entry.is_binary);
}

#[test]
fn test_binary_file_line_count() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_binary_file(&temp_dir, "binary.bin");

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        recent_only: false,
        ..Default::default()
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    assert_eq!(file_context.file_entries.len(), 1);
    let file_entry = &file_context.file_entries[0];
    assert_eq!(file_entry.lines, 0); // Binary files should have 0 lines
    assert!(file_entry.is_binary);
    assert!(file_entry.content.is_none());
}

#[test]
fn test_multiple_files_line_counting() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create multiple files with different line counts
    create_test_file(&temp_dir, "file1.txt", "Line 1\nLine 2\n"); // 2 lines
    create_test_file(&temp_dir, "file2.txt", "Single line"); // 1 line
    create_test_file(&temp_dir, "file3.txt", "A\nB\nC\nD\nE\n"); // 5 lines
    create_binary_file(&temp_dir, "binary.bin"); // 0 lines (binary)

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        recent_only: false,
        ..Default::default()
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    assert_eq!(file_context.file_entries.len(), 4);

    // Calculate total lines
    let total_lines: u64 = file_context.file_entries.iter().map(|f| f.lines).sum();
    assert_eq!(total_lines, 8); // 2 + 1 + 5 + 0 = 8

    // Verify individual files
    for file_entry in &file_context.file_entries {
        match file_entry.path.as_str() {
            path if path.ends_with("file1.txt") => assert_eq!(file_entry.lines, 2),
            path if path.ends_with("file2.txt") => assert_eq!(file_entry.lines, 1),
            path if path.ends_with("file3.txt") => assert_eq!(file_entry.lines, 5),
            path if path.ends_with("binary.bin") => {
                assert_eq!(file_entry.lines, 0);
                assert!(file_entry.is_binary);
            }
            _ => panic!("Unexpected file: {}", file_entry.path),
        }
    }
}

#[test]
fn test_file_with_only_newlines() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let content = "\n\n\n"; // Three newlines
    create_test_file(&temp_dir, "newlines_only.txt", content);

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        recent_only: false,
        show_line_numbers: false,
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    assert_eq!(file_context.file_entries.len(), 1);
    let file_entry = &file_context.file_entries[0];
    assert_eq!(file_entry.lines, 3);
    assert!(!file_entry.is_binary);
}

#[test]
fn test_recursive_directory_line_counting() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create subdirectory
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).expect("Failed to create subdirectory");

    // Create files in root and subdirectory
    create_test_file(&temp_dir, "root.txt", "Root line 1\nRoot line 2\n");

    let sub_file_path = sub_dir.join("sub.txt");
    fs::write(&sub_file_path, "Sub line 1\nSub line 2\nSub line 3\n")
        .expect("Failed to write sub file");

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: true,
        recent_only: false,
        show_line_numbers: false,
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    assert_eq!(file_context.file_entries.len(), 2);

    let total_lines: u64 = file_context.file_entries.iter().map(|f| f.lines).sum();
    assert_eq!(total_lines, 5); // 2 + 3 = 5
}

// Integration test for summary functionality
#[test]
fn test_summary_generation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create test files with known content
    create_test_file(&temp_dir, "small.txt", "A\nB\n"); // 2 lines
    create_test_file(&temp_dir, "medium.txt", "1\n2\n3\n4\n5\n"); // 5 lines
    create_binary_file(&temp_dir, "data.bin"); // 0 lines

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        recent_only: false,
        show_line_numbers: false,
    };

    let file_context = FileContext::from_root(config, temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    // Test summary calculation
    assert_eq!(file_context.file_entries.len(), 3);

    let total_lines: u64 = file_context.file_entries.iter().map(|f| f.lines).sum();
    assert_eq!(total_lines, 7); // 2 + 5 + 0 = 7

    let total_size: u64 = file_context.file_entries.iter().map(|f| f.size).sum();
    assert!(total_size > 0);

    // Verify that text files have content and line counts
    let text_files: Vec<_> = file_context
        .file_entries
        .iter()
        .filter(|f| !f.is_binary)
        .collect();

    for file in text_files {
        assert!(file.lines > 0);
        assert!(file.content.is_some());
    }

    // Verify binary file
    let binary_files: Vec<_> = file_context
        .file_entries
        .iter()
        .filter(|f| f.is_binary)
        .collect();

    assert_eq!(binary_files.len(), 1);
    assert_eq!(binary_files[0].lines, 0);
    assert!(binary_files[0].content.is_none());
}
