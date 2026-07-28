//! Non-article page renderers (home now; browse/category/about/attribution/
//! search/404 as later Arc 02 slices add them). Kept separate from
//! `article.rs` (concept/source, which have real `page.data`) since these
//! pages mostly just read `site.data` — home's `page` context is minimal.

use minijinja::{Environment, context};

use crate::error::{Error, Result};
use crate::graph::HaleikiGraph;
use crate::parser::Corpus;
use crate::render::SiteContext;

/// Renders the home page end-to-end via `pages/home.jinja`.
///
/// `env` should be built with [`crate::render::build_environment`] (reuse it
/// — auto-escape is per-`Environment`, Arc 01 v1.1; do not construct a bare
/// `minijinja::Environment`).
///
/// # Errors
///
/// Returns the minijinja render error (via [`crate::error::Error::from`]) if
/// the template fails to render.
pub fn render_home(env: &Environment<'_>, site: &SiteContext) -> Result<String> {
    let tmpl = env.get_template("pages/home.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        // home.jinja overrides {% block title %} unconditionally and reads
        // no other page.* field, so this is deliberately minimal — matching
        // the smoke-test pattern Arc 01 slice 01 established for 404.jinja.
        page => context! {
            title => site.profile.name.clone(),
            slug => "",
            data => context! {},
        },
    })?;
    Ok(html)
}

/// Renders the browse index end-to-end via `pages/browse.jinja`.
///
/// # Errors
///
/// Returns the minijinja render error if the template fails to render.
pub fn render_browse(env: &Environment<'_>, site: &SiteContext) -> Result<String> {
    let tmpl = env.get_template("pages/browse.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        // browse.jinja reads page.slug only via breadcrumb.jinja's fallback
        // path (this page is never in site.data.breadcrumbs, which is keyed
        // by article slug) and page.title nowhere directly — "Browse" covers
        // both the <title> tag and the breadcrumb fallback's last crumb.
        page => context! {
            title => "Browse",
            slug => "browse",
            data => context! {},
        },
    })?;
    Ok(html)
}

/// Renders a single category page end-to-end via `pages/category.jinja`.
///
/// `category` is the raw taxonomy category name (e.g. `"mathematics"`, as
/// used in frontmatter and `graph::category_index`'s keys — not the
/// URL-facing slug, though for demo-corpus categories the two happen to be
/// identical since they're already kebab-case).
///
/// # Errors
///
/// Returns [`Error::Config`] if `category` is not indexed (nothing in the
/// corpus tagged with it), or the minijinja render error if the template
/// fails to render.
pub fn render_category(
    env: &Environment<'_>,
    site: &SiteContext,
    corpus: &Corpus,
    graph: &HaleikiGraph,
    category: &str,
) -> Result<String> {
    let index = graph.category_index(corpus);
    let entry = index.get(category).ok_or_else(|| Error::Config {
        message: format!("no category indexed for {category:?}"),
    })?;
    let data = crate::render::build_category_page_data(corpus, entry, &site.profile);

    let tmpl = env.get_template("pages/category.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        // category.jinja has no left-nav/breadcrumb lookup keyed by page.slug
        // beyond the fallback path, so page.title (the category's display
        // name) is what makes that fallback ("Home / {{ page.title }}")
        // correct — the slice-doc's explicit call-out.
        page => context! {
            title => data.name.clone(),
            slug => category,
            data => context! { category => data },
        },
    })?;
    Ok(html)
}

/// Renders the about page end-to-end via `pages/about.jinja`. Entirely
/// `site.data.profile`-driven — no page-specific data.
///
/// # Errors
///
/// Returns the minijinja render error if the template fails to render.
pub fn render_about(env: &Environment<'_>, site: &SiteContext) -> Result<String> {
    let tmpl = env.get_template("pages/about.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        page => context! {
            title => "About",
            slug => "about",
            data => context! {},
        },
    })?;
    Ok(html)
}

/// Renders the 404 page via `pages/404.jinja`.
///
/// # Errors
///
/// Returns the minijinja render error if the template fails to render.
pub fn render_404(env: &Environment<'_>, site: &SiteContext) -> Result<String> {
    let tmpl = env.get_template("pages/404.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        page => context! {
            title => "Not Found",
            slug => "404",
            data => context! {},
        },
    })?;
    Ok(html)
}

/// Renders the search page as an **empty pre-hydration shell**:
/// `query = ""`, `results = []`, `facets = []`. The page's structure (search
/// input, filter sidebar frame, empty state) renders; `search.js` hydrates
/// live search client-side. Real facets/results/the JSON index are Arc 04
/// (arc-02 slice-03 decision #3 — do not special-case an empty query here).
///
/// # Errors
///
/// Returns the minijinja render error if the template fails to render.
pub fn render_search(env: &Environment<'_>, site: &SiteContext) -> Result<String> {
    let tmpl = env.get_template("pages/search.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        page => context! {
            title => "Search",
            slug => "search",
            data => context! {},
        },
        search => context! {
            query => "",
            results => Vec::<()>::new(),
            facets => Vec::<()>::new(),
        },
    })?;
    Ok(html)
}

/// Renders the attribution page end-to-end via `pages/attribution.jinja`.
/// `site.data.attribution` (assembled in `render::assemble_site_context`)
/// carries everything the template reads.
///
/// # Errors
///
/// Returns the minijinja render error if the template fails to render.
pub fn render_attribution(env: &Environment<'_>, site: &SiteContext) -> Result<String> {
    let tmpl = env.get_template("pages/attribution.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        page => context! {
            title => "Attribution",
            slug => "attribution",
            data => context! {},
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

    fn templates_dir() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates")
    }

    fn demo_site() -> (crate::parser::Corpus, SiteContext) {
        let corpus = crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses");
        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let profile = crate::render::resolve_profile(demo_dir().join("profile.yaml"))
            .expect("demo profile resolves");
        let site = crate::render::assemble_site_context(&corpus, &graph, profile);
        (corpus, site)
    }

    // Arc 02 Slice 01, F-4: render_home renders pages/home.jinja end-to-end
    // from the demo corpus with full chrome.
    #[test]
    fn test_render_home_full_chrome() {
        let (_corpus, site) = demo_site();
        let env = crate::render::build_environment(templates_dir());

        let html = render_home(&env, &site).expect("home renders");

        // Hero: name, subtitle, stats line.
        assert!(html.contains("class=\"hero-title\""));
        assert!(html.contains("Haleiki Demo Wiki"));
        assert!(html.contains("class=\"hero-subtitle\""));
        assert!(html.contains("class=\"hero-stats\""));
        assert!(html.contains("source pages"));
        assert!(html.contains("concept cards"));
        // Category grid: at least one card with title, count, description,
        // and an article link.
        assert!(html.contains("class=\"category-grid\""));
        assert!(html.contains("class=\"category-card\""));
        assert!(html.contains("href=\"/category/mathematics/\""));
        assert!(html.contains("class=\"category-card-count\""));
        assert!(html.contains("class=\"category-card-desc\""));
        // Featured grid.
        assert!(html.contains("class=\"featured-grid\""));
        assert!(html.contains("class=\"featured-card\""));
        assert!(html.contains("Read article"));
        // Tiers row (from site.data.profile.taxonomy.tiers).
        assert!(html.contains("class=\"tiers-row\""));
        assert!(html.contains("class=\"tier-card\""));
        assert!(html.contains("Foundational"));
        // Quick links.
        assert!(html.contains("class=\"quick-links-row\""));
        assert!(html.contains("href=\"/attribution/\""));
    }

    // Arc 02 Slice 01, F-5: escaping holds at page scale on the home render —
    // an HTML control string in an auto-escaped home_categories field is
    // escaped.
    #[test]
    fn test_render_home_escaping_holds_at_page_scale() {
        let (_corpus, mut site) = demo_site();
        site.home_categories[0].description = "<b>x</b>".to_string();

        let env = crate::render::build_environment(templates_dir());
        let html = render_home(&env, &site).expect("home renders with the injected description");

        assert!(
            !html.contains("<b>x</b>"),
            "the injected description must not appear as raw HTML: {html}"
        );
        assert!(
            html.contains("&lt;b&gt;x&lt;"),
            "the injected description must be HTML-escaped: {html}"
        );
    }

    // Arc 02 Slice 01, F-6: the home render path uses no Liquid/Cobalt
    // template — only render::build_environment + pages/home.jinja.
    //
    // The needle is built by concatenating fragments, not a literal, because
    // include_str! pulls in this test's own source and a literal anywhere in
    // this function (even a comment) would make the check find itself — hit
    // exactly this in Arc 01 slice 02's equivalent test.
    #[test]
    fn test_render_home_path_references_no_liquid_templates() {
        let liquid_extension = format!(".{}{}", "liqu", "id");
        let page_src = include_str!("page.rs");

        assert!(
            !page_src.contains(&liquid_extension),
            "page.rs must not reference any Liquid template"
        );
    }

    // Arc 02 Slice 02, F-3: render_browse renders pages/browse.jinja
    // end-to-end with full chrome.
    #[test]
    fn test_render_browse_full_chrome() {
        let (_corpus, site) = demo_site();
        let env = crate::render::build_environment(templates_dir());

        let html = render_browse(&env, &site).expect("browse renders");

        // Left-nav category list with counts.
        assert!(html.contains("class=\"left-nav-title\""));
        assert!(html.contains("class=\"toc-count\""));
        assert!(html.contains(r##"href="#mathematics""##));
        // Header stats.
        assert!(html.contains("class=\"page-stats\""));
        assert!(html.contains("categories"));
        // Per-category sections with subcategory groups + article links +
        // type badges.
        assert!(html.contains(r#"id="mathematics""#));
        assert!(html.contains("class=\"subcategory-group\""));
        assert!(html.contains("class=\"subcategory-label\""));
        assert!(html.contains("class=\"article-title-link\""));
        assert!(html.contains("class=\"tier-badge\">concept</span>"));
        assert!(html.contains("class=\"source-indicator\">source</span>"));
        // The philosophy-of-mind category's unlabeled (_uncategorized) group
        // renders — a subcategory-group with no visible label, per the
        // template's `{% if group.name %}` guard.
        assert!(html.contains(r#"id="philosophy-of-mind""#));
    }

    // Arc 02 Slice 02, F-4: render_category renders pages/category.jinja
    // end-to-end for a real category with full chrome.
    #[test]
    fn test_render_category_mathematics_full_chrome() {
        let (corpus, site) = demo_site();
        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let env = crate::render::build_environment(templates_dir());

        let html = render_category(&env, &site, &corpus, &graph, "mathematics")
            .expect("mathematics category renders");

        // Header: name, description, count, group count.
        assert!(html.contains("class=\"category-title\""));
        assert!(html.contains(">Mathematics<"));
        assert!(html.contains("class=\"category-description-text\""));
        assert!(html.contains("class=\"category-stats\""));
        assert!(html.contains("33 articles"));
        assert!(html.contains("3 groups"));
        // Tier sections with tier-header--{modifier}.
        assert!(html.contains("class=\"tier-header tier-header--foundational\""));
        assert!(html.contains("class=\"tier-header tier-header--intermediate\""));
        assert!(html.contains("class=\"tier-header tier-header--advanced\""));
        assert!(html.contains(">Foundational<"));
        // Article cards + badges.
        assert!(html.contains("class=\"article-card\""));
        assert!(html.contains("class=\"article-card-title\""));
        assert!(html.contains("class=\"tier-badge\">concept</span>"));
        assert!(html.contains("class=\"source-label\">source</span>"));
        // Sidebar info card: count / source_count / concept_count / subcategories.
        assert!(html.contains("class=\"info-card\""));
        assert!(html.contains("class=\"info-value\">33</span>"));
        assert!(html.contains("class=\"info-value\">9</span>"));
        assert!(html.contains("class=\"info-value\">24</span>"));
        assert!(html.contains("class=\"info-card-section-title\">Subcategories<"));
        assert!(html.contains(">algebra<"));
    }

    // Arc 02 Slice 02, F-5: escaping holds at page scale on browse and
    // category renders.
    #[test]
    fn test_render_browse_escaping_holds_at_page_scale() {
        let (_corpus, mut site) = demo_site();
        // "mathematics" has three named subcategory groups (algebra,
        // foundations, topology) and no _uncategorized items, so groups[0]
        // is reliably a labeled group whose name the template prints.
        let math = site
            .browse_categories
            .iter_mut()
            .find(|c| c.slug == "mathematics")
            .expect("mathematics browse category present");
        math.description = "<b>y</b>".to_string();
        math.groups[0].name = "<b>x</b>".to_string();

        let env = crate::render::build_environment(templates_dir());
        let html = render_browse(&env, &site).expect("browse renders with injected values");

        assert!(
            !html.contains("<b>x</b>"),
            "raw HTML must not appear: {html}"
        );
        assert!(
            !html.contains("<b>y</b>"),
            "raw HTML must not appear: {html}"
        );
        assert!(
            html.contains("&lt;b&gt;x&lt;"),
            "injected group name must be escaped: {html}"
        );
        assert!(
            html.contains("&lt;b&gt;y&lt;"),
            "injected description must be escaped: {html}"
        );
    }

    #[test]
    fn test_render_category_escaping_holds_at_page_scale() {
        let (corpus, site) = demo_site();
        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let mut entry = graph
            .category_index(&corpus)
            .get("mathematics")
            .expect("mathematics indexed")
            .clone();
        entry.description = "<b>x</b>".to_string();
        let data = crate::render::build_category_page_data(&corpus, &entry, &site.profile);
        assert_eq!(data.description, "<b>x</b>");

        let env = crate::render::build_environment(templates_dir());
        let tmpl = env.get_template("pages/category.jinja").unwrap();
        let html = tmpl
            .render(minijinja::context! {
                site => minijinja::context! { data => site },
                page => minijinja::context! {
                    title => data.name.clone(),
                    slug => "mathematics",
                    data => minijinja::context! { category => data },
                },
            })
            .expect("category renders with injected description");

        assert!(
            !html.contains("<b>x</b>"),
            "raw HTML must not appear: {html}"
        );
        assert!(
            html.contains("&lt;b&gt;x&lt;"),
            "injected description must be escaped: {html}"
        );
    }

    // Arc 02 Slice 02, F-6: the browse/category render path uses no
    // Liquid/Cobalt template.
    #[test]
    fn test_render_browse_category_path_references_no_liquid_templates() {
        let liquid_extension = format!(".{}{}", "liqu", "id");
        let render_src = include_str!("render.rs");

        assert!(
            !render_src.contains(&liquid_extension),
            "render.rs must not reference any Liquid template"
        );
    }

    // Arc 02 Slice 03, F-1: render_about renders pages/about.jinja end-to-end:
    // header, trust-level section (iterating trust.level_order), about-body
    // paragraphs.
    #[test]
    fn test_render_about_full_chrome() {
        let (_corpus, site) = demo_site();
        let env = crate::render::build_environment(templates_dir());

        let html = render_about(&env, &site).expect("about renders");

        assert!(html.contains("class=\"page-title\""));
        assert!(html.contains("About Haleiki"));
        assert!(html.contains("class=\"page-description\""));
        // Trust levels, in trust.level_order, each with label/description;
        // "credible" carries a note in the real demo profile.
        assert!(html.contains("id=\"trust-levels\""));
        assert!(html.contains("class=\"trust-level-card\""));
        assert!(html.contains("Corroborated"));
        assert!(html.contains("Credible"));
        assert!(html.contains("Provisional"));
        assert!(html.contains("Speculative"));
        assert!(html.contains("class=\"trust-level-card-note\""));
        // About-body paragraphs (| safe — real HTML, e.g. an <a> link).
        assert!(html.contains("id=\"about-project\""));
        assert!(html.contains("class=\"about-prose\""));
        assert!(html.contains("<a href=\"/attribution/\">Attribution</a>"));
    }

    // Arc 02 Slice 03, F-2: render_404 renders pages/404.jinja: error code,
    // name_short, action links.
    #[test]
    fn test_render_404_chrome() {
        let (_corpus, site) = demo_site();
        let env = crate::render::build_environment(templates_dir());

        let html = render_404(&env, &site).expect("404 renders");

        assert!(html.contains("class=\"error-code\""));
        assert!(html.contains("404"));
        assert!(html.contains("Haleiki"));
        assert!(html.contains("class=\"error-actions\""));
        assert!(html.contains("href=\"/\""));
        assert!(html.contains("href=\"/browse/\""));
        assert!(html.contains("href=\"/search/\""));
    }

    // Arc 02 Slice 03, F-3: render_search renders the empty pre-hydration
    // shell without error.
    #[test]
    fn test_render_search_empty_shell() {
        let (_corpus, site) = demo_site();
        let env = crate::render::build_environment(templates_dir());

        let html = render_search(&env, &site).expect("search renders");

        assert!(html.contains("class=\"search-hero-input\""));
        assert!(html.contains("value=\"\""));
        assert!(html.contains("class=\"filter-sidebar\""));
        assert!(html.contains("class=\"filter-title\""));
        // Empty results -> the empty-state block, not the results list.
        assert!(html.contains("class=\"search-empty\""));
        assert!(html.contains("No results found"));
        assert!(!html.contains("class=\"search-results-list\""));
        assert!(html.contains("src=\"/assets/js/search.js\""));
    }

    // Arc 02 Slice 03, F-5: render_attribution renders pages/attribution.jinja
    // end-to-end: intro, license callout, per-project source tables, license
    // cards.
    #[test]
    fn test_render_attribution_full_chrome() {
        let (_corpus, site) = demo_site();
        let env = crate::render::build_environment(templates_dir());

        let html = render_attribution(&env, &site).expect("attribution renders");

        assert!(html.contains("class=\"page-description\""));
        assert!(html.contains(&site.attribution.intro));
        assert!(html.contains("class=\"license-callout\""));
        assert!(html.contains(&site.attribution.general_notice));
        // Per-project source tables: a real project heading + at least one
        // source row with an internal link, license, and external original.
        assert!(html.contains("class=\"source-heading\""));
        assert!(html.contains(">en.wikipedia.org<"));
        assert!(html.contains("class=\"attr-table\""));
        assert!(html.contains("href=\"/source/"));
        assert!(html.contains("(91 articles)") || html.contains("91 articles)"));
        // License cards from profile.attribution.licenses.
        assert!(html.contains("class=\"license-card\""));
        assert!(html.contains("Content License"));
        assert!(html.contains("Framework License"));
        // minijinja auto-escapes "/" in attribute values (as &#x2f;), so
        // check for the un-slashed substring rather than the literal URL.
        assert!(html.contains("creativecommons.org"));
    }

    // Arc 02 Slice 03, F-6: escaping holds at page scale — an HTML control
    // string in an auto-escaped attribution field is escaped, while
    // profile.about_body stays | safe on the about page.
    #[test]
    fn test_render_attribution_escaping_holds_at_page_scale() {
        let (_corpus, mut site) = demo_site();
        site.attribution.sources_by_project[0].sources[0].title = "<b>x</b>".to_string();

        let env = crate::render::build_environment(templates_dir());
        let html =
            render_attribution(&env, &site).expect("attribution renders with injected title");

        assert!(
            !html.contains("<b>x</b>"),
            "raw HTML must not appear: {html}"
        );
        assert!(
            html.contains("&lt;b&gt;x&lt;"),
            "injected source title must be escaped: {html}"
        );
    }

    #[test]
    fn test_render_about_safe_body_stays_unescaped() {
        let (_corpus, mut site) = demo_site();
        site.profile.about_body = vec!["<b>safe html</b>".to_string()];

        let env = crate::render::build_environment(templates_dir());
        let html = render_about(&env, &site).expect("about renders");

        assert!(
            html.contains("<b>safe html</b>"),
            "about_body must stay | safe, unescaped: {html}"
        );
    }

    // Arc 02 Slice 03, F-7: these render paths use no Liquid/Cobalt template.
    #[test]
    fn test_render_standalone_pages_path_references_no_liquid_templates() {
        let liquid_extension = format!(".{}{}", "liqu", "id");
        let page_src = include_str!("page.rs");

        assert!(
            !page_src.contains(&liquid_extension),
            "page.rs must not reference any Liquid template"
        );
    }
}
