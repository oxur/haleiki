//! Knowledge-graph construction and traversal.
//!
//! This module uses `petgraph` to build a directed graph of concept cards
//! and source pages, with typed edges representing relationships such as
//! prerequisites, see-also links, and provenance chains.

use std::collections::{BTreeMap, HashMap};

use petgraph::Direction;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Serialize;

use crate::parser::Corpus;

/// The type of a node in the knowledge graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum NodeData {
    Source { slug: String },
    Concept { slug: String },
}

impl NodeData {
    pub fn slug(&self) -> &str {
        match self {
            Self::Source { slug } | Self::Concept { slug } => slug,
        }
    }
}

/// The type of an edge in the knowledge graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum EdgeKind {
    Prerequisite,
    Extends,
    Related,
    ContrastsWith,
    DerivedFrom,
    ConceptsGenerated,
}

/// A breadcrumb entry for navigation.
#[derive(Debug, Clone, Serialize)]
pub struct Breadcrumb {
    pub label: String,
    pub url: String,
}

/// A see-also entry with relationship type information.
#[derive(Debug, Clone, Serialize)]
pub struct SeeAlsoEntry {
    pub slug: String,
    pub title: String,
    pub relationship: String,
    pub url: String,
}

/// Statistics about the knowledge graph.
#[derive(Debug, Clone, Serialize)]
pub struct GraphStats {
    pub source_count: usize,
    pub concept_count: usize,
    pub total_nodes: usize,
    pub total_edges: usize,
    pub prerequisite_edges: usize,
    pub extends_edges: usize,
    pub related_edges: usize,
    pub contrasts_with_edges: usize,
    pub derived_from_edges: usize,
    pub concepts_generated_edges: usize,
    pub orphan_count: usize,
}

/// A single category entry in the category index JSON output.
#[derive(Debug, Clone, Serialize)]
pub struct CategoryIndexEntry {
    pub name: String,
    pub description: String,
    pub sources: Vec<String>,
    pub concepts: Vec<String>,
    pub subcategories: BTreeMap<String, SubcategoryEntry>,
}

/// A subcategory entry within a category.
#[derive(Debug, Clone, Serialize)]
pub struct SubcategoryEntry {
    pub sources: Vec<String>,
    pub concepts: Vec<String>,
}

/// The Haleiki knowledge graph wrapping petgraph.
#[derive(Debug)]
pub struct HaleikiGraph {
    pub graph: Graph<NodeData, EdgeKind>,
    pub index: HashMap<String, NodeIndex>,
}

impl HaleikiGraph {
    /// Build the knowledge graph from a parsed corpus.
    ///
    /// Adds all sources and concepts as nodes, then edges from relationship fields.
    /// Symmetric relationships (Related, `ContrastsWith`) get edges in both directions.
    pub fn build(corpus: &Corpus) -> Self {
        let mut graph = Graph::new();
        let mut index = HashMap::new();

        // Add source nodes
        for slug in corpus.sources.keys() {
            let node_idx = graph.add_node(NodeData::Source { slug: slug.clone() });
            index.insert(slug.clone(), node_idx);
        }

        // Add concept nodes
        for slug in corpus.concepts.keys() {
            let node_idx = graph.add_node(NodeData::Concept { slug: slug.clone() });
            index.insert(slug.clone(), node_idx);
        }

        // Add edges from concept relationships
        for (slug, card) in &corpus.concepts {
            let Some(&from_idx) = index.get(slug) else {
                continue;
            };

            // Prerequisites
            for prereq in &card.prerequisites {
                if let Some(&to_idx) = index.get(prereq) {
                    graph.add_edge(from_idx, to_idx, EdgeKind::Prerequisite);
                }
            }

            // Extends
            for ext in &card.extends {
                if let Some(&to_idx) = index.get(ext) {
                    graph.add_edge(from_idx, to_idx, EdgeKind::Extends);
                }
            }

            // Related (symmetric)
            for rel in &card.related {
                if let Some(&to_idx) = index.get(rel) {
                    graph.add_edge(from_idx, to_idx, EdgeKind::Related);
                    graph.add_edge(to_idx, from_idx, EdgeKind::Related);
                }
            }

            // Contrasts with (symmetric)
            for contrast in &card.contrasts_with {
                if let Some(&to_idx) = index.get(contrast) {
                    graph.add_edge(from_idx, to_idx, EdgeKind::ContrastsWith);
                    graph.add_edge(to_idx, from_idx, EdgeKind::ContrastsWith);
                }
            }

            // Derived from (concept → source)
            for entry in &card.derived_from {
                if let Some(&to_idx) = index.get(&entry.slug) {
                    graph.add_edge(from_idx, to_idx, EdgeKind::DerivedFrom);
                }
            }
        }

        // Add ConceptsGenerated edges (source → concept)
        for (slug, page) in &corpus.sources {
            let Some(&from_idx) = index.get(slug) else {
                continue;
            };
            for concept_slug in &page.concepts_generated {
                if let Some(&to_idx) = index.get(concept_slug) {
                    graph.add_edge(from_idx, to_idx, EdgeKind::ConceptsGenerated);
                }
            }
        }

        Self { graph, index }
    }

    /// Get see-also entries for a given slug.
    ///
    /// Collects all neighbors by edge type, resolving titles and URLs from the
    /// corpus per the v1.3 cross-reference resolution rule:
    ///
    /// - `derived_from` -> always source page URL
    /// - `concepts_generated` -> always concept card URL
    /// - `prerequisites`, `extends`, `related`, `contrasts_with` -> concept-first:
    ///   if a concept card with that slug exists, resolve to it; otherwise resolve
    ///   to the source page if one exists; otherwise drop (unresolved forward ref).
    pub fn see_also(&self, slug: &str, corpus: &Corpus) -> Vec<SeeAlsoEntry> {
        let Some(&node_idx) = self.index.get(slug) else {
            return Vec::new();
        };

        let mut entries = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for edge in self.graph.edges(node_idx) {
            let target_idx = edge.target();
            let target_data = &self.graph[target_idx];
            let target_slug = target_data.slug();
            let edge_kind = *edge.weight();

            // Skip provenance edges for see-also display
            if matches!(
                edge_kind,
                EdgeKind::DerivedFrom | EdgeKind::ConceptsGenerated
            ) {
                continue;
            }

            // Deduplicate
            let key = (target_slug.to_string(), edge_kind);
            if !seen.insert(key) {
                continue;
            }

            let title = resolve_title(target_slug, corpus);
            let url = resolve_url_by_edge(target_slug, edge_kind, corpus);
            let relationship = match edge_kind {
                EdgeKind::Prerequisite => "prerequisite",
                EdgeKind::Extends => "extends",
                EdgeKind::Related => "related",
                EdgeKind::ContrastsWith => "contrasts_with",
                EdgeKind::DerivedFrom | EdgeKind::ConceptsGenerated => unreachable!(),
            };

            entries.push(SeeAlsoEntry {
                slug: target_slug.to_string(),
                title,
                relationship: relationship.to_string(),
                url,
            });
        }

        entries
    }

    /// Compute breadcrumbs for a given slug.
    ///
    /// Returns: Home / Category / Subcategory / Page
    #[allow(clippy::unused_self)]
    pub fn breadcrumbs(&self, slug: &str, corpus: &Corpus) -> Vec<Breadcrumb> {
        let mut crumbs = vec![Breadcrumb {
            label: "Home".to_string(),
            url: "/".to_string(),
        }];

        let (category, subcategory, is_source) = if let Some(page) = corpus.sources.get(slug) {
            (&page.category, &page.subcategory, true)
        } else if let Some(card) = corpus.concepts.get(slug) {
            (&card.category, &card.subcategory, false)
        } else {
            return crumbs;
        };

        if !category.is_empty() {
            let cat_label = title_case(category);
            crumbs.push(Breadcrumb {
                label: cat_label,
                url: format!("/category/{category}/"),
            });

            if !subcategory.is_empty() {
                let sub_label = title_case(subcategory);
                crumbs.push(Breadcrumb {
                    label: sub_label,
                    url: format!("/category/{category}/{subcategory}/"),
                });
            }
        }

        // Final crumb: the page itself
        let title = resolve_title(slug, corpus);
        let url = if is_source {
            format!("/source/{slug}/")
        } else {
            format!("/{slug}/")
        };
        crumbs.push(Breadcrumb { label: title, url });

        crumbs
    }

    /// Build a category index with the nested shape required by the spec.
    ///
    /// Returns: category -> `CategoryIndexEntry` with `name`, `description`,
    /// top-level `sources`/`concepts` arrays, and nested `subcategories`.
    /// Uses `BTreeMap` for deterministic key ordering.
    #[allow(clippy::unused_self)]
    pub fn category_index(&self, corpus: &Corpus) -> BTreeMap<String, CategoryIndexEntry> {
        type SubcatMap = HashMap<String, (Vec<String>, Vec<String>)>;
        // Intermediate structure: category -> subcategory -> (sources, concepts)
        let mut raw: HashMap<String, SubcatMap> = HashMap::new();

        for (slug, page) in &corpus.sources {
            if !page.category.is_empty() {
                let subcat = if page.subcategory.is_empty() {
                    "_uncategorized".to_string()
                } else {
                    page.subcategory.clone()
                };
                raw.entry(page.category.clone())
                    .or_default()
                    .entry(subcat)
                    .or_default()
                    .0
                    .push(slug.clone());
            }
        }

        for (slug, card) in &corpus.concepts {
            if !card.category.is_empty() {
                let subcat = if card.subcategory.is_empty() {
                    "_uncategorized".to_string()
                } else {
                    card.subcategory.clone()
                };
                raw.entry(card.category.clone())
                    .or_default()
                    .entry(subcat)
                    .or_default()
                    .1
                    .push(slug.clone());
            }
        }

        // Build description lookup from taxonomy
        let cat_descriptions: HashMap<&str, &str> = corpus
            .taxonomy
            .categories
            .iter()
            .map(|c| (c.name.as_str(), c.description.as_str()))
            .collect();

        // Build the final structure with BTreeMap for deterministic ordering
        let mut index: BTreeMap<String, CategoryIndexEntry> = BTreeMap::new();

        for (cat_name, subcats) in &raw {
            let description = cat_descriptions
                .get(cat_name.as_str())
                .unwrap_or(&"")
                .to_string();

            // Collect all sources and concepts across subcategories
            let mut all_sources: Vec<String> = Vec::new();
            let mut all_concepts: Vec<String> = Vec::new();
            let mut subcategories: BTreeMap<String, SubcategoryEntry> = BTreeMap::new();

            for (subcat_name, (sources, concepts)) in subcats {
                all_sources.extend(sources.clone());
                all_concepts.extend(concepts.clone());

                // Don't include _uncategorized as a named subcategory
                if subcat_name != "_uncategorized" {
                    let mut sub_sources = sources.clone();
                    let mut sub_concepts = concepts.clone();
                    sub_sources.sort();
                    sub_concepts.sort();
                    sub_sources.dedup();
                    sub_concepts.dedup();
                    subcategories.insert(
                        subcat_name.clone(),
                        SubcategoryEntry {
                            sources: sub_sources,
                            concepts: sub_concepts,
                        },
                    );
                }
            }

            all_sources.sort();
            all_sources.dedup();
            all_concepts.sort();
            all_concepts.dedup();

            index.insert(
                cat_name.clone(),
                CategoryIndexEntry {
                    name: cat_name.clone(),
                    description,
                    sources: all_sources,
                    concepts: all_concepts,
                    subcategories,
                },
            );
        }

        index
    }

    /// Find orphan nodes (no typed-relationship edges, excluding provenance).
    pub fn orphans(&self) -> Vec<String> {
        let mut orphans = Vec::new();

        for (slug, &node_idx) in &self.index {
            let has_typed_edge = self
                .graph
                .edges_directed(node_idx, Direction::Outgoing)
                .chain(self.graph.edges_directed(node_idx, Direction::Incoming))
                .any(|e| {
                    !matches!(
                        e.weight(),
                        EdgeKind::DerivedFrom | EdgeKind::ConceptsGenerated
                    )
                });

            if !has_typed_edge {
                orphans.push(slug.clone());
            }
        }

        orphans.sort();
        orphans
    }

    /// Compute statistics about the graph.
    pub fn stats(&self, corpus: &Corpus) -> GraphStats {
        let mut prerequisite_edges = 0;
        let mut extends_edges = 0;
        let mut related_edges = 0;
        let mut contrasts_with_edges = 0;
        let mut derived_from_edges = 0;
        let mut concepts_generated_edges = 0;

        for edge in self.graph.edge_weights() {
            match edge {
                EdgeKind::Prerequisite => prerequisite_edges += 1,
                EdgeKind::Extends => extends_edges += 1,
                EdgeKind::Related => related_edges += 1,
                EdgeKind::ContrastsWith => contrasts_with_edges += 1,
                EdgeKind::DerivedFrom => derived_from_edges += 1,
                EdgeKind::ConceptsGenerated => concepts_generated_edges += 1,
            }
        }

        GraphStats {
            source_count: corpus.sources.len(),
            concept_count: corpus.concepts.len(),
            total_nodes: self.graph.node_count(),
            total_edges: self.graph.edge_count(),
            prerequisite_edges,
            extends_edges,
            related_edges,
            contrasts_with_edges,
            derived_from_edges,
            concepts_generated_edges,
            orphan_count: self.orphans().len(),
        }
    }
}

/// Resolve a slug to a human-readable title.
fn resolve_title(slug: &str, corpus: &Corpus) -> String {
    if let Some(page) = corpus.sources.get(slug) {
        return page.title.clone();
    }
    if let Some(card) = corpus.concepts.get(slug) {
        if card.disambiguator.is_empty() {
            return card.concept.clone();
        }
        return format!("{} ({})", card.concept, card.disambiguator);
    }
    title_case(slug)
}

/// Resolve a slug to its URL based on the edge type (v1.3 cross-reference resolution).
///
/// - `DerivedFrom` -> always source page URL
/// - `ConceptsGenerated` -> always concept card URL
/// - `Prerequisite`, `Extends`, `Related`, `ContrastsWith` -> concept-first:
///   if a concept card exists with that slug, resolve to `/<slug>/` (concept);
///   otherwise resolve to `/source/<slug>/` (source); otherwise just `/<slug>/`.
fn resolve_url_by_edge(slug: &str, edge_kind: EdgeKind, corpus: &Corpus) -> String {
    match edge_kind {
        EdgeKind::DerivedFrom => format!("/source/{slug}/"),
        EdgeKind::ConceptsGenerated => format!("/{slug}/"),
        EdgeKind::Prerequisite
        | EdgeKind::Extends
        | EdgeKind::Related
        | EdgeKind::ContrastsWith => {
            // Concept-first resolution
            if corpus.concepts.contains_key(slug) {
                format!("/{slug}/")
            } else if corpus.sources.contains_key(slug) {
                format!("/source/{slug}/")
            } else {
                // Forward reference — shouldn't normally appear in graph edges
                // (edges only exist when target is in the index), but handle gracefully
                format!("/{slug}/")
            }
        }
    }
}

/// Resolve a slug to its URL (used by breadcrumbs and other non-edge contexts).
#[allow(dead_code)]
fn resolve_url(slug: &str, corpus: &Corpus) -> String {
    if corpus.concepts.contains_key(slug) {
        format!("/{slug}/")
    } else if corpus.sources.contains_key(slug) {
        format!("/source/{slug}/")
    } else {
        format!("/{slug}/")
    }
}

/// Convert a kebab-case string to title case.
fn title_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[allow(clippy::too_many_lines)]
mod tests {
    use super::*;
    use crate::parser::{
        ConceptCard, Corpus, DerivedFromEntry, SourcePage, Taxonomy, TaxonomyEntry, TierEntry,
    };

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
                original_source: None,
                extraction_status: "complete".to_string(),
                concepts_generated: vec!["concept-a".to_string(), "concept-b".to_string()],
                last_extracted: String::new(),
                status: "published".to_string(),
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
                related: vec!["concept-b".to_string()],
                contrasts_with: vec![],
                trust_level: "credible".to_string(),
                canon_status: String::new(),
                answers_questions: vec![],
                status: "published".to_string(),
                stable_since: String::new(),
                last_reviewed: String::new(),
            },
        );
        concepts.insert(
            "concept-b".to_string(),
            ConceptCard {
                concept: "Concept B".to_string(),
                slug: "concept-b".to_string(),
                page_type: "concept".to_string(),
                disambiguator: "math".to_string(),
                category: "math".to_string(),
                subcategory: "algebra".to_string(),
                tier: "intermediate".to_string(),
                subject_headings: vec![],
                keywords: vec![],
                aliases: vec![],
                derived_from: vec![DerivedFromEntry {
                    slug: "src-1".to_string(),
                    title: "Source One".to_string(),
                }],
                external_references: vec![],
                extraction_confidence: String::new(),
                prerequisites: vec!["concept-a".to_string()],
                extends: vec![],
                related: vec![],
                contrasts_with: vec![],
                trust_level: "credible".to_string(),
                canon_status: String::new(),
                answers_questions: vec![],
                status: "published".to_string(),
                stable_since: String::new(),
                last_reviewed: String::new(),
            },
        );

        Corpus {
            sources,
            concepts,
            taxonomy: Taxonomy {
                categories: vec![TaxonomyEntry {
                    name: "math".to_string(),
                    description: "Mathematics".to_string(),
                }],
                tiers: vec![
                    TierEntry {
                        name: "foundational".to_string(),
                        description: "Core".to_string(),
                        order: 1,
                    },
                    TierEntry {
                        name: "intermediate".to_string(),
                        description: "Builds on foundations".to_string(),
                        order: 2,
                    },
                ],
            },
            profile: None,
        }
    }

    #[test]
    fn test_build_graph_node_count() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        // 1 source + 2 concepts = 3 nodes
        assert_eq!(graph.graph.node_count(), 3);
    }

    #[test]
    fn test_build_graph_edge_count() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        // concept-a related concept-b: 2 edges (symmetric)
        // concept-b prerequisite concept-a: 1 edge
        // concept-a derived_from src-1: 1 edge
        // concept-b derived_from src-1: 1 edge
        // src-1 concepts_generated concept-a: 1 edge
        // src-1 concepts_generated concept-b: 1 edge
        assert_eq!(graph.graph.edge_count(), 7);
    }

    #[test]
    fn test_see_also_excludes_provenance() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        let see_also = graph.see_also("concept-a", &corpus);
        // Should have "related: concept-b" only (derived_from excluded)
        assert!(see_also.iter().all(|e| e.relationship != "derived_from"));
        assert!(
            see_also
                .iter()
                .any(|e| e.slug == "concept-b" && e.relationship == "related")
        );
    }

    #[test]
    fn test_breadcrumbs_source() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        let crumbs = graph.breadcrumbs("src-1", &corpus);
        assert_eq!(crumbs.len(), 4); // Home, Math, Algebra, Source One
        assert_eq!(crumbs[0].label, "Home");
        assert_eq!(crumbs[0].url, "/");
        assert_eq!(crumbs[1].label, "Math");
        assert_eq!(crumbs[1].url, "/category/math/");
        assert_eq!(crumbs[2].label, "Algebra");
        assert_eq!(crumbs[2].url, "/category/math/algebra/");
        assert_eq!(crumbs[3].label, "Source One");
        assert_eq!(crumbs[3].url, "/source/src-1/");
    }

    #[test]
    fn test_breadcrumbs_concept() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        let crumbs = graph.breadcrumbs("concept-b", &corpus);
        assert_eq!(crumbs.len(), 4);
        assert_eq!(crumbs[3].label, "Concept B (math)");
        assert_eq!(crumbs[3].url, "/concept-b/");
    }

    #[test]
    fn test_category_index() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        let cat_idx = graph.category_index(&corpus);
        assert!(cat_idx.contains_key("math"));
        let math = &cat_idx["math"];
        assert_eq!(math.name, "math");
        assert_eq!(math.description, "Mathematics");
        // Top-level sources: src-1
        assert_eq!(math.sources.len(), 1);
        // Top-level concepts: concept-a, concept-b
        assert_eq!(math.concepts.len(), 2);
        // Subcategory "algebra" should contain all items
        assert!(math.subcategories.contains_key("algebra"));
        let algebra = &math.subcategories["algebra"];
        assert_eq!(algebra.sources.len(), 1);
        assert_eq!(algebra.concepts.len(), 2);
    }

    #[test]
    fn test_stats() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        let stats = graph.stats(&corpus);
        assert_eq!(stats.source_count, 1);
        assert_eq!(stats.concept_count, 2);
        assert_eq!(stats.total_nodes, 3);
        assert_eq!(stats.total_edges, 7);
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("tibetan-buddhism"), "Tibetan Buddhism");
        assert_eq!(title_case("math"), "Math");
        assert_eq!(title_case(""), "");
    }

    #[test]
    fn test_orphans_source_only_provenance_edges() {
        let corpus = make_test_corpus();
        let graph = HaleikiGraph::build(&corpus);
        // src-1 only has ConceptsGenerated edges (provenance), so it IS an orphan.
        // concept-a and concept-b have Related/Prerequisite edges, so they're not.
        let orphans = graph.orphans();
        assert_eq!(orphans, vec!["src-1"]);
    }
}
