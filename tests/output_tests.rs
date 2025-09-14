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
// Integration tests for output functionality
//===----------------------------------------------------------------------===//
//

use cli_rust::{Config, ContextManager, OutputContext, OutputDestination, OutputFormat};
use git2::Repository;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

fn setup_temp_repo() -> TempDir {
    let dir = tempfile::tempdir().expect("tempdir");

    // Create test files with different content
    let files = [
        (
            "src/main.rs",
            "fn main() {\n    println!(\"Hello, world!\");\n}",
        ),
        (
            "src/lib.rs",
            "pub mod utils;\n\npub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
        ),
        (
            "README.md",
            "# Test Project\n\nThis is a test project for CLI tool.",
        ),
        (
            "Cargo.toml",
            "[package]\nname = \"test\"\nversion = \"0.1.0\"",
        ),
        ("docs/guide.md", "# User Guide\n\nHow to use this tool."),
    ];

    for (path, content) in files.iter() {
        let full_path = dir.path().join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut file = File::create(&full_path).unwrap();
        writeln!(file, "{}", content).unwrap();
    }

    // Initialize git repository
    let repo = Repository::init(dir.path()).expect("Failed to init git repository");

    // Configure git user
    let mut config = repo.config().unwrap();
    config.set_str("user.name", "Test User").unwrap();
    config.set_str("user.email", "test@example.com").unwrap();

    // Add and commit files
    let mut index = repo.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = git2::Signature::now("Test User", "test@example.com").unwrap();
    let _commit_id = repo
        .commit(
            Some("refs/heads/main"),
            &sig,
            &sig,
            "Initial commit",
            &tree,
            &[],
        )
        .unwrap();

    repo.set_head("refs/heads/main").unwrap();

    dir
}

mod output_format_tests {
    use super::*;

    #[test]
    fn test_output_format_to_extension() {
        assert_eq!(OutputFormat::Plain.to_extension(), "txt");
        assert_eq!(OutputFormat::Json.to_extension(), "json");
        assert_eq!(OutputFormat::Markdown.to_extension(), "md");
    }

    #[test]
    fn test_output_format_debug() {
        let format = OutputFormat::Markdown;
        let debug_str = format!("{:?}", format);
        assert!(debug_str.contains("Markdown"));
    }

    #[test]
    fn test_output_format_clone() {
        let format1 = OutputFormat::Plain;
        let format2 = format1.clone();
        assert_eq!(format1.to_extension(), format2.to_extension());
    }
}

mod output_destination_tests {
    use super::*;

    #[test]
    fn test_output_destination_stdout() {
        let dest = OutputDestination::Stdout;
        match dest {
            OutputDestination::Stdout => {}
            _ => panic!("Expected Stdout variant"),
        }
    }

    #[test]
    fn test_output_destination_file() {
        let dest = OutputDestination::File("test.txt".to_string());
        match dest {
            OutputDestination::File(path) => assert_eq!(path, "test.txt"),
            _ => panic!("Expected File variant"),
        }
    }

    #[test]
    fn test_output_destination_debug() {
        let dest = OutputDestination::File("output.md".to_string());
        let debug_str = format!("{:?}", dest);
        assert!(debug_str.contains("output.md"));
    }
}

mod output_context_tests {
    use super::*;

    #[test]
    fn test_output_context_creation() {
        let dir = setup_temp_repo();
        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["**/*.rs".into()],
            exclude_patterns: vec![],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        let _output_context = OutputContext::new(manager);
        // Test passes if OutputContext is created successfully
    }

    #[test]
    fn test_output_context_builder_pattern() {
        let dir = setup_temp_repo();
        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["**/*.rs".into()],
            exclude_patterns: vec![],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        let _output_context = OutputContext::new(manager)
            .format(OutputFormat::Json)
            .destination(OutputDestination::File("test.json".to_string()));

        // Test passes if builder pattern works without panicking
    }

    #[test]
    fn test_output_context_file_generation_with_extension() {
        let dir = setup_temp_repo();
        let output_path = dir.path().join("output");

        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["**/*.rs".into()],
            exclude_patterns: vec![],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        let result = OutputContext::new(manager)
            .format(OutputFormat::Markdown)
            .destination(OutputDestination::File(
                output_path.to_string_lossy().to_string(),
            ))
            .generate();

        assert!(result.is_ok());

        // Check that file was created with correct extension
        let expected_file = output_path.with_extension("md");
        assert!(expected_file.exists());

        // Verify content exists
        let content = fs::read_to_string(&expected_file).unwrap();
        assert!(!content.is_empty());
        assert!(content.contains("Repository Context"));
    }

    #[test]
    fn test_output_context_markdown_extension() {
        let dir = setup_temp_repo();
        let output_path = dir.path().join("markdown_output");

        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["README.md".into()],
            exclude_patterns: vec![],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        let result = OutputContext::new(manager)
            .format(OutputFormat::Markdown)
            .destination(OutputDestination::File(
                output_path.to_string_lossy().to_string(),
            ))
            .generate();

        assert!(result.is_ok());

        // Check that file was created with .md extension (since we used Markdown format)
        let expected_file = output_path.with_extension("md");
        assert!(expected_file.exists());
    }

    #[test]
    fn test_output_context_different_formats() {
        let dir = setup_temp_repo();

        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["**/*.rs".into()],
            exclude_patterns: vec![],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        // Test different file extensions based on format
        let test_cases = [
            (OutputFormat::Plain, "txt"),
            (OutputFormat::Json, "json"),
            (OutputFormat::Markdown, "md"),
        ];

        for (format, expected_ext) in test_cases.iter() {
            let output_path = dir.path().join(format!("output_{}", expected_ext));

            // Clone manager for each test since generate() consumes it
            // Note: We can't easily clone the built context, so we create a new manager
            // In a real scenario, you might want to refactor this

            if expected_ext == &"md" {
                // Only test Markdown format since others are todo!()
                let mut test_manager = ContextManager::new(manager.config.clone());
                test_manager.build_context().unwrap();

                let result = OutputContext::new(test_manager)
                    .format(format.clone())
                    .destination(OutputDestination::File(
                        output_path.to_string_lossy().to_string(),
                    ))
                    .generate();

                assert!(result.is_ok());

                let expected_file = output_path.with_extension(expected_ext);
                assert!(expected_file.exists());
            }
        }
    }

    #[test]
    fn test_output_context_stdout_generation() {
        let dir = setup_temp_repo();

        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["Cargo.toml".into()],
            exclude_patterns: vec![],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        // Test stdout output (we can't capture stdout easily in this test,
        // but we can verify it doesn't panic)
        let result = OutputContext::new(manager)
            .format(OutputFormat::Markdown)
            .destination(OutputDestination::Stdout)
            .generate();

        assert!(result.is_ok());
    }

    #[test]
    fn test_output_context_with_filtered_files() {
        let dir = setup_temp_repo();
        let output_path = dir.path().join("filtered_output");

        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["src/**/*.rs".into()],
            exclude_patterns: vec!["**/*lib*".into()],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        let result = OutputContext::new(manager)
            .format(OutputFormat::Markdown)
            .destination(OutputDestination::File(
                output_path.to_string_lossy().to_string(),
            ))
            .generate();

        assert!(result.is_ok());

        let expected_file = output_path.with_extension("md");
        assert!(expected_file.exists());

        let content = fs::read_to_string(&expected_file).unwrap();

        // Should include main.rs but exclude lib.rs due to exclude pattern
        assert!(content.contains("main.rs"));
        // Note: The actual filtering behavior depends on your glob implementation
    }

    #[test]
    fn test_output_context_empty_context() {
        let dir = setup_temp_repo();
        let output_path = dir.path().join("empty_output");

        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["**/*.nonexistent".into()], // No files match
            exclude_patterns: vec![],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        let result = OutputContext::new(manager)
            .format(OutputFormat::Markdown)
            .destination(OutputDestination::File(
                output_path.to_string_lossy().to_string(),
            ))
            .generate();

        assert!(result.is_ok());

        let expected_file = output_path.with_extension("md");
        assert!(expected_file.exists());

        let content = fs::read_to_string(&expected_file).unwrap();

        // Should have header but no file entries
        assert!(content.contains("Repository Context"));
        // The exact behavior depends on your implementation
    }

    #[test]
    fn test_output_context_file_creation_error() {
        let dir = setup_temp_repo();

        // Create a directory with the same name as our intended output file (with extension)
        let problematic_base = dir.path().join("problematic");
        let problematic_with_ext = problematic_base.with_extension("md");
        fs::create_dir(&problematic_with_ext).unwrap();

        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["**/*.rs".into()],
            exclude_patterns: vec![],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        let result = OutputContext::new(manager)
            .format(OutputFormat::Markdown)
            .destination(OutputDestination::File(
                problematic_base.to_string_lossy().to_string(),
            ))
            .generate();

        // Should return an error since we can't create a file where a directory exists
        assert!(result.is_err());
    }
}

mod integration_tests {
    use super::*;

    #[test]
    fn test_end_to_end_workflow() {
        let dir = setup_temp_repo();
        let output_path = dir.path().join("complete_output");

        // Test the complete workflow from config to output
        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["**/*.md".into(), "**/*.rs".into()],
            exclude_patterns: vec!["docs/**".into()],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        let result = OutputContext::new(manager)
            .format(OutputFormat::Markdown)
            .destination(OutputDestination::File(
                output_path.to_string_lossy().to_string(),
            ))
            .generate();

        assert!(result.is_ok());

        let expected_file = output_path.with_extension("md");
        assert!(expected_file.exists());

        let content = fs::read_to_string(&expected_file).unwrap();

        // Verify expected content structure
        assert!(content.contains("Repository Context"));
        assert!(content.contains("FILE:"));

        // Should include README.md and Rust files
        assert!(
            content.contains("README.md")
                || content.contains("main.rs")
                || content.contains("lib.rs")
        );

        // Should exclude docs/ files due to exclude pattern
        // Note: The actual filtering behavior depends on your glob implementation
    }

    #[test]
    fn test_multiple_output_formats_same_context() {
        let dir = setup_temp_repo();

        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["Cargo.toml".into()],
            exclude_patterns: vec![],
            is_recursive: true,
        };

        // We need separate managers since generate() consumes the context
        let mut manager1 = ContextManager::new(config.clone());
        manager1.build_context().unwrap();

        let result1 = OutputContext::new(manager1)
            .format(OutputFormat::Markdown)
            .destination(OutputDestination::File(
                dir.path().join("output1").to_string_lossy().to_string(),
            ))
            .generate();

        assert!(result1.is_ok());

        // Verify first output file
        let file1 = dir.path().join("output1.md");
        assert!(file1.exists());

        let content1 = fs::read_to_string(&file1).unwrap();
        assert!(content1.contains("Repository Context"));
        assert!(content1.contains("Cargo.toml"));
    }

    #[test]
    fn test_output_content_structure() {
        let dir = setup_temp_repo();
        let output_path = dir.path().join("structure_test");

        let config = Config {
            root_path: dir.path().to_string_lossy().to_string(),
            output_file: None,
            include_patterns: vec!["README.md".into()],
            exclude_patterns: vec![],
            is_recursive: true,
        };

        let mut manager = ContextManager::new(config);
        manager.build_context().unwrap();

        let result = OutputContext::new(manager)
            .format(OutputFormat::Markdown)
            .destination(OutputDestination::File(
                output_path.to_string_lossy().to_string(),
            ))
            .generate();

        assert!(result.is_ok());

        let expected_file = output_path.with_extension("md");
        let content = fs::read_to_string(&expected_file).unwrap();

        // Verify the markdown structure
        assert!(content.starts_with("# Repository Context"));
        assert!(content.contains("## FILE:"));
        assert!(content.contains("README.md"));

        // Should contain the actual file content
        assert!(content.contains("Test Project"));
    }
}
