//! Markdown and frontmatter parsing for source pages and concept cards.
//!
//! This module reads Markdown files from the content directory, extracts YAML
//! frontmatter, and produces structured [`Corpus`] representations suitable for
//! graph construction and validation.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

/// The full parsed corpus: all sources, concepts, taxonomy, and optional profile.
#[derive(Debug, Clone)]
pub struct Corpus {
    /// Source pages keyed by slug.
    pub sources: HashMap<String, SourcePage>,
    /// Concept cards keyed by slug.
    pub concepts: HashMap<String, ConceptCard>,
    /// News posts keyed by their filename-derived slug.
    pub news: HashMap<String, NewsPost>,
    /// Taxonomy (categories + tiers).
    pub taxonomy: Taxonomy,
    /// Optional deployment profile.
    pub profile: Option<Profile>,
}

/// A source page parsed from `sources/*.md`.
#[derive(Debug, Clone, Deserialize)]
pub struct SourcePage {
    pub title: String,
    pub slug: String,
    #[serde(default)]
    pub page_type: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub subcategory: String,
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub original_source: Option<OriginalSource>,
    #[serde(default)]
    pub extraction_status: String,
    #[serde(default)]
    pub concepts_generated: Vec<String>,
    #[serde(default)]
    pub last_extracted: String,
    #[serde(default)]
    pub status: String,
    /// The Markdown body, populated by [`parse_sources`] after frontmatter
    /// extraction. Not part of the frontmatter itself.
    #[serde(skip)]
    pub body: String,
}

/// A news post parsed from a direct news Markdown file.
///
/// Its private fields are validated by the parser before insertion into Corpus.
/// Read-only accessors expose the values renderers need.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewsPost {
    title: String,
    date: NewsDate,
    category: String,
    category_slug: String,
    tags: Vec<NewsTag>,
    author: Option<String>,
    excerpt: Option<String>,
    slug: String,
    url: String,
    body: String,
}

impl NewsPost {
    pub(crate) fn title(&self) -> &str {
        &self.title
    }
    pub(crate) fn date(&self) -> &NewsDate {
        &self.date
    }
    pub(crate) fn category(&self) -> &str {
        &self.category
    }
    pub(crate) fn category_slug(&self) -> &str {
        &self.category_slug
    }
    pub(crate) fn tags(&self) -> &[NewsTag] {
        &self.tags
    }
    pub(crate) fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }
    pub(crate) fn excerpt(&self) -> Option<&str> {
        self.excerpt.as_deref()
    }
    pub(crate) fn slug(&self) -> &str {
        &self.slug
    }
    pub(crate) fn url(&self) -> &str {
        &self.url
    }
    pub(crate) fn body(&self) -> &str {
        &self.body
    }
}

/// A validated Gregorian publication date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewsDate {
    year: u16,
    month: u8,
    day: u8,
}

impl NewsDate {
    fn parse(value: &str) -> anyhow::Result<Self> {
        let bytes = value.as_bytes();
        anyhow::ensure!(
            bytes.len() == 10
                && bytes[4] == b'-'
                && bytes[7] == b'-'
                && bytes
                    .iter()
                    .enumerate()
                    .all(|(i, byte)| { i == 4 || i == 7 || byte.is_ascii_digit() }),
            "must be a strict YYYY-MM-DD date"
        );

        let year = value[0..4].parse::<u16>()?;
        let month = value[5..7].parse::<u8>()?;
        let day = value[8..10].parse::<u8>()?;
        anyhow::ensure!(
            (1..=9999).contains(&year),
            "year must be between 0001 and 9999"
        );
        anyhow::ensure!((1..=12).contains(&month), "month must be between 01 and 12");

        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let month_days = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if leap => 29,
            2 => 28,
            _ => unreachable!("month range checked above"),
        };
        anyhow::ensure!(
            (1..=month_days).contains(&day),
            "day is invalid for this month"
        );

        Ok(Self { year, month, day })
    }

    #[must_use]
    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    #[must_use]
    pub fn display(self) -> String {
        const MONTHS: [&str; 12] = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        format!(
            "{} {}, {}",
            MONTHS[usize::from(self.month - 1)],
            self.day,
            self.year
        )
    }
}

/// A category or tag value with its shared-convention URL slug.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NewsTag {
    pub name: String,
    pub slug: String,
}

/// Nested original-source information in source page frontmatter.
#[derive(Debug, Clone, Deserialize)]
pub struct OriginalSource {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub project: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub fetched_at: String,
    #[serde(default)]
    pub revision_id: String,
}

/// A concept card parsed from `concepts/*.md`.
#[derive(Debug, Clone, Deserialize)]
pub struct ConceptCard {
    pub concept: String,
    pub slug: String,
    #[serde(default)]
    pub page_type: String,
    #[serde(default)]
    pub disambiguator: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub subcategory: String,
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub subject_headings: Vec<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub derived_from: Vec<DerivedFromEntry>,
    #[serde(default)]
    pub external_references: Vec<String>,
    #[serde(default)]
    pub extraction_confidence: String,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub extends: Vec<String>,
    #[serde(default)]
    pub related: Vec<String>,
    #[serde(default)]
    pub contrasts_with: Vec<String>,
    #[serde(default)]
    pub trust_level: String,
    #[serde(default)]
    pub canon_status: String,
    #[serde(default)]
    pub answers_questions: Vec<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub stable_since: String,
    #[serde(default)]
    pub last_reviewed: String,
    /// The Markdown body, populated by [`parse_concepts`] after frontmatter
    /// extraction. Not part of the frontmatter itself.
    #[serde(skip)]
    pub body: String,
}

/// An entry in the `derived_from` list of a concept card.
#[derive(Debug, Clone, Deserialize)]
pub struct DerivedFromEntry {
    pub slug: String,
    #[serde(default)]
    pub title: String,
}

/// The taxonomy file (`taxonomy.yaml`).
#[derive(Debug, Clone, Deserialize)]
pub struct Taxonomy {
    #[serde(default)]
    pub categories: Vec<TaxonomyEntry>,
    #[serde(default)]
    pub tiers: Vec<TierEntry>,
}

/// A single category entry in the taxonomy.
#[derive(Debug, Clone, Deserialize)]
pub struct TaxonomyEntry {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// A single tier entry in the taxonomy.
#[derive(Debug, Clone, Deserialize)]
pub struct TierEntry {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub order: u32,
}

/// The deployment profile (`profile.yaml`).
#[derive(Debug, Clone, Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct Profile {
    #[serde(default)]
    pub profile: ProfileInfo,
    #[serde(default)]
    pub trust: TrustConfig,
    #[serde(default)]
    pub canon: CanonConfig,
    #[serde(default)]
    pub page_types: Vec<String>,
    #[serde(default)]
    pub relationships: Vec<String>,
}

/// Top-level profile identity.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProfileInfo {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub personality: String,
}

/// Trust-axis configuration.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrustConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub default: String,
}

/// Canon-axis configuration.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CanonConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Parse the entire content directory into a [`Corpus`].
///
/// Reads `sources/*.md`, `concepts/*.md`, `taxonomy.yaml`, and optionally
/// `profile.yaml` from the given content directory.
///
/// # Errors
///
/// Returns an error if required files are missing or contain invalid frontmatter.
pub fn parse_corpus(content_dir: &Path) -> Result<Corpus> {
    let sources = parse_sources(content_dir)?;
    let concepts = parse_concepts(content_dir)?;
    let news = parse_news(content_dir)?;
    let taxonomy = parse_taxonomy(content_dir)?;
    let profile = parse_profile(content_dir)?;

    Ok(Corpus {
        sources,
        concepts,
        news,
        taxonomy,
        profile,
    })
}

fn parse_sources(content_dir: &Path) -> Result<HashMap<String, SourcePage>> {
    let sources_dir = content_dir.join("sources");
    let mut map = HashMap::new();

    if !sources_dir.exists() {
        return Ok(map);
    }

    for entry in fs::read_dir(&sources_dir)
        .with_context(|| format!("failed to read {}", sources_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let stem = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if stem == ".gitkeep" {
            continue;
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let (mut page, body): (SourcePage, String) = parse_frontmatter(&content)
            .with_context(|| format!("failed to parse frontmatter in {}", path.display()))?;
        page.body = body;

        map.insert(page.slug.clone(), page);
    }

    Ok(map)
}

fn parse_concepts(content_dir: &Path) -> Result<HashMap<String, ConceptCard>> {
    let concepts_dir = content_dir.join("concepts");
    let mut map = HashMap::new();

    if !concepts_dir.exists() {
        return Ok(map);
    }

    for entry in fs::read_dir(&concepts_dir)
        .with_context(|| format!("failed to read {}", concepts_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let stem = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if stem == ".gitkeep" {
            continue;
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let (mut card, body): (ConceptCard, String) = parse_frontmatter(&content)
            .with_context(|| format!("failed to parse frontmatter in {}", path.display()))?;
        card.body = body;

        map.insert(card.slug.clone(), card);
    }

    Ok(map)
}

fn parse_news(content_dir: &Path) -> Result<HashMap<String, NewsPost>> {
    let news_dir = content_dir.join("news");
    let entries = match fs::read_dir(&news_dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(HashMap::new());
        }
        Err(error) => {
            return Err(error).with_context(|| format!("failed to read {}", news_dir.display()));
        }
    };

    let mut paths = Vec::new();
    for entry in entries {
        let entry =
            entry.with_context(|| format!("failed to read an entry in {}", news_dir.display()))?;
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to inspect {}", entry.path().display()))?;
        if file_type.is_file() && entry.path().extension().is_some_and(|ext| ext == "md") {
            paths.push(entry.path());
        }
    }
    paths.sort();

    let mut posts = HashMap::new();
    let mut slug_paths: HashMap<String, PathBuf> = HashMap::new();
    for path in paths {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read news file {}", path.display()))?;
        let (frontmatter, body) = parse_news_frontmatter(&content)
            .with_context(|| format!("failed to parse news file {}", path.display()))?;
        let post = parse_news_post(&frontmatter, body, &path)
            .with_context(|| format!("invalid news file {}", path.display()))?;

        if let Some(first_path) = slug_paths.get(post.slug()) {
            anyhow::bail!(
                "duplicate normalized news slug {:?} in {} and {}",
                post.slug(),
                first_path.display(),
                path.display()
            );
        }
        slug_paths.insert(post.slug().to_owned(), path);
        posts.insert(post.slug().to_owned(), post);
    }

    Ok(posts)
}

fn strip_news_line_ending(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

fn parse_news_frontmatter(content: &str) -> anyhow::Result<(Value, String)> {
    let mut lines = content.split_inclusive('\n');
    let opening = lines
        .next()
        .context("missing opening frontmatter delimiter")?;
    anyhow::ensure!(
        strip_news_line_ending(opening) == "---",
        "frontmatter must begin with an opening --- fence"
    );

    let yaml_start = opening.len();
    let mut offset = yaml_start;
    for line in lines {
        if strip_news_line_ending(line) == "---" {
            let yaml: Value = serde_yaml::from_str(&content[yaml_start..offset])
                .context("invalid frontmatter YAML")?;
            let body = content[offset + line.len()..].to_owned();
            return Ok((yaml, body));
        }
        offset += line.len();
    }

    anyhow::bail!("missing closing frontmatter delimiter")
}

fn news_field<'a>(frontmatter: &'a Value, name: &str, path: &Path) -> anyhow::Result<&'a Value> {
    let mapping = frontmatter
        .as_mapping()
        .ok_or_else(|| anyhow::anyhow!("{}: frontmatter must be a mapping", path.display()))?;
    mapping
        .get(Value::String(name.to_owned()))
        .ok_or_else(|| anyhow::anyhow!("{}: missing required field {name}", path.display()))
}

fn required_news_string(frontmatter: &Value, name: &str, path: &Path) -> anyhow::Result<String> {
    let value = news_field(frontmatter, name, path)?;
    value
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| anyhow::anyhow!("{}: field {name} must be a string", path.display()))
}

fn optional_news_string(
    frontmatter: &Value,
    name: &str,
    path: &Path,
) -> anyhow::Result<Option<String>> {
    let mapping = frontmatter
        .as_mapping()
        .ok_or_else(|| anyhow::anyhow!("{}: frontmatter must be a mapping", path.display()))?;
    let Some(value) = mapping.get(Value::String(name.to_owned())) else {
        return Ok(None);
    };
    if value.is_null() {
        return Ok(None);
    }
    let value = value.as_str().ok_or_else(|| {
        anyhow::anyhow!("{}: field {name} must be a string or null", path.display())
    })?;
    if value.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_owned()))
    }
}

fn nonempty_news_slug(value: &str, field: &str, path: &Path) -> anyhow::Result<String> {
    let slug = crate::markdown::slugify(value);
    anyhow::ensure!(
        !slug.is_empty(),
        "{}: field {field} produces an empty URL slug",
        path.display()
    );
    Ok(slug)
}

fn parse_news_post(frontmatter: &Value, body: String, path: &Path) -> anyhow::Result<NewsPost> {
    let kind = required_news_string(frontmatter, "type", path)?;
    anyhow::ensure!(
        kind == "news",
        "{}: field type must be exactly news",
        path.display()
    );

    let title = required_news_string(frontmatter, "title", path)?;
    anyhow::ensure!(
        !title.trim().is_empty(),
        "{}: field title must not be empty",
        path.display()
    );

    let date_text = required_news_string(frontmatter, "date", path)?;
    let date =
        NewsDate::parse(&date_text).with_context(|| format!("{}: field date", path.display()))?;

    let category = required_news_string(frontmatter, "category", path)?;
    anyhow::ensure!(
        !category.trim().is_empty(),
        "{}: field category must not be empty",
        path.display()
    );
    let category_slug = nonempty_news_slug(&category, "category", path)?;

    let tags_value = news_field(frontmatter, "tags", path)?;
    let tags_sequence = tags_value.as_sequence().ok_or_else(|| {
        anyhow::anyhow!(
            "{}: field tags must be a sequence of strings",
            path.display()
        )
    })?;
    let mut tags = Vec::with_capacity(tags_sequence.len());
    for (index, value) in tags_sequence.iter().enumerate() {
        let name = value.as_str().ok_or_else(|| {
            anyhow::anyhow!("{}: field tags[{index}] must be a string", path.display())
        })?;
        let slug = nonempty_news_slug(name, &format!("tags[{index}]"), path)?;
        tags.push(NewsTag {
            name: name.to_owned(),
            slug,
        });
    }

    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            anyhow::anyhow!("{}: news filename stem is not valid UTF-8", path.display())
        })?;
    let slug = nonempty_news_slug(stem, "filename", path)?;
    let url = format!("/news/{slug}/");

    let author = optional_news_string(frontmatter, "author", path)?;
    let excerpt = optional_news_string(frontmatter, "excerpt", path)?;

    Ok(NewsPost {
        title,
        date,
        category,
        category_slug,
        tags,
        author,
        excerpt,
        slug,
        url,
        body,
    })
}

fn parse_taxonomy(content_dir: &Path) -> Result<Taxonomy> {
    let path = content_dir.join("taxonomy.yaml");
    let content =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let taxonomy: Taxonomy =
        serde_yaml::from_str(&content).with_context(|| "failed to parse taxonomy.yaml")?;
    Ok(taxonomy)
}

fn parse_profile(content_dir: &Path) -> Result<Option<Profile>> {
    let path = content_dir.join("profile.yaml");
    if !path.exists() {
        return Ok(None);
    }
    let content =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let profile: Profile =
        serde_yaml::from_str(&content).with_context(|| "failed to parse profile.yaml")?;
    Ok(Some(profile))
}

/// Parse YAML frontmatter from a Markdown file.
///
/// Expects the content to start with `---\n`, followed by YAML, then `\n---`.
/// Returns the deserialized frontmatter and the remaining body text.
fn parse_frontmatter<T: serde::de::DeserializeOwned>(content: &str) -> Result<(T, String)> {
    let first_fence = content
        .find("---\n")
        .context("missing opening frontmatter delimiter")?;
    let after_first = first_fence + 4;

    let second_fence = content[after_first..]
        .find("\n---")
        .context("missing closing frontmatter delimiter")?;

    let yaml_str = &content[after_first..after_first + second_fence];
    let body = &content[after_first + second_fence + 4..];

    let frontmatter: T =
        serde_yaml::from_str(yaml_str).context("failed to deserialize frontmatter YAML")?;

    Ok((frontmatter, body.to_string()))
}

#[cfg(test)]
#[allow(clippy::needless_raw_string_hashes)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_source_page() {
        let content = r#"---
title: Test Page
slug: test-page
page_type: source
category: mathematics
subcategory: algebra
tier: foundational
keywords:
  - algebra
extraction_status: complete
concepts_generated:
  - test-concept
status: published
---

# Body content here
"#;
        let (page, body): (SourcePage, String) = parse_frontmatter(content).unwrap();
        assert_eq!(page.title, "Test Page");
        assert_eq!(page.slug, "test-page");
        assert_eq!(page.category, "mathematics");
        assert_eq!(page.subcategory, "algebra");
        assert_eq!(page.tier, "foundational");
        assert_eq!(page.keywords, vec!["algebra"]);
        assert_eq!(page.extraction_status, "complete");
        assert_eq!(page.concepts_generated, vec!["test-concept"]);
        assert!(body.contains("# Body content here"));
    }

    #[test]
    fn test_parse_frontmatter_concept_card() {
        let content = r#"---
# === CORE IDENTIFICATION ===
concept: "Test Concept"
slug: "test-concept"
page_type: "concept"
disambiguator: "math"

# === CLASSIFICATION ===
category: "mathematics"
subcategory: "algebra"
tier: "foundational"

# === PROVENANCE ===
derived_from:
  - slug: "test-page"
    title: "Test Page"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "other-concept"
extends: []
related:
  - "another-concept"
contrasts_with: []

# === PROFILE AXES ===
trust_level: "credible"

# === LIFECYCLE ===
status: "published"
---

## Quick Definition
Test.
"#;
        let (card, body): (ConceptCard, String) = parse_frontmatter(content).unwrap();
        assert_eq!(card.concept, "Test Concept");
        assert_eq!(card.slug, "test-concept");
        assert_eq!(card.disambiguator, "math");
        assert_eq!(card.category, "mathematics");
        assert_eq!(card.derived_from.len(), 1);
        assert_eq!(card.derived_from[0].slug, "test-page");
        assert_eq!(card.prerequisites, vec!["other-concept"]);
        assert_eq!(card.related, vec!["another-concept"]);
        assert_eq!(card.trust_level, "credible");
        assert!(body.contains("## Quick Definition"));
    }

    #[test]
    fn test_parse_frontmatter_missing_delimiter_returns_error() {
        let content = "no frontmatter here\n";
        let result: Result<(SourcePage, String)> = parse_frontmatter(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_taxonomy_struct() {
        let yaml = r#"
categories:
  - name: "math"
    description: "Mathematics"
tiers:
  - name: "foundational"
    description: "Core concepts"
    order: 1
"#;
        let taxonomy: Taxonomy = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(taxonomy.categories.len(), 1);
        assert_eq!(taxonomy.categories[0].name, "math");
        assert_eq!(taxonomy.tiers.len(), 1);
        assert_eq!(taxonomy.tiers[0].name, "foundational");
        assert_eq!(taxonomy.tiers[0].order, 1);
    }

    #[test]
    fn test_parse_profile_struct() {
        let yaml = r#"
profile:
  name: "Test Wiki"
  personality: "encyclopedia"
trust:
  enabled: true
  default: "credible"
canon:
  enabled: false
page_types:
  - source
  - concept
relationships:
  - prerequisite
  - related
"#;
        let profile: Profile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(profile.profile.name, "Test Wiki");
        assert_eq!(profile.profile.personality, "encyclopedia");
        assert!(profile.trust.enabled);
        assert_eq!(profile.trust.default, "credible");
        assert!(!profile.canon.enabled);
        assert_eq!(profile.page_types, vec!["source", "concept"]);
    }

    /// The repo-root `demo/` directory, resolved relative to this crate's
    /// manifest (`tools/Cargo.toml`) — `cargo test` runs with the crate root
    /// as its working directory, not the repo root.
    fn demo_dir() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    // F-1: the parser retains the Markdown body for a real demo source and concept.
    #[test]
    fn test_parse_corpus_retains_body_for_real_demo_source_and_concept() {
        let corpus = parse_corpus(&demo_dir()).expect("demo corpus parses");

        let source = corpus
            .sources
            .get("vimalamitra")
            .expect("demo/sources/vimalamitra.md is in the corpus");
        assert!(!source.body.is_empty());
        assert!(source.body.contains("Attributed works"));

        let concept = corpus
            .concepts
            .get("abelian-group")
            .expect("demo/concepts/abelian-group.md is in the corpus");
        assert!(!concept.body.is_empty());
        assert!(concept.body.contains("Quick Definition"));
    }

    fn news_fixture(yaml: &str) -> (assert_fs::TempDir, PathBuf) {
        let dir = assert_fs::TempDir::new().expect("temporary directory");
        let news_dir = dir.path().join("news");
        fs::create_dir_all(&news_dir).expect("news directory");
        let file = news_dir.join("search-lands-in-the-demo-wiki.md");
        fs::write(&file, format!("---\n{yaml}\n---\n\n**Body**\n")).expect("news fixture");
        (dir, file)
    }

    fn valid_news_yaml() -> &'static str {
        "type: news\ntitle: A News Post\ndate: '2026-08-21'\ncategory: Project Updates\ntags: [search, tiny-http]"
    }

    fn assert_news_error(yaml: &str, field: &str) {
        let (dir, file) = news_fixture(yaml);
        let error = format!(
            "{:#}",
            parse_news(dir.path()).expect_err("fixture should be rejected")
        );
        assert!(
            error.contains(file.file_name().unwrap().to_str().unwrap()),
            "{error}"
        );
        assert!(error.contains(field), "expected field {field} in {error}");
    }

    #[test]
    fn test_parse_news_model_derives_identity_and_retains_optional_data() {
        let yaml = format!(
            "{}\nauthor: Haleiki Maintainers\nexcerpt: A short excerpt.\nslug: ignored-authored-slug",
            valid_news_yaml()
        );
        let (dir, _) = news_fixture(&yaml);
        let post = parse_news(dir.path()).expect("valid news parses");
        let post = post
            .get("search-lands-in-the-demo-wiki")
            .expect("filename slug");
        assert_eq!(post.title(), "A News Post");
        assert_eq!(post.date().iso(), "2026-08-21");
        assert_eq!(post.date().display(), "August 21, 2026");
        assert_eq!(post.category(), "Project Updates");
        assert_eq!(post.category_slug(), "project-updates");
        assert_eq!(
            post.tags(),
            &[
                NewsTag {
                    name: "search".into(),
                    slug: "search".into()
                },
                NewsTag {
                    name: "tiny-http".into(),
                    slug: "tiny-http".into()
                },
            ]
        );
        assert_eq!(post.author(), Some("Haleiki Maintainers"));
        assert_eq!(post.excerpt(), Some("A short excerpt."));
        assert_eq!(post.slug(), "search-lands-in-the-demo-wiki");
        assert_eq!(post.url(), "/news/search-lands-in-the-demo-wiki/");
        assert!(post.body().contains("**Body**"));
    }

    #[test]
    fn test_parse_news_allows_empty_tags_and_absent_optional_fields() {
        let yaml = "type: news\ntitle: No Tags\ndate: '2024-02-29'\ncategory: Biology\ntags: []";
        let (dir, _) = news_fixture(yaml);
        let posts = parse_news(dir.path()).expect("empty tag list is valid");
        let post = posts.values().next().expect("one post");
        assert!(post.tags().is_empty());
        assert_eq!(post.author(), None);
        assert_eq!(post.excerpt(), None);
    }

    #[test]
    fn test_parse_news_normalizes_blank_optional_strings_to_absence() {
        let yaml = format!("{}\nauthor: '  '\nexcerpt: ''", valid_news_yaml());
        let (dir, _) = news_fixture(&yaml);
        let post = parse_news(dir.path()).expect("blank optional fields are valid");
        let post = post.values().next().expect("one post");
        assert_eq!(post.author(), None);
        assert_eq!(post.excerpt(), None);
    }

    #[test]
    fn test_parse_news_missing_directory_is_empty() {
        let dir = assert_fs::TempDir::new().expect("temporary directory");
        assert!(
            parse_news(dir.path())
                .expect("missing news directory means empty")
                .is_empty()
        );
    }

    #[test]
    fn test_parse_news_present_non_directory_is_an_io_error() {
        let dir = assert_fs::TempDir::new().expect("temporary directory");
        let news_path = dir.path().join("news");
        fs::write(&news_path, "not a directory").expect("news path fixture");
        let error = parse_news(dir.path()).expect_err("present unreadable news path fails");
        assert!(error.to_string().contains("failed to read"));
        assert!(error.to_string().contains(&news_path.display().to_string()));
    }

    #[test]
    fn test_parse_news_reads_only_direct_regular_markdown_files() {
        let (dir, _) = news_fixture(valid_news_yaml());
        let news_dir = dir.path().join("news");
        fs::create_dir(news_dir.join("nested.md")).expect("directory named md");
        fs::create_dir(news_dir.join("nested")).expect("nested directory");
        fs::write(news_dir.join("nested/ignored.md"), "not news").expect("nested file");
        fs::write(news_dir.join("ignored.txt"), "not news").expect("other extension");
        fs::write(news_dir.join(".gitkeep"), "").expect("gitkeep");
        let posts = parse_news(dir.path()).expect("non-news entries are ignored");
        assert_eq!(posts.len(), 1);
    }

    #[test]
    fn test_parse_news_required_fields_reject_missing_null_and_wrong_shapes() {
        let fields = ["type", "title", "date", "category", "tags"];
        for field in fields {
            let base = valid_news_yaml();
            let without = base
                .lines()
                .filter(|line| !line.starts_with(&format!("{field}:")))
                .collect::<Vec<_>>()
                .join("\n");
            assert_news_error(&without, field);
            for invalid in ["null", "42", "{value: wrong}"] {
                let yaml = if field == "tags" && invalid == "42" {
                    format!(
                        "{}\ntags: {invalid}",
                        without
                            .lines()
                            .filter(|line| !line.starts_with("tags:"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                } else {
                    format!(
                        "{}\n{field}: {invalid}",
                        without
                            .lines()
                            .filter(|line| !line.starts_with(&format!("{field}:")))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                };
                assert_news_error(&yaml, field);
            }
            if field != "tags" {
                let yaml = format!(
                    "{}\n{field}: [wrong]",
                    without
                        .lines()
                        .filter(|line| !line.starts_with(&format!("{field}:")))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                assert_news_error(&yaml, field);
            }
        }

        for element in ["1", "true", "{name: wrong}"] {
            let yaml = format!(
                "{}\ntags: [{element}]",
                valid_news_yaml()
                    .lines()
                    .filter(|line| !line.starts_with("tags:"))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            assert_news_error(&yaml, "tags[0]");
        }
        for element in ["''", "'---'"] {
            let yaml = format!(
                "{}\ntags: [{element}]",
                valid_news_yaml()
                    .lines()
                    .filter(|line| !line.starts_with("tags:"))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            assert_news_error(&yaml, "tags[0]");
        }
    }

    #[test]
    fn test_parse_news_rejects_wrong_type_blank_required_text_and_empty_slugs() {
        for (field, value) in [
            ("type", "source"),
            ("title", " '  '"),
            ("category", " '  '"),
            ("category", " '---'"),
        ] {
            let yaml = format!(
                "{}\n{field}: {value}",
                valid_news_yaml()
                    .lines()
                    .filter(|line| !line.starts_with(&format!("{field}:")))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            assert_news_error(&yaml, field);
        }

        let (dir, _) = news_fixture(valid_news_yaml());
        let news_dir = dir.path().join("news");
        fs::write(
            news_dir.join("---.md"),
            format!("---\n{}\n---\nbody", valid_news_yaml()),
        )
        .expect("empty slug fixture");
        let error = format!(
            "{:#}",
            parse_news(dir.path()).expect_err("empty filename slug rejected")
        );
        assert!(error.contains("---.md"));
        assert!(error.contains("filename"));
    }

    #[test]
    fn test_parse_news_optional_fields_reject_non_string_non_null_values() {
        for field in ["author", "excerpt"] {
            for value in ["42", "[value]", "{value: wrong}"] {
                let yaml = format!("{}\n{field}: {value}", valid_news_yaml());
                assert_news_error(&yaml, field);
            }
        }
    }

    #[test]
    fn test_news_date_accepts_gregorian_leap_and_month_boundaries() {
        for date in ["2024-02-29", "2000-02-29", "2026-04-30"] {
            assert!(NewsDate::parse(date).is_ok(), "{date}");
        }
    }

    #[test]
    fn test_parse_news_rejects_noncanonical_or_impossible_dates() {
        for date in [
            "2026-02-29",
            "1900-02-29",
            "2026-04-31",
            "2026-00-01",
            "2026-13-01",
            "2026-01-00",
            "0000-01-01",
            "2026-8-21",
            "2026-08-21T00:00:00Z",
            " 2026-08-21",
            "2026-08-21 ",
        ] {
            let yaml = format!(
                "{}\ndate: '{date}'",
                valid_news_yaml()
                    .lines()
                    .filter(|line| !line.starts_with("date:"))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            assert_news_error(&yaml, "date");
        }
    }

    #[test]
    fn test_parse_news_slug_is_filename_derived_and_duplicates_fail_with_both_paths() {
        let (dir, file) = news_fixture(
            "type: news\ntitle: A Post\ndate: '2026-08-21'\ncategory: One\ntags: []\nslug: wrong",
        );
        let original_url = parse_news(dir.path())
            .expect("valid post parses")
            .values()
            .next()
            .expect("one post")
            .url()
            .to_owned();
        fs::write(file, "---\ntype: news\ntitle: A Post\ndate: '2024-01-01'\ncategory: One\ntags: []\nslug: different\n---\nbody")
            .expect("replace fixture");
        let changed_url = parse_news(dir.path())
            .expect("changed date parses")
            .values()
            .next()
            .expect("one post")
            .url()
            .to_owned();
        assert_eq!(original_url, changed_url);

        let news_dir = dir.path().join("news");
        fs::write(
            news_dir.join("My Post.md"),
            format!("---\n{}\n---\nbody", valid_news_yaml()),
        )
        .expect("first duplicate");
        fs::write(
            news_dir.join("my-post.md"),
            format!("---\n{}\n---\nbody", valid_news_yaml()),
        )
        .expect("second duplicate");
        let error = parse_news(dir.path())
            .expect_err("normalized duplicate rejected")
            .to_string();
        assert!(error.contains("My Post.md"));
        assert!(error.contains("my-post.md"));
    }

    #[test]
    fn test_parse_news_requires_initial_frontmatter_and_closing_fence() {
        let invalid = [
            "Body first\n---\ntype: news\n---\n",
            "intro\n---\ntype: news\n---\n",
            "---\ntype: news\n",
        ];
        for content in invalid {
            let dir = assert_fs::TempDir::new().expect("temporary directory");
            let news_dir = dir.path().join("news");
            fs::create_dir_all(&news_dir).expect("news directory");
            fs::write(news_dir.join("broken.md"), content).expect("invalid fixture");
            let error = parse_news(dir.path())
                .expect_err("bad fence rejected")
                .to_string();
            assert!(error.contains("broken.md"));
        }

        let dir = assert_fs::TempDir::new().expect("temporary directory");
        let news_dir = dir.path().join("news");
        fs::create_dir_all(&news_dir).expect("news directory");
        fs::write(news_dir.join("invalid-utf8.md"), [0xff, 0xfe]).expect("invalid UTF-8 fixture");
        let error = parse_news(dir.path())
            .expect_err("invalid UTF-8 must fail")
            .to_string();
        assert!(error.contains("invalid-utf8.md"), "{error}");
    }

    #[test]
    #[ignore = "requires demo corpus on disk"]
    fn test_parse_corpus_demo() {
        let corpus = parse_corpus(Path::new("demo")).unwrap();
        assert!(corpus.sources.len() >= 90);
        assert!(corpus.concepts.len() >= 180);
        assert!(!corpus.taxonomy.categories.is_empty());
        assert!(corpus.profile.is_some());
    }
}
