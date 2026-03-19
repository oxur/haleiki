//! Command-line interface definitions for Haleiki.
//!
//! This module defines the top-level CLI structure using `clap`'s derive API.
//! Each variant of [`Command`] corresponds to a major pipeline operation.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Haleiki — a wiki-like markdown- and git-powered tool for structured knowledge.
#[derive(Debug, Parser)]
#[command(name = "haleiki", version, about, long_about = None)]
pub struct Cli {
    /// The command to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Top-level commands exposed by the Haleiki CLI.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Build the knowledge base (graph JSON, indices, derived data).
    Build,

    /// Validate all content references and frontmatter.
    Validate,

    /// Show statistics about the knowledge base.
    Stats,

    /// Generate the search index (wraps Pagefind).
    Search,

    /// Developer utilities (debug, inspect internals).
    Dev,

    /// Create new content.
    #[command(subcommand)]
    New(NewCommand),

    /// Extract concept cards from a source page.
    Extract {
        /// Path to the source page to extract concepts from.
        path: PathBuf,
    },

    /// Manage merge proposals for extracted concepts.
    #[command(subcommand)]
    Merges(MergesCommand),

    /// Demo site management (requires the `demo` feature).
    #[cfg(feature = "demo")]
    #[command(subcommand)]
    Demo(super::demo::DemoCommand),
}

/// Subcommands for creating new content.
#[derive(Debug, Subcommand)]
pub enum NewCommand {
    /// Create a new source page from a template.
    Source {
        /// Title of the new source page.
        title: String,
    },

    /// Create a new concept card from a template.
    Concept {
        /// Title of the new concept card.
        title: String,
    },
}

/// Subcommands for managing merge proposals.
#[derive(Debug, Subcommand)]
pub enum MergesCommand {
    /// List pending merge proposals.
    Pending,

    /// Accept a merge proposal by ID.
    Accept {
        /// The merge proposal identifier.
        id: String,
    },
}
