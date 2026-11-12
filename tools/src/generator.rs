//! Derived-data generation.
//!
//! This module computes see-also groups, breadcrumb trails, category indices,
//! and provenance links from the knowledge graph, then writes them as JSON
//! files consumed by the static-site build layer.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

use crate::graph::{HaleikiGraph, NodeData};
use crate::parser::Corpus;

/// Trust level description for the profile output.
#[derive(Debug, Clone, Serialize)]
struct TrustLevelDescription {
    label: String,
    description: String,
}

/// Nested trust configuration in profile.json.
#[derive(Debug, Clone, Serialize)]
struct TrustOutput {
    enabled: bool,
    default: String,
    levels: BTreeMap<String, TrustLevelDescription>,
    level_order: Vec<String>,
}

/// Nested canon configuration in profile.json.
#[derive(Debug, Clone, Serialize)]
struct CanonOutput {
    enabled: bool,
}

/// A taxonomy category entry in profile.json.
#[derive(Debug, Clone, Serialize)]
struct TaxonomyCategoryOutput {
    name: String,
    description: String,
}

/// A taxonomy tier entry in profile.json.
#[derive(Debug, Clone, Serialize)]
struct TaxonomyTierOutput {
    name: String,
    description: String,
    order: u32,
}

/// Nested taxonomy in profile.json.
#[derive(Debug, Clone, Serialize)]
struct TaxonomyOutput {
    categories: Vec<TaxonomyCategoryOutput>,
    tiers: Vec<TaxonomyTierOutput>,
}

/// Resolved profile output written to `profile.json` (nested shape per 0005 §8.1).
#[derive(Debug, Clone, Serialize)]
struct ProfileOutput {
    personality: String,
    name: String,
    trust: TrustOutput,
    canon: CanonOutput,
    page_types: Vec<String>,
    relationships: Vec<String>,
    taxonomy: TaxonomyOutput,
}

/// A single node in the graph JSON output.
#[derive(Debug, Clone, Serialize)]
struct GraphNode {
    slug: String,
    #[serde(rename = "type")]
    node_type: String,
    title: String,
}

/// A single edge in the graph JSON output.
#[derive(Debug, Clone, Serialize)]
struct GraphEdge {
    source: String,
    target: String,
    relationship: String,
}

/// The full graph JSON output.
#[derive(Debug, Clone, Serialize)]
struct GraphOutput {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}

/// A provenance entry linking source to concepts and vice versa.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct ProvenanceEntry {
    /// For a source: concepts derived from it. For a concept: sources it derives from.
    pub(crate) links: Vec<ProvenanceLink>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ProvenanceLink {
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) url: String,
}

/// A single source attribution entry in the attribution JSON output.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct AttributionSourceEntry {
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) url: String,
    pub(crate) license: String,
    pub(crate) fetched_at: String,
    pub(crate) revision_id: String,
}

/// The full attribution JSON output.
#[derive(Debug, Clone, Serialize)]
struct AttributionOutput {
    general_notice: String,
    sources_by_project: BTreeMap<String, Vec<AttributionSourceEntry>>,
    source_count: usize,
    license_versions: Vec<String>,
}

/// A featured page entry for the home page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FeaturedEntry {
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) category: String,
    pub(crate) tier: String,
    pub(crate) url: String,
}

/// Generate all output JSON files to the given directory.
///
/// Writes: `graph.json`, `categories.json`, `breadcrumbs.json`,
/// `see_also.json`, `provenance.json`, `profile.json`, `featured.json`,
/// `titles.json`, `attribution.json`.
///
/// # Errors
///
/// Returns an error if the output directory cannot be created or files cannot be written.
pub fn generate(corpus: &Corpus, graph: &HaleikiGraph, output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("failed to create output dir {}", output_dir.display()))?;

    write_graph_json(corpus, graph, output_dir)?;
    write_categories_json(corpus, graph, output_dir)?;
    write_breadcrumbs_json(corpus, graph, output_dir)?;
    write_see_also_json(corpus, graph, output_dir)?;
    write_provenance_json(corpus, output_dir)?;
    write_profile_json(corpus, output_dir)?;
    write_featured_json(corpus, graph, output_dir)?;
    write_titles_json(corpus, output_dir)?;
    write_attribution_json(corpus, output_dir)?;

    Ok(())
}

fn write_graph_json(corpus: &Corpus, graph: &HaleikiGraph, output_dir: &Path) -> Result<()> {
    let mut nodes = Vec::new();
    for node_data in graph.graph.node_weights() {
        let (slug, node_type, title) = match node_data {
            NodeData::Source { slug } => {
                let title = corpus
                    .sources
                    .get(slug)
                    .map_or_else(|| slug.clone(), |p| p.title.clone());
                (slug.clone(), "source".to_string(), title)
            }
            NodeData::Concept { slug } => {
                let title = corpus.concepts.get(slug).map_or_else(
                    || slug.clone(),
                    |c| {
                        if c.disambiguator.is_empty() {
                            c.concept.clone()
                        } else {
                            format!("{} ({})", c.concept, c.disambiguator)
                        }
                    },
                );
                (slug.clone(), "concept".to_string(), title)
            }
        };
        nodes.push(GraphNode {
            slug,
            node_type,
            title,
        });
    }
    // Sort nodes by slug for deterministic output
    nodes.sort_by(|a, b| a.slug.cmp(&b.slug));

    let mut edges = Vec::new();
    for edge_ref in graph.graph.edge_references() {
        let source_slug = graph.graph[edge_ref.source()].slug().to_string();
        let target_slug = graph.graph[edge_ref.target()].slug().to_string();
        let relationship = match edge_ref.weight() {
            crate::graph::EdgeKind::Prerequisite => "prerequisite",
            crate::graph::EdgeKind::Extends => "extends",
            crate::graph::EdgeKind::Related => "related",
            crate::graph::EdgeKind::ContrastsWith => "contrasts_with",
            crate::graph::EdgeKind::DerivedFrom => "derived_from",
            crate::graph::EdgeKind::ConceptsGenerated => "concepts_generated",
        };
        edges.push(GraphEdge {
            source: source_slug,
            target: target_slug,
            relationship: relationship.to_string(),
        });
    }
    // Sort edges by (source, target, relationship) for deterministic output
    edges.sort_by(|a, b| {
        a.source
            .cmp(&b.source)
            .then_with(|| a.target.cmp(&b.target))
            .then_with(|| a.relationship.cmp(&b.relationship))
    });

    let output = GraphOutput { nodes, edges };
    write_json(output_dir, "graph.json", &output)
}

fn write_categories_json(corpus: &Corpus, graph: &HaleikiGraph, output_dir: &Path) -> Result<()> {
    let cat_index = graph.category_index(corpus);
    write_json(output_dir, "categories.json", &cat_index)
}

fn write_breadcrumbs_json(corpus: &Corpus, graph: &HaleikiGraph, output_dir: &Path) -> Result<()> {
    let mut breadcrumbs: BTreeMap<String, Vec<crate::graph::Breadcrumb>> = BTreeMap::new();

    for slug in corpus.sources.keys() {
        breadcrumbs.insert(slug.clone(), graph.breadcrumbs(slug, corpus));
    }
    for slug in corpus.concepts.keys() {
        breadcrumbs.insert(slug.clone(), graph.breadcrumbs(slug, corpus));
    }

    write_json(output_dir, "breadcrumbs.json", &breadcrumbs)
}

fn write_see_also_json(corpus: &Corpus, graph: &HaleikiGraph, output_dir: &Path) -> Result<()> {
    let mut see_also: BTreeMap<String, Vec<crate::graph::SeeAlsoEntry>> = BTreeMap::new();

    for slug in corpus.sources.keys() {
        let mut entries = graph.see_also(slug, corpus);
        // Sort by relationship type then slug for deterministic output
        entries.sort_by(|a, b| {
            a.relationship
                .cmp(&b.relationship)
                .then_with(|| a.slug.cmp(&b.slug))
        });
        if !entries.is_empty() {
            see_also.insert(slug.clone(), entries);
        }
    }
    for slug in corpus.concepts.keys() {
        let mut entries = graph.see_also(slug, corpus);
        entries.sort_by(|a, b| {
            a.relationship
                .cmp(&b.relationship)
                .then_with(|| a.slug.cmp(&b.slug))
        });
        if !entries.is_empty() {
            see_also.insert(slug.clone(), entries);
        }
    }

    write_json(output_dir, "see_also.json", &see_also)
}

fn write_provenance_json(corpus: &Corpus, output_dir: &Path) -> Result<()> {
    let provenance = build_provenance_map(corpus);
    write_json(output_dir, "provenance.json", &provenance)
}

/// Build the provenance map: for each source, the concepts derived from it;
/// for each concept, the sources it derives from. Shared by the JSON writer
/// ([`write_provenance_json`]) and the minijinja render path
/// (`render::assemble_site_context`) — one derivation, two consumers.
pub(crate) fn build_provenance_map(corpus: &Corpus) -> BTreeMap<String, ProvenanceEntry> {
    let mut provenance: BTreeMap<String, ProvenanceEntry> = BTreeMap::new();

    // For each source: which concepts derive from it
    for (slug, page) in &corpus.sources {
        let mut links = Vec::new();
        for concept_slug in &page.concepts_generated {
            if let Some(card) = corpus.concepts.get(concept_slug) {
                let title = if card.disambiguator.is_empty() {
                    card.concept.clone()
                } else {
                    format!("{} ({})", card.concept, card.disambiguator)
                };
                links.push(ProvenanceLink {
                    slug: concept_slug.clone(),
                    title,
                    url: format!("/{concept_slug}/"),
                });
            }
        }
        // Sort links by slug for deterministic output
        links.sort_by(|a, b| a.slug.cmp(&b.slug));
        if !links.is_empty() {
            provenance.insert(slug.clone(), ProvenanceEntry { links });
        }
    }

    // For each concept: which sources it derives from
    for (slug, card) in &corpus.concepts {
        let mut links = Vec::new();
        for entry in &card.derived_from {
            if let Some(page) = corpus.sources.get(&entry.slug) {
                links.push(ProvenanceLink {
                    slug: entry.slug.clone(),
                    title: page.title.clone(),
                    url: format!("/source/{}/", entry.slug),
                });
            }
        }
        // Sort links by slug for deterministic output
        links.sort_by(|a, b| a.slug.cmp(&b.slug));
        if !links.is_empty() {
            provenance.insert(slug.clone(), ProvenanceEntry { links });
        }
    }

    provenance
}

fn write_profile_json(corpus: &Corpus, output_dir: &Path) -> Result<()> {
    let trust_levels = build_trust_levels();
    let level_order = vec![
        "corroborated".to_string(),
        "credible".to_string(),
        "provisional".to_string(),
        "speculative".to_string(),
    ];

    let taxonomy_output = TaxonomyOutput {
        categories: corpus
            .taxonomy
            .categories
            .iter()
            .map(|c| TaxonomyCategoryOutput {
                name: c.name.clone(),
                description: c.description.clone(),
            })
            .collect(),
        tiers: corpus
            .taxonomy
            .tiers
            .iter()
            .map(|t| TaxonomyTierOutput {
                name: t.name.clone(),
                description: t.description.clone(),
                order: t.order,
            })
            .collect(),
    };

    let profile_output = if let Some(profile) = &corpus.profile {
        ProfileOutput {
            personality: profile.profile.personality.clone(),
            name: profile.profile.name.clone(),
            trust: TrustOutput {
                enabled: profile.trust.enabled,
                default: profile.trust.default.clone(),
                levels: trust_levels,
                level_order,
            },
            canon: CanonOutput {
                enabled: profile.canon.enabled,
            },
            page_types: profile.page_types.clone(),
            relationships: profile.relationships.clone(),
            taxonomy: taxonomy_output,
        }
    } else {
        // Defaults when no profile exists
        ProfileOutput {
            personality: "encyclopedia".to_string(),
            name: String::new(),
            trust: TrustOutput {
                enabled: false,
                default: "credible".to_string(),
                levels: trust_levels,
                level_order,
            },
            canon: CanonOutput { enabled: false },
            page_types: vec!["source".to_string(), "concept".to_string()],
            relationships: vec![
                "prerequisite".to_string(),
                "extends".to_string(),
                "related".to_string(),
                "contrasts_with".to_string(),
            ],
            taxonomy: taxonomy_output,
        }
    };

    write_json(output_dir, "profile.json", &profile_output)
}

/// Build trust-level descriptions.
///
/// Uses `BTreeMap` for deterministic key ordering in JSON output.
fn build_trust_levels() -> BTreeMap<String, TrustLevelDescription> {
    let mut levels = BTreeMap::new();
    levels.insert(
        "corroborated".to_string(),
        TrustLevelDescription {
            label: "Corroborated".to_string(),
            description: "Verified from multiple reliable sources".to_string(),
        },
    );
    levels.insert(
        "credible".to_string(),
        TrustLevelDescription {
            label: "Credible".to_string(),
            description: "Well-sourced, reasonable interpretation".to_string(),
        },
    );
    levels.insert(
        "provisional".to_string(),
        TrustLevelDescription {
            label: "Provisional".to_string(),
            description: "Partial evidence, subject to revision".to_string(),
        },
    );
    levels.insert(
        "speculative".to_string(),
        TrustLevelDescription {
            label: "Speculative".to_string(),
            description: "Limited or fragmentary basis".to_string(),
        },
    );
    levels
}

/// Compute top-6 concept cards by in-degree (typed relationships only, excluding
/// provenance edges) and write `featured.json`.
fn write_featured_json(corpus: &Corpus, graph: &HaleikiGraph, output_dir: &Path) -> Result<()> {
    let featured = build_featured(corpus, graph);
    write_json(output_dir, "featured.json", &featured)
}

/// Selects the top-6 concepts by typed (non-provenance) in-degree, for the
/// home page's "Featured Articles" grid and `featured.json`. Shared by the
/// JSON writer ([`write_featured_json`]) and the minijinja render path
/// (`render::assemble_site_context`) — one derivation, two consumers.
pub(crate) fn build_featured(corpus: &Corpus, graph: &HaleikiGraph) -> Vec<FeaturedEntry> {
    use std::collections::HashMap;

    // Count in-degree for concept nodes from typed relationship edges only
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for edge_ref in graph.graph.edge_references() {
        // Skip provenance edges
        match edge_ref.weight() {
            crate::graph::EdgeKind::DerivedFrom | crate::graph::EdgeKind::ConceptsGenerated => {
                continue;
            }
            _ => {}
        }
        let target_data = &graph.graph[edge_ref.target()];
        if matches!(target_data, NodeData::Concept { .. }) {
            *in_degree.entry(target_data.slug().to_string()).or_insert(0) += 1;
        }
    }

    // Sort by in-degree descending, then slug for determinism
    let mut ranked: Vec<(String, usize)> = in_degree.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    ranked
        .into_iter()
        .take(6)
        .filter_map(|(slug, _)| {
            let card = corpus.concepts.get(&slug)?;
            let title = if card.disambiguator.is_empty() {
                card.concept.clone()
            } else {
                format!("{} ({})", card.concept, card.disambiguator)
            };
            Some(FeaturedEntry {
                url: format!("/{slug}/"),
                slug,
                title,
                category: card.category.clone(),
                tier: card.tier.clone(),
            })
        })
        .collect()
}

/// Write a slug-to-title lookup for all content pages (`titles.json`).
///
/// This allows Liquid templates to look up display titles by slug without
/// needing to traverse the graph nodes array.
fn write_titles_json(corpus: &Corpus, output_dir: &Path) -> Result<()> {
    let mut titles: BTreeMap<String, String> = BTreeMap::new();

    for (slug, page) in &corpus.sources {
        titles.insert(slug.clone(), page.title.clone());
    }
    for (slug, card) in &corpus.concepts {
        let title = if card.disambiguator.is_empty() {
            card.concept.clone()
        } else {
            format!("{} ({})", card.concept, card.disambiguator)
        };
        titles.insert(slug.clone(), title);
    }

    write_json(output_dir, "titles.json", &titles)
}

/// Aggregate source-page attribution data and write `attribution.json`.
///
/// Groups sources by `original_source.project`, sorted alphabetically within
/// each group. Collects unique license versions across all sources.
fn write_attribution_json(corpus: &Corpus, output_dir: &Path) -> Result<()> {
    let sources_by_project = build_sources_by_project(corpus);

    let mut license_set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for entry in sources_by_project.values().flatten() {
        if !entry.license.is_empty() {
            license_set.insert(entry.license.clone());
        }
    }

    let source_count = sources_by_project.values().map(Vec::len).sum();
    let license_versions: Vec<String> = license_set.into_iter().collect();

    let output = AttributionOutput {
        general_notice: general_notice().to_string(),
        sources_by_project,
        source_count,
        license_versions,
    };

    write_json(output_dir, "attribution.json", &output)
}

/// Groups source pages by their upstream project (`original_source.project`,
/// or `"unknown"` if absent), each group sorted alphabetically by title.
/// Shared by the JSON writer ([`write_attribution_json`]) and the minijinja
/// render path (`render::assemble_site_context`) — one derivation, two
/// consumers, mirroring `build_provenance_map` / `build_featured`.
pub(crate) fn build_sources_by_project(
    corpus: &Corpus,
) -> BTreeMap<String, Vec<AttributionSourceEntry>> {
    let mut sources_by_project: BTreeMap<String, Vec<AttributionSourceEntry>> = BTreeMap::new();

    for page in corpus.sources.values() {
        if let Some(ref os) = page.original_source {
            let project = if os.project.is_empty() {
                "unknown".to_string()
            } else {
                os.project.clone()
            };
            let entry = AttributionSourceEntry {
                slug: page.slug.clone(),
                title: page.title.clone(),
                url: os.url.clone(),
                license: os.license.clone(),
                fetched_at: os.fetched_at.clone(),
                revision_id: os.revision_id.clone(),
            };
            sources_by_project.entry(project).or_default().push(entry);
        }
    }

    // Sort each project group alphabetically by title
    for entries in sources_by_project.values_mut() {
        entries.sort_by_key(|a| a.title.to_lowercase());
    }

    sources_by_project
}

/// The site-wide content-licensing notice. Currently hardcoded here (shared
/// by the JSON writer and the render path so there's exactly one copy, not
/// two independently-maintained strings); a candidate for moving into
/// `profile.attribution` alongside `intro`/`licenses[]` in a later cleanup —
/// noted, not done this slice (arc-02 slice-03 decision #2).
pub(crate) fn general_notice() -> &'static str {
    "This knowledge base contains content derived from Wikimedia projects \
     and Rigpa Wiki, licensed under Creative Commons Attribution-ShareAlike."
}

/// Write a serializable value as pretty-printed JSON.
fn write_json<T: Serialize>(output_dir: &Path, filename: &str, value: &T) -> Result<()> {
    let path = output_dir.join(filename);
    let json = serde_json::to_string_pretty(value)
        .with_context(|| format!("failed to serialize {filename}"))?;
    fs::write(&path, json).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{
        CanonConfig, ConceptCard, Corpus, DerivedFromEntry, OriginalSource, Profile, ProfileInfo,
        SourcePage, Taxonomy, TaxonomyEntry, TierEntry, TrustConfig,
    };
    use std::collections::HashMap;

    fn make_test_corpus() -> Corpus {
        let mut sources = HashMap::new();
        sources.insert(
            "src-1".to_string(),
            SourcePage {
                title: "Source One".to_string(),
                slug: "src-1".to_string(),
                page_type: "source".to_string(),
                category: "math".to_string(),
                subcategory: "algebra".to_string(),
                tier: "foundational".to_string(),
                keywords: vec![],
                tags: vec![],
                author: String::new(),
                date: String::new(),
                original_source: Some(OriginalSource {
                    title: "Source One".to_string(),
                    project: "en.wikipedia.org".to_string(),
                    url: "https://en.wikipedia.org/wiki/Source_One".to_string(),
                    license: "CC BY-SA 4.0".to_string(),
                    fetched_at: "2026-05-03T22:20:10Z".to_string(),
                    revision_id: "12345".to_string(),
                }),
                extraction_status: "complete".to_string(),
                concepts_generated: vec!["concept-a".to_string()],
                last_extracted: String::new(),
                status: "published".to_string(),
                body: String::new(),
            },
        );

        let mut concepts = HashMap::new();
        concepts.insert(
            "concept-a".to_string(),
            ConceptCard {
                concept: "Concept A".to_string(),
                slug: "concept-a".to_string(),
                page_type: "concept".to_string(),
                disambiguator: String::new(),
                category: "math".to_string(),
                subcategory: "algebra".to_string(),
                tier: "foundational".to_string(),
                subject_headings: vec![],
                keywords: vec![],
                aliases: vec![],
                derived_from: vec![DerivedFromEntry {
                    slug: "src-1".to_string(),
                    title: "Source One".to_string(),
                }],
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
                body: String::new(),
            },
        );

        Corpus {
            sources,
            concepts,
            news: HashMap::new(),
            taxonomy: Taxonomy {
                categories: vec![TaxonomyEntry {
                    name: "math".to_string(),
                    description: "Mathematics".to_string(),
                }],
                tiers: vec![TierEntry {
                    name: "foundational".to_string(),
                    description: "Core".to_string(),
                    order: 1,
                }],
            },
            profile: Some(Profile {
                profile: ProfileInfo {
                    name: "Test Wiki".to_string(),
                    personality: "encyclopedia".to_string(),
                },
                trust: TrustConfig {
                    enabled: true,
                    default: "credible".to_string(),
                },
                canon: CanonConfig { enabled: false },
                page_types: vec!["source".to_string(), "concept".to_string()],
                relationships: vec!["prerequisite".to_string(), "related".to_string()],
            }),
        }
    }

    #[test]
    fn test_generate_creates_all_files() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        let tmp = std::env::temp_dir().join("haleiki-gen-test");
        let _ = fs::remove_dir_all(&tmp);

        generate(&corpus, &graph, &tmp).unwrap();

        assert!(tmp.join("graph.json").exists());
        assert!(tmp.join("categories.json").exists());
        assert!(tmp.join("breadcrumbs.json").exists());
        assert!(tmp.join("see_also.json").exists());
        assert!(tmp.join("provenance.json").exists());
        assert!(tmp.join("profile.json").exists());
        assert!(tmp.join("featured.json").exists());
        assert!(tmp.join("titles.json").exists());
        assert!(tmp.join("attribution.json").exists());

        // Clean up
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_profile_json_nested_shape() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        let tmp = std::env::temp_dir().join("haleiki-gen-test-profile");
        let _ = fs::remove_dir_all(&tmp);

        generate(&corpus, &graph, &tmp).unwrap();

        let content = fs::read_to_string(tmp.join("profile.json")).unwrap();
        let value: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(value["name"], "Test Wiki");
        assert_eq!(value["personality"], "encyclopedia");
        // Nested trust object
        assert!(value["trust"]["enabled"].as_bool().unwrap());
        assert_eq!(value["trust"]["default"], "credible");
        assert!(value["trust"]["levels"]["corroborated"].is_object());
        assert_eq!(
            value["trust"]["levels"]["credible"]["description"],
            "Well-sourced, reasonable interpretation"
        );
        // level_order array
        let level_order = value["trust"]["level_order"].as_array().unwrap();
        assert_eq!(level_order[0], "corroborated");
        assert_eq!(level_order[3], "speculative");
        // Nested canon
        assert!(!value["canon"]["enabled"].as_bool().unwrap());
        // Taxonomy block
        assert!(value["taxonomy"]["categories"].is_array());
        assert!(value["taxonomy"]["tiers"].is_array());

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_provenance_json_bidirectional() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        let tmp = std::env::temp_dir().join("haleiki-gen-test-prov");
        let _ = fs::remove_dir_all(&tmp);

        generate(&corpus, &graph, &tmp).unwrap();

        let content = fs::read_to_string(tmp.join("provenance.json")).unwrap();
        let value: serde_json::Value = serde_json::from_str(&content).unwrap();

        // Source should link to concept
        assert!(value["src-1"]["links"][0]["slug"] == "concept-a");
        // Concept should link back to source
        assert!(value["concept-a"]["links"][0]["slug"] == "src-1");

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_build_trust_levels() {
        let levels = build_trust_levels();
        assert_eq!(levels.len(), 4);
        assert!(levels.contains_key("corroborated"));
        assert!(levels.contains_key("credible"));
        assert!(levels.contains_key("provisional"));
        assert!(levels.contains_key("speculative"));
    }

    #[test]
    fn test_attribution_json_grouped_by_project() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        let tmp = std::env::temp_dir().join("haleiki-gen-test-attr");
        let _ = fs::remove_dir_all(&tmp);

        generate(&corpus, &graph, &tmp).unwrap();

        let content = fs::read_to_string(tmp.join("attribution.json")).unwrap();
        let value: serde_json::Value = serde_json::from_str(&content).unwrap();

        // general_notice is present
        assert!(value["general_notice"].is_string());
        assert!(
            value["general_notice"]
                .as_str()
                .unwrap()
                .contains("Creative Commons")
        );

        // sources_by_project has the expected project key
        assert!(value["sources_by_project"]["en.wikipedia.org"].is_array());
        let wp_sources = value["sources_by_project"]["en.wikipedia.org"]
            .as_array()
            .unwrap();
        assert_eq!(wp_sources.len(), 1);
        assert_eq!(wp_sources[0]["slug"], "src-1");
        assert_eq!(wp_sources[0]["title"], "Source One");
        assert_eq!(wp_sources[0]["license"], "CC BY-SA 4.0");

        // source_count
        assert_eq!(value["source_count"], 1);

        // license_versions
        let versions = value["license_versions"].as_array().unwrap();
        assert!(versions.contains(&serde_json::json!("CC BY-SA 4.0")));

        let _ = fs::remove_dir_all(&tmp);
    }

    /// The repo-root `demo/` directory, resolved relative to this crate's
    /// manifest so tests pass regardless of `cargo test`'s working directory.
    fn demo_dir() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    // Arc 02 slice 01, F-2: build_featured (extracted from write_featured_json)
    // ranks real concepts by typed in-degree, independently re-derived here by
    // counting each returned slug's actual incoming typed edges — not just
    // re-running the same internal loop the function under test uses.
    #[test]
    fn test_build_featured_real_demo_corpus_ranked_by_typed_indegree() {
        let corpus = crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses");
        let graph = HaleikiGraph::build(&corpus);

        let featured = build_featured(&corpus, &graph);

        assert!(!featured.is_empty());
        assert!(featured.len() <= 6);
        for entry in &featured {
            assert!(
                corpus.concepts.contains_key(&entry.slug),
                "featured slug {} must be a real concept",
                entry.slug
            );
            assert_eq!(entry.url, format!("/{}/", entry.slug));
        }

        // Independently recompute each entry's typed (non-provenance) in-degree
        // by walking the graph's edges directly, and confirm the returned order
        // is non-increasing by that count — a real ranking check, not just a
        // shape check.
        let indegree_of = |slug: &str| -> usize {
            let idx = graph.index[slug];
            graph
                .graph
                .edges_directed(idx, petgraph::Direction::Incoming)
                .filter(|e| {
                    !matches!(
                        e.weight(),
                        crate::graph::EdgeKind::DerivedFrom
                            | crate::graph::EdgeKind::ConceptsGenerated
                    )
                })
                .count()
        };
        let counts: Vec<usize> = featured.iter().map(|e| indegree_of(&e.slug)).collect();
        assert!(
            counts.windows(2).all(|w| w[0] >= w[1]),
            "featured entries must be non-increasing by typed in-degree: {counts:?}"
        );

        // Existing JSON writer must still agree with the shared fn (the
        // extraction is a refactor, not a behavior change).
        let tmp = std::env::temp_dir().join("haleiki-gen-test-featured-demo");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        write_featured_json(&corpus, &graph, &tmp).unwrap();
        let content = fs::read_to_string(tmp.join("featured.json")).unwrap();
        let from_json: Vec<FeaturedEntry> = serde_json::from_str(&content).unwrap();
        assert_eq!(from_json.len(), featured.len());
        for (a, b) in from_json.iter().zip(featured.iter()) {
            assert_eq!(a.slug, b.slug);
            assert_eq!(a.title, b.title);
            assert_eq!(a.category, b.category);
            assert_eq!(a.tier, b.tier);
            assert_eq!(a.url, b.url);
        }
        let _ = fs::remove_dir_all(&tmp);
    }
}
