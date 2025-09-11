use crate::types::*;
use crate::{files, git, output, tree};
use git2::Repository;

pub struct ContextManager {
    config: Config,
    context: Option<RepositoryContext>,
}

impl ContextManager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            context: None,
        }
    }

    pub fn build_context(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Build context NYI")
    }

    pub fn generate_output(&self) -> Result<String, Box<dyn std::error::Error>> {
        todo!("Generate output NYI")
        // if let Some(ctx) = &self.context {
        //     output::generate_output(ctx, &self.config.output_file)
        // } else {
        //     Err("Context not built yet".into())
        // }
    }
}
