//! Link rewriting -- convert wiki links to Haleiki internal links or
//! absolute external URLs.
//!
//! Takes cleaned HTML and rewrites `<a href>` targets based on
//! whether the link target matches a manifest article.

use std::collections::HashMap;
use std::path::Path;

use scraper::{Html, Selector};

use super::clean::staging_clean_path;
use super::fetch::is_wikimedia_project;
use super::manifest::{Manifest, normalize_wiki_title};

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

    let rewritten = rewrite_links(&html, &title_index, project);

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
) -> String {
    // Strategy: use scraper to parse and classify links, then do string-level
    // replacement on the raw HTML string. scraper can't mutate the DOM, so we
    // collect replacements and apply them to the string.

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
            .is_some_and(|c| c.contains("new"));

        let replacement = classify_and_rewrite(href, title_index, source_project, is_red_link);
        replacements.push(replacement);
    }

    // Apply replacements to the HTML string.
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

    /// Remove the `<a>` wrapper but keep the inner text.
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

    // 1. Red links -- unwrap (remove <a>, keep text)
    if is_red_link {
        return LinkReplacement {
            original_href,
            action: LinkAction::Unwrap,
        };
    }

    // 2. Anchor-only links (#fragment) -- keep as-is
    if href.starts_with('#') {
        return LinkReplacement {
            original_href,
            action: LinkAction::Keep,
        };
    }

    // 3. External links -- check for self-references first
    if href.starts_with("http://") || href.starts_with("https://") {
        // Check if this is a self-reference to the source project
        // e.g., http://www.rigpawiki.org/index.php?title=PageName
        if let Some(path_part) = strip_source_project_prefix(href, source_project) {
            if let Some(title_part) = extract_wiki_title(&path_part) {
                let (title, fragment) = split_fragment(&title_part);
                let normalized = normalize_wiki_title(&title);
                if let Some(slug) = title_index.get(&normalized) {
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
            }
            // Self-reference but not in manifest -- keep as-is (already absolute)
            return LinkReplacement {
                original_href,
                action: LinkAction::Keep,
            };
        }
        return LinkReplacement {
            original_href,
            action: LinkAction::Keep,
        };
    }

    // 4. Protocol-relative URLs (//commons.wikimedia.org/...) -- make absolute
    if href.starts_with("//") {
        return LinkReplacement {
            original_href,
            action: LinkAction::Rewrite(format!("https:{href}")),
        };
    }

    // 5. Wiki links -- /wiki/Title or ./Title formats
    if let Some(title_part) = extract_wiki_title(href) {
        let (title, fragment) = split_fragment(&title_part);
        let normalized = normalize_wiki_title(&title);

        if let Some(slug) = title_index.get(&normalized) {
            // Internal link -- rewrite to Haleiki source page URL
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

        // External wiki link -- rewrite to absolute URL using source
        // project's URL pattern. Wikimedia projects use `/wiki/{title}`;
        // non-Wikimedia MediaWiki instances use `/index.php?title=`.
        let absolute_url = if is_wikimedia_project(source_project) {
            if let Some(frag) = fragment {
                format!("https://{source_project}/wiki/{title}#{frag}")
            } else {
                format!("https://{source_project}/wiki/{title}")
            }
        } else {
            let encoded_title = title.replace(' ', "_");
            if let Some(frag) = fragment {
                format!("https://{source_project}/index.php?title={encoded_title}#{frag}")
            } else {
                format!("https://{source_project}/index.php?title={encoded_title}")
            }
        };
        return LinkReplacement {
            original_href,
            action: LinkAction::Rewrite(absolute_url),
        };
    }

    // 6. Other relative links -- make absolute against source project
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
/// - `/wiki/Article_Title` -> `Article_Title`
/// - `./Article_Title` -> `Article_Title`
/// - `/index.php?title=Article_Title` -> `Article_Title` (Rigpa Wiki)
/// - `/w/index.php?title=Article_Title&action=...` -> None (edit/action links)
/// - `/index.php?title=File:...` -> None (media namespace)
fn extract_wiki_title(href: &str) -> Option<String> {
    // /wiki/Title pattern (most common in REST API HTML)
    if let Some(rest) = href.strip_prefix("/wiki/") {
        return Some(url_decode_basic(rest));
    }

    // ./Title pattern (relative links in some REST API versions)
    if let Some(rest) = href.strip_prefix("./") {
        return Some(url_decode_basic(rest));
    }

    // /w/index.php links are action links (edit, history) -- not content links
    if href.starts_with("/w/") {
        return None;
    }

    // /index.php?title=PageName pattern (MediaWiki action API, used by Rigpa Wiki)
    if let Some(title) = extract_title_from_query(href) {
        // Skip File: namespace -- images are handled by the media pipeline
        if title.starts_with("File:") {
            return None;
        }
        return Some(url_decode_basic(&title));
    }

    None
}

/// Extract the `title` query parameter from a `/index.php?title=...` URL path.
///
/// Returns `None` for action links (those containing `action=edit`, etc.)
/// and for empty title values.
fn extract_title_from_query(href: &str) -> Option<String> {
    let query = href.strip_prefix("/index.php?").or_else(|| {
        href.find("/index.php?")
            .map(|pos| &href[pos + "/index.php?".len()..])
    })?;

    // Skip action links (edit, history, etc.)
    if query.contains("action=") {
        return None;
    }

    for param in query.split('&') {
        if let Some(value) = param.strip_prefix("title=") {
            let value = value.split('#').next().unwrap_or(value);
            if value.is_empty() {
                return None;
            }
            return Some(value.to_string());
        }
    }
    None
}

/// Strip the source project's host from an absolute URL, returning the path portion.
///
/// Matches both `http://` and `https://`, with or without `www.` prefix.
fn strip_source_project_prefix(url: &str, source_project: &str) -> Option<String> {
    let stripped = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;

    if let Some(path) = stripped.strip_prefix(source_project) {
        if path.is_empty() || path.starts_with('/') {
            return Some(path.to_string());
        }
    }
    None
}

/// Split a title into `(title, optional_fragment)`.
fn split_fragment(title: &str) -> (String, Option<String>) {
    if let Some(pos) = title.find('#') {
        let (t, f) = title.split_at(pos);
        (t.to_string(), Some(f[1..].to_string())) // skip the '#'
    } else {
        (title.to_string(), None)
    }
}

/// Basic URL decoding -- handles `%XX` sequences common in wiki URLs.
///
/// Correctly decodes multi-byte UTF-8 percent-encoded sequences
/// (e.g., `%C3%BC` -> `ü`).
fn url_decode_basic(s: &str) -> String {
    let mut bytes = Vec::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    bytes.push(byte);
                    continue;
                }
            }
            bytes.push(b'%');
            bytes.extend_from_slice(hex.as_bytes());
        } else if c.is_ascii() {
            bytes.push(c as u8);
        } else {
            let mut buf = [0u8; 4];
            let encoded = c.encode_utf8(&mut buf);
            bytes.extend_from_slice(encoded.as_bytes());
        }
    }

    String::from_utf8(bytes).unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned())
}

/// Apply link replacements to the HTML string.
///
/// For `Rewrite` actions: change the href attribute value.
/// For `Unwrap` actions: replace `<a ...>text</a>` with just text.
/// For `Keep` actions: do nothing.
fn apply_replacements(html: &str, replacements: &[LinkReplacement]) -> String {
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
                let old_pattern =
                    format!("href=\"{}\"", escape_for_search(&replacement.original_href));
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
                unwrap_link(&mut result, &replacement.original_href);
            }
        }
    }

    result
}

/// Remove an `<a>` wrapper with the given href, keeping the inner text.
fn unwrap_link(html: &mut String, href: &str) {
    let search = format!("href=\"{}\"", escape_for_search(href));

    // Find the <a tag containing this href
    if let Some(href_pos) = html.find(&search) {
        // Walk backwards to find the opening <a
        let mut open_start = href_pos;
        while open_start > 0 && &html[open_start..=open_start] != "<" {
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
                *html = format!("{}{}{}", &html[..open_start], inner, &html[close_end..],);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a simple title index for testing.
    fn test_index() -> HashMap<String, String> {
        let mut index = HashMap::new();
        index.insert(
            "memory management".to_string(),
            "memory-management".to_string(),
        );
        index.insert(
            "memory_management".to_string(),
            "memory-management".to_string(),
        );
        index.insert(
            "garbage collection (computer science)".to_string(),
            "garbage-collection".to_string(),
        );
        index.insert(
            "garbage_collection_(computer_science)".to_string(),
            "garbage-collection".to_string(),
        );
        index.insert("raii".to_string(), "raii".to_string());
        index.insert(
            "resource acquisition is initialization".to_string(),
            "raii".to_string(),
        );
        index.insert(
            "resource_acquisition_is_initialization".to_string(),
            "raii".to_string(),
        );
        index
    }

    // --- extract_wiki_title tests ---

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
        assert_eq!(extract_wiki_title("https://example.com/page"), None,);
    }

    #[test]
    fn test_extract_wiki_title_with_percent_encoding() {
        assert_eq!(extract_wiki_title("/wiki/C%2B%2B"), Some("C++".to_string()),);
    }

    // --- split_fragment tests ---

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

    // --- normalize_wiki_title tests ---

    #[test]
    fn test_normalize_wiki_title_underscores_to_spaces() {
        assert_eq!(
            normalize_wiki_title("Memory_management"),
            "memory management"
        );
    }

    #[test]
    fn test_normalize_wiki_title_lowercases() {
        assert_eq!(normalize_wiki_title("RAII"), "raii");
    }

    #[test]
    fn test_normalize_wiki_title_trims() {
        assert_eq!(normalize_wiki_title("  RAII  "), "raii");
    }

    // --- extract_wiki_title /index.php tests ---

    #[test]
    fn test_extract_wiki_title_index_php_basic() {
        assert_eq!(
            extract_wiki_title("/index.php?title=Nyingma"),
            Some("Nyingma".to_string()),
        );
    }

    #[test]
    fn test_extract_wiki_title_index_php_percent_encoded() {
        assert_eq!(
            extract_wiki_title("/index.php?title=Lhats%C3%BCn_Namkha_Jikm%C3%A9"),
            Some("Lhatsün_Namkha_Jikmé".to_string()),
        );
    }

    #[test]
    fn test_extract_wiki_title_index_php_file_namespace_returns_none() {
        assert_eq!(
            extract_wiki_title("/index.php?title=File:TBRC-tag.png"),
            None,
        );
    }

    #[test]
    fn test_extract_wiki_title_index_php_action_edit_returns_none() {
        assert_eq!(
            extract_wiki_title("/index.php?title=Nyingma&action=edit"),
            None,
        );
    }

    // --- classify_and_rewrite self-reference tests ---

    #[test]
    fn test_classify_absolute_self_reference_in_manifest() {
        let index = test_index();
        let r = classify_and_rewrite(
            "http://www.rigpawiki.org/index.php?title=Memory_management",
            &index,
            "www.rigpawiki.org",
            false,
        );
        assert_eq!(
            r.action,
            LinkAction::Rewrite("/source/memory-management/".to_string()),
        );
    }

    #[test]
    fn test_classify_absolute_self_reference_not_in_manifest() {
        let index = test_index();
        let r = classify_and_rewrite(
            "http://www.rigpawiki.org/index.php?title=Nonexistent_Page",
            &index,
            "www.rigpawiki.org",
            false,
        );
        assert_eq!(r.action, LinkAction::Keep);
    }

    #[test]
    fn test_classify_absolute_non_self_reference() {
        let index = test_index();
        let r = classify_and_rewrite(
            "http://www.lotsawahouse.org/tibetan-masters/yangthang-rinpoche",
            &index,
            "www.rigpawiki.org",
            false,
        );
        assert_eq!(r.action, LinkAction::Keep);
    }

    // --- classify_and_rewrite tests ---

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
        let r = classify_and_rewrite(
            "//commons.wikimedia.org/wiki/File:Test.svg",
            &index,
            "en.wikipedia.org",
            false,
        );
        assert_eq!(
            r.action,
            LinkAction::Rewrite("https://commons.wikimedia.org/wiki/File:Test.svg".to_string())
        );
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
        assert_eq!(
            r.action,
            LinkAction::Rewrite("/source/memory-management/".to_string())
        );
    }

    #[test]
    fn test_classify_wiki_link_in_manifest_preserves_fragment() {
        let index = test_index();
        let r = classify_and_rewrite(
            "/wiki/Memory_management#Techniques",
            &index,
            "en.wikipedia.org",
            false,
        );
        assert_eq!(
            r.action,
            LinkAction::Rewrite("/source/memory-management/#Techniques".to_string())
        );
    }

    #[test]
    fn test_classify_wiki_link_with_parens_in_manifest() {
        let index = test_index();
        let r = classify_and_rewrite(
            "/wiki/Garbage_collection_(computer_science)",
            &index,
            "en.wikipedia.org",
            false,
        );
        assert_eq!(
            r.action,
            LinkAction::Rewrite("/source/garbage-collection/".to_string())
        );
    }

    #[test]
    fn test_classify_wiki_link_not_in_manifest_made_absolute() {
        let index = test_index();
        let r = classify_and_rewrite("/wiki/Operating_system", &index, "en.wikipedia.org", false);
        assert_eq!(
            r.action,
            LinkAction::Rewrite("https://en.wikipedia.org/wiki/Operating_system".to_string())
        );
    }

    #[test]
    fn test_classify_wiki_link_not_in_manifest_preserves_fragment() {
        let index = test_index();
        let r = classify_and_rewrite(
            "/wiki/Operating_system#Types",
            &index,
            "en.wikipedia.org",
            false,
        );
        assert_eq!(
            r.action,
            LinkAction::Rewrite("https://en.wikipedia.org/wiki/Operating_system#Types".to_string())
        );
    }

    #[test]
    fn test_classify_case_insensitive_matching() {
        let index = test_index();
        // RAII is in the index as lowercase "raii"
        let r = classify_and_rewrite("/wiki/RAII", &index, "en.wikipedia.org", false);
        assert_eq!(r.action, LinkAction::Rewrite("/source/raii/".to_string()));
    }

    // --- Full HTML rewriting tests ---

    #[test]
    fn test_rewrite_links_internal_link() {
        let index = test_index();
        let html = r#"<p>See <a href="/wiki/Memory_management">memory management</a>.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org");
        assert!(
            result.contains("href=\"/source/memory-management/\""),
            "Internal link not rewritten: {result}"
        );
    }

    #[test]
    fn test_rewrite_links_external_wiki_link() {
        let index = test_index();
        let html = r#"<p>See <a href="/wiki/Operating_system">OS</a>.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org");
        assert!(
            result.contains("href=\"https://en.wikipedia.org/wiki/Operating_system\""),
            "External wiki link not made absolute: {result}"
        );
    }

    #[test]
    fn test_rewrite_links_red_link_unwrapped() {
        let index = test_index();
        let html = r#"<p>See <a href="/wiki/Nonexistent" class="new">nonexistent page</a>.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org");
        assert!(
            !result.contains("<a"),
            "Red link <a> tag not removed: {result}"
        );
        assert!(
            result.contains("nonexistent page"),
            "Red link text was removed: {result}"
        );
    }

    #[test]
    fn test_rewrite_links_anchor_only_preserved() {
        let index = test_index();
        let html = r##"<p>See <a href="#Overview">overview</a>.</p>"##;
        let result = rewrite_links(html, &index, "en.wikipedia.org");
        assert!(
            result.contains(r##"href="#Overview""##),
            "Anchor link was modified: {result}"
        );
    }

    #[test]
    fn test_rewrite_links_external_url_preserved() {
        let index = test_index();
        let html = r#"<p>See <a href="https://example.com">example</a>.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org");
        assert!(
            result.contains("href=\"https://example.com\""),
            "External URL was modified: {result}"
        );
    }

    #[test]
    fn test_rewrite_links_multiple_links_in_same_paragraph() {
        let index = test_index();
        let html = r#"<p><a href="/wiki/Memory_management">MM</a> uses <a href="/wiki/Garbage_collection_(computer_science)">GC</a> or <a href="/wiki/Operating_system">OS</a> features.</p>"#;
        let result = rewrite_links(html, &index, "en.wikipedia.org");
        assert!(
            result.contains("/source/memory-management/"),
            "First internal link not rewritten"
        );
        assert!(
            result.contains("/source/garbage-collection/"),
            "Second internal link not rewritten"
        );
        assert!(
            result.contains("en.wikipedia.org/wiki/Operating_system"),
            "External link not absolute"
        );
    }

    #[test]
    fn test_rewrite_links_protocol_relative() {
        let index = test_index();
        let html = r#"<img src="//upload.wikimedia.org/image.png" />"#;
        // img src is not an <a href>, so it shouldn't be rewritten by this module
        let result = rewrite_links(html, &index, "en.wikipedia.org");
        assert!(
            result.contains("//upload.wikimedia.org"),
            "Non-link src should not be touched"
        );
    }

    // --- url_decode_basic tests ---

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

    #[test]
    fn test_url_decode_basic_multibyte_utf8() {
        // ü is U+00FC, encoded in UTF-8 as 0xC3 0xBC
        assert_eq!(url_decode_basic("Lhats%C3%BCn"), "Lhatsün");
    }

    // --- Real article test ---

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
            "No internal links found -- expected at least some manifest matches"
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

    // --- Source-aware external wiki link URL pattern tests ---

    #[test]
    fn test_classify_external_wiki_link_wikipedia_uses_wiki_path() {
        let index = test_index();
        let r = classify_and_rewrite("./Some_Article", &index, "en.wikipedia.org", false);
        assert_eq!(
            r.action,
            LinkAction::Rewrite("https://en.wikipedia.org/wiki/Some_Article".to_string()),
        );
    }

    #[test]
    fn test_classify_external_wiki_link_rigpawiki_uses_index_php() {
        let index = test_index();
        let r = classify_and_rewrite("./Domang_Monastery", &index, "www.rigpawiki.org", false);
        assert_eq!(
            r.action,
            LinkAction::Rewrite(
                "https://www.rigpawiki.org/index.php?title=Domang_Monastery".to_string()
            ),
        );
    }

    #[test]
    fn test_classify_external_rigpawiki_preserves_fragment() {
        let index = test_index();
        let r = classify_and_rewrite(
            "./Domang_Monastery#History",
            &index,
            "www.rigpawiki.org",
            false,
        );
        assert_eq!(
            r.action,
            LinkAction::Rewrite(
                "https://www.rigpawiki.org/index.php?title=Domang_Monastery#History".to_string()
            ),
        );
    }
}
