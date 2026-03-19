//! Demo site management commands.
//!
//! This module is gated behind the `demo` feature and provides subcommands
//! for fetching, building, serving, and validating a demonstration knowledge
//! base built from publicly licensed content.

pub mod clean;
#[allow(dead_code)] // Not yet wired to a command; used by tests and future pipeline
pub mod convert;
pub mod fetch;
pub mod manifest;
#[allow(dead_code)] // Not yet wired to a command; used by tests and future pipeline
pub mod media;
#[allow(dead_code)] // Not yet wired to a command; used by tests and future pipeline
pub mod rewrite;
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

    /// [Dev] Clean raw HTML for a single article (development tool).
    #[command(hide = true)]
    CleanHtml {
        /// Article slug.
        slug: String,
    },

    /// [Dev] Convert final HTML to Markdown.
    #[command(hide = true)]
    Convert {
        /// Article slug (omit to convert all).
        slug: Option<String>,

        /// Use pandoc instead of the built-in converter.
        #[arg(long)]
        pandoc: bool,
    },
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
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(fetch::run(article.as_deref(), *dry_run, *force, *pandoc))?;
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
        DemoCommand::CleanHtml { slug } => {
            let path = clean::clean_article(slug)?;
            println!("Cleaned HTML written to {}", path.display());
        }
        DemoCommand::Convert { slug, pandoc } => {
            if let Some(slug) = slug {
                let path = convert::reconvert_article(slug, *pandoc)?;
                println!("Markdown written to {}", path.display());
            } else {
                let count = convert::convert_all_articles(*pandoc)?;
                println!("Converted {count} article(s).");
            }
        }
    }
    Ok(())
}
