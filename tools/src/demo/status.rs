//! Implementation of `haleiki demo status`.

use std::path::Path;

use super::manifest::Manifest;

/// The fetch state of an article on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchState {
    /// No staging HTML or source Markdown exists.
    Missing,
    /// Staging HTML exists but not yet converted to Markdown.
    Staged,
    /// Converted source Markdown exists.
    Converted,
}

impl std::fmt::Display for FetchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(f, "missing"),
            Self::Staged => write!(f, "staged"),
            Self::Converted => write!(f, "converted"),
        }
    }
}

/// Determine the fetch state of an article by checking disk artifacts.
fn fetch_state(slug: &str) -> FetchState {
    let source_path = Path::new("demo/sources").join(format!("{slug}.md"));
    if source_path.exists() {
        return FetchState::Converted;
    }
    let staging_path = Path::new("demo/.staging").join(format!("{slug}.html"));
    if staging_path.exists() {
        return FetchState::Staged;
    }
    FetchState::Missing
}

/// Run the `haleiki demo status` subcommand.
///
/// # Errors
///
/// Returns an error if the manifest file cannot be found or parsed.
pub fn run() -> anyhow::Result<()> {
    let manifest_path = Path::new("demo/manifest.yaml");

    if !manifest_path.exists() {
        anyhow::bail!(
            "manifest not found at {}\n\
             Hint: run this command from the repository root",
            manifest_path.display()
        );
    }

    let manifest = Manifest::from_file(manifest_path)?;

    // Validate and report issues
    let issues = manifest.validate();
    if !issues.is_empty() {
        eprintln!("Manifest validation issues:");
        for issue in &issues {
            eprintln!("  - {issue}");
        }
        eprintln!();
    }

    // Print article table
    println!();
    println!(
        "  {:<35} {:<25} {:<15} {:<10}",
        "SLUG", "CATEGORY", "TIER", "STATUS"
    );
    println!("  {}", "-".repeat(85));

    let mut converted_count = 0;
    let mut staged_count = 0;
    let mut missing_count = 0;

    for article in &manifest.articles {
        let state = fetch_state(&article.slug);
        let project = manifest.effective_project(article);

        match state {
            FetchState::Converted => converted_count += 1,
            FetchState::Staged => staged_count += 1,
            FetchState::Missing => missing_count += 1,
        }

        // Show project domain only if it differs from default
        let project_indicator = if project == manifest.defaults.project {
            String::new()
        } else {
            format!(" ({project})")
        };

        println!(
            "  {:<35} {:<25} {:<15} {}{}",
            article.slug, article.category, article.tier, state, project_indicator,
        );
    }

    println!("  {}", "-".repeat(85));
    println!(
        "  Total: {} articles ({} converted, {} staged, {} missing)",
        manifest.articles.len(),
        converted_count,
        staged_count,
        missing_count,
    );
    println!();

    Ok(())
}
