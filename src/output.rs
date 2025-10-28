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
// This module provides output formatting for repository context.
// As of release 0.1, it supports markdown output.
//===----------------------------------------------------------------------===//
//

use std::io::Write;

use crate::{ContextManager, FileContext, FileEntry, RepositoryContext};

/// Simple output format options
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Plain,
    Json,
    Markdown,
}

impl OutputFormat {
    pub fn to_extension(&self) -> &str {
        match self {
            OutputFormat::Plain => "txt",
            OutputFormat::Json => "json",
            OutputFormat::Markdown => "md",
        }
    }
}

/// Simple output destination options
#[derive(Debug, Clone)]
pub enum OutputDestination {
    Stdout,
    File(String),
}

/// Simple builder for outputting repository context
pub struct OutputContext {
    // should be moved to a ContextManager instance ideally?
    context_manager: ContextManager,
    format: OutputFormat,
    destination: OutputDestination,
    /// Output buffer: Content of the repo indexed by file path
    buffer: String,
}

impl OutputContext {
    /// Create a new OutputContext with the given ContextManager
    pub fn new(context_manager: ContextManager) -> Self {
        assert!(context_manager.context.is_some());

        Self {
            // These represent the default values
            context_manager,
            format: OutputFormat::Markdown,
            destination: OutputDestination::Stdout,
            buffer: String::new(),
        }
    }

    /// Set the output format
    pub fn format(mut self, format: OutputFormat) -> Self {
        self.format = format;
        self
    }

    /// Set the output destination
    pub fn destination(mut self, destination: OutputDestination) -> Self {
        self.destination = destination;
        self
    }

    /// Generate and output the repository context
    pub fn generate(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let context = self
            .context_manager
            .context
            .as_ref()
            .ok_or("Context not built")?;

        match &self.format {
            OutputFormat::Plain => todo!("Format as Plain Text Not yet implemented"), // I may never implement this
            OutputFormat::Json => todo!("Format as JSON Not yet implemented"),
            OutputFormat::Markdown => {
                let markdown_output = self.format_markdown(context);
                self.buffer.push_str(&markdown_output);
            }
        }

        match &self.destination {
            OutputDestination::Stdout => {
                print!("{}", self.buffer);
            }
            OutputDestination::File(path) => {
                let mut file =
                    std::fs::File::create(format!("{}.{}", path, self.format.to_extension()))?;
                file.write_all(self.buffer.as_bytes())?;
            }
        }

        Ok(())
    }

    /// Format as markdown
    fn format_markdown(&self, context: &RepositoryContext) -> String {
        let mut output = String::new();

        // dump header
        output.push_str("# Repository Context \n\n");

        //dump repo metadata
        output.push_str(&dump_repo_metadata_md(context));

        // dump tree structure
        output.push_str(&dump_tree_structure(&self.context_manager));

        // dump each file entry
        for file in &context.file_ctx.file_entries {
            output.push_str(&format!(
                "  {}\n\n",
                dump_file_entry(file, context.file_ctx.config.show_line_numbers)
            ));
        }

        output.push_str(&dump_separator_md());
        output.push_str("## Summary\n\n");

        // dump summary
        output.push_str(&dump_file_context_summary(&context.file_ctx));

        output
    }

    /// Format as JSON
    #[allow(unused_variables, dead_code)]
    fn format_json(&self, context: &RepositoryContext) -> String {
        todo!("Format as JSON NYI")
    }
}

fn dump_file_entry(file: &FileEntry, show_line_numbers: bool) -> String {
    let mut output = String::new();
    // Include file size in bytes in the file header when available
    output.push_str(&format!(
        "## FILE: {}{}\n\n",
        file.path,
        if file.size > 0 {
            format!(" ({} bytes)", file.size)
        } else {
            String::new()
        }
    ));

    if let Some(content) = &file.content {
        let language = get_file_extension(&file.path);
        output.push_str(&format!("```{}\n", language));

        if show_line_numbers {
            for (i, line) in content.lines().enumerate() {
                output.push_str(&format!("{}: {}\n", i + 1, line));
            }
            // If the original content did not end with a newline, preserve that final line ending
            if !content.ends_with('\n') {
                output.push('\n');
            }
        } else {
            output.push_str(content);
            if !content.ends_with('\n') {
                output.push('\n');
            }
        }

        output.push_str("```\n");
    } else if file.is_binary {
        output.push_str("*Binary file - content not displayed*\n");
    } else {
        output.push_str("*Content not available*\n");
    }

    output
}

fn dump_repo_metadata_md(repo_context: &RepositoryContext) -> String {
    let mut output = String::new();
    // TODO(0.1): All matadata would be dumped here

    output.push_str("## Metadata\n\n");
    output.push_str("### File System Location\n\n");
    output.push_str(&format!("{}\n\n", repo_context.root_path));
    output.push_str("### Git Information\n\n");
    output.push_str(&dump_git_info_md(&repo_context.git_info));
    output.push_str(&dump_separator_md());
    output
}

fn dump_git_info_md(git_info: &crate::types::GitInfo) -> String {
    let mut output = String::new();

    if git_info.is_repo {
        output.push_str(&format!(
            "- **Commit Hash**: {}\n",
            git_info.commit_hash.as_deref().unwrap_or("N/A")
        ));
        output.push_str(&format!(
            "- **Branch**: {}\n",
            git_info.branch.as_deref().unwrap_or("N/A")
        ));
        output.push_str(&format!(
            "- **Author**: {} <{}>\n",
            git_info.author.as_deref().unwrap_or("N/A"),
            git_info.email.as_deref().unwrap_or("N/A")
        ));
        output.push_str(&format!(
            "- **Date**: {}\n",
            git_info.date.as_deref().unwrap_or("N/A")
        ));
    } else {
        output.push_str("Couldn't retrieve Git information.\n");
    }

    output
}

fn dump_file_context_summary(file_context: &FileContext) -> String {
    let mut output = String::new();
    output.push_str(&format!(
        "Total files indexed: {}\n",
        file_context.file_entries.len()
    ));

    let total_size: u64 = file_context.file_entries.iter().map(|f| f.size).sum();
    output.push_str(&format!(
        "Total size of files: {:.2} MB\n",
        total_size as f64 / 1_048_576.0
    ));

    let total_lines: u64 = file_context.file_entries.iter().map(|f| f.lines).sum();
    output.push_str(&format!("Total lines across all files: {}\n", total_lines));

    // Language breakdown (by file extension)
    use std::collections::HashMap;
    let mut lang_counts: HashMap<String, (u64, u64, u64)> = HashMap::new();

    for f in &file_context.file_entries {
        // Use extension as a proxy for language (simple heuristic)
        let ext = match f.path.rsplit('.').next() {
            Some(seg) if seg != f.path => seg.to_lowercase(),
            _ => String::from(""),
        };

        let entry = lang_counts.entry(ext).or_insert((0, 0, 0));
        // (files, lines, bytes)
        entry.0 += 1;
        entry.1 += f.lines;
        entry.2 += f.size;
    }

    if !lang_counts.is_empty() {
        // Sort by total lines desc
        let mut items: Vec<(String, (u64, u64, u64))> = lang_counts.into_iter().collect();
        items.sort_by(|a, b| b.1.1.cmp(&a.1.1));

        output.push_str("\n### Language breakdown (by extension)\n\n");
        for (ext, (files, lines, bytes)) in items.iter().take(10) {
            let pct = if total_lines > 0 {
                (*lines as f64 / total_lines as f64) * 100.0
            } else {
                0.0
            };
            let label = if ext.is_empty() { "(no-ext)" } else { ext };
            output.push_str(&format!(
                "- {}: {} file(s), {} lines ({:.1}%), {:.2} MB\n",
                label,
                files,
                lines,
                pct,
                *bytes as f64 / 1_048_576.0
            ));
        }
    }

    // Top files by line count (quick hotspot view)
    let mut files_sorted = file_context.file_entries.clone();
    files_sorted.sort_by(|a, b| b.lines.cmp(&a.lines).then_with(|| a.path.cmp(&b.path)));

    output.push_str("\n### Top files by lines\n\n");
    for f in files_sorted.iter().take(10) {
        output.push_str(&format!(
            "- {}: {} lines, {:.2} KB\n",
            f.path,
            f.lines,
            f.size as f64 / 1024.0
        ));
    }

    output
}

fn dump_separator_md() -> String {
    let mut output = String::new();
    output.push_str("--------------------------------------------\n\n");
    output
}

/// Detect programming language from file path/extension
fn get_file_extension(file_path: &str) -> &str {
    // Get file extension efficiently
    if let Some(dot_pos) = file_path.rfind('.') {
        &file_path[dot_pos + 1..]
    } else {
        ""
    }
}

fn dump_tree_structure(ctx_manager: &ContextManager) -> String {
    let mut output = String::new();

    let tree_str = get_tree_structure(ctx_manager);

    // dump tree structure
    if !tree_str.is_empty() {
        output.push_str("## Directory Structure\n\n");
        output.push_str("```\n");
        output.push_str(&tree_str);
        output.push_str("```\n\n");
    }

    output.push_str(&dump_separator_md());
    output
}

fn get_tree_structure(ctx_manager: &ContextManager) -> String {
    // Cloning could be very expensive for large trees
    // We'll afford it for now, but consider refactoring later
    ctx_manager.context.as_ref().unwrap().tree_repr.clone()
}
