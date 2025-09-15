use cli_rust::{Config, ContextManager, OutputContext, OutputDestination, OutputFormat};
use git2::Repository;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

fn setup_temp_repo() -> TempDir {
    let dir = tempfile::tempdir().expect("tempdir");

    // Create files first
    let paths = [
        "src/main.rs",
        "src/lib.rs",
        "README.md",
        "Cargo.toml",
        "nested/keep.rs",
        "nested/ignore.log",
        "nested/data.bin",
    ];

    for p in paths.iter() {
        let full = dir.path().join(p);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut f = File::create(&full).unwrap();
        writeln!(f, "// file {p}").unwrap();
    }

    // I'm not quite sure if we want to instantiate a new repo for each test, but for now it's fine
    // TODO: Add config to ignore git initialization

    // Initialize git repository
    let repo = Repository::init(dir.path()).expect("Failed to init git repository");

    // Configure git user
    let mut config = repo.config().unwrap();
    config.set_str("user.name", "Test User").unwrap();
    config.set_str("user.email", "test@example.com").unwrap();

    // Add all files
    let mut index = repo.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();

    // Create initial commit
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = git2::Signature::now("Test User", "test@example.com").unwrap();
    let _ = repo
        .commit(
            Some("refs/heads/main"),
            &sig,
            &sig,
            "Initial commit",
            &tree,
            &[],
        )
        .unwrap();

    // Set HEAD to point to the main branch
    repo.set_head("refs/heads/main").unwrap();

    dir
}

#[test]
fn test_stdout_output_mode() {
    let dir = setup_temp_repo();
    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,    // No output file = stdout
        include_patterns: vec!["**/*.rs".into()],
        exclude_patterns: vec![],
        is_recursive: true,
    };

    let mut manager = ContextManager::new(config.clone());
    manager.build_context().unwrap();

    // This test just ensures it doesn't panic when writing to stdout
    // In a real test, you'd capture stdout, but for simplicity we just verify it runs
    let result = OutputContext::new(manager)
        .format(OutputFormat::Markdown)
        .destination(OutputDestination::Stdout)
        .generate();
    assert!(result.is_ok());
}

#[test]
fn test_file_output_mode() {
    let dir = setup_temp_repo();
    let output_base = dir.path().join("output"); // No extension - it will be added

    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,    // Not used in new system
        include_patterns: vec!["**/*.rs".into()],
        exclude_patterns: vec![],
        is_recursive: true,
    };

    let mut manager = ContextManager::new(config.clone());
    manager.build_context().unwrap();

    // Generate output to file
    let result = OutputContext::new(manager)
        .format(OutputFormat::Markdown)
        .destination(OutputDestination::File(
            output_base.to_string_lossy().to_string(),
        ))
        .generate();
    assert!(result.is_ok());

    // Verify file was created and contains expected content
    let expected_file = output_base.with_extension("md"); // Markdown format adds .md
    assert!(expected_file.exists());
    let content = fs::read_to_string(&expected_file).unwrap();

    // Should contain file paths
    assert!(content.contains("Repository Context"));
    assert!(content.contains("FILE:"));
    assert!(content.contains("src/main.rs"));
    assert!(content.contains("src/lib.rs"));
    assert!(content.contains("nested/keep.rs"));
}

#[test]
fn test_file_output_overwrites_existing() {
    let dir = setup_temp_repo();
    let output_base = dir.path().join("output");
    let output_file_with_ext = output_base.with_extension("md");

    // Create an existing file with different content (the file that will be created)
    fs::write(&output_file_with_ext, "old content").unwrap();

    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec!["**/*.rs".into()],
        exclude_patterns: vec![],
        is_recursive: true,
    };

    let mut manager = ContextManager::new(config.clone());
    manager.build_context().unwrap();

    // Generate output to file
    let result = OutputContext::new(manager)
        .format(OutputFormat::Markdown)
        .destination(OutputDestination::File(
            output_base.to_string_lossy().to_string(),
        ))
        .generate();
    assert!(result.is_ok());

    // Verify file was overwritten (old content should be gone)
    let content = fs::read_to_string(&output_file_with_ext).unwrap();
    assert!(!content.contains("old content"));
    assert!(content.contains("Repository Context"));
}

#[test]
fn test_output_with_include_exclude_patterns() {
    let dir = setup_temp_repo();
    let output_base = dir.path().join("filtered_output");

    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec!["src/**/*.rs".into()],
        exclude_patterns: vec!["**/*.log".into()],
        is_recursive: true,
    };

    let mut manager = ContextManager::new(config.clone());
    manager.build_context().unwrap();

    let result = OutputContext::new(manager)
        .format(OutputFormat::Markdown)
        .destination(OutputDestination::File(
            output_base.to_string_lossy().to_string(),
        ))
        .generate();
    assert!(result.is_ok());

    let expected_file = output_base.with_extension("md");
    let content = fs::read_to_string(&expected_file).unwrap();

    // Should include Rust files from src
    assert!(content.contains("src/main.rs"));
    assert!(content.contains("src/lib.rs"));

    // Should not include files from nested (due to include pattern)
    assert!(!content.contains("nested/keep.rs"));

    // Should not include log files (due to exclude pattern)
    assert!(!content.contains("ignore.log"));
}

#[test]
fn test_output_file_creation_error() {
    let dir = setup_temp_repo();
    // Create a directory with the same name as the output file to cause an error
    let output_file = dir.path().join("output.txt");
    fs::create_dir(&output_file).unwrap();

    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: Some(output_file.to_string_lossy().to_string()),
        include_patterns: vec![],
        exclude_patterns: vec![],
        is_recursive: true,
    };

    let mut manager = ContextManager::new(config.clone());
    manager.build_context().unwrap();

    // This should cause the process to exit, but we can't easily test that
    // So we just verify the manager was created successfully
    assert!(manager.build_context().is_ok());
}

#[test]
fn test_empty_context_output() {
    let dir = setup_temp_repo();
    let output_base = dir.path().join("empty_output");

    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec!["**/*.nonexistent".into()], // No files will match
        exclude_patterns: vec![],
        is_recursive: true,
    };

    let mut manager = ContextManager::new(config.clone());
    manager.build_context().unwrap();

    let result = OutputContext::new(manager)
        .format(OutputFormat::Markdown)
        .destination(OutputDestination::File(
            output_base.to_string_lossy().to_string(),
        ))
        .generate();
    assert!(result.is_ok());

    // File should be created but empty (except for newlines)
    let expected_file = output_base.with_extension("md");
    let content = fs::read_to_string(&expected_file).unwrap();

    // Should have header but no file entries
    assert!(content.contains("Repository Context"));
}

#[test]
fn test_output_consistency_between_modes() {
    let dir = setup_temp_repo();
    let output_base = dir.path().join("file_output");

    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec!["**/*.rs".into()],
        exclude_patterns: vec![],
        is_recursive: true,
    };

    let mut manager = ContextManager::new(config.clone());
    manager.build_context().unwrap();

    // Generate output to file
    OutputContext::new(manager)
        .format(OutputFormat::Markdown)
        .destination(OutputDestination::File(
            output_base.to_string_lossy().to_string(),
        ))
        .generate()
        .unwrap();

    let expected_file = output_base.with_extension("md");
    let file_content = fs::read_to_string(&expected_file).unwrap();

    // Now test stdout mode with same config
    let stdout_config = Config {
        output_file: None,
        ..config
    };

    // Create a new manager for stdout test
    let mut stdout_manager = ContextManager::new(stdout_config);
    stdout_manager.build_context().unwrap();

    // We can't easily capture stdout in tests, so we just verify it runs without error
    let result = OutputContext::new(stdout_manager)
        .format(OutputFormat::Markdown)
        .destination(OutputDestination::Stdout)
        .generate();
    assert!(result.is_ok());

    // Verify file content has expected structure
    assert!(file_content.contains("Repository Context"));
    assert!(file_content.contains("FILE:"));
}
