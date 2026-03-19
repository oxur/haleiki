//! Implementation of `haleiki demo status`.

use std::collections::HashMap;
use std::path::Path;

use super::manifest::Manifest;
use super::media;

/// The fetch state of an article on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchState {
    /// No staging HTML or source Markdown exists.
    Missing,
    /// Staging HTML exists but not yet cleaned.
    Staged,
    /// Cleaned HTML exists but not yet rewritten.
    Cleaned,
    /// Rewritten HTML exists but not yet converted to Markdown.
    Rewritten,
    /// Converted source Markdown exists.
    Converted,
}

impl std::fmt::Display for FetchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(f, "missing"),
            Self::Staged => write!(f, "staged"),
            Self::Cleaned => write!(f, "cleaned"),
            Self::Rewritten => write!(f, "rewritten"),
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
    let rewritten_path = Path::new("demo/.staging").join(format!("{slug}.rewritten.html"));
    if rewritten_path.exists() {
        return FetchState::Rewritten;
    }
    let clean_path = Path::new("demo/.staging").join(format!("{slug}.clean.html"));
    if clean_path.exists() {
        return FetchState::Cleaned;
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
        "  {:<35} {:<35} {:<15} {:<10}",
        "SLUG", "CATEGORY", "TIER", "STATUS"
    );
    println!("  {}", "-".repeat(95));

    let mut converted_count = 0;
    let mut rewritten_count = 0;
    let mut cleaned_count = 0;
    let mut staged_count = 0;
    let mut missing_count = 0;

    for article in &manifest.articles {
        let state = fetch_state(&article.slug);
        let project = manifest.effective_project(article);

        match state {
            FetchState::Converted => converted_count += 1,
            FetchState::Rewritten => rewritten_count += 1,
            FetchState::Cleaned => cleaned_count += 1,
            FetchState::Staged => staged_count += 1,
            FetchState::Missing => missing_count += 1,
        }

        // Show project domain only if it differs from default
        let project_indicator = if project == manifest.defaults.project {
            String::new()
        } else {
            format!(" ({project})")
        };

        let category_display = match &article.subcategory {
            Some(subcat) => format!("{}/{}", article.category, subcat),
            None => article.category.clone(),
        };

        println!(
            "  {:<35} {:<35} {:<15} {}{}",
            article.slug, category_display, article.tier, state, project_indicator,
        );
    }

    println!("  {}", "-".repeat(95));
    println!(
        "  Total: {} articles ({} converted, {} rewritten, {} cleaned, {} staged, {} missing)",
        manifest.articles.len(),
        converted_count,
        rewritten_count,
        cleaned_count,
        staged_count,
        missing_count,
    );
    println!();

    print_media_summary();

    Ok(())
}

/// Print media statistics if the media manifest exists.
fn print_media_summary() {
    let manifest = match media::load_media_manifest() {
        Ok(Some(m)) => m,
        Ok(None) => {
            println!("  Media: no manifest (run fetch + media pipeline first)");
            return;
        }
        Err(e) => {
            eprintln!("  Media manifest error: {e}");
            return;
        }
    };

    println!("  Media:");
    println!(
        "    Total: {} images across {} articles",
        manifest.total_images, manifest.articles_with_media,
    );
    println!(
        "    Size: {} ({})",
        format_bytes(manifest.total_bytes),
        manifest.total_bytes,
    );

    // Per-format breakdown
    let mut by_format: HashMap<&str, (usize, u64)> = HashMap::new();
    for entry in &manifest.images {
        let (count, bytes) = by_format.entry(&entry.format).or_insert((0, 0));
        *count += 1;
        *bytes += entry.size_bytes;
    }
    let mut formats: Vec<_> = by_format.into_iter().collect();
    formats.sort_by_key(|(_, (count, _))| std::cmp::Reverse(*count));
    for (format, (count, bytes)) in &formats {
        println!("      {format}: {count} files ({})", format_bytes(*bytes));
    }

    // Top 5 articles by media count
    let mut by_article: HashMap<&str, (usize, u64)> = HashMap::new();
    for entry in &manifest.images {
        let (count, bytes) = by_article.entry(&entry.source_article).or_insert((0, 0));
        *count += 1;
        *bytes += entry.size_bytes;
    }
    let mut articles: Vec<_> = by_article.into_iter().collect();
    articles.sort_by_key(|(_, (count, _))| std::cmp::Reverse(*count));

    if !articles.is_empty() {
        println!("    Top articles by image count:");
        for (slug, (count, bytes)) in articles.iter().take(5) {
            println!("      {slug}: {count} images ({})", format_bytes(*bytes));
        }
    }

    println!();
}

/// Format a byte count as a human-readable string.
#[allow(clippy::cast_precision_loss)] // byte sizes are always well within f64 mantissa range
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- format_bytes tests ---

    #[test]
    fn test_format_bytes_bytes() {
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn test_format_bytes_kilobytes() {
        assert_eq!(format_bytes(1536), "1.5 KB");
    }

    #[test]
    fn test_format_bytes_megabytes() {
        assert_eq!(format_bytes(5_242_880), "5.0 MB");
    }
}
