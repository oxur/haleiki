# Milestone 4.1 — Image Extraction and Download (`media.rs`)

**Version:** 1.0
**Depends on:** Milestone 3.1 (cleaned HTML with images still in DOM)
**Produces:** `demo/media/{slug}/` directories with downloaded images

---

## Overview

Walk the cleaned (but not yet link-rewritten) HTML DOM to find all `<img>` and `<figure>` elements. For each image:

1. Parse the Wikimedia CDN URL to extract the filename
2. Apply skip patterns (global + per-article) via `globset`
3. Select the appropriate resolution variant (thumbnail at `max_width` for raster, original for SVG)
4. Download to `demo/media/{slug}/{filename}`
5. Handle download failures gracefully (warn and continue — don't abort the article)

This milestone handles extraction and download only. Metadata recording (4.2) and HTML `src` rewriting (4.3) follow.

---

## Wikimedia Image URL Anatomy

Understanding the CDN URL structure is essential for selecting the right resolution:

```
Original (full resolution):
https://upload.wikimedia.org/wikipedia/commons/a/ab/Example.svg

Thumbnail (resized):
https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Example.png/1024px-Example.png
                                                                       ^^^^^^^^^^^^^^
                                                                       width prefix

In the HTML, src attributes typically look like:
//upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Example.png/220px-Example.png
^^                                                               ^^^
protocol-relative                                                current thumbnail width
```

### URL patterns to handle

| Pattern | Example | Action |
|---------|---------|--------|
| Thumbnail (raster) | `.../thumb/.../220px-File.png` | Re-request at `max_width` |
| Original (raster) | `.../commons/a/ab/File.png` | Use as-is if ≤ `max_width`, else get thumbnail |
| SVG (any) | `.../File.svg` or `.../thumb/.../220px-File.svg.png` | Download original SVG |
| Data URI | `data:image/...` | Skip |
| External (non-Wikimedia) | `https://example.com/img.png` | Skip (not our content) |
| Rigpa Wiki images | `www.rigpawiki.org/images/...` | Download directly |

### Constructing thumbnail URLs

To get a raster image at `max_width` pixels:
```
Original: https://upload.wikimedia.org/wikipedia/commons/a/ab/Example.png
Thumb:    https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Example.png/{max_width}px-Example.png
```

For SVGs, the thumbnail URL gives a PNG rendering. We want the original SVG:
```
Thumb (PNG rendering): .../thumb/a/ab/Diagram.svg/220px-Diagram.svg.png
Original SVG:          .../commons/a/ab/Diagram.svg
```

---

## Step 1: Create `tools/src/demo/media.rs`

### File: `tools/src/demo/media.rs`

```rust
//! Media extraction and download from cleaned article HTML.
//!
//! Walks the DOM to find images, applies skip patterns, selects
//! appropriate resolution, and downloads to `demo/media/{slug}/`.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use reqwest_middleware::ClientWithMiddleware;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use super::clean::staging_clean_path;
use super::manifest::{Manifest, Article};

/// Base directory for downloaded media.
const MEDIA_DIR: &str = "demo/media";

/// An image found in the article HTML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedImage {
    /// The original `src` URL from the HTML.
    pub original_src: String,

    /// The resolved absolute URL to download from.
    pub download_url: String,

    /// The extracted filename (e.g., "Mark-and-sweep.svg").
    pub filename: String,

    /// Caption text from `<figcaption>` or `alt` attribute.
    pub caption: Option<String>,

    /// Whether this image was skipped (matched a skip pattern).
    pub skipped: bool,

    /// Reason for skipping, if skipped.
    pub skip_reason: Option<String>,

    /// Whether the image is an SVG (download original, not thumbnail).
    pub is_svg: bool,

    /// Local path relative to `demo/media/` after download.
    /// e.g., "dzogchen/Diagram.svg"
    pub local_path: Option<String>,

    /// File size in bytes after download.
    pub size_bytes: Option<u64>,
}

/// Result of extracting and downloading images for a single article.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaResult {
    pub slug: String,
    pub images_found: usize,
    pub images_downloaded: usize,
    pub images_skipped: usize,
    pub images_failed: usize,
    pub total_bytes: u64,
    pub images: Vec<ExtractedImage>,
}

/// Build a GlobSet from the manifest's skip patterns (global + per-article).
fn build_skip_patterns(manifest: &Manifest, article: &Article) -> anyhow::Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();

    // Global skip patterns from defaults
    for pattern in &manifest.defaults.media.skip_patterns {
        builder.add(Glob::new(pattern)?);
    }

    // Per-article skip patterns
    if let Some(ref media) = article.media {
        for pattern in &media.skip_patterns {
            builder.add(Glob::new(pattern)?);
        }
    }

    Ok(builder.build()?)
}

/// Build a set of force-included filenames (override skip patterns).
fn build_include_set(article: &Article) -> HashSet<String> {
    article
        .media
        .as_ref()
        .map(|m| m.include.iter().cloned().collect())
        .unwrap_or_default()
}

/// Build a set of force-excluded filenames (always skip, even if not matched by patterns).
fn build_exclude_set(article: &Article) -> HashSet<String> {
    article
        .media
        .as_ref()
        .map(|m| m.exclude.iter().cloned().collect())
        .unwrap_or_default()
}

/// Extract the filename from a Wikimedia CDN URL.
///
/// Examples:
/// - `.../commons/a/ab/Example.png` → `Example.png`
/// - `.../thumb/a/ab/Example.png/220px-Example.png` → `Example.png`
/// - `.../thumb/a/ab/Diagram.svg/220px-Diagram.svg.png` → `Diagram.svg`
pub fn extract_filename(url: &str) -> Option<String> {
    // For thumbnail URLs: .../thumb/.../NNNpx-Filename.ext
    // The original filename is the path segment before the thumbnail size
    if url.contains("/thumb/") {
        let parts: Vec<&str> = url.split('/').collect();
        // The thumbnail segment is the last part (e.g., "220px-Example.png")
        // The original filename is the second-to-last part
        if parts.len() >= 2 {
            let original_segment = parts[parts.len() - 2];
            return Some(original_segment.to_string());
        }
    }

    // For non-thumbnail URLs, the filename is the last path segment
    let path = url.split('?').next().unwrap_or(url); // Strip query params
    path.split('/').last().map(|s| s.to_string())
}

/// Determine if a URL points to an SVG image.
pub fn is_svg_url(url: &str) -> bool {
    let filename = extract_filename(url).unwrap_or_default();
    filename.to_lowercase().ends_with(".svg")
}

/// Construct the download URL for an image at the desired resolution.
///
/// - SVGs: always download the original (resolution-independent)
/// - Rasters: request thumbnail at `max_width` pixels
pub fn resolve_download_url(original_src: &str, max_width: u32, source_project: &str) -> String {
    let src = if original_src.starts_with("//") {
        format!("https:{original_src}")
    } else if original_src.starts_with('/') {
        format!("https://{source_project}{original_src}")
    } else {
        original_src.to_string()
    };

    // SVGs: get the original, not a PNG rendering
    if is_svg_url(&src) {
        return resolve_svg_original(&src);
    }

    // Rasters: request at max_width
    resolve_raster_thumbnail(&src, max_width)
}

/// For SVGs, strip the thumbnail path to get the original SVG URL.
///
/// `.../thumb/a/ab/Diagram.svg/220px-Diagram.svg.png`
/// → `.../commons/a/ab/Diagram.svg`
fn resolve_svg_original(url: &str) -> String {
    if url.contains("/thumb/") {
        // Remove "/thumb" and the trailing "/NNNpx-..." segment
        let without_thumb = url.replacen("/thumb/", "/", 1);
        // Remove the last path segment (the thumbnail filename)
        if let Some(pos) = without_thumb.rfind('/') {
            return without_thumb[..pos].to_string();
        }
    }
    url.to_string()
}

/// For rasters, construct a thumbnail URL at the desired width.
///
/// If already a thumbnail URL, replace the width.
/// If an original URL, insert the thumb path.
fn resolve_raster_thumbnail(url: &str, max_width: u32) -> String {
    let filename = extract_filename(url).unwrap_or_default();

    if url.contains("/thumb/") {
        // Already a thumbnail — replace the last segment with our width
        if let Some(pos) = url.rfind('/') {
            return format!("{}/{max_width}px-{filename}", &url[..pos]);
        }
    }

    // Original URL — convert to thumbnail URL
    // .../commons/a/ab/File.png → .../commons/thumb/a/ab/File.png/1024px-File.png
    if let Some(pos) = url.find("/commons/") {
        let prefix = &url[..pos];
        let path_after_commons = &url[pos + "/commons/".len()..];
        return format!(
            "{prefix}/commons/thumb/{path_after_commons}/{max_width}px-{filename}"
        );
    }

    // Can't parse — return as-is
    url.to_string()
}

/// Extract all images from cleaned HTML for a single article.
///
/// Returns a list of `ExtractedImage` with download URLs resolved
/// and skip patterns applied.
pub fn extract_images(
    html: &str,
    slug: &str,
    manifest: &Manifest,
    article: &Article,
) -> anyhow::Result<Vec<ExtractedImage>> {
    let document = Html::parse_fragment(html);
    let img_selector = Selector::parse("img[src]").unwrap();
    let figure_selector = Selector::parse("figure").unwrap();
    let figcaption_selector = Selector::parse("figcaption").unwrap();

    let skip_patterns = build_skip_patterns(manifest, article)?;
    let include_set = build_include_set(article);
    let exclude_set = build_exclude_set(article);
    let max_width = manifest.defaults.media.max_width;
    let project = manifest.effective_project(article);

    let mut images = Vec::new();
    let mut seen_urls: HashSet<String> = HashSet::new();

    // Find all <img> elements (both standalone and within <figure>)
    for img in document.select(&img_selector) {
        let Some(src) = img.value().attr("src") else {
            continue;
        };

        // Skip data URIs
        if src.starts_with("data:") {
            continue;
        }

        // Skip already-seen URLs (duplicates in the same article)
        if !seen_urls.insert(src.to_string()) {
            continue;
        }

        let filename = match extract_filename(src) {
            Some(f) => f,
            None => continue,
        };

        // Determine caption: check if img is inside a <figure> with <figcaption>
        let caption = img
            .value()
            .attr("alt")
            .map(|s| s.to_string())
            .or_else(|| {
                // Walk up to find parent <figure> and its <figcaption>
                // scraper doesn't have parent traversal easily, so use alt as fallback
                None
            });

        let is_svg = is_svg_url(src);
        let download_url = resolve_download_url(src, max_width, project);

        // Apply skip logic
        let (skipped, skip_reason) = check_skip(
            &filename,
            src,
            &skip_patterns,
            &include_set,
            &exclude_set,
            project,
        );

        images.push(ExtractedImage {
            original_src: src.to_string(),
            download_url,
            filename,
            caption,
            skipped,
            skip_reason,
            is_svg,
            local_path: None,  // Set after download
            size_bytes: None,  // Set after download
        });
    }

    // Second pass: try to attach captions from <figure>/<figcaption> pairs
    for figure in document.select(&figure_selector) {
        if let Some(figcaption) = figure.select(&figcaption_selector).next() {
            let caption_text = figcaption.text().collect::<String>().trim().to_string();
            if caption_text.is_empty() {
                continue;
            }

            // Find the <img> inside this <figure>
            if let Some(img) = figure.select(&img_selector).next() {
                if let Some(src) = img.value().attr("src") {
                    // Update the caption for the matching image
                    for image in &mut images {
                        if image.original_src == src && image.caption.is_none() {
                            image.caption = Some(caption_text.clone());
                        }
                    }
                }
            }
        }
    }

    Ok(images)
}

/// Check whether an image should be skipped.
///
/// Returns (skipped, reason).
fn check_skip(
    filename: &str,
    src: &str,
    skip_patterns: &GlobSet,
    include_set: &HashSet<String>,
    exclude_set: &HashSet<String>,
    source_project: &str,
) -> (bool, Option<String>) {
    // Force-exclude always wins
    if exclude_set.contains(filename) {
        return (true, Some("force-excluded".to_string()));
    }

    // Force-include overrides skip patterns
    if include_set.contains(filename) {
        return (false, None);
    }

    // Skip non-Wikimedia images (external sites we don't have rights to)
    if !src.contains("upload.wikimedia.org")
        && !src.contains("wikimedia.org")
        && !src.contains(source_project)
        && !src.starts_with("//")
        && !src.starts_with('/')
    {
        return (true, Some("external image (non-Wikimedia)".to_string()));
    }

    // Apply glob skip patterns
    if skip_patterns.is_match(filename) {
        return (true, Some(format!("matched skip pattern")));
    }

    (false, None)
}

/// Download all non-skipped images for a single article.
///
/// Images are saved to `demo/media/{slug}/{filename}`.
/// Returns a `MediaResult` summarizing what happened.
pub async fn download_article_images(
    client: &ClientWithMiddleware,
    slug: &str,
    images: &mut [ExtractedImage],
) -> anyhow::Result<MediaResult> {
    let article_media_dir = Path::new(MEDIA_DIR).join(slug);
    std::fs::create_dir_all(&article_media_dir)?;

    let mut downloaded = 0;
    let mut skipped = 0;
    let mut failed = 0;
    let mut total_bytes: u64 = 0;

    for image in images.iter_mut() {
        if image.skipped {
            skipped += 1;
            continue;
        }

        let dest_path = article_media_dir.join(&image.filename);
        let local_rel = format!("{slug}/{}", image.filename);

        match download_single_image(client, &image.download_url, &dest_path).await {
            Ok(bytes) => {
                image.local_path = Some(local_rel);
                image.size_bytes = Some(bytes);
                total_bytes += bytes;
                downloaded += 1;
            }
            Err(e) => {
                eprintln!(
                    "  Warning: failed to download {} for {slug}: {e}",
                    image.filename,
                );
                failed += 1;
                // Mark as skipped so downstream stages don't reference it
                image.skipped = true;
                image.skip_reason = Some(format!("download failed: {e}"));
            }
        }
    }

    Ok(MediaResult {
        slug: slug.to_string(),
        images_found: images.len(),
        images_downloaded: downloaded,
        images_skipped: skipped,
        images_failed: failed,
        total_bytes,
        images: images.to_vec(),
    })
}

/// Download a single image to disk.
///
/// Returns the file size in bytes on success.
async fn download_single_image(
    client: &ClientWithMiddleware,
    url: &str,
    dest: &Path,
) -> anyhow::Result<u64> {
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP {} downloading {url}", response.status());
    }

    let bytes = response.bytes().await?;
    let size = bytes.len() as u64;

    std::fs::write(dest, &bytes)?;

    Ok(size)
}

/// Convenience: extract and download images for a single article.
///
/// Reads the cleaned HTML, extracts images, downloads them.
pub async fn process_article_media(
    client: &ClientWithMiddleware,
    slug: &str,
    manifest: &Manifest,
    article: &Article,
) -> anyhow::Result<MediaResult> {
    let clean_path = staging_clean_path(slug);
    if !clean_path.exists() {
        anyhow::bail!(
            "cleaned HTML not found at {}. Run cleaning first.",
            clean_path.display(),
        );
    }

    let html = std::fs::read_to_string(&clean_path)?;
    let mut images = extract_images(&html, slug, manifest, article)?;

    eprintln!(
        "  {slug}: found {} images ({} skipped by patterns)",
        images.len(),
        images.iter().filter(|i| i.skipped).count(),
    );

    let result = download_article_images(client, slug, &mut images).await?;

    eprintln!(
        "  {slug}: downloaded {} images ({} bytes), {} failed",
        result.images_downloaded,
        result.total_bytes,
        result.images_failed,
    );

    Ok(result)
}
```

---

## Step 2: Wire into `demo/mod.rs`

### Update module declarations

```rust
pub mod clean;
pub mod fetch;
pub mod manifest;
pub mod media;
pub mod rewrite;
pub mod status;
```

### Add media processing to the fetch pipeline (optional at this stage)

Media processing can be invoked standalone or as part of the fetch pipeline. For now, add it as a standalone step that can be called after fetch + clean. The full pipeline wiring happens when Phase 5 ties everything together.

Optionally add a hidden dev command:

```rust
/// [Dev] Download media for a single article (development tool)
#[command(hide = true)]
DownloadMedia {
    /// Article slug
    slug: String,
},
```

---

## Step 3: Handle Rigpa Wiki images

Rigpa Wiki images use a different URL pattern than Wikimedia:

```
Wikimedia: //upload.wikimedia.org/wikipedia/commons/...
Rigpa Wiki: /images/thumb/a/ab/File.jpg/220px-File.jpg
            or /images/a/ab/File.jpg
```

The `resolve_download_url` function needs to handle both. For Rigpa Wiki:
- Relative URLs (`/images/...`) should be resolved against `https://www.rigpawiki.org/`
- The `/images/thumb/` pattern follows the same structure as Wikimedia thumbnails

The existing code already handles this via the `src.starts_with('/')` branch in `resolve_download_url`, which prepends `https://{source_project}`.

---

## Step 4: Write tests

### Unit tests in `tools/src/demo/media.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ─── Filename extraction tests ──────────────────────

    #[test]
    fn test_extract_filename_original_url() {
        assert_eq!(
            extract_filename("https://upload.wikimedia.org/wikipedia/commons/a/ab/Example.png"),
            Some("Example.png".to_string()),
        );
    }

    #[test]
    fn test_extract_filename_thumbnail_url() {
        assert_eq!(
            extract_filename("https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Example.png/220px-Example.png"),
            Some("Example.png".to_string()),
        );
    }

    #[test]
    fn test_extract_filename_svg_thumbnail() {
        // SVG thumbnails are PNG renderings with .svg.png extension
        assert_eq!(
            extract_filename("https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Diagram.svg/220px-Diagram.svg.png"),
            Some("Diagram.svg".to_string()),
        );
    }

    #[test]
    fn test_extract_filename_with_query_params() {
        assert_eq!(
            extract_filename("https://upload.wikimedia.org/wikipedia/commons/a/ab/Example.png?v=123"),
            Some("Example.png".to_string()),
        );
    }

    // ─── SVG detection tests ────────────────────────────

    #[test]
    fn test_is_svg_url_true() {
        assert!(is_svg_url("https://upload.wikimedia.org/commons/a/ab/Diagram.svg"));
    }

    #[test]
    fn test_is_svg_url_thumbnail_of_svg() {
        // The original filename is Diagram.svg even though the thumbnail is .svg.png
        assert!(is_svg_url("https://upload.wikimedia.org/commons/thumb/a/ab/Diagram.svg/220px-Diagram.svg.png"));
    }

    #[test]
    fn test_is_svg_url_false_for_png() {
        assert!(!is_svg_url("https://upload.wikimedia.org/commons/a/ab/Photo.png"));
    }

    // ─── SVG original URL resolution ────────────────────

    #[test]
    fn test_resolve_svg_original_from_thumbnail() {
        let thumb = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Diagram.svg/220px-Diagram.svg.png";
        let original = resolve_svg_original(thumb);
        assert_eq!(
            original,
            "https://upload.wikimedia.org/wikipedia/commons/a/ab/Diagram.svg",
        );
    }

    #[test]
    fn test_resolve_svg_original_already_original() {
        let url = "https://upload.wikimedia.org/wikipedia/commons/a/ab/Diagram.svg";
        let resolved = resolve_svg_original(url);
        assert_eq!(resolved, url);
    }

    // ─── Raster thumbnail URL construction ──────────────

    #[test]
    fn test_resolve_raster_thumbnail_from_existing_thumb() {
        let thumb = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Photo.png/220px-Photo.png";
        let resolved = resolve_raster_thumbnail(thumb, 1024);
        assert!(resolved.contains("1024px-Photo.png"), "Got: {resolved}");
    }

    #[test]
    fn test_resolve_raster_thumbnail_from_original() {
        let original = "https://upload.wikimedia.org/wikipedia/commons/a/ab/Photo.png";
        let resolved = resolve_raster_thumbnail(original, 1024);
        assert!(resolved.contains("/thumb/"), "Should add /thumb/: {resolved}");
        assert!(resolved.contains("1024px-Photo.png"), "Should have width prefix: {resolved}");
    }

    // ─── Download URL resolution ────────────────────────

    #[test]
    fn test_resolve_download_url_svg_gets_original() {
        let src = "//upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Diagram.svg/220px-Diagram.svg.png";
        let url = resolve_download_url(src, 1024, "en.wikipedia.org");
        assert!(url.ends_with(".svg"), "SVG should get original, not PNG: {url}");
        assert!(!url.contains("/thumb/"), "SVG should not use thumb path: {url}");
    }

    #[test]
    fn test_resolve_download_url_raster_gets_thumbnail() {
        let src = "//upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Photo.png/220px-Photo.png";
        let url = resolve_download_url(src, 1024, "en.wikipedia.org");
        assert!(url.contains("1024px-"), "Should request at max_width: {url}");
    }

    #[test]
    fn test_resolve_download_url_protocol_relative() {
        let src = "//upload.wikimedia.org/wikipedia/commons/a/ab/Photo.png";
        let url = resolve_download_url(src, 1024, "en.wikipedia.org");
        assert!(url.starts_with("https://"), "Should add https: {url}");
    }

    #[test]
    fn test_resolve_download_url_relative_path() {
        let src = "/images/a/ab/Photo.jpg";
        let url = resolve_download_url(src, 1024, "www.rigpawiki.org");
        assert_eq!(url, "https://www.rigpawiki.org/images/a/ab/Photo.jpg");
    }

    // ─── Skip pattern tests ─────────────────────────────

    #[test]
    fn test_check_skip_matching_pattern() {
        let mut builder = GlobSetBuilder::new();
        builder.add(Glob::new("Flag_of_*").unwrap());
        let patterns = builder.build().unwrap();
        let include = HashSet::new();
        let exclude = HashSet::new();

        let (skipped, reason) = check_skip("Flag_of_USA.svg", "", &patterns, &include, &exclude, "en.wikipedia.org");
        assert!(skipped);
        assert!(reason.unwrap().contains("skip pattern"));
    }

    #[test]
    fn test_check_skip_force_include_overrides_pattern() {
        let mut builder = GlobSetBuilder::new();
        builder.add(Glob::new("Flag_of_*").unwrap());
        let patterns = builder.build().unwrap();
        let include: HashSet<String> = ["Flag_of_USA.svg".to_string()].into();
        let exclude = HashSet::new();

        let (skipped, _) = check_skip("Flag_of_USA.svg", "", &patterns, &include, &exclude, "en.wikipedia.org");
        assert!(!skipped, "Force-include should override skip pattern");
    }

    #[test]
    fn test_check_skip_force_exclude_always_wins() {
        let patterns = GlobSetBuilder::new().build().unwrap();
        let include = HashSet::new();
        let exclude: HashSet<String> = ["Photo.png".to_string()].into();

        let (skipped, reason) = check_skip("Photo.png", "", &patterns, &include, &exclude, "en.wikipedia.org");
        assert!(skipped);
        assert!(reason.unwrap().contains("force-excluded"));
    }

    #[test]
    fn test_check_skip_no_match_not_skipped() {
        let mut builder = GlobSetBuilder::new();
        builder.add(Glob::new("Flag_of_*").unwrap());
        let patterns = builder.build().unwrap();
        let include = HashSet::new();
        let exclude = HashSet::new();

        let (skipped, _) = check_skip("Diagram.svg", "//upload.wikimedia.org/Diagram.svg", &patterns, &include, &exclude, "en.wikipedia.org");
        assert!(!skipped);
    }

    // ─── Image extraction from HTML ─────────────────────

    #[test]
    fn test_extract_images_finds_img_elements() {
        let html = r#"
            <p>Text</p>
            <img src="//upload.wikimedia.org/wikipedia/commons/a/ab/Photo.png" alt="A photo" />
            <figure>
                <img src="//upload.wikimedia.org/wikipedia/commons/b/bc/Diagram.svg" />
                <figcaption>A diagram</figcaption>
            </figure>
        "#;

        let manifest = test_manifest();
        let article = &manifest.articles[0];
        let images = extract_images(html, "test", &manifest, article).unwrap();

        assert_eq!(images.len(), 2);
        assert_eq!(images[0].filename, "Photo.png");
        assert_eq!(images[1].filename, "Diagram.svg");
        assert!(images[1].is_svg);
    }

    #[test]
    fn test_extract_images_skips_data_uris() {
        let html = r#"<img src="data:image/png;base64,abc123" />"#;
        let manifest = test_manifest();
        let article = &manifest.articles[0];
        let images = extract_images(html, "test", &manifest, article).unwrap();
        assert!(images.is_empty());
    }

    #[test]
    fn test_extract_images_deduplicates() {
        let html = r#"
            <img src="//upload.wikimedia.org/commons/a/ab/Same.png" />
            <img src="//upload.wikimedia.org/commons/a/ab/Same.png" />
        "#;
        let manifest = test_manifest();
        let article = &manifest.articles[0];
        let images = extract_images(html, "test", &manifest, article).unwrap();
        assert_eq!(images.len(), 1, "Duplicate images should be deduplicated");
    }

    #[test]
    fn test_extract_images_applies_skip_patterns() {
        let html = r#"
            <img src="//upload.wikimedia.org/commons/a/ab/Flag_of_USA.svg" />
            <img src="//upload.wikimedia.org/commons/b/bc/Diagram.png" />
        "#;
        let manifest = test_manifest(); // Has "Flag_of_*" skip pattern
        let article = &manifest.articles[0];
        let images = extract_images(html, "test", &manifest, article).unwrap();

        assert_eq!(images.len(), 2);
        assert!(images[0].skipped, "Flag image should be skipped");
        assert!(!images[1].skipped, "Diagram should not be skipped");
    }

    #[test]
    fn test_extract_images_captures_figcaption() {
        let html = r#"
            <figure>
                <img src="//upload.wikimedia.org/commons/a/ab/Photo.png" />
                <figcaption>A beautiful photo</figcaption>
            </figure>
        "#;
        let manifest = test_manifest();
        let article = &manifest.articles[0];
        let images = extract_images(html, "test", &manifest, article).unwrap();

        assert_eq!(images.len(), 1);
        assert_eq!(images[0].caption, Some("A beautiful photo".to_string()));
    }

    /// Minimal manifest for testing.
    fn test_manifest() -> Manifest {
        Manifest {
            defaults: super::super::manifest::Defaults {
                project: "en.wikipedia.org".to_string(),
                license: "CC BY-SA 4.0".to_string(),
                media: super::super::manifest::MediaDefaults {
                    max_width: 1024,
                    formats: vec!["svg".to_string(), "png".to_string()],
                    skip_patterns: vec!["Flag_of_*".to_string()],
                },
            },
            taxonomy: super::super::manifest::Taxonomy {
                categories: vec!["test".to_string()],
                tiers: vec!["foundational".to_string()],
            },
            articles: vec![super::super::manifest::Article {
                title: "Test".to_string(),
                slug: "test".to_string(),
                category: "test".to_string(),
                subcategory: None,
                tier: "foundational".to_string(),
                project: None,
                license: None,
                tags: vec![],
                keywords: vec![],
                media: None,
            }],
        }
    }
}
```

---

## Verification

### 5.1: Extract images from a real article (requires prior fetch + clean)

```bash
# Run the ignored integration test
cargo test --features demo -- test_process_article_media --ignored
```

### 5.2: Manual inspection

```bash
ls demo/media/dzogchen/
# Should show downloaded images

ls demo/media/quantum-mechanics/
# Should show more images (physics articles tend to have diagrams)
```

### 5.3: Skip patterns working

No `Flag_of_*`, `Wiki-*.svg`, `Commons-logo*`, or `Ambox_*` files should appear in `demo/media/`.

### 5.4: SVGs downloaded as originals

```bash
file demo/media/*//*.svg
# Should show SVG XML, not PNG data
```

### 5.5: Tests pass

```bash
cargo test --features demo
make lint
```

---

## Acceptance Criteria

- [ ] `tools/src/demo/media.rs` implements image extraction and download
- [ ] `extract_filename()` correctly parses Wikimedia CDN URL patterns
- [ ] `is_svg_url()` correctly identifies SVGs (including thumbnail-of-SVG)
- [ ] `resolve_download_url()` returns original for SVGs, thumbnail at `max_width` for rasters
- [ ] `extract_images()` finds all `<img>` elements in cleaned HTML
- [ ] `extract_images()` deduplicates same-URL images
- [ ] `extract_images()` captures captions from `<figcaption>` and `alt`
- [ ] Skip patterns (global + per-article) correctly filter images
- [ ] Force-include overrides skip patterns
- [ ] Force-exclude always skips
- [ ] Data URIs are skipped
- [ ] Download failures warn and continue (don't abort article)
- [ ] Images saved to `demo/media/{slug}/{filename}`
- [ ] Rigpa Wiki images download correctly via relative URL resolution
- [ ] All unit tests pass (18+ tests)
- [ ] `make lint` passes

---

## Gotchas

1. **Wikimedia thumbnail URL structure**: The `/thumb/` URL has the original filename as the *second-to-last* path segment, not the last. The last segment is `{width}px-{filename}`. Getting this wrong means extracting the wrong filename.

2. **SVG thumbnail trap**: Wikimedia serves PNG renderings of SVGs at thumbnail URLs (`.svg.png` extension). We want the original `.svg`, not the PNG rendering. The `resolve_svg_original` function strips the thumbnail path to get the real SVG URL.

3. **Filename collisions**: Different images could theoretically have the same filename (from different Commons paths). Within a single article this is unlikely, but if it happens the second download overwrites the first. The `seen_urls` deduplication prevents this for same-URL duplicates, but not for different-URL/same-filename cases.

4. **Download size**: Some Wikipedia articles have many large images. The 90-article demo could total several hundred MB of media. The `max_width: 1024` setting helps by requesting smaller thumbnails. SVGs are typically small (10-100KB).

5. **Rate limiting on media downloads**: Wikimedia CDN is more lenient than the API, but downloading hundreds of images rapidly could trigger rate limits. The retry middleware from 2.1 handles transient failures. Consider adding a small delay between downloads if issues arise.

6. **`scraper` parent traversal**: `scraper` doesn't have a straightforward `.parent()` method for navigating up the DOM tree. The figcaption extraction uses a two-pass approach: first collect all images, then iterate `<figure>` elements and match their `<img>` children to update captions.

7. **Rigpa Wiki image paths**: Rigpa Wiki uses `/images/` instead of Wikimedia's `/wikipedia/commons/`. The URL resolution handles this via the relative-path branch, but the thumbnail construction logic (`/thumb/` path manipulation) may not work for non-Wikimedia sites. Test with real Rigpa Wiki articles.

8. **Empty media directories**: Articles with no images (or all images skipped) will have an empty `demo/media/{slug}/` directory. This is fine — the directory still gets created by `create_dir_all`.
