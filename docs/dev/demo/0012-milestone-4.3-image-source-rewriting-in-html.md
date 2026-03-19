# Milestone 4.3 — Image Source Rewriting in HTML

**Version:** 1.0
**Depends on:** Milestone 4.1 (images downloaded), Milestone 4.2 (media manifest exists to know what was downloaded vs. skipped)
**Produces:** Cleaned HTML with all image references pointing to local files; skipped images removed from DOM

---

## Overview

This is the final HTML transformation before Markdown conversion. After images have been downloaded (4.1) and their metadata recorded (4.2), rewrite all `<img src>` attributes in the cleaned+link-rewritten HTML to point to local relative paths. Remove images that were skipped (matched skip patterns or failed to download) from the DOM entirely rather than leaving broken references.

### Pipeline position

```
raw HTML → clean.html → rewritten.html → media-rewritten.html → (Phase 5: Markdown)
           (3.1)         (3.2 links)      (4.3 images — THIS)
```

The input is `demo/.staging/{slug}.rewritten.html` (output of milestone 3.2).
The output is `demo/.staging/{slug}.final.html` — the fully transformed HTML ready for Markdown conversion.

---

## Step 1: Build a lookup of downloaded images

The media rewriter needs to know, for each `<img src>` in the HTML:
- Was this image downloaded? → rewrite `src` to local path
- Was this image skipped/failed? → remove the `<img>` (and its parent `<figure>` if applicable)

This lookup is built from the `MediaResult` returned by milestone 4.1's `download_article_images()`.

### Add to `tools/src/demo/media.rs`

```rust
use std::collections::HashMap;

/// Build a lookup from original src URLs to their local paths.
///
/// Returns two maps:
/// - `downloaded`: original_src → local relative path (for rewriting)
/// - `skipped`: original_src → skip reason (for removal)
pub fn build_image_lookup(
    result: &MediaResult,
) -> (HashMap<String, String>, HashMap<String, String>) {
    let mut downloaded = HashMap::new();
    let mut skipped_map = HashMap::new();

    for image in &result.images {
        if image.skipped {
            if let Some(ref reason) = image.skip_reason {
                skipped_map.insert(image.original_src.clone(), reason.clone());
            } else {
                skipped_map.insert(image.original_src.clone(), "skipped".to_string());
            }
        } else if let Some(ref local_path) = image.local_path {
            // Local path is relative to demo/media/, e.g. "dzogchen/Diagram.svg"
            // In the final HTML, image references need to be relative to the
            // source page location. Since source pages are in demo/sources/
            // and media is in demo/media/, the relative path is:
            //   ../media/{local_path}
            let relative = format!("../media/{local_path}");
            downloaded.insert(image.original_src.clone(), relative);
        }
    }

    (downloaded, skipped_map)
}
```

---

## Step 2: Implement image source rewriting

### Add to `tools/src/demo/media.rs` (or create a new section)

```rust
use scraper::{Html, Selector};

/// Path where the final (media-rewritten) HTML is written.
pub fn staging_final_path(slug: &str) -> std::path::PathBuf {
    Path::new("demo/.staging").join(format!("{slug}.final.html"))
}

/// Rewrite image sources in HTML and remove skipped images.
///
/// - Downloaded images: `src` rewritten to `../media/{slug}/{filename}`
/// - Skipped/failed images: `<img>` removed; if inside `<figure>`, entire `<figure>` removed
///
/// This is the core function, separated from I/O for testability.
pub fn rewrite_image_sources(
    html: &str,
    downloaded: &HashMap<String, String>,
    skipped: &HashMap<String, String>,
) -> String {
    let mut result = html.to_string();

    // Pass 1: Remove skipped images (including parent <figure> elements)
    // Process removals before rewrites to avoid modifying removed content.
    for (src, _reason) in skipped {
        remove_image_from_html(&mut result, src);
    }

    // Pass 2: Rewrite downloaded image sources
    for (original_src, local_path) in downloaded {
        // Simple string replacement of the src attribute value
        let old_attr = format!("src=\"{original_src}\"");
        let new_attr = format!("src=\"{local_path}\"");
        result = result.replace(&old_attr, &new_attr);

        // Also handle single-quoted attributes (rare but possible)
        let old_attr_sq = format!("src='{original_src}'");
        let new_attr_sq = format!("src='{local_path}'");
        result = result.replace(&old_attr_sq, &new_attr_sq);
    }

    // Pass 3: Clean up empty <figure> elements left after image removal
    remove_empty_figures(&mut result);

    result
}

/// Remove an `<img>` with the given `src` from the HTML.
///
/// If the `<img>` is inside a `<figure>`, removes the entire `<figure>`.
/// Otherwise, removes just the `<img>` tag.
fn remove_image_from_html(html: &mut String, src: &str) {
    // Strategy: find the <img ...src="..."...> tag and remove it.
    // If it's wrapped in <figure>...</figure>, remove the whole figure.

    let src_pattern = format!("src=\"{src}\"");

    // Check if this src exists in the HTML at all
    let Some(src_pos) = html.find(&src_pattern) else {
        return;
    };

    // Walk backwards from src_pos to find the start of the <img or <figure tag
    let before = &html[..src_pos];

    // Check if we're inside a <figure>
    let last_figure_open = before.rfind("<figure");
    let last_figure_close = before.rfind("</figure>");
    let inside_figure = match (last_figure_open, last_figure_close) {
        (Some(open), Some(close)) => open > close, // <figure> opened after last close
        (Some(_), None) => true,                     // <figure> opened, never closed before here
        _ => false,
    };

    if inside_figure {
        // Remove the entire <figure>...</figure> block
        if let Some(fig_start) = last_figure_open {
            if let Some(fig_end_offset) = html[src_pos..].find("</figure>") {
                let fig_end = src_pos + fig_end_offset + "</figure>".len();
                html.replace_range(fig_start..fig_end, "");
                return;
            }
        }
    }

    // Not in a figure — remove just the <img .../> tag
    if let Some(img_start) = before.rfind("<img") {
        // Find the end of this <img> tag
        let after_img_start = &html[img_start..];
        if let Some(img_end_offset) = after_img_start.find('>') {
            let img_end = img_start + img_end_offset + 1;
            html.replace_range(img_start..img_end, "");
        }
    }
}

/// Remove any `<figure>` elements that are now empty (no `<img>` inside).
fn remove_empty_figures(html: &mut String) {
    // Iteratively remove empty figures until no more are found
    loop {
        let document = Html::parse_fragment(html);
        let figure_selector = Selector::parse("figure").unwrap();
        let img_selector = Selector::parse("img").unwrap();

        let mut found_empty = false;

        for figure in document.select(&figure_selector) {
            let has_img = figure.select(&img_selector).next().is_some();
            if !has_img {
                // This figure has no images — get its outer HTML and remove it
                let figure_html = figure.html();
                if let Some(pos) = html.find(&figure_html) {
                    html.replace_range(pos..pos + figure_html.len(), "");
                    found_empty = true;
                    break; // Restart since positions changed
                }
            }
        }

        if !found_empty {
            break;
        }
    }
}

/// Rewrite image sources for a single article.
///
/// Reads `demo/.staging/{slug}.rewritten.html`, applies image rewrites,
/// writes `demo/.staging/{slug}.final.html`.
pub fn rewrite_article_images(
    slug: &str,
    media_result: &MediaResult,
) -> anyhow::Result<std::path::PathBuf> {
    let input_path = super::rewrite::staging_rewritten_path(slug);
    if !input_path.exists() {
        anyhow::bail!(
            "rewritten HTML not found at {}. Run link rewriting first.",
            input_path.display(),
        );
    }

    let html = std::fs::read_to_string(&input_path)?;
    let (downloaded, skipped) = build_image_lookup(media_result);

    let rewritten = rewrite_image_sources(&html, &downloaded, &skipped);

    let output_path = staging_final_path(slug);
    std::fs::write(&output_path, &rewritten)?;

    Ok(output_path)
}
```

---

## Step 3: Update `haleiki demo status` to show final HTML state

### Update `status.rs` `FetchState`

Add `Final` between `Cleaned` and `Converted`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchState {
    Missing,
    Staged,     // Raw HTML exists
    Cleaned,    // Cleaned HTML exists
    Final,      // All HTML transforms done (links + media rewritten)
    Converted,  // Final Markdown exists
}

impl std::fmt::Display for FetchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(f, "missing"),
            Self::Staged => write!(f, "staged"),
            Self::Cleaned => write!(f, "cleaned"),
            Self::Final => write!(f, "ready"),  // "ready" = ready for Markdown conversion
            Self::Converted => write!(f, "converted"),
        }
    }
}

fn fetch_state(slug: &str) -> FetchState {
    let source_path = Path::new("demo/sources").join(format!("{slug}.md"));
    if source_path.exists() {
        return FetchState::Converted;
    }
    let final_path = Path::new("demo/.staging").join(format!("{slug}.final.html"));
    if final_path.exists() {
        return FetchState::Final;
    }
    let clean_path = Path::new("demo/.staging").join(format!("{slug}.clean.html"));
    if clean_path.exists() {
        return FetchState::Cleaned;
    }
    let staging_path = Path::new("demo/.staging").join(format!("{slug}.html"));
    if staging_path.exists() {
        return FetchState::Staged;
    }
    FetchState::Missing
}
```

---

## Step 4: Update `.gitignore`

The `.final.html` files are intermediate artifacts — add them to the existing staging gitignore pattern. The existing `/demo/.staging/*.html` pattern already covers them. No change needed.

---

## Step 5: Write tests

### Unit tests in `media.rs`

```rust
// ─── Image lookup tests ─────────────────────────────

#[test]
fn test_build_image_lookup_separates_downloaded_and_skipped() {
    let result = MediaResult {
        slug: "test".to_string(),
        images_found: 3,
        images_downloaded: 1,
        images_skipped: 1,
        images_failed: 1,
        total_bytes: 5000,
        images: vec![
            ExtractedImage {
                original_src: "//example.com/downloaded.png".to_string(),
                download_url: "https://example.com/downloaded.png".to_string(),
                filename: "downloaded.png".to_string(),
                caption: None,
                skipped: false,
                skip_reason: None,
                is_svg: false,
                local_path: Some("test/downloaded.png".to_string()),
                size_bytes: Some(5000),
            },
            ExtractedImage {
                original_src: "//example.com/skipped.png".to_string(),
                download_url: String::new(),
                filename: "skipped.png".to_string(),
                caption: None,
                skipped: true,
                skip_reason: Some("matched skip pattern".to_string()),
                is_svg: false,
                local_path: None,
                size_bytes: None,
            },
            ExtractedImage {
                original_src: "//example.com/failed.png".to_string(),
                download_url: "https://example.com/failed.png".to_string(),
                filename: "failed.png".to_string(),
                caption: None,
                skipped: true,
                skip_reason: Some("download failed: timeout".to_string()),
                is_svg: false,
                local_path: None,
                size_bytes: None,
            },
        ],
    };

    let (downloaded, skipped) = build_image_lookup(&result);

    assert_eq!(downloaded.len(), 1);
    assert!(downloaded.contains_key("//example.com/downloaded.png"));
    assert_eq!(
        downloaded["//example.com/downloaded.png"],
        "../media/test/downloaded.png",
    );

    assert_eq!(skipped.len(), 2);
    assert!(skipped.contains_key("//example.com/skipped.png"));
    assert!(skipped.contains_key("//example.com/failed.png"));
}

// ─── Image rewriting tests ──────────────────────────

#[test]
fn test_rewrite_image_sources_rewrites_downloaded() {
    let html = r#"<p>Text</p><img src="//cdn.example.com/photo.png" alt="photo" /><p>More</p>"#;

    let mut downloaded = HashMap::new();
    downloaded.insert(
        "//cdn.example.com/photo.png".to_string(),
        "../media/test/photo.png".to_string(),
    );
    let skipped = HashMap::new();

    let result = rewrite_image_sources(html, &downloaded, &skipped);

    assert!(
        result.contains("src=\"../media/test/photo.png\""),
        "Image src not rewritten: {result}",
    );
    assert!(
        !result.contains("cdn.example.com"),
        "Original src still present: {result}",
    );
}

#[test]
fn test_rewrite_image_sources_removes_skipped_img() {
    let html = r#"<p>Before</p><img src="//cdn.example.com/flag.svg" alt="flag" /><p>After</p>"#;

    let downloaded = HashMap::new();
    let mut skipped = HashMap::new();
    skipped.insert(
        "//cdn.example.com/flag.svg".to_string(),
        "matched skip pattern".to_string(),
    );

    let result = rewrite_image_sources(html, &downloaded, &skipped);

    assert!(
        !result.contains("<img"),
        "Skipped <img> should be removed: {result}",
    );
    assert!(result.contains("Before"), "Surrounding content should remain");
    assert!(result.contains("After"), "Surrounding content should remain");
}

#[test]
fn test_rewrite_image_sources_removes_skipped_figure() {
    let html = r#"
        <p>Before</p>
        <figure>
            <img src="//cdn.example.com/flag.svg" alt="flag" />
            <figcaption>A flag</figcaption>
        </figure>
        <p>After</p>
    "#;

    let downloaded = HashMap::new();
    let mut skipped = HashMap::new();
    skipped.insert(
        "//cdn.example.com/flag.svg".to_string(),
        "matched skip pattern".to_string(),
    );

    let result = rewrite_image_sources(html, &downloaded, &skipped);

    assert!(
        !result.contains("<figure>"),
        "Parent <figure> should be removed: {result}",
    );
    assert!(
        !result.contains("<figcaption>"),
        "Figcaption should be removed with figure: {result}",
    );
    assert!(result.contains("Before"));
    assert!(result.contains("After"));
}

#[test]
fn test_rewrite_image_sources_mixed_downloaded_and_skipped() {
    let html = r#"
        <img src="//cdn.example.com/keep.png" alt="keep" />
        <img src="//cdn.example.com/skip.svg" alt="skip" />
        <img src="//cdn.example.com/also-keep.jpg" alt="also keep" />
    "#;

    let mut downloaded = HashMap::new();
    downloaded.insert(
        "//cdn.example.com/keep.png".to_string(),
        "../media/test/keep.png".to_string(),
    );
    downloaded.insert(
        "//cdn.example.com/also-keep.jpg".to_string(),
        "../media/test/also-keep.jpg".to_string(),
    );

    let mut skipped = HashMap::new();
    skipped.insert(
        "//cdn.example.com/skip.svg".to_string(),
        "skipped".to_string(),
    );

    let result = rewrite_image_sources(html, &downloaded, &skipped);

    assert!(result.contains("../media/test/keep.png"), "First image not rewritten");
    assert!(result.contains("../media/test/also-keep.jpg"), "Third image not rewritten");
    assert!(!result.contains("skip.svg"), "Skipped image not removed");
    // Count remaining <img> tags
    let img_count = result.matches("<img").count();
    assert_eq!(img_count, 2, "Should have exactly 2 images remaining, got {img_count}");
}

#[test]
fn test_rewrite_image_sources_no_changes_when_maps_empty() {
    let html = r#"<img src="//cdn.example.com/photo.png" />"#;
    let downloaded = HashMap::new();
    let skipped = HashMap::new();

    let result = rewrite_image_sources(html, &downloaded, &skipped);
    assert_eq!(result, html, "Empty maps should leave HTML unchanged");
}

#[test]
fn test_rewrite_image_sources_preserves_non_image_content() {
    let html = r#"
        <h2>Section</h2>
        <p>Paragraph with <a href="/source/raii/">a link</a>.</p>
        <img src="//cdn.example.com/photo.png" />
        <table><tr><td>Data</td></tr></table>
    "#;

    let mut downloaded = HashMap::new();
    downloaded.insert(
        "//cdn.example.com/photo.png".to_string(),
        "../media/test/photo.png".to_string(),
    );
    let skipped = HashMap::new();

    let result = rewrite_image_sources(html, &downloaded, &skipped);

    assert!(result.contains("<h2>Section</h2>"));
    assert!(result.contains("<a href=\"/source/raii/\">"));
    assert!(result.contains("<table>"));
}

// ─── Staging path tests ─────────────────────────────

#[test]
fn test_staging_final_path() {
    let path = staging_final_path("dzogchen");
    assert_eq!(
        path,
        std::path::PathBuf::from("demo/.staging/dzogchen.final.html"),
    );
}

// ─── Real article test ──────────────────────────────

#[test]
#[ignore] // Requires full pipeline: fetch + clean + rewrite + media download
fn test_rewrite_article_images_real() {
    // This test requires:
    // 1. demo/.staging/dzogchen.rewritten.html (from milestone 3.2)
    // 2. A MediaResult for dzogchen (from milestone 4.1)
    //
    // Run manually after the full pipeline has been executed for at least
    // one article.

    let final_path = staging_final_path("dzogchen");
    if final_path.exists() {
        let html = std::fs::read_to_string(&final_path).unwrap();

        // Should not contain any Wikimedia CDN URLs in img src
        assert!(
            !html.contains("upload.wikimedia.org"),
            "Found unrewritten Wikimedia image URL in final HTML",
        );

        // Should contain local media paths
        // (only if the article actually has images)
        if html.contains("<img") {
            assert!(
                html.contains("../media/"),
                "Images should point to local media paths",
            );
        }
    }
}
```

---

## Verification

### 6.1: Full pipeline test (manual, requires prior stages)

```bash
cd /Users/oubiwann/lab/oxur/haleiki

# After fetch + clean + link-rewrite + media-download for an article:
ls demo/.staging/dzogchen.final.html
# Should exist

# Verify no Wikimedia CDN URLs remain in img src attributes
grep -c 'src="//upload.wikimedia.org' demo/.staging/dzogchen.final.html
# Should be 0

# Verify local paths are present
grep -c 'src="../media/' demo/.staging/dzogchen.final.html
# Should be > 0 (if article has images)

# Verify no broken <figure> elements (empty figures)
grep -c '<figure>' demo/.staging/dzogchen.final.html
grep -c '<img' demo/.staging/dzogchen.final.html
# Every <figure> should contain an <img>
```

### 6.2: Status shows "ready" state

```bash
cargo run --features demo -- demo status
```

Articles that have completed the full HTML pipeline should show `ready`.

### 6.3: Tests pass

```bash
cargo test --features demo
make lint
```

---

## Acceptance Criteria

- [ ] `build_image_lookup()` separates downloaded images (with local paths) from skipped/failed
- [ ] Local paths are relative: `../media/{slug}/{filename}` (correct for source pages in `demo/sources/`)
- [ ] `rewrite_image_sources()` rewrites downloaded image `src` attributes to local paths
- [ ] `rewrite_image_sources()` removes `<img>` tags for skipped/failed images
- [ ] When a skipped `<img>` is inside a `<figure>`, the entire `<figure>` is removed
- [ ] `<figcaption>` is removed along with its parent `<figure>`
- [ ] Empty `<figure>` elements (left after image removal) are cleaned up
- [ ] Non-image content (headings, paragraphs, links, tables) is preserved unchanged
- [ ] `rewrite_article_images()` reads `.rewritten.html` and writes `.final.html`
- [ ] No Wikimedia CDN URLs remain in `<img src>` attributes after rewriting
- [ ] `haleiki demo status` shows `ready` for articles with `.final.html`
- [ ] All unit tests pass (9+ tests)
- [ ] `make lint` passes

---

## Gotchas

1. **Removal before rewriting**: Skipped images must be removed from the HTML BEFORE downloaded images are rewritten. If done in the wrong order, a removal could accidentally match a rewritten path string. The implementation processes removals first, then rewrites.

2. **`<figure>` detection**: The `remove_image_from_html` function walks backwards from the `src` position to find whether the `<img>` is inside a `<figure>`. This is a heuristic based on string position, not a proper DOM walk. It works for well-formed HTML where `<figure>` directly wraps `<img>`, which is the standard Wikipedia pattern. Deeply nested or malformed structures could fool it.

3. **Multiple images in one `<figure>`**: Wikipedia occasionally has figures with multiple images (e.g., side-by-side comparison). If one image is skipped and another is kept, the current logic removes the entire `<figure>`. This is acceptable for the demo — losing a comparison figure is better than showing a broken one.

4. **Relative path correctness**: The local path `../media/{slug}/{filename}` assumes the source page will be at `demo/sources/{slug}.md`. When Cobalt renders it, the HTML page will be at `/source/{slug}/index.html`, and media will be at `/media/{slug}/{filename}`. The relative path `../media/` from `/source/{slug}/` resolves correctly. If the URL structure changes, these paths need updating.

5. **Protocol-relative src matching**: Image sources in Wikipedia HTML are protocol-relative (`//upload.wikimedia.org/...`). The downloaded/skipped maps key on the exact `original_src` string from the DOM. The rewrite uses exact string matching (`src="//upload..."` → `src="../media/..."`). Case sensitivity matters — if the HTML has inconsistent casing, matches could fail.

6. **Self-closing `<img>` tags**: HTML5 allows `<img>` without a closing `/` (`<img src="...">` vs `<img src="..." />`). The removal logic handles both by searching for `>` after the opening `<img`.

7. **Staging file chain**: The full staging chain is now: `.html` → `.clean.html` → `.rewritten.html` → `.final.html`. Each stage is independently inspectable for debugging. Phase 5 reads `.final.html` as its input.

8. **`remove_empty_figures` iteration**: This function uses `scraper` to parse the HTML on each iteration, which is O(n) per empty figure found. For the demo's article sizes (10-100 figures max), this is instant. Don't optimize unless profiling shows otherwise.
