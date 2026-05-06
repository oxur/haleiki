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
#[allow(dead_code)]
mod graph;
mod merger;
#[allow(dead_code)]
mod parser;
mod search;
mod validator;

use std::path::Path;

use clap::Parser;

use crate::cli::{Cli, Command, MergesCommand, NewCommand};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(&cli.command)
}

fn run(cmd: &Command) -> anyhow::Result<()> {
    match cmd {
        Command::Build {
            content_dir,
            output_dir,
        } => {
            let output = output_dir
                .clone()
                .unwrap_or_else(|| content_dir.join("_data"));
            run_build(content_dir, &output)?;
        }
        Command::Validate { content_dir } => {
            run_validate(content_dir)?;
        }
        Command::Stats { content_dir } => {
            run_stats(content_dir)?;
        }
        Command::Search => {
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

/// Run the full build pipeline: parse → validate → graph → generate.
fn run_build(content_dir: &Path, output_dir: &Path) -> anyhow::Result<()> {
    eprintln!("Parsing corpus from {}...", content_dir.display());
    let corpus = parser::parse_corpus(content_dir)?;
    eprintln!(
        "  Parsed {} sources, {} concepts",
        corpus.sources.len(),
        corpus.concepts.len()
    );

    eprintln!("Validating...");
    let report = validator::validate(&corpus, content_dir);
    if report.has_errors() {
        report.print();
        anyhow::bail!("Validation failed with {} error(s).", report.errors.len());
    }
    if report.warnings.is_empty() {
        eprintln!("  No issues found.");
    } else {
        report.print();
        eprintln!("  {} warning(s) (non-blocking)", report.warnings.len());
    }

    eprintln!("Building graph...");
    let hgraph = graph::HaleikiGraph::build(&corpus);
    let stats = hgraph.stats(&corpus);
    eprintln!("  {} nodes, {} edges", stats.total_nodes, stats.total_edges);

    eprintln!("Generating data to {}...", output_dir.display());
    generator::generate(&corpus, &hgraph, output_dir)?;
    eprintln!("  Done.");

    Ok(())
}

/// Run validation only and report results.
fn run_validate(content_dir: &Path) -> anyhow::Result<()> {
    eprintln!("Parsing corpus from {}...", content_dir.display());
    let corpus = parser::parse_corpus(content_dir)?;
    eprintln!(
        "  Parsed {} sources, {} concepts",
        corpus.sources.len(),
        corpus.concepts.len()
    );

    eprintln!("Validating...");
    let report = validator::validate(&corpus, content_dir);

    if report.has_errors() {
        report.print();
        eprintln!(
            "\n{} error(s), {} warning(s)",
            report.errors.len(),
            report.warnings.len()
        );
        anyhow::bail!("Validation failed.");
    }

    if !report.warnings.is_empty() {
        report.print();
    }

    eprintln!(
        "Validation passed: 0 errors, {} warning(s).",
        report.warnings.len()
    );
    Ok(())
}

/// Show statistics about the knowledge base.
fn run_stats(content_dir: &Path) -> anyhow::Result<()> {
    let corpus = parser::parse_corpus(content_dir)?;
    let hgraph = graph::HaleikiGraph::build(&corpus);
    let stats = hgraph.stats(&corpus);
    let forward_refs = validator::count_forward_refs(&corpus);

    println!("Knowledge Base Statistics");
    println!("========================");
    println!("Sources:              {}", stats.source_count);
    println!("Concepts:             {}", stats.concept_count);
    println!("Total nodes:          {}", stats.total_nodes);
    println!("Total edges:          {}", stats.total_edges);
    println!("  Prerequisite:       {}", stats.prerequisite_edges);
    println!("  Extends:            {}", stats.extends_edges);
    println!("  Related:            {}", stats.related_edges);
    println!("  Contrasts with:     {}", stats.contrasts_with_edges);
    println!("  Derived from:       {}", stats.derived_from_edges);
    println!("  Concepts generated: {}", stats.concepts_generated_edges);
    println!("Orphan nodes:         {}", stats.orphan_count);
    println!("Categories:           {}", corpus.taxonomy.categories.len());
    println!("Tiers:                {}", corpus.taxonomy.tiers.len());
    if forward_refs > 0 {
        println!(
            "Forward references:   {forward_refs} (typed-relationship slugs referencing content not yet in the corpus)"
        );
    }

    Ok(())
}
