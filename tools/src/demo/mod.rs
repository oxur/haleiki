//! Demo site management commands.
//!
//! This module is gated behind the `demo` feature and provides subcommands
//! for fetching, building, serving, and validating a demonstration knowledge
//! base built from publicly licensed content.

pub mod clean;
pub mod convert;
pub mod fetch;
pub mod frontmatter;
pub mod manifest;
pub mod media;
pub mod rewrite;
pub mod status;

use std::path::Path;

use clap::Subcommand;

/// Pipeline stages for the demo content pipeline.
///
/// Controls how far the pipeline runs when invoked via `demo fetch --stage`.
/// Each stage depends on all preceding stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PipelineStage {
    Fetch,
    Clean,
    Rewrite,
    Media,
    Convert,
    Frontmatter,
}

fn parse_stage(s: &str) -> Result<PipelineStage, String> {
    match s {
        "fetch" => Ok(PipelineStage::Fetch),
        "clean" => Ok(PipelineStage::Clean),
        "rewrite" => Ok(PipelineStage::Rewrite),
        "media" => Ok(PipelineStage::Media),
        "convert" => Ok(PipelineStage::Convert),
        "frontmatter" => Ok(PipelineStage::Frontmatter),
        _ => Err(format!(
            "unknown stage '{s}'. Valid stages: fetch, clean, rewrite, media, convert, frontmatter"
        )),
    }
}

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

        /// Stop the pipeline after this stage (default: full pipeline).
        /// Stages: fetch, clean, rewrite, media, convert, frontmatter.
        #[arg(long, value_parser = parse_stage)]
        stage: Option<PipelineStage>,
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

    /// [Dev] Inject frontmatter into staged Markdown.
    #[command(hide = true)]
    Frontmatter {
        /// Article slug (omit to process all).
        slug: Option<String>,
    },

    /// [Dev] Rewrite links in cleaned HTML.
    #[command(hide = true)]
    RewriteLinks {
        /// Article slug.
        slug: String,
    },

    /// [Dev] Process media for a single article.
    #[command(hide = true)]
    MediaProcess {
        /// Article slug.
        slug: String,
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
            stage,
        } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(fetch::run(
                article.as_deref(),
                *dry_run,
                *force,
                *pandoc,
                *stage,
            ))?;
        }
        DemoCommand::Build => println!("demo build: not yet implemented"),
        DemoCommand::Serve => println!("demo serve: not yet implemented"),
        DemoCommand::Status => {
            status::run()?;
        }
        DemoCommand::Validate => println!("demo validate: not yet implemented"),
        DemoCommand::Attribution => println!("demo attribution: not yet implemented"),
        DemoCommand::Clean => println!("demo clean: not yet implemented"),
        DemoCommand::Refresh => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(fetch::run(None, false, true, false, Some(PipelineStage::Frontmatter)))?;
        }
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
        DemoCommand::Frontmatter { slug } => {
            let manifest_path = Path::new("demo/manifest.yaml");
            let manifest = manifest::Manifest::from_file(manifest_path)?;
            if let Some(slug) = slug {
                frontmatter::inject_frontmatter(slug, &manifest)?;
            } else {
                frontmatter::inject_all(&manifest)?;
            }
        }
        DemoCommand::RewriteLinks { slug } => {
            let manifest_path = Path::new("demo/manifest.yaml");
            let manifest = manifest::Manifest::from_file(manifest_path)?;
            let path = rewrite::rewrite_article(slug, &manifest)?;
            println!("Rewritten HTML written to {}", path.display());
        }
        DemoCommand::MediaProcess { slug } => {
            let manifest_path = Path::new("demo/manifest.yaml");
            let manifest = manifest::Manifest::from_file(manifest_path)?;
            let article = manifest
                .articles
                .iter()
                .find(|a| a.slug == *slug)
                .ok_or_else(|| anyhow::anyhow!("article '{slug}' not found in manifest"))?;
            let rt = tokio::runtime::Runtime::new()?;
            let result = rt.block_on(async {
                let client = fetch::build_client()?;
                media::process_article_media(&client, slug, &manifest, article).await
            })?;
            let path = media::rewrite_article_images(slug, &result)?;
            println!("Final HTML written to {}", path.display());
        }
    }
    Ok(())
}
