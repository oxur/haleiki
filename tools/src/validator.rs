//! Content validation rules.
//!
//! This module checks that frontmatter conforms to the schema, categories and
//! tiers match the taxonomy, provenance links are consistent, and no duplicate
//! slugs exist within a content type. It produces a [`ValidationReport`] with
//! typed errors and warnings.
//!
//! Note: forward references (typed-relationship slugs pointing to content not
//! yet in the corpus) are intentionally NOT flagged — they are normal in a wiki.
//! Use [`count_forward_refs`] for an informational count.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use anyhow::Context;

use crate::parser::Corpus;

/// Severity of a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

/// Categorization of a validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum ValidationKind {
    DuplicateSlug,
    SlugFilenameMismatch,
    UnknownCategory,
    UnknownTier,
    DanglingProvenance,
    MissingConceptCard,
    BidirectionalMismatch,
    OrphanConcept,
    OrphanSource,
}

/// A single validation finding.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ValidationItem {
    pub severity: Severity,
    pub kind: ValidationKind,
    pub slug: String,
    pub message: String,
}

/// The result of running validation on a corpus.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub errors: Vec<ValidationItem>,
    pub warnings: Vec<ValidationItem>,
}

impl ValidationReport {
    fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Returns `true` if there are any errors (not just warnings).
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Print the report to stderr, grouped by kind with summary counts first.
    pub fn print(&self) {
        // Group errors by kind
        let mut error_groups: HashMap<&ValidationKind, Vec<&ValidationItem>> = HashMap::new();
        for item in &self.errors {
            error_groups.entry(&item.kind).or_default().push(item);
        }

        // Group warnings by kind
        let mut warning_groups: HashMap<&ValidationKind, Vec<&ValidationItem>> = HashMap::new();
        for item in &self.warnings {
            warning_groups.entry(&item.kind).or_default().push(item);
        }

        // Print summary counts first
        if !self.errors.is_empty() {
            eprintln!("  Errors:");
            let mut kinds: Vec<_> = error_groups.keys().collect();
            kinds.sort_by_key(|k| format!("{k:?}"));
            for kind in &kinds {
                eprintln!("    {:?}: {}", kind, error_groups[*kind].len());
            }
        }
        if !self.warnings.is_empty() {
            eprintln!("  Warnings:");
            let mut kinds: Vec<_> = warning_groups.keys().collect();
            kinds.sort_by_key(|k| format!("{k:?}"));
            for kind in &kinds {
                eprintln!("    {:?}: {}", kind, warning_groups[*kind].len());
            }
        }

        // Print detail
        if !self.errors.is_empty() {
            eprintln!();
            for item in &self.errors {
                eprintln!(
                    "  ERROR [{:?}] [{}]: {}",
                    item.kind, item.slug, item.message
                );
            }
        }
        if !self.warnings.is_empty() {
            eprintln!();
            for item in &self.warnings {
                eprintln!(
                    "  WARN  [{:?}] [{}]: {}",
                    item.kind, item.slug, item.message
                );
            }
        }
    }

    fn error(&mut self, kind: ValidationKind, slug: &str, message: impl Into<String>) {
        self.errors.push(ValidationItem {
            severity: Severity::Error,
            kind,
            slug: slug.to_string(),
            message: message.into(),
        });
    }

    fn warn(&mut self, kind: ValidationKind, slug: &str, message: impl Into<String>) {
        self.warnings.push(ValidationItem {
            severity: Severity::Warning,
            kind,
            slug: slug.to_string(),
            message: message.into(),
        });
    }
}

/// Validate the corpus against all defined rules.
///
/// Returns a report containing errors (which should block the build) and
/// warnings (informational).
///
/// The `content_dir` parameter is needed for the filesystem-level duplicate slug
/// check (the parser's `HashMap` silently overwrites duplicates, so we must re-walk
/// the filesystem to detect them).
pub fn validate(corpus: &Corpus, content_dir: &Path) -> ValidationReport {
    let mut report = ValidationReport::new();

    check_duplicate_slugs(content_dir, &mut report);
    check_unknown_categories(corpus, &mut report);
    check_unknown_tiers(corpus, &mut report);
    check_dangling_provenance(corpus, &mut report);
    check_concepts_generated(corpus, &mut report);
    check_bidirectional_mismatch(corpus, &mut report);
    check_orphan_concepts(corpus, &mut report);
    check_orphan_sources(corpus, &mut report);

    report
}

/// Count forward references (typed-relationship slugs referencing content not yet
/// in the corpus). Returns the count for informational display in `stats`.
pub fn count_forward_refs(corpus: &Corpus) -> usize {
    let all_slugs: HashSet<&str> = corpus
        .sources
        .keys()
        .chain(corpus.concepts.keys())
        .map(String::as_str)
        .collect();

    let mut count = 0;
    for card in corpus.concepts.values() {
        for prereq in &card.prerequisites {
            if !all_slugs.contains(prereq.as_str()) {
                count += 1;
            }
        }
        for ext in &card.extends {
            if !all_slugs.contains(ext.as_str()) {
                count += 1;
            }
        }
        for rel in &card.related {
            if !all_slugs.contains(rel.as_str()) {
                count += 1;
            }
        }
        for contrast in &card.contrasts_with {
            if !all_slugs.contains(contrast.as_str()) {
                count += 1;
            }
        }
    }
    count
}

/// Check for slugs that appear more than once within the same content type.
///
/// Re-walks the filesystem and parses just the `slug` field from each file's
/// frontmatter. Groups by slug within sources and within concepts separately.
/// Cross-type duplicates (a source and a concept sharing a slug) are NOT errors
/// per Path B (type-namespaced slugs).
///
/// This catches cases where the parser's `HashMap` would silently overwrite.
fn check_duplicate_slugs(content_dir: &Path, report: &mut ValidationReport) {
    check_duplicate_slugs_in_dir(content_dir, "sources", report);
    check_duplicate_slugs_in_dir(content_dir, "concepts", report);
}

/// Walk a subdirectory and detect duplicate slug fields.
fn check_duplicate_slugs_in_dir(content_dir: &Path, subdir: &str, report: &mut ValidationReport) {
    let dir = content_dir.join(subdir);
    if !dir.exists() {
        return;
    }

    let entries =
        match fs::read_dir(&dir).with_context(|| format!("failed to read {}", dir.display())) {
            Ok(entries) => entries,
            Err(e) => {
                report.error(
                    ValidationKind::DuplicateSlug,
                    subdir,
                    format!("could not read directory for duplicate check: {e}"),
                );
                return;
            }
        };

    let mut slug_to_files: HashMap<String, Vec<String>> = HashMap::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if let Ok(content) = fs::read_to_string(&path) {
            if let Some(slug) = extract_slug_field(&content) {
                slug_to_files.entry(slug).or_default().push(filename);
            }
        }
    }

    for (slug, files) in &slug_to_files {
        if files.len() > 1 {
            report.error(
                ValidationKind::DuplicateSlug,
                slug,
                format!(
                    "duplicate slug in {subdir}/: found in files [{}]",
                    files.join(", ")
                ),
            );
        }
    }
}

/// Extract just the `slug` field from YAML frontmatter without fully parsing.
fn extract_slug_field(content: &str) -> Option<String> {
    let first_fence = content.find("---\n")?;
    let after_first = first_fence + 4;
    let second_fence = content[after_first..].find("\n---")?;
    let yaml_str = &content[after_first..after_first + second_fence];

    for line in yaml_str.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("slug:") {
            let value = rest.trim().trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

/// Check that category fields match taxonomy entries.
fn check_unknown_categories(corpus: &Corpus, report: &mut ValidationReport) {
    let valid_categories: HashSet<&str> = corpus
        .taxonomy
        .categories
        .iter()
        .map(|c| c.name.as_str())
        .collect();

    for (slug, page) in &corpus.sources {
        if !page.category.is_empty() && !valid_categories.contains(page.category.as_str()) {
            report.error(
                ValidationKind::UnknownCategory,
                slug,
                format!("unknown category '{}'", page.category),
            );
        }
    }

    for (slug, card) in &corpus.concepts {
        if !card.category.is_empty() && !valid_categories.contains(card.category.as_str()) {
            report.error(
                ValidationKind::UnknownCategory,
                slug,
                format!("unknown category '{}'", card.category),
            );
        }
    }
}

/// Check that tier fields match taxonomy entries.
fn check_unknown_tiers(corpus: &Corpus, report: &mut ValidationReport) {
    let valid_tiers: HashSet<&str> = corpus
        .taxonomy
        .tiers
        .iter()
        .map(|t| t.name.as_str())
        .collect();

    for (slug, page) in &corpus.sources {
        if !page.tier.is_empty() && !valid_tiers.contains(page.tier.as_str()) {
            report.error(
                ValidationKind::UnknownTier,
                slug,
                format!("unknown tier '{}'", page.tier),
            );
        }
    }

    for (slug, card) in &corpus.concepts {
        if !card.tier.is_empty() && !valid_tiers.contains(card.tier.as_str()) {
            report.error(
                ValidationKind::UnknownTier,
                slug,
                format!("unknown tier '{}'", card.tier),
            );
        }
    }
}

/// Check that `derived_from` entries point to existing source pages.
fn check_dangling_provenance(corpus: &Corpus, report: &mut ValidationReport) {
    let source_slugs: HashSet<&str> = corpus.sources.keys().map(String::as_str).collect();

    for (slug, card) in &corpus.concepts {
        for entry in &card.derived_from {
            if !source_slugs.contains(entry.slug.as_str()) {
                report.error(
                    ValidationKind::DanglingProvenance,
                    slug,
                    format!("derived_from slug '{}' is not a source page", entry.slug),
                );
            }
        }
    }
}

/// Check that `concepts_generated` entries in sources correspond to existing concept cards.
fn check_concepts_generated(corpus: &Corpus, report: &mut ValidationReport) {
    for (slug, page) in &corpus.sources {
        for concept_slug in &page.concepts_generated {
            if !corpus.concepts.contains_key(concept_slug) {
                report.error(
                    ValidationKind::MissingConceptCard,
                    slug,
                    format!("concepts_generated lists '{concept_slug}' but no concept card exists"),
                );
            }
        }
    }
}

/// Check bidirectional consistency between `derived_from` and `concepts_generated`.
fn check_bidirectional_mismatch(corpus: &Corpus, report: &mut ValidationReport) {
    // For each concept's derived_from entry, check that the source lists this concept
    for (concept_slug, card) in &corpus.concepts {
        for entry in &card.derived_from {
            if let Some(source) = corpus.sources.get(&entry.slug) {
                if !source.concepts_generated.contains(concept_slug) {
                    report.warn(
                        ValidationKind::BidirectionalMismatch,
                        concept_slug,
                        format!(
                            "derived_from '{}' but source does not list this concept in concepts_generated",
                            entry.slug
                        ),
                    );
                }
            }
        }
    }

    // For each source's concepts_generated entry, check that the concept lists this source
    for (source_slug, page) in &corpus.sources {
        for concept_slug in &page.concepts_generated {
            if let Some(card) = corpus.concepts.get(concept_slug) {
                let has_back_ref = card.derived_from.iter().any(|e| e.slug == *source_slug);
                if !has_back_ref {
                    report.warn(
                        ValidationKind::BidirectionalMismatch,
                        source_slug,
                        format!(
                            "concepts_generated lists '{concept_slug}' but concept does not list this source in derived_from"
                        ),
                    );
                }
            }
        }
    }
}

/// Check for orphan concepts (no typed-relationship edges).
fn check_orphan_concepts(corpus: &Corpus, report: &mut ValidationReport) {
    for (slug, card) in &corpus.concepts {
        let has_relationships = !card.prerequisites.is_empty()
            || !card.extends.is_empty()
            || !card.related.is_empty()
            || !card.contrasts_with.is_empty();

        // Also check if any other concept references this one
        let is_referenced = corpus.concepts.values().any(|other| {
            other.slug != *slug
                && (other.prerequisites.contains(slug)
                    || other.extends.contains(slug)
                    || other.related.contains(slug)
                    || other.contrasts_with.contains(slug))
        });

        if !has_relationships && !is_referenced {
            report.warn(
                ValidationKind::OrphanConcept,
                slug,
                "concept has no typed-relationship edges".to_string(),
            );
        }
    }
}

/// Check for orphan sources: extraction complete, empty `concepts_generated`,
/// and no concept derives from it. Skip if `extraction_status` is "skipped".
fn check_orphan_sources(corpus: &Corpus, report: &mut ValidationReport) {
    for (slug, page) in &corpus.sources {
        // Skip pages with extraction_status "skipped"
        if page.extraction_status == "skipped" {
            continue;
        }

        if page.extraction_status == "complete"
            && page.concepts_generated.is_empty()
            && !corpus
                .concepts
                .values()
                .any(|card| card.derived_from.iter().any(|e| e.slug == *slug))
        {
            report.warn(
                ValidationKind::OrphanSource,
                slug,
                "source has extraction_status 'complete' but no concepts generated or derived"
                    .to_string(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{
        ConceptCard, Corpus, DerivedFromEntry, SourcePage, Taxonomy, TaxonomyEntry, TierEntry,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn test_content_dir() -> PathBuf {
        // Use a non-existent path; the duplicate slug check gracefully handles
        // missing directories by returning early.
        PathBuf::from("/tmp/haleiki-validator-test-nonexistent")
    }

    fn make_taxonomy() -> Taxonomy {
        Taxonomy {
            categories: vec![TaxonomyEntry {
                name: "math".to_string(),
                description: "Mathematics".to_string(),
            }],
            tiers: vec![TierEntry {
                name: "foundational".to_string(),
                description: "Core".to_string(),
                order: 1,
            }],
        }
    }

    fn make_source(slug: &str, category: &str, tier: &str) -> SourcePage {
        SourcePage {
            title: slug.to_string(),
            slug: slug.to_string(),
            page_type: "source".to_string(),
            category: category.to_string(),
            subcategory: String::new(),
            tier: tier.to_string(),
            keywords: vec![],
            tags: vec![],
            author: String::new(),
            date: String::new(),
            original_source: None,
            extraction_status: "complete".to_string(),
            concepts_generated: vec![],
            last_extracted: String::new(),
            status: "published".to_string(),
        }
    }

    fn make_concept(slug: &str, category: &str, tier: &str) -> ConceptCard {
        ConceptCard {
            concept: slug.to_string(),
            slug: slug.to_string(),
            page_type: "concept".to_string(),
            disambiguator: String::new(),
            category: category.to_string(),
            subcategory: String::new(),
            tier: tier.to_string(),
            subject_headings: vec![],
            keywords: vec![],
            aliases: vec![],
            derived_from: vec![],
            external_references: vec![],
            extraction_confidence: String::new(),
            prerequisites: vec![],
            extends: vec![],
            related: vec![],
            contrasts_with: vec![],
            trust_level: "credible".to_string(),
            canon_status: String::new(),
            answers_questions: vec![],
            status: "published".to_string(),
            stable_since: String::new(),
            last_reviewed: String::new(),
        }
    }

    #[test]
    fn test_validate_clean_corpus_no_errors() {
        let mut sources = HashMap::new();
        let mut src = make_source("src-1", "math", "foundational");
        src.concepts_generated = vec!["concept-1".to_string()];
        sources.insert("src-1".to_string(), src);

        let mut concepts = HashMap::new();
        let mut concept = make_concept("concept-1", "math", "foundational");
        concept.derived_from = vec![DerivedFromEntry {
            slug: "src-1".to_string(),
            title: "Source 1".to_string(),
        }];
        concept.related = vec!["concept-1".to_string()]; // self-ref is okay for edge check
        concepts.insert("concept-1".to_string(), concept);

        let corpus = Corpus {
            sources,
            concepts,
            taxonomy: make_taxonomy(),
            profile: None,
        };

        let report = validate(&corpus, &test_content_dir());
        assert!(
            !report.has_errors(),
            "unexpected errors: {:?}",
            report.errors
        );
    }

    #[test]
    fn test_validate_unknown_category_error() {
        let mut sources = HashMap::new();
        sources.insert(
            "s1".to_string(),
            make_source("s1", "bogus-cat", "foundational"),
        );

        let corpus = Corpus {
            sources,
            concepts: HashMap::new(),
            taxonomy: make_taxonomy(),
            profile: None,
        };

        let report = validate(&corpus, &test_content_dir());
        assert!(report.has_errors());
        assert_eq!(report.errors[0].kind, ValidationKind::UnknownCategory);
    }

    #[test]
    fn test_validate_unknown_tier_error() {
        let mut sources = HashMap::new();
        sources.insert("s1".to_string(), make_source("s1", "math", "expert"));

        let corpus = Corpus {
            sources,
            concepts: HashMap::new(),
            taxonomy: make_taxonomy(),
            profile: None,
        };

        let report = validate(&corpus, &test_content_dir());
        assert!(report.has_errors());
        assert_eq!(report.errors[0].kind, ValidationKind::UnknownTier);
    }

    #[test]
    fn test_count_forward_refs() {
        let mut concepts = HashMap::new();
        let mut card = make_concept("c1", "math", "foundational");
        card.prerequisites = vec!["nonexistent".to_string()];
        card.related = vec!["also-missing".to_string()];
        concepts.insert("c1".to_string(), card);

        let corpus = Corpus {
            sources: HashMap::new(),
            concepts,
            taxonomy: make_taxonomy(),
            profile: None,
        };

        // Forward references are counted but not flagged as errors/warnings
        let count = count_forward_refs(&corpus);
        assert_eq!(count, 2);

        let report = validate(&corpus, &test_content_dir());
        // No errors from forward refs
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_dangling_provenance_error() {
        let mut concepts = HashMap::new();
        let mut card = make_concept("c1", "math", "foundational");
        card.derived_from = vec![DerivedFromEntry {
            slug: "nonexistent-source".to_string(),
            title: "Gone".to_string(),
        }];
        concepts.insert("c1".to_string(), card);

        let corpus = Corpus {
            sources: HashMap::new(),
            concepts,
            taxonomy: make_taxonomy(),
            profile: None,
        };

        let report = validate(&corpus, &test_content_dir());
        assert!(report.has_errors());
        assert_eq!(report.errors[0].kind, ValidationKind::DanglingProvenance);
    }

    #[test]
    fn test_validate_missing_concept_card_error() {
        let mut sources = HashMap::new();
        let mut src = make_source("s1", "math", "foundational");
        src.concepts_generated = vec!["ghost-concept".to_string()];
        sources.insert("s1".to_string(), src);

        let corpus = Corpus {
            sources,
            concepts: HashMap::new(),
            taxonomy: make_taxonomy(),
            profile: None,
        };

        let report = validate(&corpus, &test_content_dir());
        assert!(report.has_errors());
        assert_eq!(report.errors[0].kind, ValidationKind::MissingConceptCard);
    }

    #[test]
    fn test_validate_orphan_concept_warning() {
        let mut concepts = HashMap::new();
        concepts.insert(
            "lonely".to_string(),
            make_concept("lonely", "math", "foundational"),
        );

        let corpus = Corpus {
            sources: HashMap::new(),
            concepts,
            taxonomy: make_taxonomy(),
            profile: None,
        };

        let report = validate(&corpus, &test_content_dir());
        assert!(!report.has_errors());
        assert!(
            report
                .warnings
                .iter()
                .any(|w| w.kind == ValidationKind::OrphanConcept)
        );
    }

    #[test]
    fn test_validate_orphan_source_warning() {
        let mut sources = HashMap::new();
        let src = make_source("orphan-src", "math", "foundational");
        sources.insert("orphan-src".to_string(), src);

        let corpus = Corpus {
            sources,
            concepts: HashMap::new(),
            taxonomy: make_taxonomy(),
            profile: None,
        };

        let report = validate(&corpus, &test_content_dir());
        assert!(!report.has_errors());
        assert!(
            report
                .warnings
                .iter()
                .any(|w| w.kind == ValidationKind::OrphanSource)
        );
    }

    #[test]
    fn test_validate_skipped_source_not_orphan() {
        let mut sources = HashMap::new();
        let mut src = make_source("skipped-src", "math", "foundational");
        src.extraction_status = "skipped".to_string();
        sources.insert("skipped-src".to_string(), src);

        let corpus = Corpus {
            sources,
            concepts: HashMap::new(),
            taxonomy: make_taxonomy(),
            profile: None,
        };

        let report = validate(&corpus, &test_content_dir());
        assert!(!report.has_errors());
        assert!(
            !report
                .warnings
                .iter()
                .any(|w| w.kind == ValidationKind::OrphanSource),
            "skipped sources should not be flagged as orphans"
        );
    }

    #[test]
    fn test_validate_bidirectional_mismatch_warning() {
        let mut sources = HashMap::new();
        let mut src = make_source("s1", "math", "foundational");
        src.concepts_generated = vec!["c1".to_string()];
        sources.insert("s1".to_string(), src);

        let mut concepts = HashMap::new();
        // concept does NOT list s1 in derived_from
        let card = make_concept("c1", "math", "foundational");
        concepts.insert("c1".to_string(), card);

        let corpus = Corpus {
            sources,
            concepts,
            taxonomy: make_taxonomy(),
            profile: None,
        };

        let report = validate(&corpus, &test_content_dir());
        assert!(!report.has_errors());
        assert!(
            report
                .warnings
                .iter()
                .any(|w| w.kind == ValidationKind::BidirectionalMismatch)
        );
    }
}
