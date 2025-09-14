// Integration tests for file operations
mod files;
mod output_tests;
mod unit;

// Re-export the test modules so they can be discovered by cargo test
pub use files::*;
