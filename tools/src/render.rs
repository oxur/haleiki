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
/// `provenance`, `breadcrumbs`), each keyed by slug.
#[derive(Debug, Clone, Serialize)]
pub struct SiteContext {
    pub profile: Profile,
    pub see_also: BTreeMap<String, Vec<crate::graph::SeeAlsoEntry>>,
    pub provenance: BTreeMap<String, crate::generator::ProvenanceEntry>,
    pub breadcrumbs: BTreeMap<String, Vec<crate::graph::Breadcrumb>>,
}

/// Assembles [`SiteContext`] from a parsed [`crate::parser::Corpus`], its
/// [`crate::graph::HaleikiGraph`], and the resolved [`Profile`].
///
/// Reuses `graph.see_also` / `graph.breadcrumbs` and
/// `generator::build_provenance_map` — the derivations are shared with the
/// JSON-writing build path (`generator::generate`), not re-implemented here.
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

    SiteContext {
        profile,
        see_also,
        provenance,
        breadcrumbs,
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
}
