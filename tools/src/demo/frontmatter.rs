//! Frontmatter generation for demo source pages.
//!
//! Generates YAML frontmatter from manifest entries and fetch metadata,
//! prepends it to staged Markdown, and writes the final source page
//! to `demo/sources/{slug}.md`.

use std::path::{Path, PathBuf};

use serde::Serialize;

use super::fetch::{self, FetchMeta};
use super::manifest::{Article, Manifest};

/// Directory where final source pages are written.
const SOURCES_DIR: &str = "demo/sources";

/// Frontmatter structure for a Haleiki source page.
///
/// Serialized as YAML between `---` delimiters at the top of the `.md` file.
#[derive(Debug, Clone, Serialize)]
pub struct SourceFrontmatter {
    // === CORE IDENTIFICATION ===
    pub title: String,
    pub slug: String,
    pub page_type: String,

    // === CLASSIFICATION ===
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    pub tier: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    // === AUTHORSHIP ===
    pub author: String,
    pub date: String,

    // === ORIGINAL SOURCE ===
    pub original_source: OriginalSource,

    // === EXTRACTION STATUS ===
    pub extraction_status: String,
    pub concepts_generated: Vec<String>,

    // === METADATA ===
    pub status: String,
}

/// Original source attribution block.
#[derive(Debug, Clone, Serialize)]
pub struct OriginalSource {
    /// Original article title on the source wiki.
    pub title: String,

    /// Source project domain.
    pub project: String,

    /// Full URL to the original article.
    pub url: String,

    /// Content license.
    pub license: String,

    /// ISO 8601 timestamp of when the article was fetched.
    pub fetched_at: String,

    /// Revision ID from the source wiki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

/// Build frontmatter for a source page from manifest + fetch metadata.
pub fn build_frontmatter(
    manifest: &Manifest,
    article: &Article,
    fetch_meta: &FetchMeta,
) -> SourceFrontmatter {
    let project = manifest.effective_project(article);
    let license = manifest.effective_license(article);

    // Build the canonical URL to the original article
    let original_url = build_original_url(project, &article.title);

    // Extract the date (YYYY-MM-DD) from the fetch timestamp
    let date = fetch_meta
        .fetched_at
        .split('T')
        .next()
        .unwrap_or(&fetch_meta.fetched_at)
        .to_string();

    // Author depends on source
    let author = if project.contains("wikipedia.org") {
        "Wikipedia contributors".to_string()
    } else if project.contains("wikibooks.org") {
        "Wikibooks contributors".to_string()
    } else if project.contains("rigpawiki.org") {
        "Rigpa Wiki contributors".to_string()
    } else {
        format!("{project} contributors")
    };

    SourceFrontmatter {
        title: article.title.clone(),
        slug: article.slug.clone(),
        page_type: "source".to_string(),

        category: article.category.clone(),
        subcategory: article.subcategory.clone(),
        tier: article.tier.clone(),
        keywords: article.keywords.clone(),
        tags: article.tags.clone(),

        author,
        date,

        original_source: OriginalSource {
            title: article.title.clone(),
            project: project.to_string(),
            url: original_url,
            license: license.to_string(),
            fetched_at: fetch_meta.fetched_at.clone(),
            revision_id: fetch_meta.revision_id.clone(),
        },

        extraction_status: "pending".to_string(),
        concepts_generated: Vec::new(),

        status: "published".to_string(),
    }
}

/// Build the canonical URL to view an article on its source wiki.
///
/// Wikipedia: `https://en.wikipedia.org/wiki/Article_Title`
/// Rigpa Wiki: `https://www.rigpawiki.org/index.php?title=Article_Title`
fn build_original_url(project: &str, title: &str) -> String {
    let encoded_title = title.replace(' ', "_");

    if project.contains("wikipedia.org")
        || project.contains("wikibooks.org")
        || project.contains("wikiversity.org")
        || project.contains("wikimedia.org")
    {
        format!("https://{project}/wiki/{encoded_title}")
    } else {
        // Generic MediaWiki — use index.php?title= format
        format!("https://{project}/index.php?title={encoded_title}")
    }
}

/// Serialize frontmatter as a YAML string between `---` delimiters.
pub fn serialize_frontmatter(fm: &SourceFrontmatter) -> anyhow::Result<String> {
    let yaml = serde_yaml::to_string(fm)?;

    // serde_yaml produces a leading `---\n` already in some versions,
    // but not always. Normalize to ensure consistent output.
    let yaml = yaml.trim_start_matches("---").trim_start_matches('\n');

    Ok(format!("---\n{yaml}---\n"))
}

/// Load the fetch metadata for an article from its `.meta.json` file.
fn load_fetch_meta(slug: &str) -> anyhow::Result<FetchMeta> {
    let meta_path = fetch::staging_meta_path(slug);
    if !meta_path.exists() {
        anyhow::bail!(
            "fetch metadata not found at {}. Run `haleiki demo fetch` first.",
            meta_path.display(),
        );
    }
    let content = std::fs::read_to_string(&meta_path)?;
    let meta: FetchMeta = serde_json::from_str(&content)?;
    Ok(meta)
}

/// Path where the final source page is written.
pub fn source_page_path(slug: &str) -> PathBuf {
    Path::new(SOURCES_DIR).join(format!("{slug}.md"))
}

/// Inject frontmatter into staged Markdown and write the final source page.
///
/// Reads `demo/.staging/{slug}.md` (staged Markdown without frontmatter),
/// prepends YAML frontmatter, writes `demo/sources/{slug}.md`.
pub fn inject_frontmatter(slug: &str, manifest: &Manifest) -> anyhow::Result<PathBuf> {
    // Load staged Markdown
    let staged_md_path = super::convert::staging_markdown_path(slug);
    if !staged_md_path.exists() {
        anyhow::bail!(
            "staged Markdown not found at {}. Run conversion first.",
            staged_md_path.display(),
        );
    }
    let body = std::fs::read_to_string(&staged_md_path)?;

    // Find the article in the manifest
    let article = manifest
        .articles
        .iter()
        .find(|a| a.slug == slug)
        .ok_or_else(|| anyhow::anyhow!("article \"{slug}\" not found in manifest"))?;

    // Load fetch metadata
    let fetch_meta = load_fetch_meta(slug)?;

    // Build and serialize frontmatter
    let fm = build_frontmatter(manifest, article, &fetch_meta);
    let fm_yaml = serialize_frontmatter(&fm)?;

    // Combine frontmatter + body
    let full_page = format!("{fm_yaml}\n{body}");

    // Write to final location
    let output_path = source_page_path(slug);
    std::fs::create_dir_all(SOURCES_DIR)?;
    std::fs::write(&output_path, &full_page)?;

    Ok(output_path)
}

/// Inject frontmatter for all articles that have staged Markdown.
///
/// Returns the number of source pages written.
pub fn inject_all(manifest: &Manifest) -> anyhow::Result<usize> {
    std::fs::create_dir_all(SOURCES_DIR)?;

    let mut written = 0;
    let mut failed = 0;
    let mut skipped = 0;

    for article in &manifest.articles {
        let slug = &article.slug;

        // Check if staged Markdown exists
        let staged_path = super::convert::staging_markdown_path(slug);
        if !staged_path.exists() {
            eprintln!("  {slug}: no staged Markdown, skipping");
            skipped += 1;
            continue;
        }

        // Check if already written (skip unless force)
        let output_path = source_page_path(slug);
        if output_path.exists() {
            eprintln!("  {slug}: already exists, skipping");
            skipped += 1;
            continue;
        }

        match inject_frontmatter(slug, manifest) {
            Ok(path) => {
                written += 1;
                eprintln!("  {slug}: wrote {}", path.display());
            }
            Err(e) => {
                eprintln!("  {slug}: FAILED — {e}");
                failed += 1;
            }
        }
    }

    eprintln!();
    eprintln!("Frontmatter injection: {written} written, {skipped} skipped, {failed} failed",);

    if failed > 0 {
        anyhow::bail!("{failed} article(s) failed frontmatter injection");
    }

    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo::manifest::*;

    fn sample_article() -> Article {
        Article {
            title: "Garbage collection (computer science)".to_string(),
            slug: "garbage-collection".to_string(),
            category: "memory-management".to_string(),
            subcategory: Some("automatic-memory".to_string()),
            tier: "intermediate".to_string(),
            project: None,
            license: None,
            tags: vec!["runtime".to_string(), "automatic-memory".to_string()],
            keywords: vec!["GC".to_string(), "mark-and-sweep".to_string()],
            media: None,
        }
    }

    fn sample_manifest() -> Manifest {
        Manifest {
            defaults: Defaults {
                project: "en.wikipedia.org".to_string(),
                license: "CC BY-SA 4.0".to_string(),
                media: MediaDefaults {
                    max_width: 1024,
                    formats: vec!["svg".to_string()],
                    skip_patterns: vec![],
                },
            },
            taxonomy: Taxonomy {
                categories: vec!["memory-management".to_string()],
                tiers: vec!["intermediate".to_string()],
            },
            articles: vec![sample_article()],
        }
    }

    fn sample_fetch_meta() -> FetchMeta {
        FetchMeta {
            slug: "garbage-collection".to_string(),
            project: "en.wikipedia.org".to_string(),
            title: "Garbage collection (computer science)".to_string(),
            api_url: "https://en.wikipedia.org/api/rest_v1/page/html/Garbage_collection_(computer_science)".to_string(),
            revision_id: Some("1234567890".to_string()),
            fetched_at: "2026-03-18T14:30:00Z".to_string(),
            http_status: 200,
            html_bytes: 50000,
        }
    }

    // ─── build_frontmatter tests ────────────────────────

    #[test]
    fn test_build_frontmatter_core_fields() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());

        assert_eq!(fm.title, "Garbage collection (computer science)");
        assert_eq!(fm.slug, "garbage-collection");
        assert_eq!(fm.page_type, "source");
    }

    #[test]
    fn test_build_frontmatter_classification() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());

        assert_eq!(fm.category, "memory-management");
        assert_eq!(fm.subcategory, Some("automatic-memory".to_string()));
        assert_eq!(fm.tier, "intermediate");
        assert_eq!(fm.keywords, vec!["GC", "mark-and-sweep"]);
        assert_eq!(fm.tags, vec!["runtime", "automatic-memory"]);
    }

    #[test]
    fn test_build_frontmatter_author_wikipedia() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());

        assert_eq!(fm.author, "Wikipedia contributors");
    }

    #[test]
    fn test_build_frontmatter_author_rigpawiki() {
        let m = sample_manifest();
        let mut a = sample_article();
        a.project = Some("www.rigpawiki.org".to_string());
        let mut meta = sample_fetch_meta();
        meta.project = "www.rigpawiki.org".to_string();

        let fm = build_frontmatter(&m, &a, &meta);
        assert_eq!(fm.author, "Rigpa Wiki contributors");
    }

    #[test]
    fn test_build_frontmatter_author_wikibooks() {
        let m = sample_manifest();
        let mut a = sample_article();
        a.project = Some("en.wikibooks.org".to_string());
        let mut meta = sample_fetch_meta();
        meta.project = "en.wikibooks.org".to_string();

        let fm = build_frontmatter(&m, &a, &meta);
        assert_eq!(fm.author, "Wikibooks contributors");
    }

    #[test]
    fn test_build_frontmatter_date_from_fetch_timestamp() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());

        assert_eq!(fm.date, "2026-03-18");
    }

    #[test]
    fn test_build_frontmatter_original_source() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());

        assert_eq!(
            fm.original_source.title,
            "Garbage collection (computer science)"
        );
        assert_eq!(fm.original_source.project, "en.wikipedia.org");
        assert_eq!(fm.original_source.license, "CC BY-SA 4.0");
        assert_eq!(fm.original_source.fetched_at, "2026-03-18T14:30:00Z");
        assert_eq!(
            fm.original_source.revision_id,
            Some("1234567890".to_string())
        );
    }

    #[test]
    fn test_build_frontmatter_original_source_url_wikipedia() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());

        assert_eq!(
            fm.original_source.url,
            "https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)",
        );
    }

    #[test]
    fn test_build_frontmatter_original_source_url_rigpawiki() {
        let m = sample_manifest();
        let mut a = sample_article();
        a.project = Some("www.rigpawiki.org".to_string());
        a.title = "Longchenpa".to_string();
        let mut meta = sample_fetch_meta();
        meta.project = "www.rigpawiki.org".to_string();

        let fm = build_frontmatter(&m, &a, &meta);
        assert_eq!(
            fm.original_source.url,
            "https://www.rigpawiki.org/index.php?title=Longchenpa",
        );
    }

    #[test]
    fn test_build_frontmatter_extraction_status() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());

        assert_eq!(fm.extraction_status, "pending");
        assert!(fm.concepts_generated.is_empty());
        assert_eq!(fm.status, "published");
    }

    #[test]
    fn test_build_frontmatter_no_subcategory() {
        let m = sample_manifest();
        let mut a = sample_article();
        a.subcategory = None;
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());

        assert_eq!(fm.subcategory, None);
    }

    #[test]
    fn test_build_frontmatter_license_override() {
        let mut m = sample_manifest();
        let mut a = sample_article();
        a.license = Some("CC BY-SA 3.0".to_string());
        m.articles = vec![a.clone()];

        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());
        assert_eq!(fm.original_source.license, "CC BY-SA 3.0");
    }

    // ─── serialize_frontmatter tests ────────────────────

    #[test]
    fn test_serialize_frontmatter_has_delimiters() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());
        let yaml = serialize_frontmatter(&fm).unwrap();

        assert!(yaml.starts_with("---\n"), "Should start with ---: {yaml}");
        assert!(yaml.ends_with("---\n"), "Should end with ---: {yaml}");
    }

    #[test]
    fn test_serialize_frontmatter_contains_required_fields() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());
        let yaml = serialize_frontmatter(&fm).unwrap();

        assert!(yaml.contains("title:"), "Missing title: {yaml}");
        assert!(yaml.contains("slug:"), "Missing slug: {yaml}");
        assert!(yaml.contains("page_type:"), "Missing page_type: {yaml}");
        assert!(yaml.contains("category:"), "Missing category: {yaml}");
        assert!(yaml.contains("tier:"), "Missing tier: {yaml}");
        assert!(yaml.contains("author:"), "Missing author: {yaml}");
        assert!(yaml.contains("date:"), "Missing date: {yaml}");
        assert!(
            yaml.contains("original_source:"),
            "Missing original_source: {yaml}"
        );
        assert!(
            yaml.contains("extraction_status:"),
            "Missing extraction_status: {yaml}"
        );
        assert!(yaml.contains("status:"), "Missing status: {yaml}");
    }

    #[test]
    fn test_serialize_frontmatter_skips_empty_optional_fields() {
        let m = sample_manifest();
        let mut a = sample_article();
        a.subcategory = None;
        a.tags = vec![];
        a.keywords = vec![];
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());
        let yaml = serialize_frontmatter(&fm).unwrap();

        assert!(
            !yaml.contains("subcategory:"),
            "Should skip None subcategory: {yaml}"
        );
        assert!(
            !yaml.contains("keywords:"),
            "Should skip empty keywords: {yaml}"
        );
        assert!(!yaml.contains("tags:"), "Should skip empty tags: {yaml}");
    }

    #[test]
    fn test_serialize_frontmatter_roundtrip_parseable() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());
        let yaml = serialize_frontmatter(&fm).unwrap();

        // Strip delimiters and parse back
        let inner = yaml
            .strip_prefix("---\n")
            .unwrap()
            .strip_suffix("---\n")
            .unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(inner).unwrap();

        assert_eq!(parsed["slug"].as_str().unwrap(), "garbage-collection",);
        assert_eq!(parsed["page_type"].as_str().unwrap(), "source",);
    }

    // ─── build_original_url tests ───────────────────────

    #[test]
    fn test_build_original_url_wikipedia() {
        let url = build_original_url("en.wikipedia.org", "Quantum mechanics");
        assert_eq!(url, "https://en.wikipedia.org/wiki/Quantum_mechanics");
    }

    #[test]
    fn test_build_original_url_wikipedia_with_parens() {
        let url = build_original_url("en.wikipedia.org", "Garbage collection (computer science)");
        assert_eq!(
            url,
            "https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)",
        );
    }

    #[test]
    fn test_build_original_url_wikibooks() {
        let url = build_original_url("en.wikibooks.org", "Intro/Memory Management");
        assert_eq!(url, "https://en.wikibooks.org/wiki/Intro/Memory_Management");
    }

    #[test]
    fn test_build_original_url_rigpawiki() {
        let url = build_original_url("www.rigpawiki.org", "Longchenpa");
        assert_eq!(url, "https://www.rigpawiki.org/index.php?title=Longchenpa");
    }

    #[test]
    fn test_build_original_url_rigpawiki_with_spaces() {
        let url = build_original_url("www.rigpawiki.org", "Gyatrul Rinpoche");
        assert_eq!(
            url,
            "https://www.rigpawiki.org/index.php?title=Gyatrul_Rinpoche"
        );
    }

    // ─── Full source page test ──────────────────────────

    #[test]
    fn test_full_source_page_structure() {
        let m = sample_manifest();
        let a = sample_article();
        let fm = build_frontmatter(&m, &a, &sample_fetch_meta());
        let fm_yaml = serialize_frontmatter(&fm).unwrap();

        let body = "## Overview\n\nGarbage collection is a form of automatic memory management.\n";
        let full_page = format!("{fm_yaml}\n{body}");

        // Should have frontmatter then body
        assert!(full_page.starts_with("---\n"));
        assert!(full_page.contains("---\n\n## Overview"));
        assert!(full_page.contains("automatic memory management"));

        // Parse the frontmatter portion
        let parts: Vec<&str> = full_page.splitn(3, "---").collect();
        assert_eq!(parts.len(), 3, "Should split into 3 parts on ---");
        // parts[0] = "" (before first ---)
        // parts[1] = frontmatter YAML
        // parts[2] = body (after second ---)

        let fm_parsed: serde_yaml::Value = serde_yaml::from_str(parts[1].trim()).unwrap();
        assert_eq!(fm_parsed["slug"].as_str().unwrap(), "garbage-collection");
    }

    // ─── Real article test ──────────────────────────────

    #[test]
    #[ignore] // Requires full pipeline through 5.2
    fn test_inject_frontmatter_real_article() {
        use std::path::Path;

        let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("demo/manifest.yaml");

        if !manifest_path.exists() {
            eprintln!("No manifest found, skipping");
            return;
        }

        let manifest = Manifest::from_file(&manifest_path).unwrap();

        // Try the first article that has staged Markdown
        for article in &manifest.articles {
            let staged = super::super::convert::staging_markdown_path(&article.slug);
            let meta = fetch::staging_meta_path(&article.slug);

            if staged.exists() && meta.exists() {
                let result = inject_frontmatter(&article.slug, &manifest);
                assert!(
                    result.is_ok(),
                    "Injection failed for {}: {:?}",
                    article.slug,
                    result.err(),
                );

                let path = result.unwrap();
                let content = std::fs::read_to_string(&path).unwrap();

                assert!(
                    content.starts_with("---\n"),
                    "Missing frontmatter delimiter"
                );
                assert!(content.contains("page_type: source"), "Missing page_type");
                assert!(
                    content.contains(&format!("slug: {}", article.slug)),
                    "Missing slug"
                );

                eprintln!("OK: {} ({} bytes)", article.slug, content.len());

                // Clean up
                let _ = std::fs::remove_file(&path);
                return;
            }
        }

        eprintln!("No articles with staged Markdown found, skipping");
    }
}
