# Milestone 1.3 — Manifest Parsing and Validation (`manifest.rs`)

**Version:** 1.0
**Depends on:** Milestone 1.1 (manifest.yaml exists), Milestone 1.2 (CLI skeleton compiles)
**Produces:** `haleiki demo status` reads `demo/manifest.yaml` and prints an article table with fetch status

---

## Overview

Implement serde deserialization for the full manifest schema (defaults, taxonomy, article entries with per-article overrides), write a `validate_manifest()` function that catches common errors, and wire `haleiki demo status` as the first real (non-stub) subcommand.

---

## Step 1: Define manifest types in `tools/src/demo/manifest.rs`

The types must match the YAML schema exactly from design doc section 3. Key design decisions:

- Use `#[serde(default)]` for optional fields so articles inherit from `defaults`
- The `Manifest` struct owns a `resolve()` or `resolved_article()` method that merges defaults into per-article values
- Types derive `Debug, Clone, PartialEq` (Rust idiom) and `Serialize, Deserialize`

### File: `tools/src/demo/manifest.rs`

```rust
//! Demo manifest parsing and validation.
//!
//! Deserializes `demo/manifest.yaml` and validates article entries
//! against the embedded taxonomy.

use std::collections::HashSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// The root manifest structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    /// Default settings inherited by all articles unless overridden.
    pub defaults: Defaults,

    /// Taxonomy definition (categories and tiers).
    pub taxonomy: Taxonomy,

    /// The list of articles to fetch and convert.
    pub articles: Vec<Article>,
}

/// Default settings that articles inherit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Defaults {
    /// Wikimedia project domain (e.g., "en.wikipedia.org").
    pub project: String,

    /// Content license.
    pub license: String,

    /// Media download settings.
    pub media: MediaDefaults,
}

/// Default media settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaDefaults {
    /// Maximum width in pixels for thumbnail downloads.
    pub max_width: u32,

    /// Preferred image formats in priority order.
    pub formats: Vec<String>,

    /// Glob patterns for images to skip.
    pub skip_patterns: Vec<String>,
}

/// Taxonomy definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taxonomy {
    /// Valid category names.
    pub categories: Vec<String>,

    /// Valid tier names.
    pub tiers: Vec<String>,
}

/// A single article entry in the manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Article {
    /// Exact Wikimedia page title (including disambiguation).
    pub title: String,

    /// Haleiki slug — becomes the URL and filename.
    pub slug: String,

    /// Content category (must match taxonomy).
    pub category: String,

    /// Knowledge tier (must match taxonomy).
    pub tier: String,

    /// Override: Wikimedia project domain.
    #[serde(default)]
    pub project: Option<String>,

    /// Override: content license.
    #[serde(default)]
    pub license: Option<String>,

    /// Content tags.
    #[serde(default)]
    pub tags: Vec<String>,

    /// Search keywords.
    #[serde(default)]
    pub keywords: Vec<String>,

    /// Per-article media overrides.
    #[serde(default)]
    pub media: Option<ArticleMedia>,
}

/// Per-article media overrides.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArticleMedia {
    /// Additional skip patterns for this article.
    #[serde(default)]
    pub skip_patterns: Vec<String>,

    /// Force-include specific filenames even if they match a skip pattern.
    #[serde(default)]
    pub include: Vec<String>,

    /// Force-exclude specific filenames.
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl Manifest {
    /// Load and parse a manifest from a YAML file.
    pub fn from_file(path: &Path) -> crate::error::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest: Self = serde_yaml::from_str(&content)?;
        Ok(manifest)
    }

    /// Get the effective project domain for an article (article override or default).
    pub fn effective_project(&self, article: &Article) -> &str {
        article
            .project
            .as_deref()
            .unwrap_or(&self.defaults.project)
    }

    /// Get the effective license for an article (article override or default).
    pub fn effective_license(&self, article: &Article) -> &str {
        article
            .license
            .as_deref()
            .unwrap_or(&self.defaults.license)
    }

    /// Build the Wikimedia REST API URL for an article.
    pub fn api_url(&self, article: &Article) -> String {
        let project = self.effective_project(article);
        let encoded_title = article.title.replace(' ', "_");
        format!("https://{project}/api/rest_v1/page/html/{encoded_title}")
    }
}
```

---

## Step 2: Implement manifest validation

Add a `validate()` method to `Manifest` and a `ValidationError` type that collects all problems (not just the first one).

### Add to `tools/src/demo/manifest.rs`

```rust
/// A validation problem found in the manifest.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationIssue {
    /// Which article (by slug) has the problem, or None for manifest-level issues.
    pub article: Option<String>,
    /// Description of the problem.
    pub message: String,
}

impl std::fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.article {
            Some(slug) => write!(f, "[{slug}] {}", self.message),
            None => write!(f, "{}", self.message),
        }
    }
}

impl Manifest {
    /// Validate the manifest for internal consistency.
    ///
    /// Returns a list of issues found. An empty list means the manifest is valid.
    pub fn validate(&self) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check for empty articles list
        if self.articles.is_empty() {
            issues.push(ValidationIssue {
                article: None,
                message: "manifest contains no articles".to_string(),
            });
            return issues;
        }

        // Check for duplicate slugs
        let mut seen_slugs: HashSet<&str> = HashSet::new();
        for article in &self.articles {
            if !seen_slugs.insert(&article.slug) {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: format!("duplicate slug: \"{}\"", article.slug),
                });
            }
        }

        // Check for duplicate titles
        let mut seen_titles: HashSet<&str> = HashSet::new();
        for article in &self.articles {
            if !seen_titles.insert(&article.title) {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: format!("duplicate title: \"{}\"", article.title),
                });
            }
        }

        // Validate categories and tiers against taxonomy
        let valid_categories: HashSet<&str> =
            self.taxonomy.categories.iter().map(String::as_str).collect();
        let valid_tiers: HashSet<&str> =
            self.taxonomy.tiers.iter().map(String::as_str).collect();

        for article in &self.articles {
            // Check category
            if !valid_categories.contains(article.category.as_str()) {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: format!(
                        "unknown category \"{}\"; valid categories: {:?}",
                        article.category,
                        self.taxonomy.categories
                    ),
                });
            }

            // Check tier
            if !valid_tiers.contains(article.tier.as_str()) {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: format!(
                        "unknown tier \"{}\"; valid tiers: {:?}",
                        article.tier,
                        self.taxonomy.tiers
                    ),
                });
            }

            // Check slug format (should be lowercase kebab-case)
            if article.slug != article.slug.to_lowercase() {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: "slug contains uppercase characters".to_string(),
                });
            }
            if article.slug.contains(' ') {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: "slug contains spaces (use hyphens)".to_string(),
                });
            }

            // Check required fields are non-empty
            if article.title.is_empty() {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: "title is empty".to_string(),
                });
            }
            if article.slug.is_empty() {
                issues.push(ValidationIssue {
                    article: None,
                    message: "article has empty slug".to_string(),
                });
            }
        }

        issues
    }
}
```

---

## Step 3: Implement `haleiki demo status`

This is the first real subcommand. It loads the manifest, validates it, and prints a table showing each article's fetch status.

### Add to `tools/src/demo/mod.rs`

Replace the `Status` arm in the `run()` function:

```rust
DemoCommand::Status => {
    status::run()?;
}
```

### Create file: `tools/src/demo/status.rs`

```rust
//! Implementation of `haleiki demo status`.

use std::path::Path;

use super::manifest::Manifest;

/// The fetch state of an article on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchState {
    /// No source file exists in demo/sources/
    Missing,
    /// Source file exists in demo/sources/
    Fetched,
}

impl std::fmt::Display for FetchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(f, "missing"),
            Self::Fetched => write!(f, "fetched"),
        }
    }
}

/// Determine the fetch state of an article by checking if its source file exists.
fn fetch_state(slug: &str) -> FetchState {
    let source_path = Path::new("demo/sources").join(format!("{slug}.md"));
    if source_path.exists() {
        FetchState::Fetched
    } else {
        FetchState::Missing
    }
}

/// Run the `haleiki demo status` subcommand.
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

    let mut fetched_count = 0;
    let mut missing_count = 0;

    for article in &manifest.articles {
        let state = fetch_state(&article.slug);
        let project = manifest.effective_project(article);

        match state {
            FetchState::Fetched => fetched_count += 1,
            FetchState::Missing => missing_count += 1,
        }

        // Show project domain only if it differs from default
        let project_indicator = if project != manifest.defaults.project {
            format!(" ({project})")
        } else {
            String::new()
        };

        println!(
            "  {:<35} {:<25} {:<15} {}{}",
            article.slug,
            article.category,
            article.tier,
            state,
            project_indicator,
        );
    }

    println!("  {}", "-".repeat(85));
    println!(
        "  Total: {} articles ({} fetched, {} missing)",
        manifest.articles.len(),
        fetched_count,
        missing_count,
    );
    println!();

    Ok(())
}
```

### Update `tools/src/demo/mod.rs`

Add the `status` module declaration and update the match arm:

```rust
pub mod manifest;
pub mod status;
```

And in the `run()` function, change the `Status` arm from:
```rust
DemoCommand::Status => {
    eprintln!("haleiki demo status: not yet implemented");
}
```
to:
```rust
DemoCommand::Status => {
    status::run()?;
}
```

---

## Step 4: Write tests

### File: `tools/src/demo/manifest_tests.rs` (or inline `#[cfg(test)]` module)

Following the project's test naming convention: `test_<fn>_<scenario>_<expectation>`.

Add a `#[cfg(test)]` module at the bottom of `tools/src/demo/manifest.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> Manifest {
        Manifest {
            defaults: Defaults {
                project: "en.wikipedia.org".to_string(),
                license: "CC BY-SA 4.0".to_string(),
                media: MediaDefaults {
                    max_width: 1024,
                    formats: vec!["svg".to_string(), "png".to_string(), "jpg".to_string()],
                    skip_patterns: vec!["Flag_of_*".to_string()],
                },
            },
            taxonomy: Taxonomy {
                categories: vec![
                    "memory-management".to_string(),
                    "type-systems".to_string(),
                ],
                tiers: vec![
                    "foundational".to_string(),
                    "intermediate".to_string(),
                    "advanced".to_string(),
                ],
            },
            articles: vec![
                Article {
                    title: "Memory management".to_string(),
                    slug: "memory-management".to_string(),
                    category: "memory-management".to_string(),
                    tier: "foundational".to_string(),
                    project: None,
                    license: None,
                    tags: vec!["overview".to_string()],
                    keywords: vec!["allocation".to_string()],
                    media: None,
                },
                Article {
                    title: "Garbage collection (computer science)".to_string(),
                    slug: "garbage-collection".to_string(),
                    category: "memory-management".to_string(),
                    tier: "intermediate".to_string(),
                    project: None,
                    license: None,
                    tags: vec![],
                    keywords: vec![],
                    media: None,
                },
            ],
        }
    }

    // ─── Deserialization tests ───────────────────────────

    #[test]
    fn test_from_file_valid_manifest_parses_successfully() {
        let manifest = Manifest::from_file(Path::new("demo/manifest.yaml"));
        assert!(manifest.is_ok(), "Failed to parse demo/manifest.yaml: {:?}", manifest.err());
    }

    #[test]
    fn test_from_file_valid_manifest_has_12_articles() {
        let manifest = Manifest::from_file(Path::new("demo/manifest.yaml")).unwrap();
        assert_eq!(manifest.articles.len(), 12);
    }

    #[test]
    fn test_from_file_defaults_populated() {
        let manifest = Manifest::from_file(Path::new("demo/manifest.yaml")).unwrap();
        assert_eq!(manifest.defaults.project, "en.wikipedia.org");
        assert_eq!(manifest.defaults.license, "CC BY-SA 4.0");
        assert_eq!(manifest.defaults.media.max_width, 1024);
    }

    #[test]
    fn test_from_file_taxonomy_has_correct_categories() {
        let manifest = Manifest::from_file(Path::new("demo/manifest.yaml")).unwrap();
        assert_eq!(manifest.taxonomy.categories.len(), 4);
        assert!(manifest.taxonomy.categories.contains(&"memory-management".to_string()));
        assert!(manifest.taxonomy.categories.contains(&"type-systems".to_string()));
        assert!(manifest.taxonomy.categories.contains(&"programming-concepts".to_string()));
        assert!(manifest.taxonomy.categories.contains(&"data-structures".to_string()));
    }

    #[test]
    fn test_from_file_wikibooks_article_overrides_project() {
        let manifest = Manifest::from_file(Path::new("demo/manifest.yaml")).unwrap();
        let wikibooks = manifest
            .articles
            .iter()
            .find(|a| a.slug == "wikibooks-memory-management")
            .expect("Wikibooks article not found");
        assert_eq!(wikibooks.project.as_deref(), Some("en.wikibooks.org"));
        assert_eq!(wikibooks.license.as_deref(), Some("CC BY-SA 3.0"));
    }

    #[test]
    fn test_from_file_nonexistent_file_returns_error() {
        let result = Manifest::from_file(Path::new("nonexistent.yaml"));
        assert!(result.is_err());
    }

    // ─── Effective value resolution tests ────────────────

    #[test]
    fn test_effective_project_no_override_returns_default() {
        let m = sample_manifest();
        let article = &m.articles[0]; // no project override
        assert_eq!(m.effective_project(article), "en.wikipedia.org");
    }

    #[test]
    fn test_effective_project_with_override_returns_override() {
        let m = sample_manifest();
        let mut article = m.articles[0].clone();
        article.project = Some("en.wikibooks.org".to_string());
        assert_eq!(m.effective_project(&article), "en.wikibooks.org");
    }

    #[test]
    fn test_effective_license_no_override_returns_default() {
        let m = sample_manifest();
        assert_eq!(m.effective_license(&m.articles[0]), "CC BY-SA 4.0");
    }

    #[test]
    fn test_effective_license_with_override_returns_override() {
        let m = sample_manifest();
        let mut article = m.articles[0].clone();
        article.license = Some("CC BY-SA 3.0".to_string());
        assert_eq!(m.effective_license(&article), "CC BY-SA 3.0");
    }

    // ─── API URL construction tests ─────────────────────

    #[test]
    fn test_api_url_default_project_wikipedia() {
        let m = sample_manifest();
        let url = m.api_url(&m.articles[0]);
        assert_eq!(
            url,
            "https://en.wikipedia.org/api/rest_v1/page/html/Memory_management"
        );
    }

    #[test]
    fn test_api_url_title_with_parentheses() {
        let m = sample_manifest();
        let url = m.api_url(&m.articles[1]); // "Garbage collection (computer science)"
        assert_eq!(
            url,
            "https://en.wikipedia.org/api/rest_v1/page/html/Garbage_collection_(computer_science)"
        );
    }

    #[test]
    fn test_api_url_overridden_project() {
        let m = sample_manifest();
        let mut article = m.articles[0].clone();
        article.project = Some("en.wikibooks.org".to_string());
        article.title = "Intro/Memory".to_string();
        let url = m.api_url(&article);
        assert_eq!(
            url,
            "https://en.wikibooks.org/api/rest_v1/page/html/Intro/Memory"
        );
    }

    // ─── Validation tests ───────────────────────────────

    #[test]
    fn test_validate_valid_manifest_returns_no_issues() {
        let m = sample_manifest();
        let issues = m.validate();
        assert!(issues.is_empty(), "Unexpected issues: {issues:?}");
    }

    #[test]
    fn test_validate_duplicate_slugs_detected() {
        let mut m = sample_manifest();
        m.articles[1].slug = "memory-management".to_string(); // duplicate
        let issues = m.validate();
        assert!(
            issues.iter().any(|i| i.message.contains("duplicate slug")),
            "Expected duplicate slug issue, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_unknown_category_detected() {
        let mut m = sample_manifest();
        m.articles[0].category = "nonexistent-category".to_string();
        let issues = m.validate();
        assert!(
            issues.iter().any(|i| i.message.contains("unknown category")),
            "Expected unknown category issue, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_unknown_tier_detected() {
        let mut m = sample_manifest();
        m.articles[0].tier = "expert".to_string();
        let issues = m.validate();
        assert!(
            issues.iter().any(|i| i.message.contains("unknown tier")),
            "Expected unknown tier issue, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_empty_articles_detected() {
        let mut m = sample_manifest();
        m.articles.clear();
        let issues = m.validate();
        assert!(
            issues.iter().any(|i| i.message.contains("no articles")),
            "Expected empty articles issue, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_slug_with_spaces_detected() {
        let mut m = sample_manifest();
        m.articles[0].slug = "memory management".to_string();
        let issues = m.validate();
        assert!(
            issues.iter().any(|i| i.message.contains("spaces")),
            "Expected slug spaces issue, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_slug_with_uppercase_detected() {
        let mut m = sample_manifest();
        m.articles[0].slug = "Memory-Management".to_string();
        let issues = m.validate();
        assert!(
            issues.iter().any(|i| i.message.contains("uppercase")),
            "Expected uppercase slug issue, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_real_manifest_passes() {
        let manifest = Manifest::from_file(Path::new("demo/manifest.yaml")).unwrap();
        let issues = manifest.validate();
        assert!(
            issues.is_empty(),
            "Real manifest has validation issues: {issues:?}"
        );
    }

    // ─── Serde round-trip test ──────────────────────────

    #[test]
    fn test_serde_roundtrip_preserves_data() {
        let m = sample_manifest();
        let yaml = serde_yaml::to_string(&m).unwrap();
        let m2: Manifest = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(m, m2);
    }
}
```

### Important: Test working directory

Tests that call `Manifest::from_file(Path::new("demo/manifest.yaml"))` assume the working directory is the repository root. Cargo runs tests from the workspace root by default when using a workspace manifest, which is correct. If tests fail with "file not found", the issue is the working directory.

**Alternative approach** if cwd is unreliable: use `env!("CARGO_MANIFEST_DIR")` to construct absolute paths:

```rust
fn manifest_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()  // go from tools/ to repo root
        .unwrap()
        .join("demo/manifest.yaml")
}
```

---

## Step 5: Integration test for CLI

### File: `tools/tests/demo_status.rs`

```rust
//! Integration test for `haleiki demo status`.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
#[cfg(feature = "demo")]
fn test_demo_status_prints_article_table() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .arg("demo")
        .arg("status")
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("SLUG")
                .and(predicate::str::contains("CATEGORY"))
                .and(predicate::str::contains("STATUS"))
                .and(predicate::str::contains("memory-management"))
                .and(predicate::str::contains("garbage-collection"))
                .and(predicate::str::contains("12 articles"))
                .and(predicate::str::contains("missing")),
        );
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_status_shows_wikibooks_project() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .arg("demo")
        .arg("status")
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(predicate::str::contains("en.wikibooks.org"));
}
```

---

## Step 6: Verification

### 6.1: All tests pass

```bash
cd /Users/oubiwann/lab/oxur/haleiki
cargo test --features demo
```

### 6.2: `haleiki demo status` produces expected output

```bash
cargo run --features demo -- demo status
```

Expected output (approximately):

```
  SLUG                                CATEGORY                  TIER            STATUS
  -------------------------------------------------------------------------------------
  memory-management                   memory-management         foundational    missing
  garbage-collection                  memory-management         intermediate    missing
  reference-counting                  memory-management         intermediate    missing
  stack-memory                        memory-management         foundational    missing
  region-based-memory                 memory-management         advanced        missing
  raii                                memory-management         intermediate    missing
  smart-pointer                       data-structures           intermediate    missing
  pointer                             programming-concepts      foundational    missing
  dangling-pointer                    memory-management         intermediate    missing
  memory-safety                       type-systems              foundational    missing
  type-safety                         type-systems              foundational    missing
  wikibooks-memory-management         memory-management         foundational    missing (en.wikibooks.org)
  -------------------------------------------------------------------------------------
  Total: 12 articles (0 fetched, 12 missing)
```

### 6.3: Linting passes

```bash
make lint
```

### 6.4: Coverage check

```bash
cargo test --features demo -- --test-threads=1
# Then check that all validation paths are covered
```

Target: ≥95% coverage for `manifest.rs` and `status.rs`.

---

## Acceptance Criteria

- [ ] `tools/src/demo/manifest.rs` implements `Manifest`, `Defaults`, `MediaDefaults`, `Taxonomy`, `Article`, `ArticleMedia` types with serde derive
- [ ] `Manifest::from_file()` successfully parses `demo/manifest.yaml`
- [ ] `Manifest::validate()` detects: duplicate slugs, duplicate titles, unknown categories, unknown tiers, empty slug, uppercase slug, slug with spaces, empty articles list
- [ ] `Manifest::effective_project()` and `effective_license()` correctly resolve per-article overrides vs defaults
- [ ] `Manifest::api_url()` constructs correct Wikimedia REST API URLs (spaces → underscores, preserves parentheses and slashes)
- [ ] `tools/src/demo/status.rs` implements the `haleiki demo status` subcommand
- [ ] `haleiki demo status` prints a formatted table of all 12 articles
- [ ] All articles show "missing" status (no source files exist yet)
- [ ] The Wikibooks article shows its non-default project domain
- [ ] Summary line shows total, fetched, and missing counts
- [ ] If `demo/manifest.yaml` is missing, a helpful error message is printed
- [ ] All unit tests pass (`cargo test --features demo`)
- [ ] Integration test passes (assert_cmd)
- [ ] `make lint` passes
- [ ] Coverage ≥95% for `manifest.rs`
- [ ] Serde round-trip test passes

---

## Gotchas

1. **`serde_yaml` crate status**: `serde_yaml` 0.9 is the latest, but the crate was deprecated in favor of other YAML parsers. As of early 2026, it still works fine. If compilation issues arise, consider `serde_yml` as an alternative.

2. **Title encoding in API URLs**: Wikimedia titles use underscores for spaces but preserve other characters (parentheses, slashes). The `api_url()` function should only replace spaces with underscores, NOT percent-encode other characters. The REST API handles them literally.

3. **Test working directory**: Tests that read `demo/manifest.yaml` depend on the working directory being the repo root. Use `env!("CARGO_MANIFEST_DIR")` + `parent()` for reliability. Alternatively, set the working directory in `assert_cmd` tests with `.current_dir()`.

4. **The `manifest.validate()` design**: Returns a `Vec<ValidationIssue>` rather than `Result<(), Vec<...>>`. This is deliberate — validation warnings shouldn't block the status display. The `status` command prints issues to stderr and continues.

5. **Column width in output**: The format string widths (`{:<35}`, `{:<25}`, etc.) should be wide enough for the longest slug (`wikibooks-memory-management` = 29 chars) and category. Test with the real data to ensure alignment.

6. **`#[cfg(feature = "demo")]` on integration test**: The integration test file needs `#[cfg(feature = "demo")]` on each test function, since the `demo` subcommand doesn't exist without the feature. Without this, `cargo test` (without `--features demo`) will try to run the test and fail.

7. **`HashSet` import**: Use `std::collections::HashSet`, not `hashbrown`. The standard library version is fine for validation of 12 articles.

8. **`Display` for `ValidationIssue`**: Implementing `Display` (not just `Debug`) allows using `{issue}` in format strings for clean error output.
