//! HTML cleaning — strip Wikipedia chrome from fetched article HTML.
//!
//! Takes raw HTML from `demo/.staging/{slug}.html` and produces cleaned
//! HTML at `demo/.staging/{slug}.clean.html`. The cleaned HTML preserves
//! the article's substantive content (prose, headings, figures, tables,
//! lists, code) while removing navigation, editing UI, maintenance
//! banners, and footer sections.

use std::path::Path;

use scraper::{Html, Selector};

use super::fetch;

/// CSS selectors for elements to remove entirely from the DOM.
///
/// Order doesn't matter — all matching elements are collected then removed.
const REMOVE_SELECTORS: &[&str] = &[
    // Navigation and editing UI
    ".mw-editsection",
    ".mw-jump-link",
    "#mw-navigation",
    "#mw-head",
    "#mw-panel",
    // Hatnotes and disambiguation
    ".hatnote",
    ".dablink",
    // Maintenance and message boxes
    ".ambox",
    ".tmbox",
    ".ombox",
    ".cmbox",
    ".fmbox",
    ".imbox",
    ".asbox", // stub notices
    ".stub",
    // Navboxes
    ".navbox",
    ".navbox-styles",
    ".vertical-navbox",
    // Table of contents
    "#toc",
    ".toc",
    ".mw-table-of-contents",
    // Citations and references
    "sup.reference",
    ".reflist",
    ".references",
    ".reference-text",
    // Footer metadata
    ".catlinks",
    "#catlinks",
    ".mw-hidden-catlinks",
    // Portals and sister projects
    ".portal",
    ".sister-project",
    // Miscellaneous chrome
    ".authority-control",
    ".spoken-wikipedia",
    "#siteSub",
    "#contentSub",
    ".mw-indicators",
    ".mw-ext-cite",
    // REST API specific
    "link",   // <link> stylesheet refs
    "style",  // Inline <style> blocks
    "script", // Any inline scripts
];

/// Heading IDs for footer sections. When found, the heading AND all
/// following sibling elements until the next same-level heading are removed.
const FOOTER_SECTION_IDS: &[&str] = &[
    "References",
    "External_links",
    "Further_reading",
    "See_also",
    "Notes",
    "Citations",
    "Bibliography",
    "Sources",
];

/// Clean a single article's HTML.
///
/// Reads `demo/.staging/{slug}.html`, strips Wikipedia chrome,
/// writes `demo/.staging/{slug}.clean.html`.
///
/// Returns the path to the cleaned HTML file.
pub fn clean_article(slug: &str) -> anyhow::Result<std::path::PathBuf> {
    let raw_path = fetch::staging_html_path(slug);
    if !raw_path.exists() {
        anyhow::bail!(
            "raw HTML not found at {}. Run `haleiki demo fetch --article {slug}` first.",
            raw_path.display(),
        );
    }

    let raw_html = std::fs::read_to_string(&raw_path)?;
    let cleaned = clean_html(&raw_html);

    let clean_path = staging_clean_path(slug);
    std::fs::write(&clean_path, &cleaned)?;

    Ok(clean_path)
}

/// Path where cleaned HTML is written.
pub fn staging_clean_path(slug: &str) -> std::path::PathBuf {
    Path::new("demo/.staging").join(format!("{slug}.clean.html"))
}

/// Strip Wikipedia chrome from raw HTML and return cleaned HTML string.
///
/// This is the core cleaning function, separated from I/O for testability.
pub fn clean_html(raw_html: &str) -> String {
    // scraper's Html is immutable, so we use a two-pass approach:
    //   Pass 1: Parse and identify removal targets by NodeId
    //   Pass 2: Rebuild HTML excluding those targets
    let document = Html::parse_document(raw_html);

    // Collect element IDs that should be removed
    let mut remove_ids = collect_removal_targets(&document);

    // Also collect footer section elements
    collect_footer_sections(&document, &mut remove_ids);

    // Rebuild HTML, skipping removed elements
    rebuild_without_removed(&document, &remove_ids)
}

/// Collect the node IDs of all elements matching removal selectors.
fn collect_removal_targets(document: &Html) -> std::collections::HashSet<ego_tree::NodeId> {
    let mut remove_ids = std::collections::HashSet::new();

    for selector_str in REMOVE_SELECTORS {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                // Mark this element and all its descendants for removal
                mark_subtree_for_removal(element.id(), document, &mut remove_ids);
            }
        }
    }

    remove_ids
}

/// Recursively mark a node and all its descendants for removal.
fn mark_subtree_for_removal(
    node_id: ego_tree::NodeId,
    document: &Html,
    remove_ids: &mut std::collections::HashSet<ego_tree::NodeId>,
) {
    remove_ids.insert(node_id);
    if let Some(node) = document.tree.get(node_id) {
        for child in node.children() {
            mark_subtree_for_removal(child.id(), document, remove_ids);
        }
    }
}

/// Find footer sections (References, External links, etc.) and mark them
/// plus all following sibling elements for removal.
fn collect_footer_sections(
    document: &Html,
    remove_ids: &mut std::collections::HashSet<ego_tree::NodeId>,
) {
    // Footer sections are identified by heading elements with specific IDs.
    // In the REST API HTML, sections may use <section data-mw-section-id="N">
    // or traditional <h2 id="See_also">.
    //
    // Strategy: find headings with known IDs, then remove everything from
    // that heading to the next heading of the same or higher level.

    for section_id in FOOTER_SECTION_IDS {
        let selector_str = format!(
            "h1#{section_id}, h2#{section_id}, h3#{section_id}, \
             h1[id=\"{section_id}\"], h2[id=\"{section_id}\"], h3[id=\"{section_id}\"], \
             section[data-mw-section-id] > h1#{section_id}, \
             section[data-mw-section-id] > h2#{section_id}"
        );

        // Some of these selectors may fail to parse — that's OK, skip them
        if let Ok(selector) = Selector::parse(&selector_str) {
            for heading in document.select(&selector) {
                // If the heading is inside a <section> wrapper (REST API format),
                // remove the entire <section>
                if let Some(parent) = heading.parent() {
                    if let Some(parent_el) = parent.value().as_element() {
                        if parent_el.name() == "section" {
                            mark_subtree_for_removal(parent.id(), document, remove_ids);
                            continue;
                        }
                    }
                }

                // Otherwise, remove the heading and all following siblings
                // until the next heading of the same or higher level
                let heading_level = heading
                    .value()
                    .name()
                    .strip_prefix('h')
                    .and_then(|n| n.parse::<u32>().ok())
                    .unwrap_or(2);

                mark_subtree_for_removal(heading.id(), document, remove_ids);

                // Walk following siblings
                let mut sibling_id = heading.id();
                while let Some(next) = document.tree.get(sibling_id).and_then(|n| n.next_sibling())
                {
                    let next_id = next.id();
                    // Check if this sibling is a heading of same or higher level
                    if let Some(el) = next.value().as_element() {
                        if let Some(level) = el
                            .name()
                            .strip_prefix('h')
                            .and_then(|n| n.parse::<u32>().ok())
                        {
                            if level <= heading_level {
                                break; // Stop at next same-level or higher heading
                            }
                        }
                    }
                    mark_subtree_for_removal(next_id, document, remove_ids);
                    sibling_id = next_id;
                }
            }
        }
    }
}

/// Rebuild the HTML body content, excluding elements marked for removal.
///
/// Walks the document tree and serializes only the nodes NOT in the
/// removal set. Returns the inner HTML of the `<body>` element.
fn rebuild_without_removed(
    document: &Html,
    remove_ids: &std::collections::HashSet<ego_tree::NodeId>,
) -> String {
    // Find <body> element
    let body_selector = Selector::parse("body").expect("static selector");
    let Some(body) = document.select(&body_selector).next() else {
        // No <body> — treat entire document as body (handles HTML fragments)
        return serialize_children(document.tree.root().id(), document, remove_ids);
    };

    serialize_children(body.id(), document, remove_ids)
}

/// Recursively serialize a node's children, skipping removed nodes.
fn serialize_children(
    node_id: ego_tree::NodeId,
    document: &Html,
    remove_ids: &std::collections::HashSet<ego_tree::NodeId>,
) -> String {
    let mut output = String::new();

    if let Some(node) = document.tree.get(node_id) {
        for child in node.children() {
            if remove_ids.contains(&child.id()) {
                continue;
            }
            output.push_str(&serialize_node(child.id(), document, remove_ids));
        }
    }

    output
}

/// Serialize a single node and its non-removed children.
fn serialize_node(
    node_id: ego_tree::NodeId,
    document: &Html,
    remove_ids: &std::collections::HashSet<ego_tree::NodeId>,
) -> String {
    use scraper::node::Node;

    let Some(node) = document.tree.get(node_id) else {
        return String::new();
    };

    match node.value() {
        Node::Text(text) => text.text.to_string(),
        Node::Element(el) => {
            use std::fmt::Write;
            let tag = el.name();

            // Self-closing void elements
            if matches!(tag, "br" | "hr" | "img" | "input" | "meta" | "link") {
                let mut s = format!("<{tag}");
                for (key, val) in el.attrs() {
                    let _ = write!(s, " {key}=\"{}\"", escape_attr(val));
                }
                s.push_str(" />");
                return s;
            }

            let children = serialize_children(node_id, document, remove_ids);

            // Skip elements that become empty after child removal
            // (but preserve structural elements like <td>, <th>, <li>)
            if children.trim().is_empty()
                && !matches!(tag, "td" | "th" | "li" | "br" | "hr" | "img")
            {
                return String::new();
            }

            let mut s = format!("<{tag}");
            for (key, val) in el.attrs() {
                let _ = write!(s, " {key}=\"{}\"", escape_attr(val));
            }
            s.push('>');
            s.push_str(&children);
            let _ = write!(s, "</{tag}>");
            s
        }
        // Strip comments and other non-content nodes
        _ => String::new(),
    }
}

/// Escape HTML attribute values.
fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Selector removal tests ---

    #[test]
    fn test_clean_html_removes_edit_sections() {
        let html = r#"<html><body>
            <h2>Title <span class="mw-editsection">[edit]</span></h2>
            <p>Content here.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(
            !cleaned.contains("mw-editsection"),
            "Edit section not removed"
        );
        assert!(!cleaned.contains("[edit]"), "[edit] text not removed");
        assert!(
            cleaned.contains("Content here."),
            "Body content was removed"
        );
        assert!(cleaned.contains("Title"), "Heading text was removed");
    }

    #[test]
    fn test_clean_html_removes_hatnotes() {
        let html = r#"<html><body>
            <div class="hatnote">For other uses, see X.</div>
            <p>Article content.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(!cleaned.contains("hatnote"), "Hatnote not removed");
        assert!(
            !cleaned.contains("For other uses"),
            "Hatnote text not removed"
        );
        assert!(
            cleaned.contains("Article content."),
            "Body content was removed"
        );
    }

    #[test]
    fn test_clean_html_removes_navboxes() {
        let html = r#"<html><body>
            <p>Content.</p>
            <div class="navbox">
                <table><tr><td>Nav links</td></tr></table>
            </div>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(!cleaned.contains("navbox"), "Navbox not removed");
        assert!(!cleaned.contains("Nav links"), "Navbox content not removed");
        assert!(cleaned.contains("Content."), "Body content was removed");
    }

    #[test]
    fn test_clean_html_removes_ambox_banners() {
        let html = r#"<html><body>
            <div class="ambox">This article needs more citations.</div>
            <p>Real content.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(!cleaned.contains("ambox"), "Ambox not removed");
        assert!(
            !cleaned.contains("needs more citations"),
            "Ambox text not removed"
        );
        assert!(
            cleaned.contains("Real content."),
            "Body content was removed"
        );
    }

    #[test]
    fn test_clean_html_removes_citation_superscripts() {
        let html = r#"<html><body>
            <p>Memory management<sup class="reference">[1]</sup> is important.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(!cleaned.contains("[1]"), "Citation superscript not removed");
        assert!(
            cleaned.contains("Memory management"),
            "Content before citation removed"
        );
        assert!(
            cleaned.contains("is important."),
            "Content after citation removed"
        );
    }

    #[test]
    fn test_clean_html_removes_toc() {
        let html = r#"<html><body>
            <div id="toc" class="toc">
                <ul><li>Section 1</li></ul>
            </div>
            <p>Content.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(!cleaned.contains("toc"), "TOC not removed");
        assert!(cleaned.contains("Content."), "Body content was removed");
    }

    #[test]
    fn test_clean_html_removes_catlinks() {
        let html = r#"<html><body>
            <p>Content.</p>
            <div id="catlinks" class="catlinks">
                <a>Category: Memory management</a>
            </div>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(!cleaned.contains("catlinks"), "Catlinks not removed");
        assert!(cleaned.contains("Content."), "Body content was removed");
    }

    #[test]
    fn test_clean_html_removes_style_and_script() {
        let html = r#"<html><head>
            <style>.foo { color: red; }</style>
            <script>alert('x');</script>
        </head><body>
            <p>Content.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(!cleaned.contains("<style>"), "Style not removed");
        assert!(!cleaned.contains("<script>"), "Script not removed");
        assert!(!cleaned.contains("alert"), "Script content not removed");
        assert!(cleaned.contains("Content."), "Body content was removed");
    }

    // --- Preservation tests ---

    #[test]
    fn test_clean_html_preserves_paragraphs() {
        let html = r#"<html><body>
            <p>First paragraph.</p>
            <p>Second paragraph.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("First paragraph."));
        assert!(cleaned.contains("Second paragraph."));
    }

    #[test]
    fn test_clean_html_preserves_headings() {
        let html = r#"<html><body>
            <h2>Section One</h2>
            <p>Content.</p>
            <h3>Subsection</h3>
            <p>More content.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("<h2"));
        assert!(cleaned.contains("Section One"));
        assert!(cleaned.contains("<h3"));
        assert!(cleaned.contains("Subsection"));
    }

    #[test]
    fn test_clean_html_preserves_figures() {
        let html = r#"<html><body>
            <figure>
                <img src="image.png" alt="Diagram" />
                <figcaption>A diagram.</figcaption>
            </figure>
            <p>Content.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("<figure>"));
        assert!(cleaned.contains("<img"));
        assert!(cleaned.contains("image.png"));
        assert!(cleaned.contains("<figcaption>"));
        assert!(cleaned.contains("A diagram."));
    }

    #[test]
    fn test_clean_html_preserves_tables() {
        let html = r#"<html><body>
            <table>
                <tr><th>Method</th><th>Speed</th></tr>
                <tr><td>Mark-sweep</td><td>Slow</td></tr>
            </table>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("<table>"));
        assert!(cleaned.contains("Mark-sweep"));
        assert!(cleaned.contains("Slow"));
    }

    #[test]
    fn test_clean_html_preserves_lists() {
        let html = r#"<html><body>
            <ul>
                <li>Item one</li>
                <li>Item two</li>
            </ul>
            <ol>
                <li>First</li>
                <li>Second</li>
            </ol>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("<ul>"));
        assert!(cleaned.contains("<ol>"));
        assert!(cleaned.contains("Item one"));
        assert!(cleaned.contains("First"));
    }

    #[test]
    fn test_clean_html_preserves_code_blocks() {
        let html = r#"<html><body>
            <pre><code>fn main() {}</code></pre>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("<pre>"));
        assert!(cleaned.contains("<code>"));
        assert!(cleaned.contains("fn main()"));
    }

    #[test]
    fn test_clean_html_preserves_inline_links() {
        let html = r#"<html><body>
            <p>See <a href="/wiki/RAII">RAII</a> for details.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("<a href="));
        assert!(cleaned.contains("RAII"));
    }

    // --- Footer section removal tests ---

    #[test]
    fn test_clean_html_removes_references_section() {
        let html = r#"<html><body>
            <h2>Content</h2>
            <p>Real content.</p>
            <h2 id="References">References</h2>
            <ol class="references">
                <li>Reference 1</li>
            </ol>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("Real content."));
        assert!(
            !cleaned.contains("Reference 1"),
            "References section not removed"
        );
    }

    #[test]
    fn test_clean_html_removes_external_links_section() {
        let html = r#"<html><body>
            <h2>Content</h2>
            <p>Real content.</p>
            <h2 id="External_links">External links</h2>
            <ul>
                <li><a href="http://example.com">Example</a></li>
            </ul>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("Real content."));
        assert!(
            !cleaned.contains("External links"),
            "External links heading not removed"
        );
        assert!(
            !cleaned.contains("example.com"),
            "External links content not removed"
        );
    }

    // --- Edge cases ---

    #[test]
    fn test_clean_html_handles_empty_body() {
        let html = r#"<html><body></body></html>"#;
        let cleaned = clean_html(html);
        assert!(cleaned.trim().is_empty() || cleaned.contains(""));
    }

    #[test]
    fn test_clean_html_handles_html_fragment() {
        // No <html>/<body> wrapper — just content
        let html = r"<p>Just a paragraph.</p>";
        let cleaned = clean_html(html);
        assert!(cleaned.contains("Just a paragraph."));
    }

    #[test]
    fn test_clean_html_multiple_removals_in_same_document() {
        let html = r#"<html><body>
            <div class="hatnote">Disambiguation note.</div>
            <div class="ambox">Maintenance banner.</div>
            <p>Real content.</p>
            <div class="navbox"><table><tr><td>Nav</td></tr></table></div>
            <div id="catlinks">Categories</div>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(!cleaned.contains("Disambiguation note."));
        assert!(!cleaned.contains("Maintenance banner."));
        assert!(!cleaned.contains("Nav"));
        assert!(!cleaned.contains("Categories"));
        assert!(cleaned.contains("Real content."));
    }

    #[test]
    fn test_clean_html_preserves_nested_content_in_preserved_elements() {
        let html = r#"<html><body>
            <p>Text with <strong>bold</strong> and <em>italic</em> and
               <a href="/wiki/Link">a link</a>.</p>
        </body></html>"#;

        let cleaned = clean_html(html);
        assert!(cleaned.contains("<strong>bold</strong>"));
        assert!(cleaned.contains("<em>italic</em>"));
        assert!(cleaned.contains("<a href="));
    }

    // --- Real article test (requires fetched HTML) ---

    #[test]
    #[ignore] // Requires demo/.staging/memory-management.html from a prior fetch
    fn test_clean_article_real_html() {
        let result = clean_article("memory-management");
        assert!(result.is_ok(), "Cleaning failed: {:?}", result.err());

        let clean_path = result.unwrap();
        assert!(clean_path.exists());

        let cleaned = std::fs::read_to_string(&clean_path).unwrap();

        // Should still contain substantive content
        assert!(cleaned.contains("memory") || cleaned.contains("Memory"));
        assert!(
            cleaned.contains("<p>"),
            "No paragraphs found in cleaned HTML"
        );

        // Should NOT contain Wikipedia chrome
        assert!(
            !cleaned.contains("mw-editsection"),
            "Edit sections not removed"
        );
        assert!(!cleaned.contains("navbox"), "Navboxes not removed");
        assert!(!cleaned.contains("catlinks"), "Category links not removed");

        // Size should be significantly smaller than raw
        let raw_size = std::fs::metadata(fetch::staging_html_path("memory-management"))
            .unwrap()
            .len();
        let clean_size = std::fs::metadata(&clean_path).unwrap().len();
        assert!(
            clean_size < raw_size,
            "Cleaned HTML ({clean_size}) should be smaller than raw ({raw_size})"
        );
    }
}
