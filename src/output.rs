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

use crate::{ContextManager, FileEntry, RepositoryContext};

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
        // Ensure the context is built

        // if self.context_manager.context.is_none() {
        //     self.context_manager.build_context()?;
        // }

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

        for file in &context.file_ctx.file_entries {
            output.push_str(&format!("  {}\n\n", dump_file_entry(file)));
        }

        output
    }

    /// Format as JSON
    #[allow(unused_variables, dead_code)]
    fn format_json(&self, context: &RepositoryContext) -> String {
        todo!("Format as JSON NYI")
    }
}

fn dump_file_entry(file: &FileEntry) -> String {
    let mut output = String::new();
    // TODO(0.1): All matadata would be dumped here
    output.push_str(&format!("## FILE: {}\n\n\n", file.path));
    output.push_str(file.content.as_deref().unwrap_or(""));
    output
}

fn dump_repo_metadata_md(repo_context: &RepositoryContext) -> String {
    let mut output = String::new();
    // TODO(0.1): All matadata would be dumped here

    output.push_str("## Repository Metadata\n\n");
    output.push_str("### File System Location\n\n");
    output.push_str(&format!("{}\n\n", repo_context.root_path));
    output.push_str("### Git Information\n\n");
    output.push_str(&dump_git_info_md(&repo_context.git_info));
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
