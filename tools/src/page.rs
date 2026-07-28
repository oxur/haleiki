//! Non-article page renderers (home now; browse/category/about/attribution/
//! search/404 as later Arc 02 slices add them). Kept separate from
//! `article.rs` (concept/source, which have real `page.data`) since these
//! pages mostly just read `site.data` — home's `page` context is minimal.

use minijinja::{Environment, context};

use crate::error::Result;
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
}
