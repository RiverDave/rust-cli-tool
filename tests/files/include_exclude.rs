use cli_rust::{Config, FileContext};
use std::fs::{self, File};
use std::io::Write;

fn setup_temp_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    // Create files
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

    dir
}

#[test]
fn exclude_glob_filters_out_matches() {
    let dir = setup_temp_repo();
    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec![],
        exclude_patterns: vec!["**/*.log".into(), "**/*.bin".into()],
        is_recursive: true,
    };

    let file_ctx = FileContext::from_root(config.clone(), &config.root_path).unwrap();
    let collected: Vec<String> = file_ctx
        .file_entries
        .iter()
        .map(|f| f.path.clone())
        .collect();

    assert!(collected.iter().any(|p| p == "src/main.rs"));
    assert!(!collected.iter().any(|p| p.ends_with("ignore.log")));
    assert!(!collected.iter().any(|p| p.ends_with("data.bin")));
}

#[test]
fn include_glob_only_includes_matches() {
    let dir = setup_temp_repo();
    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec!["src/**/*.rs".into(), "nested/keep.rs".into()],
        exclude_patterns: vec![],
        is_recursive: true,
    };

    let file_ctx = FileContext::from_root(config.clone(), &config.root_path).unwrap();
    let collected: Vec<String> = file_ctx
        .file_entries
        .iter()
        .map(|f| f.path.clone())
        .collect();

    // Only .rs files from src and nested/keep.rs
    assert!(collected.iter().all(|p| p.ends_with(".rs")));
    assert!(collected.iter().any(|p| p == "nested/keep.rs"));
    assert!(!collected.iter().any(|p| p.ends_with("ignore.log")));
    assert!(!collected.iter().any(|p| p == "README.md"));
}

#[test]
fn include_and_exclude_combined() {
    let dir = setup_temp_repo();
    let config = Config {
        root_path: dir.path().to_string_lossy().to_string(),
        target_paths: vec![], // Empty for this test, will use from_root
        output_file: None,
        include_patterns: vec!["**/*.rs".into(), "**/*.md".into()],
        exclude_patterns: vec!["nested/*".into()],
        is_recursive: true,
    };

    let file_ctx = FileContext::from_root(config.clone(), &config.root_path).unwrap();
    let collected: Vec<String> = file_ctx
        .file_entries
        .iter()
        .map(|f| f.path.clone())
        .collect();

    assert!(collected.iter().any(|p| p == "src/main.rs"));
    assert!(collected.iter().any(|p| p == "src/lib.rs"));
    assert!(collected.iter().any(|p| p == "README.md"));
    assert!(!collected.iter().any(|p| p.starts_with("nested/")));
}
