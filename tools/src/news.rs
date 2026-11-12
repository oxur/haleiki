//! News-post context assembly and rendering through the shipped post template.

use minijinja::{Environment, context};
use serde::Serialize;

use crate::error::{Error, Result};
use crate::markdown::render_markdown;
use crate::parser::{Corpus, NewsPost};
use crate::render::SiteContext;

/// A link to a neighboring or related post.
#[derive(Debug, Clone, Serialize)]
struct PostLink {
    title: String,
    url: String,
    date_short: String,
}

/// The exact post context consumed by pages/news-post.jinja.
#[derive(Debug, Clone, Serialize)]
struct PostContext {
    title: String,
    body: String,
    date_display: String,
    category: String,
    category_slug: String,
    tags: Vec<crate::parser::NewsTag>,
    author: Option<String>,
    prev: Option<PostLink>,
    next: Option<PostLink>,
    related: Vec<PostLink>,
}

impl PostContext {
    fn from_post(post: &NewsPost) -> Self {
        PostContext {
            title: post.title().to_owned(),
            body: render_markdown(post.body()).html,
            date_display: post.date().display(),
            category: post.category().to_owned(),
            category_slug: post.category_slug().to_owned(),
            tags: post.tags().to_vec(),
            author: post.author().map(str::to_owned),
            prev: None,
            next: None,
            related: Vec::new(),
        }
    }
}

/// Render a single news post with the existing site environment and context.
///
/// Only the Markdown body is rendered to HTML; the template marks that body
/// safe. All metadata remains an ordinary autoescaped string.
///
/// # Errors
///
/// Returns `Error::Config` when slug is absent, or propagates template and
/// Markdown rendering errors.
pub fn render_post(
    env: &Environment<'_>,
    site: &SiteContext,
    corpus: &Corpus,
    slug: &str,
) -> Result<String> {
    let post = corpus.news.get(slug).ok_or_else(|| Error::Config {
        message: format!("no news post for slug {slug:?}"),
    })?;

    let post_context = PostContext::from_post(post);
    let tmpl = env.get_template("pages/news-post.jinja")?;
    let html = tmpl.render(context! {
        site => context! { data => site },
        page => context! {
            title => post.title(),
            slug => post.slug(),
            data => context! {},
        },
        post => post_context,
    })?;
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::HaleikiGraph;
    use crate::parser::parse_corpus;
    use crate::render::{assemble_site_context, build_environment, resolve_profile};
    use std::path::{Path, PathBuf};

    fn demo_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    fn templates_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates")
    }

    fn site_context(corpus: &Corpus, profile_path: &Path) -> SiteContext {
        let graph = HaleikiGraph::build(corpus);
        let profile = resolve_profile(profile_path).expect("demo profile resolves");
        assemble_site_context(corpus, &graph, profile)
    }

    fn isolated_corpus(frontmatter: &str, body: &str) -> (assert_fs::TempDir, Corpus) {
        let dir = assert_fs::TempDir::new().expect("temporary directory");
        let demo = demo_dir();
        std::fs::copy(demo.join("taxonomy.yaml"), dir.path().join("taxonomy.yaml"))
            .expect("copy taxonomy");
        std::fs::copy(demo.join("profile.yaml"), dir.path().join("profile.yaml"))
            .expect("copy profile");
        let news_dir = dir.path().join("news");
        std::fs::create_dir_all(&news_dir).expect("news directory");
        std::fs::write(
            news_dir.join("render-fixture.md"),
            format!("---\n{frontmatter}\n---\n\n{body}\n"),
        )
        .expect("news post");
        let corpus = parse_corpus(dir.path()).expect("fixture corpus parses");
        (dir, corpus)
    }

    #[test]
    fn test_render_post_uses_real_template_and_expected_fields() {
        let corpus = parse_corpus(&demo_dir()).expect("demo parses");
        let site = site_context(&corpus, &demo_dir().join("profile.yaml"));
        let env = build_environment(templates_dir());

        let html = render_post(&env, &site, &corpus, "search-lands-in-the-demo-wiki")
            .expect("approved post renders");

        assert!(html.contains("Search Lands in the Demo Wiki"));
        assert!(html.contains("August 21, 2026"));
        assert!(html.contains("Project Updates"));
        assert!(html.contains("/news/category/project-updates/"));
        assert!(html.contains("/news/tag/search/"));
        assert!(html.contains("/news/tag/pagefind/"));
        assert!(html.contains("/news/tag/tiny-http/"));
        assert!(html.contains("Haleiki Maintainers"));
        assert!(html.contains("<p>Search now supports highlighted results and category, tier,"));
        assert!(html.contains("content-type, and trust filters.</p>"));
        assert!(!html.contains("trust-bar"));
    }

    #[test]
    fn test_render_post_escapes_metadata_and_keeps_only_markdown_body_html_safe() {
        let frontmatter = "type: news\ntitle: '<b>Title</b>'\ndate: '2026-08-21'\ncategory: '<u>Updates</u>'\ntags: ['<em>tag</em>']\nauthor: '<i>Author</i>'";
        let (dir, corpus) = isolated_corpus(frontmatter, "**Body**");
        let site = site_context(&corpus, &dir.path().join("profile.yaml"));
        let env = build_environment(templates_dir());

        let html = render_post(&env, &site, &corpus, "render-fixture").expect("fixture renders");
        assert!(html.contains("&lt;b&gt;Title&lt;&#x2f;b&gt;"));
        assert!(html.contains("&lt;u&gt;Updates&lt;&#x2f;u&gt;"));
        assert!(html.contains("&lt;em&gt;tag&lt;&#x2f;em&gt;"));
        assert!(html.contains("&lt;i&gt;Author&lt;&#x2f;i&gt;"));
        assert!(html.contains("<strong>Body</strong>"));
        assert!(!html.contains("<b>Title</b>"));
        assert!(!html.contains("<i>Author</i>"));
    }

    #[test]
    fn test_post_context_serializes_empty_navigation_and_related_collection_explicitly() {
        let (dir, corpus) = isolated_corpus(
            "type: news\ntitle: No Byline\ndate: '2026-08-21'\ncategory: Updates\ntags: []",
            "Body",
        );
        let post = corpus.news.get("render-fixture").expect("post parsed");
        let value =
            serde_json::to_value(PostContext::from_post(post)).expect("post context serializes");

        assert_eq!(value.get("prev"), Some(&serde_json::Value::Null));
        assert_eq!(value.get("next"), Some(&serde_json::Value::Null));
        assert_eq!(value["related"], serde_json::json!([]));

        let site = site_context(&corpus, &dir.path().join("profile.yaml"));
        let env = build_environment(templates_dir());
        let html = render_post(&env, &site, &corpus, "render-fixture").expect("post renders");
        assert!(!html.contains("post-author"));
        assert!(!html.contains("post-nav"));
        assert!(!html.contains("related-posts"));
    }

    #[test]
    fn test_render_post_missing_slug_returns_config_error() {
        let corpus = parse_corpus(&demo_dir()).expect("demo parses");
        let site = site_context(&corpus, &demo_dir().join("profile.yaml"));
        let env = build_environment(templates_dir());

        let error =
            render_post(&env, &site, &corpus, "missing").expect_err("missing slug rejected");
        assert!(matches!(error, Error::Config { .. }));
    }
}
