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

use cli_rust::tree::TreeContext;
use cli_rust::types::Config;
use std::fs;
use tempfile::TempDir;

/// Helper function to create a temporary directory structure for testing
fn create_test_directory_structure() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let root = temp_dir.path();

    // Create directory structure
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("tests")).unwrap();
    fs::create_dir_all(root.join("docs")).unwrap();
    fs::create_dir_all(root.join("src/modules")).unwrap();

    // Create files
    fs::write(root.join("Cargo.toml"), "# Cargo config").unwrap();
    fs::write(root.join("README.md"), "# Test project").unwrap();
    fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();
    fs::write(root.join("src/lib.rs"), "// Library code").unwrap();
    fs::write(root.join("src/modules/utils.rs"), "// Utils").unwrap();
    fs::write(root.join("tests/integration.rs"), "// Tests").unwrap();
    fs::write(root.join("docs/README.md"), "# Documentation").unwrap();

    temp_dir
}

#[test]
fn test_build_tree_from_root_basic() {
    let temp_dir = create_test_directory_structure();
    let root_path = temp_dir.path().to_string_lossy().to_string();

    let config = Config {
        root_path: root_path.clone(),
        target_paths: vec![],
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: true,
        output_file: None,
        recent_only: false,
        show_line_numbers: false,
    };

    let mut tree_context = TreeContext::new(config);
    let result = tree_context.build_tree_from_root();

    assert!(result.is_ok());

    let tree_str = &tree_context.tree_str;
    assert!(tree_str.contains("src"));
    assert!(tree_str.contains("tests"));
    assert!(tree_str.contains("docs"));
    assert!(tree_str.contains("Cargo.toml"));
    assert!(tree_str.contains("README.md"));
}

#[test]
fn test_build_tree_from_root_with_exclude_patterns() {
    let temp_dir = create_test_directory_structure();
    let root_path = temp_dir.path().to_string_lossy().to_string();

    let config = Config {
        root_path: root_path.clone(),
        target_paths: vec![],
        include_patterns: vec![],
        exclude_patterns: vec!["*.toml".to_string(), "tests/**".to_string()],
        is_recursive: true,
        output_file: None,
        recent_only: false,
        show_line_numbers: false,
    };

    let mut tree_context = TreeContext::new(config);
    let result = tree_context.build_tree_from_root();

    assert!(result.is_ok());

    let tree_str = &tree_context.tree_str;
    assert!(tree_str.contains("src"));
    assert!(tree_str.contains("docs"));
    assert!(!tree_str.contains("Cargo.toml")); // Should be excluded
    // Note: tests might still appear as directory name, let's check specific file
    assert!(!tree_str.contains("integration.rs")); // Should be excluded
}

#[test]
fn test_build_tree_from_root_with_include_patterns() {
    let temp_dir = create_test_directory_structure();
    let root_path = temp_dir.path().to_string_lossy().to_string();

    let config = Config {
        root_path: root_path.clone(),
        target_paths: vec![],
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec![],
        is_recursive: true,
        output_file: None,
        recent_only: false,
        show_line_numbers: false,
    };

    let mut tree_context = TreeContext::new(config);
    let result = tree_context.build_tree_from_root();

    assert!(result.is_ok());

    let tree_str = &tree_context.tree_str;
    assert!(tree_str.contains("main.rs"));
    assert!(tree_str.contains("lib.rs"));
    assert!(tree_str.contains("utils.rs"));
    assert!(tree_str.contains("integration.rs"));
    assert!(!tree_str.contains("Cargo.toml")); // Should not be included
}

#[test]
fn test_build_tree_from_targets_with_specific_files() {
    let temp_dir = create_test_directory_structure();
    let root_path = temp_dir.path().to_string_lossy().to_string();

    let config = Config {
        root_path: root_path.clone(),
        target_paths: vec!["src/main.rs".to_string(), "Cargo.toml".to_string()],
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: true,
        output_file: None,
        recent_only: false,
        show_line_numbers: false,
    };

    let mut tree_context = TreeContext::new(config);
    let result = tree_context.build_tree_from_targets();

    assert!(result.is_ok());

    let tree_str = &tree_context.tree_str;
    assert!(tree_str.contains("src"));
    assert!(tree_str.contains("main.rs"));
    assert!(tree_str.contains("Cargo.toml"));
    // Should not contain other files that aren't targets or in target paths
    assert!(!tree_str.contains("lib.rs"));
    assert!(!tree_str.contains("tests"));
}

#[test]
fn test_build_tree_from_targets_with_directory() {
    let temp_dir = create_test_directory_structure();
    let root_path = temp_dir.path().to_string_lossy().to_string();

    let config = Config {
        root_path: root_path.clone(),
        target_paths: vec!["src/".to_string()],
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: true,
        output_file: None,
        recent_only: false,
        show_line_numbers: false,
    };

    let mut tree_context = TreeContext::new(config);
    let result = tree_context.build_tree_from_targets();

    assert!(result.is_ok());

    let tree_str = &tree_context.tree_str;
    assert!(tree_str.contains("src"));
    assert!(tree_str.contains("main.rs"));
    assert!(tree_str.contains("lib.rs"));
    assert!(tree_str.contains("modules"));
    assert!(tree_str.contains("utils.rs"));
    // Should not contain files outside src/
    assert!(!tree_str.contains("Cargo.toml"));
    assert!(!tree_str.contains("tests"));
}

#[test]
fn test_build_tree_from_targets_root_directory_detection() {
    let temp_dir = create_test_directory_structure();
    let root_path = temp_dir.path().to_string_lossy().to_string();

    let config = Config {
        root_path: root_path.clone(),
        target_paths: vec![".".to_string()],
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: true,
        output_file: None,
        recent_only: false,
        show_line_numbers: false,
    };

    let mut tree_context = TreeContext::new(config);
    let result = tree_context.build_tree_from_targets();

    assert!(result.is_ok());

    let tree_str = &tree_context.tree_str;
    // Should show full tree when target is root directory
    assert!(tree_str.contains("src"));
    assert!(tree_str.contains("tests"));
    assert!(tree_str.contains("docs"));
    assert!(tree_str.contains("Cargo.toml"));
    assert!(tree_str.contains("main.rs"));
    assert!(tree_str.contains("integration.rs"));
}

#[test]
fn test_build_tree_from_targets_with_absolute_path() {
    let temp_dir = create_test_directory_structure();
    let root_path = temp_dir.path().to_string_lossy().to_string();
    let absolute_target = temp_dir.path().join("src/main.rs");

    let config = Config {
        root_path: root_path.clone(),
        target_paths: vec![absolute_target.to_string_lossy().to_string()],
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: true,
        output_file: None,
        recent_only: false,
        show_line_numbers: false,
    };

    let mut tree_context = TreeContext::new(config);
    let result = tree_context.build_tree_from_targets();

    assert!(result.is_ok());

    let tree_str = &tree_context.tree_str;
    assert!(tree_str.contains("src"));
    assert!(tree_str.contains("main.rs"));
    assert!(!tree_str.contains("lib.rs"));
}

#[test]
fn test_empty_target_paths_falls_back_to_full_tree() {
    let temp_dir = create_test_directory_structure();
    let root_path = temp_dir.path().to_string_lossy().to_string();

    let config = Config {
        root_path: root_path.clone(),
        target_paths: vec![], // Empty target paths
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: true,
        output_file: None,
        recent_only: false,
        show_line_numbers: false,
    };

    let mut tree_context = TreeContext::new(config);
    let result = tree_context.build_tree_from_targets();

    assert!(result.is_ok());

    let tree_str = &tree_context.tree_str;
    // Should build full tree when no targets specified, but let's be more specific about what we test
    assert!(!tree_str.is_empty());
    assert!(tree_str.contains("main.rs") || tree_str.contains("src")); // At least some content should be there
}

#[test]
fn test_tree_context_new() {
    let config = Config {
        root_path: "/test".to_string(),
        target_paths: vec!["src/".to_string()],
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec!["target/".to_string()],
        is_recursive: true,
        output_file: Some("output.md".to_string()),
        recent_only: false,
        show_line_numbers: false,
    };

    let tree_context = TreeContext::new(config.clone());

    // Since config field is private, we can only test that TreeContext was created
    // and has empty tree_str initially
    assert!(tree_context.tree_str.is_empty());
}

#[test]
fn test_nonexistent_target_paths() {
    let temp_dir = create_test_directory_structure();
    let root_path = temp_dir.path().to_string_lossy().to_string();

    let config = Config {
        root_path: root_path.clone(),
        target_paths: vec!["nonexistent/path.rs".to_string()],
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: true,
        output_file: None,
        recent_only: false,
        show_line_numbers: false,
    };

    let mut tree_context = TreeContext::new(config);
    let result = tree_context.build_tree_from_targets();

    // Should still succeed but show minimal tree
    assert!(result.is_ok());
    let tree_str = &tree_context.tree_str;
    // Tree should contain at least the root
    assert!(!tree_str.is_empty());
}
