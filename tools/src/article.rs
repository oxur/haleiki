//! Concept and source `page.data` assembly, and the article render pipeline.
//!
//! Ties slice 01 (`render::build_environment`, `render::SiteContext`) and
//! slice 02 (`markdown::render_markdown`) into rendering a real demo concept
//! or source page end-to-end through `pages/concept.jinja` /
//! `pages/source.jinja` — the capability Arc 01's composition rows (A-4…A-7)
//! reproduce.

use minijinja::{Environment, context};
use serde::Serialize;

use crate::error::{Error, Result};
use crate::markdown::{self, TocEntry, render_markdown};
use crate::parser::{ConceptCard, Corpus, SourcePage};
use crate::render::SiteContext;

/// `page.data` for a concept page (`pages/concept.jinja` /
/// `partials/sidebar-taxonomy.jinja`). `page.data.type` (the sidebar's
/// optional "Type" row) is deliberately omitted — arc-01 slice-03's
/// "decisions to confirm" default: no meaningful value has been identified
/// for it yet, and the sidebar template only renders the row when present.
#[derive(Debug, Clone, Serialize)]
pub struct ConceptPageData {
    pub concept: String,
    pub disambiguator: String,
    pub category: String,
    pub category_slug: String,
    pub subcategory: String,
    pub tier: String,
    pub page_type: &'static str,
    pub trust_level: String,
    pub subject_headings: Vec<String>,
    pub aliases: Vec<String>,
    pub facts: Vec<Fact>,
    pub toc: Vec<TocEntry>,
}

/// `page.data` for a source page — the subset `pages/source.jinja` actually
/// reads (`category`, `category_slug`, `subcategory`, `tier`, `page_type`). No
/// `trust_level` field: `base.jinja`'s `page.data.trust_level is defined`
/// check must see it as undefined so sources fall back to
/// `site.data.profile.trust.default` — no demo source carries a trust level
/// (arc-01 slice-03 sizing note #2), and a present-but-empty field would
/// satisfy `is defined` and wrongly select an empty trust level instead of
/// falling back.
#[derive(Debug, Clone, Serialize)]
pub struct SourcePageData {
    pub category: String,
    pub category_slug: String,
    pub subcategory: String,
    pub tier: String,
    pub page_type: &'static str,
}

/// One info-card fact row (`page.data.facts[]`, `partials/sidebar-taxonomy.jinja`).
#[derive(Debug, Clone, Serialize)]
pub struct Fact {
    pub label: String,
    pub value: String,
}

/// Slugifies a category name for `page.data.category_slug` and
/// `/category/{slug}/` links. Demo categories are already kebab-case (so
/// this is idempotent for them today), reusing `markdown::slugify` rather
/// than a second slugify convention (arc-01 slice-03 decision #3).
#[must_use]
pub fn category_slug(category: &str) -> String {
    markdown::slugify(category)
}

/// Resolves a concept's display title: the bare concept name, or
/// `"{concept} ({disambiguator})"` when a disambiguator is present. Mirrors
/// `graph::resolve_title`'s (private) concept-title logic — `page.title` and
/// cross-reference titles (breadcrumbs, see-also cards) must agree on what a
/// concept is called.
#[must_use]
pub fn concept_title(card: &ConceptCard) -> String {
    if card.disambiguator.is_empty() {
        card.concept.clone()
    } else {
        format!("{} ({})", card.concept, card.disambiguator)
    }
}

/// The non-empty subset of a concept card's frontmatter proposed as
/// info-card fact rows (arc-01 slice-03 decision #1, proposed default):
/// `stable_since` → "Stable since", `extraction_confidence` → "Confidence",
/// `last_reviewed` → "Last reviewed", in that order. Values are the raw
/// frontmatter strings, unreformatted.
#[must_use]
pub fn concept_facts(card: &ConceptCard) -> Vec<Fact> {
    let mut facts = Vec::new();
    if !card.stable_since.is_empty() {
        facts.push(Fact {
            label: "Stable since".to_string(),
            value: card.stable_since.clone(),
        });
    }
    if !card.extraction_confidence.is_empty() {
        facts.push(Fact {
            label: "Confidence".to_string(),
            value: card.extraction_confidence.clone(),
        });
    }
    if !card.last_reviewed.is_empty() {
        facts.push(Fact {
            label: "Last reviewed".to_string(),
            value: card.last_reviewed.clone(),
        });
    }
    facts
}

/// Assembles `page.data` for a concept card. `toc` is passed in (from
/// `render_markdown(&card.body).toc`) rather than recomputed here, so this
/// function stays a pure, directly testable mapping from frontmatter to the
/// contract shape.
#[must_use]
pub fn assemble_concept_page_data(card: &ConceptCard, toc: Vec<TocEntry>) -> ConceptPageData {
    ConceptPageData {
        concept: card.concept.clone(),
        disambiguator: card.disambiguator.clone(),
        category: card.category.clone(),
        category_slug: category_slug(&card.category),
        subcategory: card.subcategory.clone(),
        tier: card.tier.clone(),
        page_type: "concept",
        trust_level: card.trust_level.clone(),
        subject_headings: card.subject_headings.clone(),
        aliases: card.aliases.clone(),
        facts: concept_facts(card),
        toc,
    }
}

/// Assembles `page.data` for a source page.
#[must_use]
pub fn assemble_source_page_data(page: &SourcePage) -> SourcePageData {
    SourcePageData {
        category: page.category.clone(),
        category_slug: category_slug(&page.category),
        subcategory: page.subcategory.clone(),
        tier: page.tier.clone(),
        page_type: "source",
    }
}

/// Renders a real concept end-to-end via `pages/concept.jinja`.
///
/// `env` should be built with [`crate::render::build_environment`] (reuse
/// it — auto-escape is per-`Environment`, arc-01 v1.1; do not construct a
/// bare `minijinja::Environment`).
///
/// # Errors
///
/// Returns [`Error::Config`] if `slug` is not a concept in `corpus`, or the
/// minijinja render error (via [`Error::from`]) if the template fails to
/// render.
pub fn render_concept(
    env: &Environment<'_>,
    site: &SiteContext,
    corpus: &Corpus,
    slug: &str,
) -> Result<String> {
    let card = corpus.concepts.get(slug).ok_or_else(|| Error::Config {
        message: format!("no concept card for slug {slug:?}"),
    })?;

    let rendered = render_markdown(&card.body);
    let data = assemble_concept_page_data(card, rendered.toc);
    let title = concept_title(card);

    let tmpl = env.get_template("pages/concept.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        page => context! {
            title => title,
            slug => slug,
            content => rendered.html,
            data => data,
        },
    })?;
    Ok(html)
}

/// Renders a real source page end-to-end via `pages/source.jinja`.
///
/// # Errors
///
/// Returns [`Error::Config`] if `slug` is not a source in `corpus`, or the
/// minijinja render error if the template fails to render.
pub fn render_source(
    env: &Environment<'_>,
    site: &SiteContext,
    corpus: &Corpus,
    slug: &str,
) -> Result<String> {
    let page = corpus.sources.get(slug).ok_or_else(|| Error::Config {
        message: format!("no source page for slug {slug:?}"),
    })?;

    let rendered = render_markdown(&page.body);
    let data = assemble_source_page_data(page);

    let tmpl = env.get_template("pages/source.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        page => context! {
            title => page.title.clone(),
            slug => slug,
            content => rendered.html,
            data => data,
        },
    })?;
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn demo_dir() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    fn demo_corpus() -> Corpus {
        crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses")
    }

    fn demo_site(corpus: &Corpus) -> SiteContext {
        let graph = crate::graph::HaleikiGraph::build(corpus);
        let profile = crate::render::resolve_profile(demo_dir().join("profile.yaml"))
            .expect("demo profile resolves");
        crate::render::assemble_site_context(corpus, &graph, profile)
    }

    // F-4: concept page.data assembled correctly for a real demo concept.
    #[test]
    fn test_assemble_concept_page_data_matches_abelian_group_frontmatter() {
        let corpus = demo_corpus();
        let card = &corpus.concepts["abelian-group"];
        let rendered = render_markdown(&card.body);
        let expected_toc_len = rendered.toc.len();

        let data = assemble_concept_page_data(card, rendered.toc);

        assert_eq!(data.concept, "Abelian Group");
        assert_eq!(data.disambiguator, "");
        assert_eq!(data.category, "mathematics");
        assert_eq!(data.category_slug, "mathematics");
        assert_eq!(data.subcategory, "algebra");
        assert_eq!(data.tier, "intermediate");
        assert_eq!(data.page_type, "concept");
        assert_eq!(data.trust_level, "credible");
        assert_eq!(data.subject_headings, vec!["abstract-algebra"]);
        assert_eq!(data.aliases, vec!["commutative group"]);
        assert_eq!(data.toc.len(), expected_toc_len);
        assert!(expected_toc_len > 0);

        // abelian-group.md has extraction_confidence + last_reviewed, no
        // stable_since -> two facts, in the declared label order.
        assert_eq!(data.facts.len(), 2);
        assert_eq!(data.facts[0].label, "Confidence");
        assert_eq!(data.facts[0].value, "high");
        assert_eq!(data.facts[1].label, "Last reviewed");
        assert_eq!(data.facts[1].value, "2026-05-03");
    }

    // F-4 (facts ordering + all three types): a synthetic card isolates the
    // facts-selection logic from any one real fixture's particular gaps.
    #[test]
    fn test_concept_facts_includes_all_three_types_in_order_when_present() {
        let mut card = demo_corpus().concepts["abelian-group"].clone();
        card.stable_since = "1.0".to_string();
        card.extraction_confidence = "high".to_string();
        card.last_reviewed = "2026-01-01".to_string();

        let facts = concept_facts(&card);

        assert_eq!(facts.len(), 3);
        assert_eq!(facts[0].label, "Stable since");
        assert_eq!(facts[1].label, "Confidence");
        assert_eq!(facts[2].label, "Last reviewed");
    }

    #[test]
    fn test_concept_title_appends_disambiguator_when_present() {
        let mut card = demo_corpus().concepts["abelian-group"].clone();
        assert_eq!(concept_title(&card), "Abelian Group");

        card.disambiguator = "math".to_string();
        assert_eq!(concept_title(&card), "Abelian Group (math)");
    }

    // F-5: a real demo concept renders end-to-end via pages/concept.jinja
    // with full chrome.
    #[test]
    fn test_render_concept_abelian_group_full_chrome() {
        let corpus = demo_corpus();
        let site = demo_site(&corpus);
        let env = crate::render::build_environment(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates"),
        );

        let html =
            render_concept(&env, &site, &corpus, "abelian-group").expect("abelian-group renders");

        // Trust bar (label).
        assert!(html.contains("trust-bar trust-bar--credible"));
        assert!(html.contains(">Credible<"));
        // Left-nav TOC entries (ids match the rendered anchors).
        assert!(html.contains("class=\"toc-list\""));
        assert!(html.contains(r##"href="#quick-definition""##));
        assert!(html.contains(r#"<h2 id="quick-definition">"#));
        // Breadcrumb.
        assert!(html.contains("class=\"breadcrumb\""));
        assert!(html.contains(">Mathematics<"));
        // Article title + overline.
        assert!(html.contains("Abelian Group"));
        assert!(html.contains("· Concept"));
        // Prose containing data-math-style spans (math-heavy body).
        assert!(html.contains("class=\"prose\""));
        assert!(html.contains(r#"data-math-style="inline""#));
        // See-also cards grouped by relationship.
        assert!(html.contains("id=\"see-also\""));
        assert!(html.contains(">Related<"));
        // Sidebar info card: category, tier, subject headings, aliases.
        assert!(html.contains("class=\"info-card\""));
        assert!(html.contains("commutative group"));
        assert!(html.contains("abstract-algebra"));
        // Provenance "Derived From" links.
        assert!(html.contains(">Derived From<"));
        assert!(html.contains(r#"href="/source/group-mathematics/""#));
    }

    // F-6: a real demo source renders end-to-end via pages/source.jinja.
    #[test]
    fn test_render_source_vimalamitra_full_chrome() {
        let corpus = demo_corpus();
        let site = demo_site(&corpus);
        let env = crate::render::build_environment(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates"),
        );

        let html = render_source(&env, &site, &corpus, "vimalamitra").expect("vimalamitra renders");

        assert!(html.contains("class=\"breadcrumb\""));
        // Trust bar at the profile default (no demo source carries a trust level).
        assert!(html.contains("trust-bar trust-bar--credible"));
        assert!(html.contains("· Source Page"));
        assert!(html.contains("Vimalamitra"));
        assert!(html.contains("class=\"prose\""));
        assert!(html.contains("<strong>Vimalamitra</strong>"));
        assert!(html.contains("id=\"see-also\""));
    }

    // F-7: escaping holds at full-page scale. An HTML control string injected
    // into an auto-escaped page field (an alias) is escaped in the rendered
    // page, while page.content (rendered Markdown, `| safe`) stays intact —
    // real <h2>/<span> tags unescaped.
    #[test]
    fn test_render_concept_escaping_holds_at_page_scale() {
        let mut corpus = demo_corpus();
        corpus
            .concepts
            .get_mut("abelian-group")
            .expect("abelian-group exists")
            .aliases
            .push("<b>x</b>".to_string());

        let site = demo_site(&corpus);
        let env = crate::render::build_environment(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates"),
        );

        let html = render_concept(&env, &site, &corpus, "abelian-group")
            .expect("abelian-group renders with the injected alias");

        assert!(
            !html.contains("<b>x</b>"),
            "the injected alias must not appear as raw HTML: {html}"
        );
        assert!(
            html.contains("&lt;b&gt;x&lt;"),
            "the injected alias must be HTML-escaped: {html}"
        );
        // page.content stays `| safe` and structurally intact: real heading
        // and math-span tags survive unescaped alongside the escaped alias.
        assert!(html.contains(r#"<h2 id="quick-definition">"#));
        assert!(html.contains(r#"data-math-style="inline""#));
    }

    // F-8: the render path uses no Liquid/Cobalt template — only
    // render::build_environment + pages/*.jinja. Greps this module and
    // render.rs (the only two places that resolve templates for the article
    // path) for any reference to the Liquid file extension.
    //
    // The needle is built by concatenating two halves rather than written as
    // one string literal: `include_str!` pulls in this test's own source, and
    // writing the file extension out in full (dot included) anywhere in this
    // function — even in a comment — would make the check find itself.
    #[test]
    fn test_render_path_references_no_liquid_templates() {
        let liquid_extension = format!(".{}{}", "liqu", "id");
        let article_src = include_str!("article.rs");
        let render_src = include_str!("render.rs");

        assert!(
            !article_src.contains(&liquid_extension),
            "article.rs must not reference any Liquid template"
        );
        assert!(
            !render_src.contains(&liquid_extension),
            "render.rs must not reference any Liquid template"
        );
    }
}
