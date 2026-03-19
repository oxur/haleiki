# Milestone 5.3 — Frontmatter Injection (`frontmatter.rs`)

**Version:** 1.0
**Depends on:** Milestone 5.2 (staged Markdown exists), Milestone 1.3 (manifest data), Milestone 4.2 (fetch metadata with revision IDs)
**Produces:** Complete Haleiki source page `.md` files in `demo/sources/`

---

## Overview

This is the final step of the demo fetch pipeline. Take the staged Markdown (from 5.2), generate YAML frontmatter from the manifest entry + fetch metadata, prepend it to the Markdown body, and write the final `.md` file to `demo/sources/{slug}.md`.

After this milestone, the demo has real source pages ready for `haleiki build` and Cobalt.

### Frontmatter schema

From the architecture doc (section 3.1) and demo design doc (section 7):

```yaml
---
# === CORE IDENTIFICATION ===
title: "Garbage Collection (Computer Science)"
slug: "garbage-collection"
page_type: "source"

# === CLASSIFICATION ===
category: "memory-management"
subcategory: "automatic-memory"
tier: "intermediate"
keywords:
  - "GC"
  - "mark-and-sweep"
  - "tracing"
tags:
  - "runtime"
  - "automatic-memory"

# === AUTHORSHIP ===
author: "Wikipedia contributors"
date: "2026-03-18"

# === ORIGINAL SOURCE ===
original_source:
  title: "Garbage collection (computer science)"
  project: "en.wikipedia.org"
  url: "https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)"
  license: "CC BY-SA 4.0"
  fetched_at: "2026-03-18T14:30:00Z"
  revision_id: "1234567890"

# === EXTRACTION STATUS ===
extraction_status: "pending"
concepts_generated: []

# === METADATA ===
status: "published"
---
```

---

## Step 1: Create `tools/src/demo/frontmatter.rs`

### File: `tools/src/demo/frontmatter.rs`

```rust
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
pub fn inject_frontmatter(
    slug: &str,
    manifest: &Manifest,
) -> anyhow::Result<PathBuf> {
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
    eprintln!(
        "Frontmatter injection: {written} written, {skipped} skipped, {failed} failed",
    );

    if failed > 0 {
        anyhow::bail!("{failed} article(s) failed frontmatter injection");
    }

    Ok(written)
}
```

---

## Step 2: Wire into `demo/mod.rs`

### Update module declarations

```rust
pub mod clean;
pub mod convert;
pub mod fetch;
pub mod frontmatter;
pub mod manifest;
pub mod media;
pub mod rewrite;
pub mod status;
```

### Optional hidden dev command

```rust
/// [Dev] Inject frontmatter into staged Markdown
#[command(hide = true)]
Frontmatter {
    /// Article slug (omit to process all)
    slug: Option<String>,
},
```

With handler:

```rust
DemoCommand::Frontmatter { slug } => {
    let manifest_path = Path::new("demo/manifest.yaml");
    let manifest = manifest::Manifest::from_file(manifest_path)?;
    if let Some(slug) = slug {
        frontmatter::inject_frontmatter(&slug, &manifest)?;
    } else {
        frontmatter::inject_all(&manifest)?;
    }
}
```

---

## Step 3: Update `haleiki demo status` for final state

The `FetchState::Converted` check in `status.rs` already looks for `demo/sources/{slug}.md`. Once frontmatter injection writes there, articles will show `converted` in the status output. No code changes needed — the existing `fetch_state()` function handles it:

```rust
fn fetch_state(slug: &str) -> FetchState {
    let source_path = Path::new("demo/sources").join(format!("{slug}.md"));
    if source_path.exists() {
        return FetchState::Converted;  // ← This fires after 5.3
    }
    // ... other states ...
}
```

---

## Step 4: Write tests

### Unit tests in `tools/src/demo/frontmatter.rs`

```rust
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

        assert_eq!(fm.original_source.title, "Garbage collection (computer science)");
        assert_eq!(fm.original_source.project, "en.wikipedia.org");
        assert_eq!(fm.original_source.license, "CC BY-SA 4.0");
        assert_eq!(fm.original_source.fetched_at, "2026-03-18T14:30:00Z");
        assert_eq!(fm.original_source.revision_id, Some("1234567890".to_string()));
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
        assert!(yaml.contains("original_source:"), "Missing original_source: {yaml}");
        assert!(yaml.contains("extraction_status:"), "Missing extraction_status: {yaml}");
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

        assert!(!yaml.contains("subcategory:"), "Should skip None subcategory: {yaml}");
        assert!(!yaml.contains("keywords:"), "Should skip empty keywords: {yaml}");
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
            .strip_prefix("---\n").unwrap()
            .strip_suffix("---\n").unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(inner).unwrap();

        assert_eq!(
            parsed["slug"].as_str().unwrap(),
            "garbage-collection",
        );
        assert_eq!(
            parsed["page_type"].as_str().unwrap(),
            "source",
        );
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
        assert_eq!(url, "https://www.rigpawiki.org/index.php?title=Gyatrul_Rinpoche");
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
            let staged = super::convert::staging_markdown_path(&article.slug);
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

                assert!(content.starts_with("---\n"), "Missing frontmatter delimiter");
                assert!(content.contains("page_type: source"), "Missing page_type");
                assert!(content.contains(&format!("slug: {}", article.slug)), "Missing slug");

                eprintln!("OK: {} ({} bytes)", article.slug, content.len());

                // Clean up
                let _ = std::fs::remove_file(&path);
                return;
            }
        }

        eprintln!("No articles with staged Markdown found, skipping");
    }
}
```

---

## Step 5: Source page path in `.gitignore`

The demo design doc (section 10) says demo content IS committed. `demo/sources/*.md` should NOT be gitignored — these are the final committed source pages.

Verify that `demo/sources/` is not in `.gitignore`. It shouldn't be, but check.

---

## Verification

### 6.1: Inject frontmatter for a single article

```bash
cd /Users/oubiwann/lab/oxur/haleiki

# Single article (requires staged .md + .meta.json)
cargo run --features demo -- frontmatter dzogchen

# Inspect
head -30 demo/sources/dzogchen.md
```

Expected:
```yaml
---
title: Dzogchen
slug: dzogchen
page_type: source
category: tibetan-buddhism
subcategory: practice
tier: foundational
keywords:
- great perfection
- rigpa
...
---

## Overview

Dzogchen is ...
```

### 6.2: Inject all articles

```bash
cargo run --features demo -- frontmatter
```

Expected: source pages written to `demo/sources/` for every article with staged Markdown.

### 6.3: Status shows "converted"

```bash
cargo run --features demo -- demo status
```

Articles with source pages in `demo/sources/` should show `converted`.

### 6.4: Frontmatter is valid YAML

```bash
# Parse every source page's frontmatter
for f in demo/sources/*.md; do
    python3 -c "
import yaml
content = open('$f').read()
parts = content.split('---', 2)
if len(parts) >= 3:
    yaml.safe_load(parts[1])
    print(f'OK: $f')
else:
    print(f'FAIL: $f — no frontmatter delimiters')
"
done
```

### 6.5: Article count matches

```bash
ls demo/sources/*.md | wc -l
# Should match the number of articles that completed the full pipeline
```

### 6.6: Tests pass

```bash
cargo test --features demo
make lint
```

---

## Acceptance Criteria

- [ ] `tools/src/demo/frontmatter.rs` implements frontmatter generation
- [ ] `SourceFrontmatter` struct matches the architecture doc schema (section 3.1)
- [ ] `build_frontmatter()` populates all fields from manifest + fetch metadata
- [ ] `page_type` is always `"source"`
- [ ] `author` varies by project: Wikipedia/Wikibooks/Rigpa Wiki contributors
- [ ] `date` extracted from fetch timestamp (YYYY-MM-DD)
- [ ] `subcategory` included when present, omitted when None
- [ ] Empty `tags` and `keywords` lists omitted from YAML (not serialized as `[]`)
- [ ] `original_source.url` correct for Wikipedia and Rigpa Wiki URL patterns
- [ ] `original_source.revision_id` included when available
- [ ] `extraction_status` is `"pending"`, `concepts_generated` is empty
- [ ] `status` is `"published"`
- [ ] `serialize_frontmatter()` wraps YAML in `---` delimiters
- [ ] `inject_frontmatter()` combines frontmatter + body, writes to `demo/sources/{slug}.md`
- [ ] `inject_all()` processes all articles with staged Markdown
- [ ] Existing source pages are skipped (not overwritten)
- [ ] Missing fetch metadata produces helpful error
- [ ] `haleiki demo status` shows `converted` for articles with source pages
- [ ] All unit tests pass (20+ tests)
- [ ] `make lint` passes

---

## Gotchas

1. **`serde_yaml` serialization order**: `serde_yaml` serializes struct fields in declaration order by default. The `SourceFrontmatter` struct is ordered to match the architecture doc's frontmatter sections (core → classification → authorship → original source → extraction → metadata).

2. **`skip_serializing_if`**: Used on `subcategory`, `keywords`, `tags` to keep the YAML clean. Without this, empty vectors serialize as `keywords: []` which is valid but noisy.

3. **`serde_yaml` leading `---`**: Some versions of `serde_yaml` emit a leading `---` and some don't. The `serialize_frontmatter()` function normalizes this by stripping any existing `---` prefix before adding its own. Double-check the output format.

4. **Revision ID for Rigpa Wiki**: The MediaWiki `action=parse` API returns `revid` as an integer. The `FetchMeta.revision_id` stores it as `Option<String>`. The conversion from integer to string happens in milestone 2.3's `fetch_mediawiki_article()`.

5. **Title casing**: The `title` field in frontmatter preserves the original Wikipedia title including disambiguation parentheticals: `"Garbage collection (computer science)"`. Don't title-case or modify it.

6. **Unicode in YAML**: Titles like `"Chögyam Trungpa"` and `"Ólafur Arnalds"` contain non-ASCII characters. `serde_yaml` handles these correctly — it outputs UTF-8, not escaped sequences.

7. **Date extraction**: The `date` field is extracted from `fetched_at` by splitting on `T`. This is a simple approach that works for ISO 8601 timestamps. If `fetched_at` ever has a different format, this will break.

8. **`demo/sources/` directory**: `inject_frontmatter()` calls `create_dir_all(SOURCES_DIR)`. This creates the directory if it doesn't exist (milestone 1.1 created it with `.gitkeep`, but just in case).

9. **Pipeline completeness**: This milestone assumes the full pipeline has been run: fetch → clean → rewrite → media → convert → frontmatter. If any step is missing, `inject_frontmatter()` will fail with a descriptive error. Each step checks for its prerequisite files.
