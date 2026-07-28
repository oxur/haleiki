//! minijinja rendering spine.
//!
//! Owns the [`Environment`] configured with a [`path_loader`] over the
//! repo-root `templates/` directory, and the typed [`Profile`] context that
//! resolves `profile.yaml` into the exact `site.data.profile` shape defined
//! by `templates/README.md` (the binding context contract for the refactor
//! template set).

use std::collections::BTreeMap;
use std::path::Path;

use minijinja::{AutoEscape, Environment, path_loader};
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Builds a minijinja [`Environment`] that resolves templates (and the
/// partials/pages they include) relative to `templates_dir`.
///
/// minijinja's default auto-escape detection keys off a trailing `.html`/
/// `.htm`/`.xml` extension and treats `.jinja`/`.jinja2`/`.j2` as a
/// stripped, ignored suffix — so `pages/404.jinja` would otherwise resolve
/// to [`AutoEscape::None`]. The whole template set here is HTML, so escaping
/// is forced on unconditionally rather than relying on naming templates
/// `*.html.jinja`.
pub fn build_environment(templates_dir: impl AsRef<Path>) -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader(templates_dir.as_ref()));
    env.set_auto_escape_callback(|_name| AutoEscape::Html);
    env
}

/// Parses a `profile.yaml` file into the typed `site.data.profile` context.
///
/// # Errors
///
/// Returns [`crate::error::Error::Io`] if the file cannot be read and
/// [`crate::error::Error::Yaml`] if its contents don't match the
/// `site.data.profile` shape.
pub fn resolve_profile(path: impl AsRef<Path>) -> Result<Profile> {
    let raw = std::fs::read_to_string(path.as_ref())?;
    let profile: Profile = serde_yaml::from_str(&raw)?;
    Ok(profile)
}

/// The `site.data.profile` context — see `templates/README.md`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub name_short: String,
    pub tagline: String,
    pub hero_subtitle: String,
    pub about_intro: String,
    /// Pre-rendered HTML paragraphs, printed `| safe` by the templates.
    pub about_body: Vec<String>,
    /// The single source of truth for the content licence notice.
    pub content_notice: String,
    pub trust: Trust,
    pub canon: Canon,
    pub taxonomy: Taxonomy,
    pub footer: Footer,
}

/// Epistemic-trust configuration (`site.data.profile.trust`).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Trust {
    pub enabled: bool,
    pub default: String,
    pub level_order: Vec<String>,
    pub levels: BTreeMap<String, TrustLevel>,
}

/// A single trust level's display copy.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TrustLevel {
    pub label: String,
    pub description: String,
    #[serde(default)]
    pub note: Option<String>,
}

/// Canon-axis configuration (`site.data.profile.canon`).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Canon {
    pub enabled: bool,
}

/// Taxonomy configuration (`site.data.profile.taxonomy`).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Taxonomy {
    pub categories: Vec<TaxonomyCategory>,
    pub tiers: Vec<TaxonomyTier>,
}

/// A taxonomy category entry.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TaxonomyCategory {
    pub name: String,
}

/// A taxonomy tier entry.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TaxonomyTier {
    pub name: String,
    pub description: String,
}

/// The unified footer (`site.data.profile.footer`).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Footer {
    pub columns: Vec<FooterColumn>,
    pub copyright: String,
    pub meta: Vec<FooterLink>,
}

/// A footer column: a title plus a list of links.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FooterColumn {
    pub title: String,
    pub links: Vec<FooterLink>,
}

/// A single footer link.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FooterLink {
    pub label: String,
    pub url: String,
    #[serde(default)]
    pub external: bool,
}

/// The `site.data` context: everything the article templates read that
/// isn't page-specific — built once per build/render session from the
/// resolved [`Profile`] plus the graph-derived lookups (`see_also`,
/// `provenance`, `breadcrumbs`), each keyed by slug, plus the site-index data
/// the home and browse pages read (`stats`, `featured`, `home_categories`,
/// `browse_categories`).
#[derive(Debug, Clone, Serialize)]
pub struct SiteContext {
    pub profile: Profile,
    pub see_also: BTreeMap<String, Vec<crate::graph::SeeAlsoEntry>>,
    pub provenance: BTreeMap<String, crate::generator::ProvenanceEntry>,
    pub breadcrumbs: BTreeMap<String, Vec<crate::graph::Breadcrumb>>,
    pub stats: Stats,
    pub featured: Vec<crate::generator::FeaturedEntry>,
    pub home_categories: Vec<HomeCategory>,
    pub browse_categories: Vec<BrowseCategory>,
}

/// `site.data.stats` — corpus-wide counts for the home hero line.
#[derive(Debug, Clone, Serialize)]
pub struct Stats {
    pub sources: usize,
    pub concepts: usize,
    pub categories: usize,
}

/// One `site.data.home_categories[]` entry (the home page's category grid).
#[derive(Debug, Clone, Serialize)]
pub struct HomeCategory {
    pub slug: String,
    pub name: String,
    pub count: usize,
    pub description: String,
    pub articles: Vec<ArticleLink>,
}

/// A `{title, url}` article link, as used in `home_categories[].articles[]`.
#[derive(Debug, Clone, Serialize)]
pub struct ArticleLink {
    pub title: String,
    pub url: String,
}

/// A `{title, url, type}` article reference, as used in
/// `browse_categories[].groups[].articles[]` and
/// `page.data.category.groups[].articles[]`. `r#type` is `"concept"` or
/// `"source"` (drives the badge); serializes as the key `type`.
#[derive(Debug, Clone, Serialize)]
pub struct ArticleRef {
    pub title: String,
    pub url: String,
    pub r#type: &'static str,
}

/// One `site.data.browse_categories[]` entry (the browse-index page),
/// grouped **by subcategory** — see [`build_browse_categories`].
#[derive(Debug, Clone, Serialize)]
pub struct BrowseCategory {
    pub slug: String,
    pub name: String,
    pub count: usize,
    pub description: String,
    pub groups: Vec<BrowseGroup>,
}

/// One subcategory group within a [`BrowseCategory`]. `name` is `""` for the
/// `"_uncategorized"` bucket — `browse.jinja` hides the label when empty.
#[derive(Debug, Clone, Serialize)]
pub struct BrowseGroup {
    pub name: String,
    pub articles: Vec<ArticleRef>,
}

/// `page.data.category` for a single category page, grouped **by tier** —
/// see [`build_category_page_data`].
#[derive(Debug, Clone, Serialize)]
pub struct CategoryPageData {
    pub name: String,
    pub description: String,
    pub count: usize,
    pub source_count: usize,
    pub concept_count: usize,
    pub subcategories: Vec<String>,
    pub groups: Vec<TierGroup>,
}

/// One tier group within [`CategoryPageData`]. `modifier` is the raw tier
/// name (e.g. `"foundational"`), used for the `tier-header--{modifier}` CSS
/// class; `name` is the display label (e.g. `"Foundational"`).
#[derive(Debug, Clone, Serialize)]
pub struct TierGroup {
    pub name: String,
    pub modifier: String,
    pub articles: Vec<ArticleRef>,
}

/// Resolves a slug to its article reference: concept-first (a slug can only
/// be one or the other in practice, but this order matches the resolution
/// convention used elsewhere, e.g. `graph::resolve_url_by_edge`). Shared by
/// `home`/`browse`/`category` reshaping — one title/url/type convention, not
/// a second one per page type.
fn resolve_article_ref(corpus: &crate::parser::Corpus, slug: &str) -> Option<ArticleRef> {
    if let Some(card) = corpus.concepts.get(slug) {
        let title = if card.disambiguator.is_empty() {
            card.concept.clone()
        } else {
            format!("{} ({})", card.concept, card.disambiguator)
        };
        return Some(ArticleRef {
            title,
            url: format!("/{slug}/"),
            r#type: "concept",
        });
    }
    if let Some(page) = corpus.sources.get(slug) {
        return Some(ArticleRef {
            title: page.title.clone(),
            url: format!("/source/{slug}/"),
            r#type: "source",
        });
    }
    None
}

/// Sorts article refs concepts-first, then alphabetically by title within
/// each group (arc-02 slice-01 decision #1 / slice-02 decision #4's shared
/// "concepts-then-sources, alphabetical" convention).
fn sort_articles_concepts_then_sources(articles: &mut [ArticleRef]) {
    fn rank(article_type: &str) -> u8 {
        u8::from(article_type != "concept")
    }
    articles.sort_by(|a, b| {
        rank(a.r#type)
            .cmp(&rank(b.r#type))
            .then_with(|| a.title.cmp(&b.title))
    });
}

/// Capitalizes the first character of a tier name for display
/// (`"foundational"` → `"Foundational"`); `category.jinja` prints
/// `group.name` with no template-side filter, unlike `home.jinja`'s
/// `tier.name | capitalize`, so the label must already be display-ready.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Assembles [`SiteContext`] from a parsed [`crate::parser::Corpus`], its
/// [`crate::graph::HaleikiGraph`], and the resolved [`Profile`].
///
/// Reuses `graph.see_also` / `graph.breadcrumbs` / `graph.category_index` and
/// `generator::build_provenance_map` / `generator::build_featured` — the
/// derivations are shared with the JSON-writing build path
/// (`generator::generate`), not re-implemented here.
#[must_use]
pub fn assemble_site_context(
    corpus: &crate::parser::Corpus,
    graph: &crate::graph::HaleikiGraph,
    profile: Profile,
) -> SiteContext {
    let all_slugs = corpus.sources.keys().chain(corpus.concepts.keys());

    let mut see_also = BTreeMap::new();
    for slug in all_slugs.clone() {
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

    let mut breadcrumbs = BTreeMap::new();
    for slug in all_slugs {
        breadcrumbs.insert(slug.clone(), graph.breadcrumbs(slug, corpus));
    }

    let provenance = crate::generator::build_provenance_map(corpus);
    let stats = build_stats(corpus);
    let featured = crate::generator::build_featured(corpus, graph);
    let home_categories = build_home_categories(corpus, graph, &profile);
    let browse_categories = build_browse_categories(corpus, graph, &profile);

    SiteContext {
        profile,
        see_also,
        provenance,
        breadcrumbs,
        stats,
        featured,
        home_categories,
        browse_categories,
    }
}

fn build_stats(corpus: &crate::parser::Corpus) -> Stats {
    Stats {
        sources: corpus.sources.len(),
        concepts: corpus.concepts.len(),
        categories: corpus.taxonomy.categories.len(),
    }
}

/// Builds `site.data.home_categories`: one entry per `profile.taxonomy
/// .categories` entry, **in that order** (the deployment profile drives the
/// home grid order), reshaped from `graph.category_index`. `articles[]` is
/// concepts first then sources, alphabetical by title within each group,
/// truncated to 3 (arc-02 slice-01 decision #1's proposed default).
///
/// A taxonomy category with no matching `category_index` entry (nothing in
/// the corpus tagged with it) is skipped rather than rendered empty.
fn build_home_categories(
    corpus: &crate::parser::Corpus,
    graph: &crate::graph::HaleikiGraph,
    profile: &Profile,
) -> Vec<HomeCategory> {
    let index = graph.category_index(corpus);

    profile
        .taxonomy
        .categories
        .iter()
        .filter_map(|cat| {
            let entry = index.get(&cat.name)?;

            let mut articles: Vec<ArticleRef> = entry
                .concepts
                .iter()
                .chain(entry.sources.iter())
                .filter_map(|slug| resolve_article_ref(corpus, slug))
                .collect();
            sort_articles_concepts_then_sources(&mut articles);
            articles.truncate(3);

            Some(HomeCategory {
                slug: crate::markdown::slugify(&cat.name),
                name: entry.name.clone(),
                count: entry.sources.len() + entry.concepts.len(),
                description: entry.description.clone(),
                articles: articles
                    .into_iter()
                    .map(|a| ArticleLink {
                        title: a.title,
                        url: a.url,
                    })
                    .collect(),
            })
        })
        .collect()
}

/// Builds `site.data.browse_categories`: one entry per `profile.taxonomy
/// .categories` entry, in that order, reshaped from `graph.category_index`.
/// `groups[]` is an unlabeled group first (for articles with no
/// subcategory — see below), then one group per named subcategory, in
/// `category_index`'s own (`BTreeMap`, alphabetical) order.
///
/// `graph::category_index` deliberately does **not** put a `"_uncategorized"`
/// key in `entry.subcategories` (see its own "don't include `_uncategorized`
/// as a named subcategory" comment) — those articles only show up folded
/// into the flat `entry.concepts`/`entry.sources`. So the unlabeled group
/// here is computed as the set difference: every article slug in
/// `entry.concepts`/`entry.sources` that doesn't appear in *any*
/// `entry.subcategories[*]`.
fn build_browse_categories(
    corpus: &crate::parser::Corpus,
    graph: &crate::graph::HaleikiGraph,
    profile: &Profile,
) -> Vec<BrowseCategory> {
    let index = graph.category_index(corpus);

    profile
        .taxonomy
        .categories
        .iter()
        .filter_map(|cat| {
            let entry = index.get(&cat.name)?;

            let categorized: std::collections::HashSet<&str> = entry
                .subcategories
                .values()
                .flat_map(|s| s.concepts.iter().chain(s.sources.iter()))
                .map(String::as_str)
                .collect();

            let mut groups = Vec::new();

            let mut uncategorized: Vec<ArticleRef> = entry
                .concepts
                .iter()
                .chain(entry.sources.iter())
                .filter(|slug| !categorized.contains(slug.as_str()))
                .filter_map(|slug| resolve_article_ref(corpus, slug))
                .collect();
            if !uncategorized.is_empty() {
                sort_articles_concepts_then_sources(&mut uncategorized);
                groups.push(BrowseGroup {
                    name: String::new(),
                    articles: uncategorized,
                });
            }

            groups.extend(
                entry
                    .subcategories
                    .iter()
                    .map(|(subcat_name, subcat_entry)| {
                        let mut articles: Vec<ArticleRef> = subcat_entry
                            .concepts
                            .iter()
                            .chain(subcat_entry.sources.iter())
                            .filter_map(|slug| resolve_article_ref(corpus, slug))
                            .collect();
                        sort_articles_concepts_then_sources(&mut articles);
                        BrowseGroup {
                            name: subcat_name.clone(),
                            articles,
                        }
                    }),
            );

            Some(BrowseCategory {
                slug: crate::markdown::slugify(&cat.name),
                name: entry.name.clone(),
                count: entry.sources.len() + entry.concepts.len(),
                description: entry.description.clone(),
                groups,
            })
        })
        .collect()
}

/// Builds `page.data.category` for a single category, grouping its full
/// article set (across every subcategory) **by tier**, in
/// `profile.taxonomy.tiers` order, non-empty tiers only. Article
/// `description` is intentionally omitted (arc-02 slice-02 decision #2) —
/// `category.jinja`'s `{% if a.description %}` guard skips it.
#[must_use]
pub fn build_category_page_data(
    corpus: &crate::parser::Corpus,
    entry: &crate::graph::CategoryIndexEntry,
    profile: &Profile,
) -> CategoryPageData {
    let mut by_tier: BTreeMap<&str, Vec<ArticleRef>> = BTreeMap::new();
    for slug in entry.concepts.iter().chain(entry.sources.iter()) {
        let Some(article) = resolve_article_ref(corpus, slug) else {
            continue;
        };
        let tier = corpus
            .concepts
            .get(slug)
            .map(|c| c.tier.as_str())
            .or_else(|| corpus.sources.get(slug).map(|p| p.tier.as_str()));
        let Some(tier) = tier else { continue };
        by_tier.entry(tier).or_default().push(article);
    }

    let groups = profile
        .taxonomy
        .tiers
        .iter()
        .filter_map(|tier| {
            let mut articles = by_tier.remove(tier.name.as_str())?;
            sort_articles_concepts_then_sources(&mut articles);
            Some(TierGroup {
                name: capitalize(&tier.name),
                modifier: tier.name.clone(),
                articles,
            })
        })
        .collect();

    let subcategories = entry
        .subcategories
        .keys()
        .filter(|name| name.as_str() != "_uncategorized")
        .cloned()
        .collect();

    CategoryPageData {
        name: entry.name.clone(),
        description: entry.description.clone(),
        count: entry.sources.len() + entry.concepts.len(),
        source_count: entry.sources.len(),
        concept_count: entry.concepts.len(),
        subcategories,
        groups,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use minijinja::context;

    /// The repo-root `templates/` directory, resolved relative to this
    /// crate's manifest (`tools/Cargo.toml`) so tests pass regardless of the
    /// working directory `cargo test` is invoked from.
    fn templates_dir() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates")
    }

    fn sample_profile() -> Profile {
        resolve_profile(templates_dir().join("profile.sample.yaml"))
            .expect("profile.sample.yaml resolves")
    }

    /// The repo-root `demo/` directory, resolved relative to this crate's
    /// manifest so tests pass regardless of `cargo test`'s working directory.
    fn demo_dir() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    // F-2: demo/profile.yaml is migrated to the site.data.profile shape —
    // resolve_profile succeeds on it, AND parse_corpus (which separately
    // deserializes the same file into the older, Cobalt-era parser::Profile
    // shape) still succeeds, unaffected by the migration.
    #[test]
    fn test_resolve_profile_demo_yaml_migrated_and_parse_corpus_unaffected() {
        let profile = resolve_profile(demo_dir().join("profile.yaml"))
            .expect("demo/profile.yaml resolves to the render Profile shape");

        assert_eq!(profile.name, "Haleiki Demo Wiki");
        assert!(profile.trust.enabled);
        assert_eq!(profile.trust.default, "credible");
        assert_eq!(profile.trust.levels.len(), 4);
        assert_eq!(
            profile.trust.level_order,
            vec!["corroborated", "credible", "provisional", "speculative"]
        );
        assert!(!profile.footer.columns.is_empty());
        // Taxonomy mirrors the real demo/taxonomy.yaml (10 categories, 3 tiers),
        // not templates/profile.sample.yaml's smaller illustrative set.
        assert_eq!(profile.taxonomy.categories.len(), 10);
        assert_eq!(profile.taxonomy.tiers.len(), 3);

        let corpus = crate::parser::parse_corpus(&demo_dir())
            .expect("parse_corpus still succeeds against the migrated demo/profile.yaml");
        assert!(corpus.profile.is_some());
    }

    // F-3: site.data assembly builds see_also/provenance/breadcrumbs in the
    // template-contract shapes, for a known real demo slug.
    #[test]
    fn test_assemble_site_context_shapes_for_known_demo_slug() {
        let corpus = crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses");
        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let profile = resolve_profile(demo_dir().join("profile.yaml")).expect("profile resolves");

        let site = assemble_site_context(&corpus, &graph, profile);

        // abelian-group: prerequisites/extends -> the source "group-mathematics"
        // (no concept card of that slug exists, so concept-first resolution
        // falls back to the source); related -> the concepts
        // "ring-mathematics" and "group-theory-mathematics" (both exist as
        // concept cards).
        let see_also = site
            .see_also
            .get("abelian-group")
            .expect("abelian-group has see_also entries");
        let prereq = see_also
            .iter()
            .find(|e| e.relationship == "prerequisite")
            .expect("a prerequisite entry exists");
        assert_eq!(prereq.slug, "group-mathematics");
        assert_eq!(prereq.url, "/source/group-mathematics/");
        assert!(
            see_also
                .iter()
                .any(|e| e.relationship == "related" && e.url == "/ring-mathematics/")
        );

        // provenance[abelian-group].links: derived_from group-mathematics + abstract-algebra
        // (both real sources), each {slug,title,url}.
        let prov = site
            .provenance
            .get("abelian-group")
            .expect("abelian-group has provenance links");
        assert!(
            prov.links
                .iter()
                .any(|l| l.slug == "group-mathematics" && l.url == "/source/group-mathematics/")
        );
        assert!(prov.links.iter().any(|l| l.slug == "abstract-algebra"));

        // provenance[vimalamitra].links: concepts_generated -> vimalamitra-teacher.
        let source_prov = site
            .provenance
            .get("vimalamitra")
            .expect("vimalamitra has provenance links");
        assert!(
            source_prov
                .links
                .iter()
                .any(|l| l.slug == "vimalamitra-teacher" && l.url == "/vimalamitra-teacher/")
        );

        // breadcrumbs[abelian-group]: Home / Mathematics / Algebra / Abelian Group.
        let crumbs = site
            .breadcrumbs
            .get("abelian-group")
            .expect("abelian-group has breadcrumbs");
        assert_eq!(crumbs.len(), 4);
        assert_eq!(crumbs[0].label, "Home");
        assert_eq!(crumbs[1].label, "Mathematics");
        assert_eq!(crumbs[2].label, "Algebra");
        assert_eq!(crumbs[3].label, "Abelian Group");
    }

    // F-2: profile.yaml parses into the site.data.profile shape.
    #[test]
    fn test_resolve_profile_sample_yaml_parses_full_shape() {
        let profile = sample_profile();

        assert_eq!(profile.name, "Haleiki Demo Wiki");
        assert_eq!(profile.name_short, "Haleiki");
        assert!(!profile.about_body.is_empty());

        assert!(profile.trust.enabled);
        assert_eq!(profile.trust.default, "credible");
        assert_eq!(
            profile.trust.level_order,
            vec!["corroborated", "credible", "provisional", "speculative"]
        );
        let credible = profile
            .trust
            .levels
            .get("credible")
            .expect("credible trust level present");
        assert_eq!(credible.label, "Credible");
        assert!(!credible.description.is_empty());
        assert!(credible.note.is_some());

        assert!(!profile.canon.enabled);

        assert!(!profile.taxonomy.categories.is_empty());
        assert!(!profile.taxonomy.tiers.is_empty());

        assert!(!profile.footer.columns.is_empty());
        let first_col = &profile.footer.columns[0];
        assert!(!first_col.links.is_empty());
        assert!(!profile.footer.meta.is_empty());
    }

    // F-1: the Environment with path_loader resolves pages/404.jinja and the
    // partials/*.jinja it includes (via base.jinja), with no load/parse error.
    #[test]
    fn test_environment_loads_and_renders_404_template() {
        let env = build_environment(templates_dir());
        let tmpl = env
            .get_template("pages/404.jinja")
            .expect("pages/404.jinja and its includes resolve");
        let profile = sample_profile();
        let html = tmpl
            .render(context! {
                site => context! { data => context! { profile => profile } },
                page => context! { title => "Not Found", data => context! {} },
            })
            .expect("pages/404.jinja renders");

        assert!(!html.is_empty());
    }

    // F-3: base.jinja renders with the resolved profile: <title> from
    // profile.name, the haleiki.css link, the no-flash theme script, and the
    // included topnav + footer.
    #[test]
    fn test_base_jinja_renders_chrome_from_profile() {
        let env = build_environment(templates_dir());
        let tmpl = env.get_template("base.jinja").expect("base.jinja resolves");
        let profile = sample_profile();
        let html = tmpl
            .render(context! {
                site => context! { data => context! { profile => profile.clone() } },
                page => context! { title => "Test Page", data => context! {} },
            })
            .expect("base.jinja renders");

        assert!(html.contains(&format!("<title>Test Page — {}</title>", profile.name)));
        assert!(html.contains(r#"<link rel="stylesheet" href="/assets/css/haleiki.css">"#));
        assert!(html.contains("localStorage.getItem('haleiki-theme')"));
        assert!(html.contains("class=\"topnav\""));
        assert!(html.contains("class=\"site-footer\""));
    }

    // F-4: profile.about_body (a `| safe` field) emits unescaped HTML, while
    // an HTML control string in an auto-escaped field (profile.about_intro)
    // is escaped. pages/about.jinja is the README-documented consumer of
    // both fields (per-page context table: "about | site.data.profile
    // .about_intro / about_body / trust").
    #[test]
    fn test_escaping_safe_vs_autoescaped_fields() {
        let env = build_environment(templates_dir());
        let tmpl = env
            .get_template("pages/about.jinja")
            .expect("pages/about.jinja resolves");

        let mut profile = sample_profile();
        profile.about_intro = "<b>x</b>".to_string();
        profile.about_body = vec!["<b>safe html</b>".to_string()];

        let html = tmpl
            .render(context! {
                site => context! { data => context! { profile => profile } },
                page => context! { title => "About", data => context! {} },
            })
            .expect("pages/about.jinja renders");

        assert!(
            !html.contains("<b>x</b>"),
            "auto-escaped profile.about_intro should not contain a raw <b> tag: {html}"
        );
        assert!(
            html.contains("&lt;b&gt;x&lt;"),
            "auto-escaped profile.about_intro should be HTML-escaped: {html}"
        );
        assert!(
            html.contains("<b>safe html</b>"),
            "profile.about_body should be printed | safe, unescaped: {html}"
        );
    }

    // F-7: the active_nav block-capture works — a page overriding
    // {% block active_nav %} marks the corresponding item in the included
    // topnav as active.
    #[test]
    fn test_active_nav_block_capture_marks_topnav_item() {
        let mut env = build_environment(templates_dir());
        env.add_template(
            "pages/test-active-nav.jinja",
            concat!(
                "{% extends \"base.jinja\" %}\n",
                "{% block active_nav %}browse{% endblock %}\n",
                "{% block main %}<main id=\"main-content\"></main>{% endblock %}\n",
            ),
        )
        .expect("test template compiles");
        let tmpl = env
            .get_template("pages/test-active-nav.jinja")
            .expect("test template resolves");
        let profile = sample_profile();
        let html = tmpl
            .render(context! {
                site => context! { data => context! { profile => profile } },
                page => context! { title => "Browse", data => context! {} },
            })
            .expect("template renders");

        assert!(
            html.contains(r#"<a href="/browse/" class="active" aria-current="page">Browse</a>"#),
            "browse nav item should be marked active: {html}"
        );
        assert!(
            !html.contains(r#"<a href="/category/" class="active""#),
            "categories nav item should not be marked active: {html}"
        );
    }

    // Arc 02 Slice 01, F-1: site.data.stats has the correct demo-corpus counts.
    #[test]
    fn test_assemble_site_context_stats_match_demo_corpus_counts() {
        let corpus = crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses");
        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let profile = resolve_profile(demo_dir().join("profile.yaml")).expect("profile resolves");
        let expected_sources = corpus.sources.len();
        let expected_concepts = corpus.concepts.len();
        let expected_categories = corpus.taxonomy.categories.len();

        let site = assemble_site_context(&corpus, &graph, profile);

        assert_eq!(site.stats.sources, expected_sources);
        assert_eq!(site.stats.concepts, expected_concepts);
        assert_eq!(site.stats.categories, expected_categories);
        // Sanity: these are the real demo corpus, not an empty/trivial one.
        assert!(site.stats.sources > 50);
        assert!(site.stats.concepts > 100);
        assert_eq!(site.stats.categories, 10);
    }

    // Arc 02 Slice 01, F-3: site.data.home_categories built from category_index
    // in profile.taxonomy.categories order, with correct count and article
    // title/url shapes for a known real category.
    #[test]
    fn test_assemble_site_context_home_categories_order_and_shape() {
        let corpus = crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses");
        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let profile = resolve_profile(demo_dir().join("profile.yaml")).expect("profile resolves");
        let expected_order: Vec<String> = profile
            .taxonomy
            .categories
            .iter()
            .map(|c| c.name.clone())
            .collect();

        let site = assemble_site_context(&corpus, &graph, profile);

        // Order matches profile.taxonomy.categories, not alphabetical or
        // category_index's own (BTreeMap, alphabetical) iteration order.
        let actual_order: Vec<String> = site
            .home_categories
            .iter()
            .map(|c| c.slug.clone())
            .collect();
        let expected_slugs: Vec<String> = expected_order
            .iter()
            .map(|n| crate::markdown::slugify(n))
            .collect();
        assert_eq!(actual_order, expected_slugs);
        assert_ne!(
            actual_order,
            {
                let mut sorted = actual_order.clone();
                sorted.sort();
                sorted
            },
            "sanity check: taxonomy order must not coincidentally equal alphabetical order"
        );

        // "mathematics": 24 real demo concepts + 9 real demo sources = 33.
        let math = site
            .home_categories
            .iter()
            .find(|c| c.slug == "mathematics")
            .expect("mathematics category present");
        assert_eq!(math.name, "Mathematics");
        assert_eq!(math.count, 33);
        assert!(!math.description.is_empty());
        assert!(math.articles.len() <= 3);
        assert!(!math.articles.is_empty());
        for a in &math.articles {
            assert!(
                a.url.starts_with('/') && a.url.ends_with('/'),
                "article url should be an absolute path: {}",
                a.url
            );
            assert!(!a.title.is_empty());
        }
        // Concepts sort before sources (decision #1's proposed default) — the
        // math category has more than 3 concepts alone, so all 3 slots should
        // be concept links (never /source/).
        assert!(math.articles.iter().all(|a| !a.url.starts_with("/source/")));
    }

    // Arc 02 Slice 02, F-1: site.data.browse_categories, grouped by
    // subcategory, taxonomy order, "_uncategorized" -> unlabeled group.
    #[test]
    fn test_assemble_site_context_browse_categories_grouped_by_subcategory() {
        let corpus = crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses");
        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let profile = resolve_profile(demo_dir().join("profile.yaml")).expect("profile resolves");
        let expected_slugs: Vec<String> = profile
            .taxonomy
            .categories
            .iter()
            .map(|c| crate::markdown::slugify(&c.name))
            .collect();

        let site = assemble_site_context(&corpus, &graph, profile);

        let actual_slugs: Vec<String> = site
            .browse_categories
            .iter()
            .map(|c| c.slug.clone())
            .collect();
        assert_eq!(actual_slugs, expected_slugs);

        // "philosophy-of-mind": 8 items with no subcategory (biology/cognition/
        // consciousness are the named ones) -> 4 groups, "_uncategorized" first
        // (BTreeMap order; "_" < "a"-"z" in ASCII) and unlabeled.
        let phil = site
            .browse_categories
            .iter()
            .find(|c| c.slug == "philosophy-of-mind")
            .expect("philosophy-of-mind category present");
        assert_eq!(phil.groups.len(), 4);
        assert_eq!(phil.groups[0].name, "");
        assert_eq!(phil.groups[0].articles.len(), 8);
        let named: Vec<&str> = phil.groups[1..].iter().map(|g| g.name.as_str()).collect();
        assert_eq!(named, vec!["biology", "cognition", "consciousness"]);

        // Article type is correct and every article in every group has a
        // well-formed url; concepts sort before sources within a group.
        for group in &phil.groups {
            let mut seen_source = false;
            for a in &group.articles {
                assert!(matches!(a.r#type, "concept" | "source"));
                if a.r#type == "concept" {
                    assert!(
                        !seen_source,
                        "concepts must sort before sources within a group"
                    );
                    assert!(a.url.starts_with('/') && !a.url.starts_with("/source/"));
                } else {
                    seen_source = true;
                    assert!(a.url.starts_with("/source/"));
                }
            }
        }
    }

    // Arc 02 Slice 02, F-2: build_category_page_data groups by tier, in
    // taxonomy tier order, non-empty tiers only, for a known real category.
    #[test]
    fn test_build_category_page_data_mathematics_grouped_by_tier() {
        let corpus = crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses");
        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let profile = resolve_profile(demo_dir().join("profile.yaml")).expect("profile resolves");
        let index = graph.category_index(&corpus);
        let entry = index.get("mathematics").expect("mathematics indexed");

        let data = build_category_page_data(&corpus, entry, &profile);

        assert_eq!(data.name, "Mathematics");
        assert!(!data.description.is_empty());
        assert_eq!(data.count, 33);
        assert_eq!(data.source_count, 9);
        assert_eq!(data.concept_count, 24);
        // Subcategories exclude "_uncategorized" (mathematics has none anyway,
        // but the exclusion rule is the thing under test).
        assert!(!data.subcategories.iter().any(|s| s == "_uncategorized"));
        assert_eq!(
            {
                let mut s = data.subcategories.clone();
                s.sort();
                s
            },
            vec!["algebra", "foundations", "topology"]
        );

        // Tier order + counts: foundational=7, intermediate=20, advanced=6 —
        // all three real, non-empty, in taxonomy tier order (not count order).
        assert_eq!(data.groups.len(), 3);
        assert_eq!(data.groups[0].modifier, "foundational");
        assert_eq!(data.groups[0].name, "Foundational");
        assert_eq!(data.groups[0].articles.len(), 7);
        assert_eq!(data.groups[1].modifier, "intermediate");
        assert_eq!(data.groups[1].articles.len(), 20);
        assert_eq!(data.groups[2].modifier, "advanced");
        assert_eq!(data.groups[2].articles.len(), 6);

        let total: usize = data.groups.iter().map(|g| g.articles.len()).sum();
        assert_eq!(total, data.count);
    }
}
