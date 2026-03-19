//! Demo site management commands.
//!
//! This module is gated behind the `demo` feature and provides subcommands
//! for fetching, building, serving, and validating a demonstration knowledge
//! base built from publicly licensed content.

pub mod manifest;
pub mod status;

use clap::Subcommand;

/// Subcommands for managing the demo site.
#[derive(Debug, Subcommand)]
pub enum DemoCommand {
    /// Fetch source articles for the demo knowledge base.
    Fetch {
        /// Fetch only a specific article by slug.
        #[arg(long)]
        article: Option<String>,

        /// Show what would be fetched without downloading.
        #[arg(long)]
        dry_run: bool,

        /// Re-fetch even if the article already exists locally.
        #[arg(long)]
        force: bool,

        /// Use pandoc for Markdown conversion instead of the built-in converter.
        #[arg(long)]
        pandoc: bool,
    },

    /// Build the demo site from fetched sources.
    Build,

    /// Serve the demo site locally for preview.
    Serve,

    /// Show the status of demo content (fetched, built, etc.).
    Status,

    /// Validate demo site content and references.
    Validate,

    /// Show attribution information for demo content.
    Attribution,

    /// Remove all generated demo artifacts.
    Clean,

    /// Re-fetch and rebuild all demo content.
    Refresh,
}

/// Execute a demo subcommand.
///
/// # Errors
///
/// Returns an error if the subcommand fails.
#[allow(clippy::unnecessary_wraps)] // stubs will return errors once implemented
pub fn run(cmd: &DemoCommand) -> anyhow::Result<()> {
    match cmd {
        DemoCommand::Fetch {
            article,
            dry_run,
            force,
            pandoc,
        } => {
            let _ = (article, dry_run, force, pandoc);
            println!("demo fetch: not yet implemented");
        }
        DemoCommand::Build => println!("demo build: not yet implemented"),
        DemoCommand::Serve => println!("demo serve: not yet implemented"),
        DemoCommand::Status => {
            status::run()?;
        }
        DemoCommand::Validate => println!("demo validate: not yet implemented"),
        DemoCommand::Attribution => println!("demo attribution: not yet implemented"),
        DemoCommand::Clean => println!("demo clean: not yet implemented"),
        DemoCommand::Refresh => println!("demo refresh: not yet implemented"),
    }
    Ok(())
}
