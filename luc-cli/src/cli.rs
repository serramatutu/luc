use clap::{Parser, Subcommand};

/// A simple request runner
#[derive(Debug, Parser)]
#[command(name = "luc")]
#[command(about="A simple request runner.", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run requests
    #[command(arg_required_else_help = true)]
    Run {
        /// Files to get request specifications from
        files: Vec<String>,
    },
}
