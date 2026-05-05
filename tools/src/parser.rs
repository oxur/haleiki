//! Markdown and frontmatter parsing for source pages and concept cards.
//!
//! This module reads Markdown files from the content directory, extracts YAML
//! frontmatter, and produces structured [`Corpus`] representations suitable for
//! graph construction and validation.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

/// The full parsed corpus: all sources, concepts, taxonomy, and optional profile.
#[derive(Debug, Clone)]
pub struct Corpus {
    /// Source pages keyed by slug.
    pub sources: HashMap<String, SourcePage>,
    /// Concept cards keyed by slug.
    pub concepts: HashMap<String, ConceptCard>,
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
    let taxonomy = parse_taxonomy(content_dir)?;
    let profile = parse_profile(content_dir)?;

    Ok(Corpus {
        sources,
        concepts,
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

        let (page, _body): (SourcePage, String) = parse_frontmatter(&content)
            .with_context(|| format!("failed to parse frontmatter in {}", path.display()))?;

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

        let (card, _body): (ConceptCard, String) = parse_frontmatter(&content)
            .with_context(|| format!("failed to parse frontmatter in {}", path.display()))?;

        map.insert(card.slug.clone(), card);
    }

    Ok(map)
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

    #[test]
    #[ignore] // requires demo corpus on disk
    fn test_parse_corpus_demo() {
        let corpus = parse_corpus(Path::new("demo")).unwrap();
        assert!(corpus.sources.len() >= 90);
        assert!(corpus.concepts.len() >= 180);
        assert!(!corpus.taxonomy.categories.is_empty());
        assert!(corpus.profile.is_some());
    }
}
