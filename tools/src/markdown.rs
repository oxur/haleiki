//! Markdown → HTML rendering primitive.
//!
//! A pure function over `&str`: turns a concept-card or source-page body into
//! the `page.content` HTML the templates print `| safe`, plus a matching
//! `page.data.toc[]`. Not wired to the parser or `Corpus` — see slice 03.
//!
//! ## Design decisions (Arc 01 Slice 02)
//!
//! - **Crate: `comrak`**, `default-features = false` (drops the bundled CLI,
//!   `syntect` server-side syntax highlighting, and the `bon` builder — none
//!   of which this primitive uses; core parsing/rendering and the `table` /
//!   `math_dollars` extensions are not feature-gated).
//! - **Math (design option A):** `extension.math_dollars` tokenizes `$…$` /
//!   `$$…$$` as a distinct AST node *before* `CommonMark`'s regular inline
//!   parsing runs, so math interiors are never reinterpreted as emphasis
//!   (`_`/`*`) and backslashes survive. It also respects code context: `` `$1+2$` ``
//!   stays literal text inside `<code>`, never treated as math. Rendered as
//!   `<span data-math-style="inline|display">…</span>` with the TeX
//!   HTML-escaped — `assets/js/math.js` now reads `el.textContent` (which the
//!   DOM un-escapes) and calls `katex.render` directly, rather than scanning
//!   the page for `$` delimiters.
//! - **Heading ids + TOC: a custom [`HeadingAdapter`]**, not comrak's built-in
//!   `Anchorizer`/`header_id_prefix`. The Anchorizer dedupes as `foo`, `foo-1`,
//!   `foo-2` and drops (rather than collapses) punctuation runs, neither of
//!   which matches this contract's `foo`, `foo-2`, `foo-3` dedupe convention
//!   or its "collapse runs of non-alphanumerics to one dash" slugify rule. The
//!   adapter also collects the `## `-level (`h2`) TOC in the same pass.
//! - **Raw HTML passthrough: allowed** (`render.r#unsafe = true`). Content is
//!   first-party (the demo corpus), not user-submitted, and the templates
//!   already treat `page.content` as `| safe`.
//! - **No server-side syntax highlighting.** `codefence_syntax_highlighter`
//!   stays unset; fenced code renders as plain
//!   `<pre><code class="language-…">…</code></pre>`, matching `haleiki.css`'s
//!   `.prose pre code` rules (no `.token`/`.hljs` classes exist to target).

use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use comrak::adapters::{HeadingAdapter, HeadingMeta};
use comrak::nodes::Sourcepos;
use comrak::options::Plugins;
use comrak::{Options, markdown_to_html_with_plugins};
use serde::Serialize;

/// The result of rendering a Markdown body: the HTML (`page.content`) and its
/// matching table of contents (`page.data.toc`).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RenderedContent {
    pub html: String,
    pub toc: Vec<TocEntry>,
}

/// One `## H2` heading, in document order.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TocEntry {
    pub id: String,
    pub label: String,
}

/// Renders a Markdown body to HTML and extracts its `h2` table of contents.
///
/// Every heading (any level) gets a deterministic, deduped `id="…"` anchor;
/// only level-2 headings are collected into the returned `toc`, in document
/// order, with `toc[i].id` equal to that heading's emitted anchor.
#[must_use]
pub fn render_markdown(body: &str) -> RenderedContent {
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.math_dollars = true;
    options.render.r#unsafe = true;

    let collector = TocCollector::default();
    let mut plugins = Plugins::default();
    plugins.render.heading_adapter = Some(&collector);

    let html = markdown_to_html_with_plugins(body, &options, &plugins);
    let toc = collector
        .toc
        .into_inner()
        .expect("TocCollector toc poisoned");

    RenderedContent { html, toc }
}

/// Slugifies heading text: lowercase, runs of non-alphanumerics collapse to a
/// single `-`, leading/trailing dashes trimmed. Unicode-aware (`char::is_alphanumeric`),
/// not ASCII-only, since source-page headings are not guaranteed to be English.
///
/// `"Quick Definition"` → `"quick-definition"`; `"Context & Application"` →
/// `"context-application"` (the `&` and surrounding spaces collapse to one `-`).
fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    let mut pending_dash = false;
    for ch in text.chars() {
        if ch.is_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            pending_dash = false;
            slug.extend(ch.to_lowercase());
        } else {
            pending_dash = true;
        }
    }
    slug
}

/// A [`HeadingAdapter`] that assigns deterministic, deduped anchor ids to
/// every heading and collects the `h2` ids/labels into a TOC as it renders.
///
/// Dedupe convention: first occurrence of a slug is unsuffixed; the second
/// gets `-2`, the third `-3`, etc. (not comrak's own `Anchorizer`, which
/// starts its suffix at `-1`).
#[derive(Debug, Default)]
struct TocCollector {
    counts: Mutex<HashMap<String, u32>>,
    toc: Mutex<Vec<TocEntry>>,
}

impl TocCollector {
    /// Returns `slug` if this is its first occurrence, else `slug-N` for the
    /// `N`th occurrence (`N` starting at 2), recording the occurrence.
    fn dedupe(&self, slug: &str) -> String {
        let mut counts = self.counts.lock().expect("TocCollector counts poisoned");
        let count = counts.entry(slug.to_string()).or_insert(0);
        *count += 1;
        if *count == 1 {
            slug.to_string()
        } else {
            format!("{slug}-{count}")
        }
    }
}

impl HeadingAdapter for TocCollector {
    fn enter(
        &self,
        output: &mut dyn fmt::Write,
        heading: &HeadingMeta,
        _sourcepos: Option<Sourcepos>,
    ) -> fmt::Result {
        let slug = slugify(&heading.content);
        let slug = if slug.is_empty() {
            "section".to_string()
        } else {
            slug
        };
        let id = self.dedupe(&slug);

        if heading.level == 2 {
            self.toc
                .lock()
                .expect("TocCollector toc poisoned")
                .push(TocEntry {
                    id: id.clone(),
                    label: heading.content.clone(),
                });
        }

        // `id` is always alphanumeric-and-dash-only (from `slugify`), so no
        // attribute-value escaping is needed here.
        write!(output, "<h{} id=\"{id}\">", heading.level)
    }

    fn exit(&self, output: &mut dyn fmt::Write, heading: &HeadingMeta) -> fmt::Result {
        write!(output, "</h{}>", heading.level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn demo_dir() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    /// Splits `---\nfrontmatter\n---\nbody` and returns the body.
    fn strip_frontmatter(raw: &str) -> &str {
        let rest = raw.strip_prefix("---\n").expect("starts with frontmatter");
        let end = rest.find("\n---\n").expect("frontmatter closes");
        &rest[end + 5..]
    }

    // F-1: CommonMark basics.
    #[test]
    fn test_render_markdown_commonmark_basics() {
        let body = "\
A paragraph with **bold** and *emphasis*.

- one
- two

1. first
2. second

See [the docs](/docs/) for more.
";
        let rendered = render_markdown(body);

        assert!(
            rendered
                .html
                .contains("<p>A paragraph with <strong>bold</strong> and <em>emphasis</em>.</p>")
        );
        assert!(rendered.html.contains("<ul>"));
        assert!(rendered.html.contains("<li>one</li>"));
        assert!(rendered.html.contains("<ol>"));
        assert!(rendered.html.contains("<li>first</li>"));
        assert!(rendered.html.contains(r#"<a href="/docs/">the docs</a>"#));
    }

    // F-2: deterministic slugs + collision dedupe.
    #[test]
    fn test_render_markdown_heading_ids_are_deterministic_and_deduped() {
        let body = "\
## Quick Definition

## Context & Application

## Background

Some text.

## Background

More text.

## Background

Even more text.
";
        let rendered = render_markdown(body);

        assert!(rendered.html.contains(r#"<h2 id="quick-definition">"#));
        assert!(rendered.html.contains(r#"<h2 id="context-application">"#));
        assert!(rendered.html.contains(r#"<h2 id="background">"#));
        assert!(rendered.html.contains(r#"<h2 id="background-2">"#));
        assert!(rendered.html.contains(r#"<h2 id="background-3">"#));
    }

    // F-3: TOC extraction — ids match emitted anchors, order preserved, H2 only.
    #[test]
    fn test_render_markdown_toc_matches_document_order_and_anchors() {
        let body = "\
# Not In TOC (h1)

## First Section

### A Subsection (h3, not in TOC)

## Second Section
";
        let rendered = render_markdown(body);

        assert_eq!(rendered.toc.len(), 2);
        assert_eq!(rendered.toc[0].id, "first-section");
        assert_eq!(rendered.toc[0].label, "First Section");
        assert_eq!(rendered.toc[1].id, "second-section");
        assert_eq!(rendered.toc[1].label, "Second Section");

        for entry in &rendered.toc {
            assert!(
                rendered.html.contains(&format!("id=\"{}\"", entry.id)),
                "toc entry {entry:?} has no matching anchor in html: {}",
                rendered.html
            );
        }
        // The h3 got its own (non-TOC'd) anchor too.
        assert!(
            rendered
                .html
                .contains(r#"<h3 id="a-subsection-h3-not-in-toc">"#)
        );
    }

    // F-4: math preserved for KaTeX; `$` inside code left alone.
    #[test]
    fn test_render_markdown_math_preserved_and_code_dollar_untouched() {
        let body = "\
Inline: $a_1 * b_2$ and display:

$$\\sum_{i=0}^{n} x_i$$

Shell example: `echo $HOME`
";
        let rendered = render_markdown(body);

        assert!(
            rendered
                .html
                .contains(r#"<span data-math-style="inline">a_1 * b_2</span>"#),
            "inline math should survive byte-intact, no emphasis conversion: {}",
            rendered.html
        );
        assert!(
            rendered
                .html
                .contains(r#"<span data-math-style="display">\sum_{i=0}^{n} x_i</span>"#),
            "display math should survive byte-intact: {}",
            rendered.html
        );
        assert!(
            rendered.html.contains("<code>echo $HOME</code>"),
            "$ inside inline code must not be treated as math: {}",
            rendered.html
        );
    }

    // F-5: fenced code plain (no highlighting), HTML-escaped, not markdown-processed;
    // inline code likewise.
    #[test]
    fn test_render_markdown_fenced_and_inline_code_plain_and_escaped() {
        let body = "\
```rust
let x = a < b && *ptr;
```

Inline `*not emphasis*` here.
";
        let rendered = render_markdown(body);

        assert!(
            rendered
                .html
                .contains("<pre><code class=\"language-rust\">let x = a &lt; b &amp;&amp; *ptr;\n</code></pre>"),
            "fenced code should be plain, escaped, and not markdown-processed: {}",
            rendered.html
        );
        assert!(
            rendered.html.contains("<code>*not emphasis*</code>"),
            "inline code should not be markdown-processed: {}",
            rendered.html
        );
    }

    // F-6: GFM pipe tables.
    #[test]
    fn test_render_markdown_gfm_table() {
        let body = "\
| Name | Value |
|------|-------|
| a    | 1     |
| b    | 2     |
";
        let rendered = render_markdown(body);

        assert!(rendered.html.contains("<table>"));
        assert!(rendered.html.contains("<th>Name</th>"));
        assert!(rendered.html.contains("<td>a</td>"));
        assert!(rendered.html.contains("</table>"));
    }

    // F-7: a real demo concept body renders without error; its H2 sections
    // appear in the TOC with matching anchors in the HTML.
    //
    // Note: the slice-doc/ledger describe this fixture as having "8" H2
    // sections; `demo/concepts/abelian-group.md` actually has 9 (Quick
    // Definition, Core Definition, Prerequisites, Key Properties, Context &
    // Application, Examples, Relationships, Source Reference, Verification
    // Notes) as of this writing — verified by `grep -c '^## '`. This test
    // asserts the real count rather than a hardcoded 8, and the discrepancy
    // is flagged in the closing report rather than silently matched.
    #[test]
    fn test_render_markdown_real_demo_concept_body() {
        let raw = std::fs::read_to_string(demo_dir().join("concepts/abelian-group.md"))
            .expect("demo/concepts/abelian-group.md reads");
        let body = strip_frontmatter(&raw);
        let expected_headings = body.lines().filter(|l| l.starts_with("## ")).count();

        let rendered = render_markdown(body);

        assert_eq!(rendered.toc.len(), expected_headings);
        for entry in &rendered.toc {
            assert!(
                rendered.html.contains(&format!("id=\"{}\"", entry.id)),
                "toc entry {entry:?} has no matching anchor in html"
            );
        }
        // Math-heavy: at least one inline math span survives.
        assert!(rendered.html.contains(r#"data-math-style="inline""#));
        // A cross-reference link from the body survives.
        assert!(
            rendered
                .html
                .contains(r#"href="/source/group-mathematics/""#)
        );
    }

    // F-7 (continued): a real demo source body — non-math, image + link +
    // bold + Unicode-heavy prose — renders without error.
    #[test]
    fn test_render_markdown_real_demo_source_body() {
        let raw = std::fs::read_to_string(demo_dir().join("sources/vimalamitra.md"))
            .expect("demo/sources/vimalamitra.md reads");
        let body = strip_frontmatter(&raw);

        let rendered = render_markdown(body);

        assert_eq!(rendered.toc.len(), 1);
        assert_eq!(rendered.toc[0].label, "Attributed works");
        assert!(rendered.html.contains(r#"<h2 id="attributed-works">"#));
        assert!(rendered.html.contains("<strong>Vimalamitra</strong>"));
        assert!(rendered.html.contains("<img "));
    }

    // F-8: RenderedContent.html flows through render::build_environment as
    // {{ page.content | safe }} unescaped and structurally intact.
    #[test]
    fn test_render_markdown_html_passes_through_safe_context_intact() {
        use crate::render::build_environment;
        use minijinja::context;

        let body = "## Heading\n\nText with $x^2$ math and a <sub>raw tag</sub>.\n";
        let rendered = render_markdown(body);

        let mut env = build_environment(Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates"));
        env.add_template("test-content.jinja", "<div>{{ page.content | safe }}</div>")
            .expect("test template compiles");
        let tmpl = env.get_template("test-content.jinja").expect("resolves");
        let out = tmpl
            .render(context! { page => context! { content => rendered.html.clone() } })
            .expect("renders");

        assert_eq!(out, format!("<div>{}</div>", rendered.html));
        assert!(out.contains(r#"<h2 id="heading">"#));
        assert!(out.contains(r#"data-math-style="inline""#));
        assert!(out.contains("<sub>raw tag</sub>"));
    }
}
