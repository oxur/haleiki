# Milestone 3.2 — Link Rewriting (`rewrite.rs`)

**Version:** 1.0
**Depends on:** Milestone 3.1 (cleaned HTML exists), Milestone 1.3 (manifest for title→slug index)
**Produces:** Cleaned HTML with correct internal/external links

---

## Overview

Implement the link rewriting pass over cleaned HTML. For every `<a href>` in the document:

1. **Internal wiki links** whose title matches a manifest entry → rewrite to `/source/{slug}/`
2. **Internal wiki links** NOT in the manifest → rewrite to absolute Wikimedia URLs
3. **Anchor-only links** (`#fragment`) → leave untouched
4. **External links** (already absolute `http://` or `https://`) → leave untouched
5. **Red links** (dead wiki links, typically `class="new"`) → remove the `<a>` wrapper, keep the text
6. **Interwiki links** (e.g., `//commons.wikimedia.org/...`) → rewrite to absolute URLs

Fragment anchors (`#section`) are preserved in all cases.

The rewritten HTML replaces the cleaned HTML: `demo/.staging/{slug}.clean.html` is updated in place (or a new `.rewritten.html` is produced — see design decision below).

---

## Design Decision: Output file

**Option A:** Overwrite `{slug}.clean.html` in place — simpler, fewer files.
**Option B:** Write to `{slug}.rewritten.html` — preserves the pre-rewrite version for debugging.

**Recommendation:** Option B during development. The pipeline stages are: `.html` → `.clean.html` → `.rewritten.html` → `.md`. This makes debugging easier. Each stage's output is inspectable.

---

## Step 1: Build the title→slug lookup index

The rewriter needs to know which Wikipedia article titles correspond to manifest entries. This is built from the manifest.

### Add to `tools/src/demo/manifest.rs`

```rust
use std::collections::HashMap;

impl Manifest {
    /// Build a lookup index from Wikimedia article titles to Haleiki slugs.
    ///
    /// Titles are normalized (lowercased, underscores→spaces) for fuzzy matching.
    /// Multiple keys may map to the same slug (title + variants).
    pub fn title_to_slug_index(&self) -> HashMap<String, String> {
        let mut index = HashMap::new();

        for article in &self.articles {
            // Exact title
            let normalized = normalize_wiki_title(&article.title);
            index.insert(normalized, article.slug.clone());

            // Also index with underscores (as they appear in URLs)
            let underscore_form = article.title.replace(' ', "_").to_lowercase();
            index.insert(underscore_form, article.slug.clone());
        }

        index
    }
}

/// Normalize a Wikipedia article title for comparison.
///
/// Lowercases, trims, replaces underscores with spaces.
pub fn normalize_wiki_title(title: &str) -> String {
    title
        .trim()
        .replace('_', " ")
        .to_lowercase()
}
```

---

## Step 2: Create `tools/src/demo/rewrite.rs`

### File: `tools/src/demo/rewrite.rs`

```rust
//! Link rewriting — convert wiki links to Haleiki internal links or
//! absolute external URLs.
//!
//! Takes cleaned HTML and rewrites `<a href>` targets based on
//! whether the link target matches a manifest article.

use std::collections::HashMap;
use std::path::Path;

use scraper::{Html, Selector};

use super::clean::staging_clean_path;
use super::manifest::{normalize_wiki_title, Manifest};

/// Path where rewritten HTML is written.
pub fn staging_rewritten_path(slug: &str) -> std::path::PathBuf {
    Path::new("demo/.staging").join(format!("{slug}.rewritten.html"))
}

/// Rewrite links in a single article's cleaned HTML.
///
/// Reads `demo/.staging/{slug}.clean.html`, rewrites links,
/// writes `demo/.staging/{slug}.rewritten.html`.
pub fn rewrite_article(slug: &str, manifest: &Manifest) -> anyhow::Result<std::path::PathBuf> {
    let clean_path = staging_clean_path(slug);
    if !clean_path.exists() {
        anyhow::bail!(
            "cleaned HTML not found at {}. Run cleaning first.",
            clean_path.display(),
        );
    }

    let html = std::fs::read_to_string(&clean_path)?;
    let title_index = manifest.title_to_slug_index();
    let default_project = &manifest.defaults.project;

    // Find the article's own project for resolving relative URLs
    let article = manifest.articles.iter().find(|a| a.slug == slug);
    let project = article
        .and_then(|a| a.project.as_deref())
        .unwrap_or(default_project);

    let rewritten = rewrite_links(&html, &title_index, project)?;

    let out_path = staging_rewritten_path(slug);
    std::fs::write(&out_path, &rewritten)?;

    Ok(out_path)
}

/// Rewrite all `<a href>` links in the HTML string.
///
/// This is the core rewriting function, separated from I/O for testability.
pub fn rewrite_links(
    html: &str,
    title_index: &HashMap<String, String>,
    source_project: &str,
) -> anyhow::Result<String> {
    // Strategy: use regex to find and replace <a ...> tags.
    // scraper can't mutate, so we do string-level replacement guided by
    // parsed link analysis.
    //
    // We find all <a ...href="..."...> occurrences and replace the href
    // value based on link classification.

    let document = Html::parse_fragment(html);
    let a_selector = Selector::parse("a[href]").unwrap();

    // Collect all link replacements: (original_href, new_href, is_red_link)
    let mut replacements: Vec<LinkReplacement> = Vec::new();

    for element in document.select(&a_selector) {
        let Some(href) = element.value().attr("href") else {
            continue;
        };

        let is_red_link = element
            .value()
            .attr("class")
            .map_or(false, |c| c.contains("new"));

        let replacement = classify_and_rewrite(href, title_index, source_project, is_red_link);
        replacements.push(replacement);
    }

    // Apply replacements to the HTML string.
    // We need to be careful about order and overlapping matches.
    // Process from the end of the string backwards to preserve positions.
    apply_replacements(html, &replacements)
}

/// Classification of a link and its rewrite action.
#[derive(Debug, Clone, PartialEq)]
struct LinkReplacement {
    /// The original href value (as it appears in the HTML).
    original_href: String,

    /// What to do with this link.
    action: LinkAction,
}

#[derive(Debug, Clone, PartialEq)]
enum LinkAction {
    /// Rewrite href to this new value.
    Rewrite(String),

    /// Remove the <a> wrapper but keep the inner text.
    Unwrap,

    /// Leave the link unchanged.
    Keep,
}

/// Classify a link and determine the rewrite action.
fn classify_and_rewrite(
    href: &str,
    title_index: &HashMap<String, String>,
    source_project: &str,
    is_red_link: bool,
) -> LinkReplacement {
    let original_href = href.to_string();

    // 1. Red links — unwrap (remove <a>, keep text)
    if is_red_link {
        return LinkReplacement {
            original_href,
            action: LinkAction::Unwrap,
        };
    }

    // 2. Anchor-only links (#fragment) — keep as-is
    if href.starts_with('#') {
        return LinkReplacement {
            original_href,
            action: LinkAction::Keep,
        };
    }

    // 3. External links (absolute http/https) — keep as-is
    if href.starts_with("http://") || href.starts_with("https://") {
        return LinkReplacement {
            original_href,
            action: LinkAction::Keep,
        };
    }

    // 4. Protocol-relative URLs (//commons.wikimedia.org/...) — make absolute
    if href.starts_with("//") {
        return LinkReplacement {
            original_href,
            action: LinkAction::Rewrite(format!("https:{href}")),
        };
    }

    // 5. Wiki links — /wiki/Title or ./Title formats
    if let Some(title_part) = extract_wiki_title(href) {
        let (title, fragment) = split_fragment(&title_part);
        let normalized = normalize_wiki_title(&title);

        if let Some(slug) = title_index.get(&normalized) {
            // Internal link — rewrite to Haleiki source page URL
            let haleiki_url = if let Some(frag) = fragment {
                format!("/source/{slug}/#{frag}")
            } else {
                format!("/source/{slug}/")
            };
            return LinkReplacement {
                original_href,
                action: LinkAction::Rewrite(haleiki_url),
            };
        }

        // External wiki link — rewrite to absolute Wikimedia URL
        let absolute_url = if let Some(frag) = fragment {
            format!("https://{source_project}/wiki/{title}#{frag}")
        } else {
            format!("https://{source_project}/wiki/{title}")
        };
        return LinkReplacement {
            original_href,
            action: LinkAction::Rewrite(absolute_url),
        };
    }

    // 6. Other relative links — make absolute against source project
    if !href.contains("://") {
        return LinkReplacement {
            original_href,
            action: LinkAction::Rewrite(format!("https://{source_project}{href}")),
        };
    }

    // Fallback: keep as-is
    LinkReplacement {
        original_href,
        action: LinkAction::Keep,
    }
}

/// Extract the article title from a wiki-style href.
///
/// Handles:
/// - `/wiki/Article_Title` → `Article_Title`
/// - `./Article_Title` → `Article_Title`
/// - `/w/index.php?title=Article_Title&action=...` → None (edit/action links)
fn extract_wiki_title(href: &str) -> Option<String> {
    // /wiki/Title pattern (most common in REST API HTML)
    if let Some(rest) = href.strip_prefix("/wiki/") {
        return Some(url_decode_basic(rest));
    }

    // ./Title pattern (relative links in some REST API versions)
    if let Some(rest) = href.strip_prefix("./") {
        return Some(url_decode_basic(rest));
    }

    // /w/index.php links are action links (edit, history) — not content links
    if href.starts_with("/w/") {
        return None;
    }

    None
}

/// Split a title into (title, optional_fragment).
fn split_fragment(title: &str) -> (String, Option<String>) {
    if let Some(pos) = title.find('#') {
        let (t, f) = title.split_at(pos);
        (t.to_string(), Some(f[1..].to_string())) // skip the '#'
    } else {
        (title.to_string(), None)
    }
}

/// Basic URL decoding — handles %XX sequences common in wiki URLs.
fn url_decode_basic(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                    continue;
                }
            }
            result.push('%');
            result.push_str(&hex);
        } else {
            result.push(c);
        }
    }

    result
}

/// Apply link replacements to the HTML string.
///
/// For Rewrite actions: change the href attribute value.
/// For Unwrap actions: replace <a ...>text</a> with just text.
/// For Keep actions: do nothing.
fn apply_replacements(html: &str, replacements: &[LinkReplacement]) -> anyhow::Result<String> {
    let mut result = html.to_string();

    // Process replacements.
    // Since multiple links may have the same href, we process each occurrence.
    // Work from end to start to preserve string positions.

    for replacement in replacements {
        match &replacement.action {
            LinkAction::Keep => {}
            LinkAction::Rewrite(new_href) => {
                // Replace the first occurrence of the original href in an <a> tag.
                // We need to be precise: match href="original" and replace with href="new".
                let old_pattern = format!("href=\"{}\"", escape_for_search(&replacement.original_href));
                let new_pattern = format!("href=\"{new_href}\"");

                // Replace first occurrence only (each replacement corresponds to one link)
                if let Some(pos) = result.find(&old_pattern) {
                    result = format!(
                        "{}{}{}",
                        &result[..pos],
                        new_pattern,
                        &result[pos + old_pattern.len()..],
                    );
                }
            }
            LinkAction::Unwrap => {
                // Find <a ...href="original"...>text</a> and replace with just text.
                // This is trickier — we need to find the full <a> tag and its closing.
                unwrap_link(&mut result, &replacement.original_href);
            }
        }
    }

    Ok(result)
}

/// Remove an <a> wrapper with the given href, keeping the inner text.
fn unwrap_link(html: &mut String, href: &str) {
    let search = format!("href=\"{}\"", escape_for_search(href));

    // Find the <a tag containing this href
    if let Some(href_pos) = html.find(&search) {
        // Walk backwards to find the opening <a
        let mut open_start = href_pos;
        while open_start > 0 && &html[open_start..open_start + 1] != "<" {
            open_start -= 1;
        }

        // Find the end of the opening tag
        if let Some(open_end) = html[href_pos..].find('>') {
            let open_end = href_pos + open_end + 1;

            // Find the closing </a>
            if let Some(close_start) = html[open_end..].find("</a>") {
                let close_start = open_end + close_start;
                let close_end = close_start + 4; // "</a>".len()

                // Extract inner text
                let inner = html[open_end..close_start].to_string();

                // Replace <a ...>inner</a> with inner
                *html = format!(
                    "{}{}{}",
                    &html[..open_start],
                    inner,
                    &html[close_end..],
                );
            }
        }
    }
}

/// Escape special regex/search characters in a string for literal matching.
fn escape_for_search(s: &str) -> String {
    // For simple string matching we just need to handle the characters
    // that might appear in URLs and could interfere with our search.
    s.to_string()
}
```

---

## Step 3: Wire into `demo/mod.rs`

### Update module declarations

```rust
pub mod clean;
pub mod fetch;
pub mod manifest;
pub mod rewrite;
pub mod status;
```

---

## Step 4: Write tests

### Unit tests in `tools/src/demo/rewrite.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// Build a simple title index for testing.
    fn test_index() -> HashMap<String, String> {
        let mut index = HashMap::new();
        index.insert("memory management".to_string(), "memory-management".to_string());
        index.insert("memory_management".to_string(), "memory-management".to_string());
        index.insert("garbage collection (computer science)".to_string(), "garbage-collection".to_string());
        index.insert("garbage_collection_(computer_science)".to_string(), "garbage-collection".to_string());
        index.insert("raii".to_string(), "raii".to_string());
        index.insert("resource acquisition is initialization".to_string(), "raii".to_string());
        index.insert("resource_acquisition_is_initialization".to_string(), "raii".to_string());
        index
    }

    // ─── extract_wiki_title tests ───────────────────────

    #[test]
    fn test_extract_wiki_title_standard_wiki_link() {
        assert_eq!(
            extract_wiki_title("/wiki/Memory_management"),
            Some("Memory_management".to_string()),
        );
    }

    #[test]
    fn test_extract_wiki_title_with_parentheses() {
        assert_eq!(
            extract_wiki_title("/wiki/Garbage_collection_(computer_science)"),
            Some("Garbage_collection_(computer_science)".to_string()),
        );
    }

    #[test]
    fn test_extract_wiki_title_relative_link() {
        assert_eq!(
            extract_wiki_title("./Memory_management"),
            Some("Memory_management".to_string()),
        );
    }

    #[test]
    fn test_extract_wiki_title_action_link_returns_none() {
        assert_eq!(
            extract_wiki_title("/w/index.php?title=Memory&action=edit"),
            None,
        );
    }

    #[test]
    fn test_extract_wiki_title_external_returns_none() {
        assert_eq!(
            extract_wiki_title("https://example.com/page"),
            None,
        );
    }

    #[test]
    fn test_extract_wiki_title_with_percent_encoding() {
        assert_eq!(
            extract_wiki_title("/wiki/C%2B%2B"),
            Some("C++".to_string()),
        );
    }

    // ─── split_fragment tests ───────────────────────────

    #[test]
    fn test_split_fragment_no_fragment() {
        let (title, frag) = split_fragment("Memory_management");
        assert_eq!(title, "Memory_management");
        assert_eq!(frag, None);
    }

    #[test]
    fn test_split_fragment_with_fragment() {
        let (title, frag) = split_fragment("Memory_management#Techniques");
        assert_eq!(title, "Memory_management");
        assert_eq!(frag, Some("Techniques".to_string()));
    }

    // ─── normalize_wiki_title tests ─────────────────────

    #[test]
    fn test_normalize_wiki_title_underscores_to_spaces() {
        assert_eq!(normalize_wiki_title("Memory_management"), "memory management");
    }

    #[test]
    fn test_normalize_wiki_title_lowercases() {
        assert_eq!(normalize_wiki_title("RAII"), "raii");
    }

    #[test]
    fn test_normalize_wiki_title_trims() {
        assert_eq!(normalize_wiki_title("  RAII  "), "raii");
    }

    // ─── classify_and_rewrite tests ─────────────────────

    #[test]
    fn test_classify_anchor_only_kept() {
        let index = test_index();
        let r = classify_and_rewrite("#section", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Keep);
    }

    #[test]
    fn test_classify_external_http_kept() {
        let index = test_index();
        let r = classify_and_rewrite("https://example.com", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Keep);
    }

    #[test]
    fn test_classify_protocol_relative_made_absolute() {
        let index = test_index();
        let r = classify_and_rewrite("//commons.wikimedia.org/wiki/File:Test.svg", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Rewrite("https://commons.wikimedia.org/wiki/File:Test.svg".to_string()));
    }

    #[test]
    fn test_classify_red_link_unwrapped() {
        let index = test_index();
        let r = classify_and_rewrite("/wiki/Nonexistent", &index, "en.wikipedia.org", true);
        assert_eq!(r.action, LinkAction::Unwrap);
    }

    #[test]
    fn test_classify_wiki_link_in_manifest_rewritten_to_haleiki() {
        let index = test_index();
        let r = classify_and_rewrite("/wiki/Memory_management", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Rewrite("/source/memory-management/".to_string()));
    }

    #[test]
    fn test_classify_wiki_link_in_manifest_preserves_fragment() {
        let index = test_index();
        let r = classify_and_rewrite("/wiki/Memory_management#Techniques", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Rewrite("/source/memory-management/#Techniques".to_string()));
    }

    #[test]
    fn test_classify_wiki_link_with_parens_in_manifest() {
        let index = test_index();
        let r = classify_and_rewrite("/wiki/Garbage_collection_(computer_science)", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Rewrite("/source/garbage-collection/".to_string()));
    }

    #[test]
    fn test_classify_wiki_link_not_in_manifest_made_absolute() {
        let index = test_index();
        let r = classify_and_rewrite("/wiki/Operating_system", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Rewrite("https://en.wikipedia.org/wiki/Operating_system".to_string()));
    }

    #[test]
    fn test_classify_wiki_link_not_in_manifest_preserves_fragment() {
        let index = test_index();
        let r = classify_and_rewrite("/wiki/Operating_system#Types", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Rewrite("https://en.wikipedia.org/wiki/Operating_system#Types".to_string()));
    }

    #[test]
    fn test_classify_case_insensitive_matching() {
        let index = test_index();
        // RAII is in the index as lowercase "raii"
        let r = classify_and_rewrite("/wiki/RAII", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Rewrite("/source/raii/".to_string()));
    }

    // ─── Full HTML rewriting tests ──────────────────────

    #[test]
    fn test_rewrite_links_internal_link() {
        let index = test_index();
        let html = r#"<p>See <a href="/wiki/Memory_management">memory management</a>.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org").unwrap();
        assert!(result.contains("href=\"/source/memory-management/\""),
            "Internal link not rewritten: {result}");
    }

    #[test]
    fn test_rewrite_links_external_wiki_link() {
        let index = test_index();
        let html = r#"<p>See <a href="/wiki/Operating_system">OS</a>.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org").unwrap();
        assert!(result.contains("href=\"https://en.wikipedia.org/wiki/Operating_system\""),
            "External wiki link not made absolute: {result}");
    }

    #[test]
    fn test_rewrite_links_red_link_unwrapped() {
        let index = test_index();
        let html = r#"<p>See <a href="/wiki/Nonexistent" class="new">nonexistent page</a>.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org").unwrap();
        assert!(!result.contains("<a"), "Red link <a> tag not removed: {result}");
        assert!(result.contains("nonexistent page"), "Red link text was removed: {result}");
    }

    #[test]
    fn test_rewrite_links_anchor_only_preserved() {
        let index = test_index();
        let html = r#"<p>See <a href="#Overview">overview</a>.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org").unwrap();
        assert!(result.contains("href=\"#Overview\""),
            "Anchor link was modified: {result}");
    }

    #[test]
    fn test_rewrite_links_external_url_preserved() {
        let index = test_index();
        let html = r#"<p>See <a href="https://example.com">example</a>.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org").unwrap();
        assert!(result.contains("href=\"https://example.com\""),
            "External URL was modified: {result}");
    }

    #[test]
    fn test_rewrite_links_multiple_links_in_same_paragraph() {
        let index = test_index();
        let html = r#"<p><a href="/wiki/Memory_management">MM</a> uses <a href="/wiki/Garbage_collection_(computer_science)">GC</a> or <a href="/wiki/Operating_system">OS</a> features.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org").unwrap();
        assert!(result.contains("/source/memory-management/"), "First internal link not rewritten");
        assert!(result.contains("/source/garbage-collection/"), "Second internal link not rewritten");
        assert!(result.contains("en.wikipedia.org/wiki/Operating_system"), "External link not absolute");
    }

    #[test]
    fn test_rewrite_links_protocol_relative() {
        let index = test_index();
        let html = r#"<img src="//upload.wikimedia.org/image.png" />"#;
        // img src is not an <a href>, so it shouldn't be rewritten by this module
        let result = rewrite_links(html, &index, "en.wikipedia.org").unwrap();
        assert!(result.contains("//upload.wikimedia.org"), "Non-link src should not be touched");
    }

    // ─── url_decode_basic tests ─────────────────────────

    #[test]
    fn test_url_decode_basic_plus() {
        assert_eq!(url_decode_basic("C%2B%2B"), "C++");
    }

    #[test]
    fn test_url_decode_basic_spaces() {
        assert_eq!(url_decode_basic("Memory%20management"), "Memory management");
    }

    #[test]
    fn test_url_decode_basic_no_encoding() {
        assert_eq!(url_decode_basic("simple"), "simple");
    }

    // ─── Real article test ──────────────────────────────

    #[test]
    #[ignore] // Requires demo/.staging/memory-management.clean.html
    fn test_rewrite_article_real_html() {
        use super::super::manifest::Manifest;
        use std::path::Path;

        let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("demo/manifest.yaml");
        let manifest = Manifest::from_file(&manifest_path).unwrap();

        let result = rewrite_article("memory-management", &manifest);
        assert!(result.is_ok(), "Rewriting failed: {:?}", result.err());

        let rewritten_path = result.unwrap();
        let html = std::fs::read_to_string(&rewritten_path).unwrap();

        // Should contain some /source/ links (internal)
        let internal_count = html.matches("/source/").count();
        eprintln!("Internal links found: {internal_count}");
        // The memory management article links to several in our manifest
        assert!(
            internal_count > 0,
            "No internal links found — expected at least some manifest matches"
        );

        // Should contain some absolute wikipedia.org links (external)
        assert!(
            html.contains("https://en.wikipedia.org/wiki/"),
            "No external wiki links found"
        );

        // Should NOT contain bare /wiki/ links
        let bare_wiki_count = html.matches("href=\"/wiki/").count();
        assert_eq!(
            bare_wiki_count, 0,
            "Found {bare_wiki_count} bare /wiki/ links that should have been rewritten"
        );
    }
}
```

---

## Step 5: Add to `manifest.rs` tests

### Additional tests for `title_to_slug_index`

Add to the existing `#[cfg(test)]` module in `manifest.rs`:

```rust
#[test]
fn test_title_to_slug_index_contains_all_articles() {
    let m = sample_manifest();
    let index = m.title_to_slug_index();
    // Each article adds 2 entries (normalized + underscore form)
    assert!(index.len() >= m.articles.len());
}

#[test]
fn test_title_to_slug_index_lookup_with_underscores() {
    let m = sample_manifest();
    let index = m.title_to_slug_index();
    assert_eq!(
        index.get("memory_management"),
        Some(&"memory-management".to_string()),
    );
}

#[test]
fn test_title_to_slug_index_lookup_case_insensitive() {
    let m = sample_manifest();
    let index = m.title_to_slug_index();
    // normalize_wiki_title lowercases
    assert_eq!(
        index.get("memory management"),
        Some(&"memory-management".to_string()),
    );
}

#[test]
fn test_title_to_slug_index_parenthetical_title() {
    let m = sample_manifest();
    let index = m.title_to_slug_index();
    assert_eq!(
        index.get("garbage collection (computer science)"),
        Some(&"garbage-collection".to_string()),
    );
}
```

---

## Verification

### 6.1: Rewrite a cleaned article (requires prior fetch + clean)

```bash
cd /Users/oubiwann/lab/oxur/haleiki

# Run the ignored test against real HTML
cargo test --features demo -- test_rewrite_article_real_html --ignored
```

### 6.2: Visual verification

Open `demo/.staging/memory-management.rewritten.html` in a browser and inspect links:
- Links to "Garbage collection" → should point to `/source/garbage-collection/`
- Links to "RAII" → should point to `/source/raii/`
- Links to articles NOT in the manifest (e.g., "Computer science") → should point to `https://en.wikipedia.org/wiki/Computer_science`
- No bare `/wiki/` links remaining

### 6.3: Link statistics

```bash
# Quick check: count internal vs external links
grep -c '/source/' demo/.staging/memory-management.rewritten.html
grep -c 'en.wikipedia.org/wiki/' demo/.staging/memory-management.rewritten.html
grep -c 'href="/wiki/' demo/.staging/memory-management.rewritten.html  # Should be 0
```

### 6.4: All tests pass

```bash
cargo test --features demo
```

### 6.5: Lint passes

```bash
make lint
```

---

## Acceptance Criteria

- [ ] `tools/src/demo/rewrite.rs` implements link rewriting
- [ ] `Manifest::title_to_slug_index()` builds a title→slug lookup from all articles
- [ ] Wiki links matching manifest titles → rewritten to `/source/{slug}/`
- [ ] Wiki links NOT in manifest → rewritten to absolute `https://{project}/wiki/{title}`
- [ ] Fragment anchors (`#section`) preserved in both internal and external rewrites
- [ ] Anchor-only links (`#fragment`) left untouched
- [ ] External URLs (`http://`, `https://`) left untouched
- [ ] Protocol-relative URLs (`//...`) made absolute with `https:`
- [ ] Red links (dead wiki links) → `<a>` tag removed, inner text preserved
- [ ] Case-insensitive title matching (RAII matches /wiki/RAII)
- [ ] Percent-encoded titles decoded before matching (%2B → +)
- [ ] No bare `/wiki/` links remain after rewriting
- [ ] Title matching works with underscores and spaces interchangeably
- [ ] All unit tests pass (20+ tests)
- [ ] Real article test passes (when HTML is available)
- [ ] `make lint` passes

---

## Gotchas

1. **Multiple links with same href**: A Wikipedia article may link to the same target multiple times. The `apply_replacements` function processes each replacement against the first remaining occurrence. Since all links to the same target get the same rewrite, this produces correct results — but the "first occurrence" matching means processing order matters.

2. **`scraper` for analysis, string ops for mutation**: We use `scraper` to parse and classify links but perform the actual replacement on the raw HTML string. This avoids the scraper immutability limitation. It's a pragmatic approach — not a full DOM manipulation.

3. **Nested `<a>` tags**: HTML doesn't allow nested `<a>` tags, so we don't need to handle them.

4. **`href` attribute quoting**: Wikipedia HTML uses double-quoted attributes (`href="..."`). The search/replace logic assumes double quotes. Single-quoted or unquoted attributes would break — but Wikipedia's REST API consistently uses double quotes.

5. **Title normalization edge cases**: Some Wikipedia titles have unusual capitalization rules (e.g., "iPhone" stays lowercase-i even at the start of a title). Our case-insensitive matching handles this correctly by lowercasing both the link target and the index keys.

6. **Wikibooks links**: The Wikibooks article title `"Introduction to Computer Science/Memory Management"` contains a `/`. When this appears as a link target in another article, the `/` is preserved in the URL: `/wiki/Introduction_to_Computer_Science/Memory_Management`. The `extract_wiki_title` function handles this correctly — it strips the `/wiki/` prefix and returns the rest.

7. **Interlanguage links**: Wikipedia articles may contain links to other language versions (e.g., `//fr.wikipedia.org/wiki/...`). These are protocol-relative and get the `https:` prefix. They're not in our manifest so they stay external.

8. **`apply_replacements` order sensitivity**: Because we modify the string in place and each replacement changes string positions, we must be careful. The current approach finds and replaces one occurrence at a time. For the 12-article demo this is fine. For larger scales, consider a single-pass rewriter that processes the string left-to-right.

9. **URL encoding in href attributes**: Wikipedia REST API may encode some characters in hrefs (e.g., `%2B` for `+`). The `url_decode_basic` function handles common cases. More exotic encodings (multi-byte UTF-8) may need `percent-encoding` crate if they appear in practice.

10. **Testing against real articles**: The synthetic unit tests cover the logic, but real Wikipedia HTML has quirks (e.g., links inside `<span>` inside `<a>`, link text containing HTML tags). The `#[ignore]` real-article test is essential for catching these.
