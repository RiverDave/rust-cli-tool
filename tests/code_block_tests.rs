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
// Tests for code block formatting in markdown output
//===----------------------------------------------------------------------===//

use rusty_repo_context_manager::types::{Config, FileContext, GitInfo, RepositoryContext};
use rusty_repo_context_manager::{ContextManager, OutputContext, OutputDestination, OutputFormat};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_code_block_formatting() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let rust_file = temp_dir.path().join("example.rs");
    fs::write(
        &rust_file,
        r#"fn main() {
    println!("Hello, world!");
}
"#,
    )
    .expect("Failed to write Rust file");

    let python_file = temp_dir.path().join("script.py");
    fs::write(&python_file, "print(\"Hello, Python!\")").expect("Failed to write Python file");

    let binary_file = temp_dir.path().join("data.bin");
    fs::write(&binary_file, vec![0u8, 255u8, 128u8]).expect("Failed to write binary file");

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        show_line_numbers: false,
        recent_only: false,
    };

    let file_ctx = FileContext::from_root(config.clone(), temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    let git_info = GitInfo {
        is_repo: false,
        commit_hash: None,
        branch: None,
        author: None,
        email: None,
        date: None,
    };

    let repo_context = RepositoryContext {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        git_info,
        file_ctx,
        tree_repr: String::new(), // Use empty tree representation for test
    };

    let mut context_manager = ContextManager::new(config);
    context_manager.context = Some(repo_context);

    let output_context = OutputContext::new(context_manager).format(OutputFormat::Markdown);

    let output_file = temp_dir.path().join("test_output");
    let output_path = output_file.to_string_lossy().to_string();

    let output_context = output_context.destination(OutputDestination::File(output_path.clone()));
    output_context
        .generate()
        .expect("Failed to generate output");

    // Read the generated markdown file
    let generated_content =
        fs::read_to_string(format!("{}.md", output_path)).expect("Failed to read generated file");

    // Verify code blocks have proper language specifiers
    assert!(generated_content.contains("```rs\n"));
    assert!(generated_content.contains("```py\n"));
    assert!(generated_content.contains("fn main() {"));
    assert!(generated_content.contains("print(\"Hello, Python!\")"));
}

#[test]
fn test_file_without_extension() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create file without extension
    let no_ext_file = temp_dir.path().join("README");
    fs::write(&no_ext_file, "This is a README file\nwith some content")
        .expect("Failed to write file");

    let config = Config {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: false,
        show_line_numbers: false,
        recent_only: false,
    };

    let file_ctx = FileContext::from_root(config.clone(), temp_dir.path().to_str().unwrap())
        .expect("Failed to create FileContext");

    let git_info = GitInfo {
        is_repo: false,
        commit_hash: None,
        branch: None,
        author: None,
        email: None,
        date: None,
    };

    let repo_context = RepositoryContext {
        root_path: temp_dir.path().to_string_lossy().to_string(),
        git_info,
        file_ctx,
        tree_repr: String::new(), // Use empty tree representation for test
    };

    let mut context_manager = ContextManager::new(config);
    context_manager.context = Some(repo_context);

    let output_context = OutputContext::new(context_manager).format(OutputFormat::Markdown);

    let output_file = temp_dir.path().join("test_output");
    let output_path = output_file.to_string_lossy().to_string();

    let output_context = output_context.destination(OutputDestination::File(output_path.clone()));
    output_context
        .generate()
        .expect("Failed to generate output");

    // Read the generated markdown file
    let generated_content =
        fs::read_to_string(format!("{}.md", output_path)).expect("Failed to read generated file");

    // Verify file without extension uses empty language specifier
    assert!(generated_content.contains("```\n"));
    assert!(generated_content.contains("This is a README file"));
}
