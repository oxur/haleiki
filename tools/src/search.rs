//! The Pagefind search index — Arc 04's `/pagefind/` bundle.
//!
//! Per the operator's 2026-07-29 reversal of pivot decision #3 (custom JSON
//! index → Pagefind; `docs/tiny-http/design-delta.md` Addendum A), search is
//! **client-only**: the `pagefind` crate builds an in-process index from the
//! corpus (no `npx`/Node/binary shell-out) and emits Pagefind's own
//! `/pagefind/` bundle (WASM + JS + binary index/fragment files), which both
//! sinks host as-is — it is not folded into `render_routes` as a page.
//!
//! [`build_records`] is the pure record builder (unit-testable without
//! Pagefind itself); [`build_bundle`] drives the `pagefind` crate's async
//! service API (bridged onto a single-threaded Tokio runtime, since the rest
//! of Haleiki is synchronous) to actually write the bundle to disk.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};

use crate::parser::{ConceptCard, Corpus, SourcePage};

/// Fallback trust bucket for source records, which carry no `trust_level`
/// (concept-only field) — used only if the profile itself has no
/// `trust.default` configured. Mirrors `article.rs`'s established convention
/// of falling back to `site.data.profile.trust.default` for the same reason
/// (arc-01 slice-03): sources display and now search-facet with the same
/// trust bucket, so the two never disagree.
const FALLBACK_TRUST_BUCKET: &str = "unrated";

/// A leading excerpt is truncated to this many characters (on a whitespace
/// boundary), for `meta.excerpt`.
const EXCERPT_MAX_CHARS: usize = 200;

/// One Pagefind record, ready to hand to [`pagefind::api::PagefindIndex::add_custom_record`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchRecord {
    /// `/{slug}/` for a concept, `/source/{slug}/` for a source — the same
    /// scheme `server::render_routes` uses.
    pub url: String,
    /// Indexed text: title + keywords + aliases (concepts only) + a
    /// plain-text rendering of the body.
    pub content: String,
    /// Result-card metadata: `title`, `url`, `excerpt`, plus `type`/
    /// `category`/`tier` for the template's badges.
    pub meta: BTreeMap<String, String>,
    /// Facet values: `category`, `tier`, `type`, `trust` — each single-valued
    /// but carried as Pagefind's `Vec<String>` filter shape.
    pub filters: BTreeMap<String, Vec<String>>,
}

/// Builds one [`SearchRecord`] per concept and per source in `corpus`, sorted
/// by slug (matching `render_routes`'s own sorted walk) so repeated builds
/// feed Pagefind records in the same order — the bundle's byte-for-byte
/// determinism depends on this, the same way `export_site`'s does.
#[must_use]
pub fn build_records(corpus: &Corpus) -> Vec<SearchRecord> {
    let default_trust = corpus
        .profile
        .as_ref()
        .map(|p| p.trust.default.clone())
        .filter(|d| !d.is_empty())
        .unwrap_or_else(|| FALLBACK_TRUST_BUCKET.to_string());

    let mut records = Vec::with_capacity(corpus.concepts.len() + corpus.sources.len());

    let mut concept_slugs: Vec<&String> = corpus.concepts.keys().collect();
    concept_slugs.sort();
    for slug in concept_slugs {
        records.push(concept_record(&corpus.concepts[slug], slug));
    }

    let mut source_slugs: Vec<&String> = corpus.sources.keys().collect();
    source_slugs.sort();
    for slug in source_slugs {
        records.push(source_record(&corpus.sources[slug], slug, &default_trust));
    }

    records
}

fn concept_record(card: &ConceptCard, slug: &str) -> SearchRecord {
    let plain_body = markdown_to_plain_text(&card.body);
    let excerpt = leading_excerpt(&plain_body);
    let url = format!("/{slug}/");

    let mut content_parts = vec![card.concept.clone()];
    content_parts.extend(card.keywords.iter().cloned());
    content_parts.extend(card.aliases.iter().cloned());
    content_parts.push(plain_body);

    let trust = if card.trust_level.is_empty() {
        // No demo concept is missing trust_level today, but an empty value
        // would otherwise leave the `trust` filter empty-valued (G-4's
        // concern applies here too, not just to sources).
        FALLBACK_TRUST_BUCKET.to_string()
    } else {
        card.trust_level.clone()
    };

    SearchRecord {
        url: url.clone(),
        content: content_parts.join(" "),
        meta: result_meta(
            &card.concept,
            &url,
            &excerpt,
            &card.page_type,
            &card.category,
            &card.tier,
        ),
        filters: result_filters(&card.category, &card.tier, &card.page_type, &trust),
    }
}

fn source_record(page: &SourcePage, slug: &str, default_trust: &str) -> SearchRecord {
    let plain_body = markdown_to_plain_text(&page.body);
    let excerpt = leading_excerpt(&plain_body);
    let url = format!("/source/{slug}/");

    // SourcePage carries no `aliases` field (concept-only), so content is
    // title + keywords + body, not title + keywords + aliases + body.
    let mut content_parts = vec![page.title.clone()];
    content_parts.extend(page.keywords.iter().cloned());
    content_parts.push(plain_body);

    SearchRecord {
        url: url.clone(),
        content: content_parts.join(" "),
        meta: result_meta(
            &page.title,
            &url,
            &excerpt,
            &page.page_type,
            &page.category,
            &page.tier,
        ),
        filters: result_filters(&page.category, &page.tier, &page.page_type, default_trust),
    }
}

fn result_meta(
    title: &str,
    url: &str,
    excerpt: &str,
    page_type: &str,
    category: &str,
    tier: &str,
) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("title".to_string(), title.to_string()),
        ("url".to_string(), url.to_string()),
        ("excerpt".to_string(), excerpt.to_string()),
        ("type".to_string(), page_type.to_string()),
        ("category".to_string(), category.to_string()),
        ("tier".to_string(), tier.to_string()),
    ])
}

fn result_filters(
    category: &str,
    tier: &str,
    page_type: &str,
    trust: &str,
) -> BTreeMap<String, Vec<String>> {
    BTreeMap::from([
        ("category".to_string(), vec![category.to_string()]),
        ("tier".to_string(), vec![tier.to_string()]),
        ("type".to_string(), vec![page_type.to_string()]),
        ("trust".to_string(), vec![trust.to_string()]),
    ])
}

/// Renders `body` (Markdown source) to HTML via the same [`crate::markdown`]
/// primitive the pages use, then strips tags and unescapes the handful of
/// entities comrak emits. Pagefind's custom-record `content` is indexed and
/// excerpted **verbatim** (no HTML parsing happens for records added via
/// `add_custom_record` — only the file/synthetic-HTML paths get that), so
/// passing raw Markdown or HTML here would leak `##`/`**`/`<p>` syntax into
/// search results.
fn markdown_to_plain_text(body: &str) -> String {
    let html = crate::markdown::render_markdown(body).html;

    let mut text = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if in_tag => {}
            _ => text.push(ch),
        }
    }

    let text = text
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");

    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A leading, whitespace-safe excerpt of `text`, at most `EXCERPT_MAX_CHARS`
/// characters (never splitting a multi-byte character or a word in the
/// middle when a shorter word-boundary cut is available).
fn leading_excerpt(text: &str) -> String {
    if text.chars().count() <= EXCERPT_MAX_CHARS {
        return text.to_string();
    }

    let truncated: String = text.chars().take(EXCERPT_MAX_CHARS).collect();
    match truncated.rfind(' ') {
        Some(boundary) if boundary > 0 => truncated[..boundary].to_string(),
        _ => truncated,
    }
}

/// Builds the Pagefind index from `corpus` and writes the `/pagefind/`
/// bundle to `out_dir` (created if missing). Bridges the `pagefind` crate's
/// async service API onto a single-threaded Tokio runtime — the rest of
/// Haleiki is synchronous, so this keeps the async boundary contained to
/// this one function.
///
/// # Errors
///
/// Returns an error if the Tokio runtime fails to start, if indexing any
/// record fails, or if the bundle cannot be written to `out_dir`.
pub fn build_bundle(corpus: &Corpus, out_dir: &Path) -> Result<()> {
    let records = build_records(corpus);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to start the search-index runtime")?;

    runtime.block_on(write_bundle(records, out_dir))
}

async fn write_bundle(records: Vec<SearchRecord>, out_dir: &Path) -> Result<()> {
    let mut index = pagefind::api::PagefindIndex::new(None)
        .context("failed to configure the pagefind index")?;

    for record in records {
        index
            .add_custom_record(
                record.url,
                record.content,
                "en".to_string(),
                Some(record.meta),
                Some(record.filters),
                None,
            )
            .await
            .context("failed to index a search record")?;
    }

    index
        .write_files(Some(out_dir.to_string_lossy().into_owned()))
        .await
        .context("failed to write the pagefind bundle")?;

    Ok(())
}

/// Parses and validates `content_dir`, then builds the `/pagefind/` bundle
/// into `out_dir` — the entry point `export::export_site`, `server::serve`,
/// and the `haleiki search` subcommand all share.
///
/// # Errors
///
/// Returns an error if the corpus fails to parse or validate, or if the
/// bundle fails to build (see [`build_bundle`]).
pub fn build_index(content_dir: &Path, out_dir: &Path) -> Result<()> {
    let corpus = crate::parser::parse_corpus(content_dir)
        .with_context(|| format!("failed to parse corpus from {}", content_dir.display()))?;

    let report = crate::validator::validate(&corpus, content_dir);
    if report.has_errors() {
        report.print();
        anyhow::bail!("Validation failed with {} error(s).", report.errors.len());
    }

    build_bundle(&corpus, out_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn demo_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    fn demo_corpus() -> Corpus {
        crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses")
    }

    // G-1 (record-count half): exactly one record per concept + source.
    #[test]
    fn test_build_records_count_matches_corpus_size() {
        let corpus = demo_corpus();
        let records = build_records(&corpus);
        assert_eq!(records.len(), corpus.concepts.len() + corpus.sources.len());
        assert!(corpus.news.contains_key("search-lands-in-the-demo-wiki"));
        assert!(
            records
                .iter()
                .all(|record| !record.url.starts_with("/news/")),
            "news posts are not indexed by this slice"
        );
    }

    // G-1 (no-shell-out half): search.rs never invokes an external pagefind
    // binary or npx as a subprocess. Checks for the actual invocation shapes
    // (`Command::new(...)`), not a bare word match — this file's own doc
    // comments legitimately *say* "npx" (to explain that it isn't used),
    // which a bare substring check would misfire on.
    #[test]
    fn test_search_never_shells_out_to_an_external_pagefind_binary() {
        let search_src = include_str!("search.rs");
        let command_new_pagefind = format!("Command::new(\"{}\")", "pagefind");
        let command_new_npx = format!("Command::new(\"{}{}\")", "n", "px");

        assert!(!search_src.contains(&command_new_pagefind));
        assert!(!search_src.contains(&command_new_npx));
    }

    // G-2 + G-3: a sampled concept record carries the four filters, the
    // result meta, and the correct /{slug}/ url.
    #[test]
    fn test_concept_record_has_filters_meta_and_url() {
        let corpus = demo_corpus();
        let records = build_records(&corpus);

        let record = records
            .iter()
            .find(|r| r.url == "/abelian-group/")
            .expect("abelian-group record exists");

        assert_eq!(record.meta["title"], "Abelian Group");
        assert_eq!(record.meta["url"], "/abelian-group/");
        assert!(!record.meta["excerpt"].is_empty());
        assert_eq!(record.meta["type"], "concept");
        assert_eq!(record.meta["category"], "mathematics");
        assert_eq!(record.meta["tier"], "intermediate");

        assert_eq!(record.filters["category"], vec!["mathematics".to_string()]);
        assert_eq!(record.filters["tier"], vec!["intermediate".to_string()]);
        assert_eq!(record.filters["type"], vec!["concept".to_string()]);
        assert_eq!(record.filters["trust"], vec!["credible".to_string()]);

        assert!(record.content.contains("Abelian Group"));
        assert!(record.content.contains("commutativity"));
        assert!(record.content.contains("commutative group"));
        // Plain text, not raw Markdown/HTML: no leaked syntax.
        assert!(!record.content.contains("##"));
        assert!(!record.content.contains("<p>"));
    }

    // G-2 + G-3: a sampled source record carries its filters, meta, and the
    // correct /source/{slug}/ url.
    #[test]
    fn test_source_record_has_filters_meta_and_url() {
        let corpus = demo_corpus();
        let records = build_records(&corpus);

        let record = records
            .iter()
            .find(|r| r.url == "/source/vimalamitra/")
            .expect("vimalamitra record exists");

        assert_eq!(record.meta["title"], "Vimalamitra");
        assert_eq!(record.meta["url"], "/source/vimalamitra/");
        assert!(!record.meta["excerpt"].is_empty());
        assert_eq!(record.meta["type"], "source");
        assert_eq!(record.meta["category"], "tibetan-buddhism");
        assert_eq!(record.meta["tier"], "advanced");

        assert_eq!(
            record.filters["category"],
            vec!["tibetan-buddhism".to_string()]
        );
        assert_eq!(record.filters["tier"], vec!["advanced".to_string()]);
        assert_eq!(record.filters["type"], vec!["source".to_string()]);

        assert!(record.content.contains("Vimalamitra"));
        assert!(record.content.contains("Vima Nyingtik"));
    }

    // G-4: sources have no trust_level, so their trust filter falls back to
    // the profile's default trust bucket — never empty-valued. The demo
    // profile's default is "credible" (matches how `article.rs` already
    // falls sources back to `site.data.profile.trust.default` for display —
    // search and display agree on the same bucket).
    #[test]
    fn test_source_record_trust_filter_uses_profile_default_bucket() {
        let corpus = demo_corpus();
        let default_trust = corpus
            .profile
            .as_ref()
            .map(|p| p.trust.default.clone())
            .expect("demo profile has a trust.default");
        assert_eq!(default_trust, "credible");

        let records = build_records(&corpus);
        let record = records
            .iter()
            .find(|r| r.url == "/source/vimalamitra/")
            .expect("vimalamitra record exists");

        assert_eq!(record.filters["trust"], vec![default_trust]);
        assert!(!record.filters["trust"][0].is_empty());
    }

    // Records must come out in the same order every time — the pagefind
    // bundle's determinism (beyond this slice's own known upstream filter-
    // ordering gap; see export.rs's determinism test) depends on feeding
    // records in a stable order. Compares against slugs sorted independently
    // (not against a re-sort of the *url* strings): a slug that's a strict
    // prefix of another, e.g. "language" / "language-family", sorts
    // differently as a bare slug than as a `/`-wrapped url string, so the
    // url list is not itself sorted — only the slug walk that produced it is.
    #[test]
    fn test_build_records_is_sorted_by_slug_for_determinism() {
        let corpus = demo_corpus();

        let mut expected_concept_slugs: Vec<&String> = corpus.concepts.keys().collect();
        expected_concept_slugs.sort();
        let expected_concept_urls: Vec<String> = expected_concept_slugs
            .iter()
            .map(|slug| format!("/{slug}/"))
            .collect();

        let mut expected_source_slugs: Vec<&String> = corpus.sources.keys().collect();
        expected_source_slugs.sort();
        let expected_source_urls: Vec<String> = expected_source_slugs
            .iter()
            .map(|slug| format!("/source/{slug}/"))
            .collect();

        let records = build_records(&corpus);
        let concept_urls: Vec<&str> = records
            .iter()
            .map(|r| r.url.as_str())
            .filter(|u| !u.starts_with("/source/"))
            .collect();
        let source_urls: Vec<&str> = records
            .iter()
            .map(|r| r.url.as_str())
            .filter(|u| u.starts_with("/source/"))
            .collect();

        assert_eq!(concept_urls, expected_concept_urls);
        assert_eq!(source_urls, expected_source_urls);
    }

    #[test]
    fn test_leading_excerpt_truncates_on_a_word_boundary() {
        let short = "A short excerpt.";
        assert_eq!(leading_excerpt(short), short);

        let long = "word ".repeat(100);
        let excerpt = leading_excerpt(&long);
        assert!(excerpt.chars().count() <= EXCERPT_MAX_CHARS);
        assert!(!excerpt.ends_with(' '));
        assert!(long.starts_with(&excerpt));
    }

    #[test]
    fn test_markdown_to_plain_text_strips_tags_and_unescapes_entities() {
        let plain = markdown_to_plain_text("# Title\n\nSome **bold** text & more.");
        assert!(!plain.contains('#'));
        assert!(!plain.contains("**"));
        assert!(!plain.contains('<'));
        assert!(plain.contains("Title"));
        assert!(plain.contains("bold"));
        assert!(plain.contains('&'));
    }

    // G-5: haleiki export emits _site/pagefind/ with the JS entry, a .wasm,
    // pagefind-entry.json, and index/fragment files.
    #[test]
    fn test_build_index_writes_the_expected_bundle_files() {
        let out = std::env::temp_dir().join("haleiki-search-build-index-test");
        let _ = std::fs::remove_dir_all(&out);

        build_index(&demo_dir(), &out).expect("index builds for the demo corpus");

        assert!(out.join("pagefind.js").exists());
        assert!(out.join("pagefind-entry.json").exists());
        let has_extension = |ext: &str| {
            std::fs::read_dir(&out)
                .unwrap()
                .chain(
                    std::fs::read_dir(out.join("fragment"))
                        .into_iter()
                        .flatten(),
                )
                .chain(std::fs::read_dir(out.join("index")).into_iter().flatten())
                .filter_map(std::result::Result::ok)
                .any(|entry| {
                    entry
                        .path()
                        .to_string_lossy()
                        .to_ascii_lowercase()
                        .ends_with(ext)
                })
        };
        // The WASM binary's real on-disk name is `wasm.{lang}.pagefind`
        // (extension `.pagefind`, not `.wasm` — confirmed against a real
        // build; see `server::pagefind_content_type_for`'s doc comment).
        assert!(
            has_extension(".pagefind"),
            "expected a wasm.*.pagefind file in the bundle"
        );
        assert!(
            has_extension(".pf_fragment"),
            "expected a .pf_fragment file"
        );
        assert!(has_extension(".pf_index"), "expected a .pf_index file");

        let _ = std::fs::remove_dir_all(&out);
    }
}
