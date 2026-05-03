//! Demo manifest parsing and validation.
//!
//! Deserializes `demo/manifest.yaml` and validates article entries
//! against the embedded taxonomy.

use std::collections::{HashMap, HashSet};
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

    /// Haleiki slug -- becomes the URL and filename.
    pub slug: String,

    /// Content category (must match taxonomy).
    pub category: String,

    /// Optional subcategory within the category.
    #[serde(default)]
    pub subcategory: Option<String>,

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

/// Normalize a Wikipedia article title for comparison.
///
/// Lowercases, trims, replaces underscores with spaces.
#[allow(dead_code)] // used by rewrite module and tests
pub fn normalize_wiki_title(title: &str) -> String {
    title.trim().replace('_', " ").to_lowercase()
}

/// Encode an article title for use in a URL path segment.
///
/// Spaces become underscores (Wikimedia convention), and characters that
/// are reserved in URL path segments are percent-encoded.
fn encode_title_for_path(title: &str) -> String {
    use std::fmt::Write;
    let with_underscores = title.replace(' ', "_");
    let mut result = String::with_capacity(with_underscores.len() * 2);
    for byte in with_underscores.bytes() {
        match byte {
            // Unreserved characters pass through
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~'
            // Allow these in path segments (parentheses are common in Wikipedia titles)
            | b'(' | b')' | b',' | b':' | b'@' | b'!' | b'\'' => {
                result.push(byte as char);
            }
            // Everything else gets percent-encoded (including /, ?, #, &, %, etc.)
            _ => {
                let _ = write!(result, "%{byte:02X}");
            }
        }
    }
    result
}

/// A validation problem found in the manifest.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationIssue {
    /// Which article (by slug) has the problem, or `None` for manifest-level issues.
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
    /// Build a lookup index from Wikimedia article titles to Haleiki slugs.
    ///
    /// Titles are normalized (lowercased, underscores->spaces) for fuzzy matching.
    /// Multiple keys may map to the same slug (title + variants).
    #[allow(dead_code)] // used by rewrite module and tests
    pub fn title_to_slug_index(&self) -> HashMap<String, String> {
        let mut index = HashMap::new();

        for article in &self.articles {
            // Exact title
            let normalized = normalize_wiki_title(&article.title);
            index.insert(normalized, article.slug.clone());

            // Also index with underscores (as they appear in URLs)
            let underscore_form = article.title.replace(' ', "_").to_lowercase();
            index.insert(underscore_form, article.slug.clone());
        }

        index
    }

    /// Load and parse a manifest from a YAML file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn from_file(path: &Path) -> crate::error::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest: Self = serde_yaml::from_str(&content)?;
        Ok(manifest)
    }

    /// Get the effective project domain for an article (article override or default).
    pub fn effective_project<'a>(&'a self, article: &'a Article) -> &'a str {
        article.project.as_deref().unwrap_or(&self.defaults.project)
    }

    /// Get the effective license for an article (article override or default).
    #[allow(dead_code)] // used in tests; will be used by fetch pipeline
    pub fn effective_license<'a>(&'a self, article: &'a Article) -> &'a str {
        article.license.as_deref().unwrap_or(&self.defaults.license)
    }

    /// Build the API URL for fetching an article's HTML.
    ///
    /// Wikimedia projects use the REST API: `/api/rest_v1/page/html/{title}`
    /// Other `MediaWiki` sites use the parse API:
    /// `/api.php?action=parse&page={title}&format=json&prop=text|revid|displaytitle`
    pub fn api_url(&self, article: &Article) -> String {
        let project = self.effective_project(article);

        if super::fetch::is_wikimedia_project(project) {
            let encoded_title = encode_title_for_path(&article.title);
            format!("https://{project}/api/rest_v1/page/html/{encoded_title}")
        } else {
            let url_title = super::fetch::url_encode_title(&article.title);
            format!(
                "https://{project}/api.php?action=parse&page={url_title}&format=json&prop=text%7Crevid%7Cdisplaytitle"
            )
        }
    }

    /// Validate the manifest for internal consistency.
    ///
    /// Returns a list of issues found. An empty list means the manifest is valid.
    #[allow(clippy::too_many_lines)] // validation logic is inherently sequential
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
        let valid_categories: HashSet<&str> = self
            .taxonomy
            .categories
            .iter()
            .map(String::as_str)
            .collect();
        let valid_tiers: HashSet<&str> = self.taxonomy.tiers.iter().map(String::as_str).collect();

        for article in &self.articles {
            // Check category
            if !valid_categories.contains(article.category.as_str()) {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: format!(
                        "unknown category \"{}\"; valid categories: {:?}",
                        article.category, self.taxonomy.categories
                    ),
                });
            }

            // Check tier
            if !valid_tiers.contains(article.tier.as_str()) {
                issues.push(ValidationIssue {
                    article: Some(article.slug.clone()),
                    message: format!(
                        "unknown tier \"{}\"; valid tiers: {:?}",
                        article.tier, self.taxonomy.tiers
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

            // Validate subcategory format (kebab-case)
            if let Some(ref subcat) = article.subcategory {
                if subcat != &subcat.to_lowercase() {
                    issues.push(ValidationIssue {
                        article: Some(article.slug.clone()),
                        message: "subcategory contains uppercase characters".to_string(),
                    });
                }
                if subcat.contains(' ') {
                    issues.push(ValidationIssue {
                        article: Some(article.slug.clone()),
                        message: "subcategory contains spaces (use hyphens)".to_string(),
                    });
                }
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

        // Check subcategory consistency: warn if the same subcategory appears in different categories
        let mut subcategory_to_category: HashMap<&str, &str> = HashMap::new();
        for article in &self.articles {
            if let Some(ref subcat) = article.subcategory {
                if let Some(&existing_cat) = subcategory_to_category.get(subcat.as_str()) {
                    if existing_cat != article.category.as_str() {
                        issues.push(ValidationIssue {
                            article: Some(article.slug.clone()),
                            message: format!(
                                "subcategory \"{}\" used in both \"{}\" and \"{}\" categories",
                                subcat, existing_cat, article.category,
                            ),
                        });
                    }
                } else {
                    subcategory_to_category.insert(subcat.as_str(), article.category.as_str());
                }
            }
        }

        issues
    }
}

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
                categories: vec!["memory-management".to_string(), "type-systems".to_string()],
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
                    subcategory: None,
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
                    subcategory: None,
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

    fn manifest_path() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("demo/manifest.yaml")
    }

    // --- Deserialization tests ---

    #[test]
    fn test_from_file_valid_manifest_parses_successfully() {
        let manifest = Manifest::from_file(&manifest_path());
        assert!(
            manifest.is_ok(),
            "Failed to parse demo/manifest.yaml: {:?}",
            manifest.err()
        );
    }

    #[test]
    fn test_from_file_valid_manifest_has_expected_articles() {
        let manifest = Manifest::from_file(&manifest_path()).unwrap();
        assert!(
            manifest.articles.len() >= 12,
            "Expected at least 12 articles, got {}",
            manifest.articles.len()
        );
    }

    #[test]
    fn test_from_file_defaults_populated() {
        let manifest = Manifest::from_file(&manifest_path()).unwrap();
        assert_eq!(manifest.defaults.project, "en.wikipedia.org");
        assert_eq!(manifest.defaults.license, "CC BY-SA 4.0");
        assert_eq!(manifest.defaults.media.max_width, 1024);
    }

    #[test]
    fn test_from_file_taxonomy_has_categories_and_tiers() {
        let manifest = Manifest::from_file(&manifest_path()).unwrap();
        assert!(
            !manifest.taxonomy.categories.is_empty(),
            "Taxonomy should have at least one category"
        );
        assert_eq!(manifest.taxonomy.tiers.len(), 3);
        assert!(
            manifest
                .taxonomy
                .tiers
                .contains(&"foundational".to_string())
        );
        assert!(
            manifest
                .taxonomy
                .tiers
                .contains(&"intermediate".to_string())
        );
        assert!(manifest.taxonomy.tiers.contains(&"advanced".to_string()));
    }

    #[test]
    fn test_from_file_non_default_project_overrides() {
        let manifest = Manifest::from_file(&manifest_path()).unwrap();
        // At least one article should override the default project
        let has_override = manifest.articles.iter().any(|a| a.project.is_some());
        assert!(
            has_override,
            "Expected at least one article with a project override"
        );
        // Any overridden article should also override the license
        for article in manifest.articles.iter().filter(|a| a.project.is_some()) {
            assert!(
                article.license.is_some(),
                "Article '{}' overrides project but not license",
                article.slug
            );
        }
    }

    #[test]
    fn test_from_file_nonexistent_file_returns_error() {
        let result = Manifest::from_file(Path::new("nonexistent.yaml"));
        assert!(result.is_err());
    }

    // --- Effective value resolution tests ---

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

    // --- API URL construction tests ---

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
        // Slash is percent-encoded for path-segment safety
        assert_eq!(
            url,
            "https://en.wikibooks.org/api/rest_v1/page/html/Intro%2FMemory"
        );
    }

    // --- Validation tests ---

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
    fn test_validate_duplicate_titles_detected() {
        let mut m = sample_manifest();
        m.articles[1].title = "Memory management".to_string(); // duplicate
        let issues = m.validate();
        assert!(
            issues.iter().any(|i| i.message.contains("duplicate title")),
            "Expected duplicate title issue, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_unknown_category_detected() {
        let mut m = sample_manifest();
        m.articles[0].category = "nonexistent-category".to_string();
        let issues = m.validate();
        assert!(
            issues
                .iter()
                .any(|i| i.message.contains("unknown category")),
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
    fn test_validate_empty_title_detected() {
        let mut m = sample_manifest();
        m.articles[0].title = String::new();
        let issues = m.validate();
        assert!(
            issues.iter().any(|i| i.message.contains("title is empty")),
            "Expected empty title issue, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_empty_slug_detected() {
        let mut m = sample_manifest();
        m.articles[0].slug = String::new();
        let issues = m.validate();
        assert!(
            issues
                .iter()
                .any(|i| i.message.contains("article has empty slug")),
            "Expected empty slug issue, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_real_manifest_passes() {
        let manifest = Manifest::from_file(&manifest_path()).unwrap();
        let issues = manifest.validate();
        // Filter out expected cross-category subcategory warnings (e.g., "foundations"
        // is intentionally used in both mathematics and theoretical-physics).
        let non_subcategory_issues: Vec<_> = issues
            .iter()
            .filter(|i| !i.message.contains("subcategory"))
            .collect();
        assert!(
            non_subcategory_issues.is_empty(),
            "Real manifest has validation issues: {non_subcategory_issues:?}"
        );
    }

    // --- API URL dispatch tests ---

    fn sample_manifest_with_rigpawiki() -> Manifest {
        let mut m = sample_manifest();
        // Add a Rigpa Wiki article with a project override
        m.articles.push(Article {
            title: "Longchenpa".to_string(),
            slug: "longchenpa".to_string(),
            category: "memory-management".to_string(), // reuse existing category for test
            subcategory: None,
            tier: "foundational".to_string(),
            project: Some("www.rigpawiki.org".to_string()),
            license: Some("CC BY-NC-SA 3.0".to_string()),
            tags: vec![],
            keywords: vec![],
            media: None,
        });
        m
    }

    #[test]
    fn test_api_url_wikimedia_uses_rest_api() {
        let m = sample_manifest_with_rigpawiki();
        // "Memory management" uses the default en.wikipedia.org project
        let wp_article = m
            .articles
            .iter()
            .find(|a| a.slug == "memory-management")
            .unwrap();
        let url = m.api_url(wp_article);
        assert!(
            url.contains("/api/rest_v1/page/html/"),
            "Expected REST API URL: {url}"
        );
    }

    #[test]
    fn test_api_url_rigpawiki_uses_action_parse() {
        let m = sample_manifest_with_rigpawiki();
        let rw_article = m.articles.iter().find(|a| a.slug == "longchenpa").unwrap();
        let url = m.api_url(rw_article);
        assert!(
            url.contains("api.php?action=parse"),
            "Expected action=parse URL: {url}"
        );
        assert!(
            url.contains("www.rigpawiki.org"),
            "Expected rigpawiki domain: {url}"
        );
    }

    #[test]
    fn test_api_url_rigpawiki_contains_encoded_title() {
        let m = sample_manifest_with_rigpawiki();
        let rw_article = m.articles.iter().find(|a| a.slug == "longchenpa").unwrap();
        let url = m.api_url(rw_article);
        assert!(
            url.contains("page=Longchenpa"),
            "Expected title in URL: {url}"
        );
    }

    // --- Display tests ---

    #[test]
    fn test_validation_issue_display_with_article() {
        let issue = ValidationIssue {
            article: Some("test-slug".to_string()),
            message: "something went wrong".to_string(),
        };
        assert_eq!(format!("{issue}"), "[test-slug] something went wrong");
    }

    #[test]
    fn test_validation_issue_display_without_article() {
        let issue = ValidationIssue {
            article: None,
            message: "manifest-level problem".to_string(),
        };
        assert_eq!(format!("{issue}"), "manifest-level problem");
    }

    // --- Subcategory tests ---

    #[test]
    fn test_article_subcategory_optional() {
        let yaml = r#"
defaults:
  project: "en.wikipedia.org"
  license: "CC BY-SA 4.0"
  media:
    max_width: 1024
    formats: ["png"]
    skip_patterns: []
taxonomy:
  categories: ["music"]
  tiers: ["foundational"]
articles:
  - title: "Jazz"
    slug: "jazz"
    category: "music"
    tier: "foundational"
"#;
        let m: Manifest = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(m.articles[0].subcategory, None);
    }

    #[test]
    fn test_article_subcategory_present() {
        let yaml = r#"
defaults:
  project: "en.wikipedia.org"
  license: "CC BY-SA 4.0"
  media:
    max_width: 1024
    formats: ["png"]
    skip_patterns: []
taxonomy:
  categories: ["music"]
  tiers: ["foundational"]
articles:
  - title: "Jazz"
    slug: "jazz"
    category: "music"
    subcategory: "genres"
    tier: "foundational"
"#;
        let m: Manifest = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(m.articles[0].subcategory, Some("genres".to_string()));
    }

    #[test]
    fn test_validate_subcategory_cross_category_warns() {
        let mut m = sample_manifest();
        m.articles[0].subcategory = Some("overlap".to_string());
        m.articles[0].category = "cat-a".to_string();
        m.articles[1].subcategory = Some("overlap".to_string());
        m.articles[1].category = "cat-b".to_string();
        // Add both categories to taxonomy so they pass category validation
        m.taxonomy.categories.push("cat-a".to_string());
        m.taxonomy.categories.push("cat-b".to_string());

        let issues = m.validate();
        assert!(
            issues
                .iter()
                .any(|i| i.message.contains("subcategory") && i.message.contains("both")),
            "Expected cross-category subcategory warning, got: {issues:?}"
        );
    }

    #[test]
    fn test_validate_subcategory_uppercase_warns() {
        let mut m = sample_manifest();
        m.articles[0].subcategory = Some("BadCase".to_string());
        let issues = m.validate();
        assert!(
            issues
                .iter()
                .any(|i| i.message.contains("subcategory") && i.message.contains("uppercase")),
            "Expected uppercase warning, got: {issues:?}"
        );
    }

    // --- Serde round-trip test ---

    #[test]
    fn test_serde_roundtrip_preserves_data() {
        let m = sample_manifest();
        let yaml = serde_yaml::to_string(&m).unwrap();
        let m2: Manifest = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(m, m2);
    }

    // --- title_to_slug_index tests ---

    #[test]
    fn test_title_to_slug_index_contains_all_articles() {
        let m = sample_manifest();
        let index = m.title_to_slug_index();
        // Each article adds 2 entries (normalized + underscore form)
        assert!(index.len() >= m.articles.len());
    }

    #[test]
    fn test_title_to_slug_index_lookup_with_underscores() {
        let m = sample_manifest();
        let index = m.title_to_slug_index();
        assert_eq!(
            index.get("memory_management"),
            Some(&"memory-management".to_string()),
        );
    }

    #[test]
    fn test_title_to_slug_index_lookup_case_insensitive() {
        let m = sample_manifest();
        let index = m.title_to_slug_index();
        // normalize_wiki_title lowercases
        assert_eq!(
            index.get("memory management"),
            Some(&"memory-management".to_string()),
        );
    }

    #[test]
    fn test_title_to_slug_index_parenthetical_title() {
        let m = sample_manifest();
        let index = m.title_to_slug_index();
        assert_eq!(
            index.get("garbage collection (computer science)"),
            Some(&"garbage-collection".to_string()),
        );
    }

    // --- encode_title_for_path tests ---

    #[test]
    fn test_encode_title_for_path_simple() {
        assert_eq!(
            encode_title_for_path("Memory management"),
            "Memory_management",
        );
    }

    #[test]
    fn test_encode_title_for_path_slash() {
        let encoded = encode_title_for_path("AdS/CFT correspondence");
        assert_eq!(encoded, "AdS%2FCFT_correspondence");
    }

    #[test]
    fn test_encode_title_for_path_parentheses_preserved() {
        assert_eq!(
            encode_title_for_path("Rust (programming language)"),
            "Rust_(programming_language)",
        );
    }

    #[test]
    fn test_encode_title_for_path_diacritics_encoded() {
        let encoded = encode_title_for_path("Arvo Pärt");
        // ä = C3 A4 in UTF-8
        assert!(
            encoded.contains("P%C3%A4rt"),
            "Diacritics should be percent-encoded: {encoded}",
        );
    }

    #[test]
    fn test_encode_title_for_path_hash_encoded() {
        let encoded = encode_title_for_path("C# (language)");
        assert!(
            encoded.contains("%23"),
            "Hash should be percent-encoded: {encoded}",
        );
    }
}
