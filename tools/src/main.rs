//! Haleiki CLI — the pre-build tool for the Haleiki knowledge base framework.
//!
//! This binary parses Markdown content, builds a relationship graph, validates
//! references, computes derived data, and writes JSON for the static-site
//! build layer.

mod cli;
#[cfg(feature = "demo")]
mod demo;
mod error;
mod generator;
mod graph;
mod merger;
mod parser;
mod search;
mod validator;

use clap::Parser;

use crate::cli::{Cli, Command, MergesCommand, NewCommand};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(&cli.command)
}

#[allow(clippy::unnecessary_wraps)] // demo feature arm returns Result; stubs will too
fn run(cmd: &Command) -> anyhow::Result<()> {
    match cmd {
        Command::Build => {
            println!("build: not yet implemented");
        }
        Command::Validate => {
            println!("validate: not yet implemented");
        }
        Command::Stats => {
            println!("stats: not yet implemented");
        }
        Command::Search { query } => {
            let _ = query;
            println!("search: not yet implemented");
        }
        Command::Dev => {
            println!("dev: not yet implemented");
        }
        Command::New(sub) => match sub {
            NewCommand::Source { title } => {
                let _ = title;
                println!("new source: not yet implemented");
            }
            NewCommand::Concept { title } => {
                let _ = title;
                println!("new concept: not yet implemented");
            }
        },
        Command::Extract { path } => {
            let _ = path;
            println!("extract: not yet implemented");
        }
        Command::Merges(sub) => match sub {
            MergesCommand::Pending => {
                println!("merges pending: not yet implemented");
            }
            MergesCommand::Accept { id } => {
                let _ = id;
                println!("merges accept: not yet implemented");
            }
        },
        #[cfg(feature = "demo")]
        Command::Demo(sub) => demo::run(sub)?,
    }
    Ok(())
}
