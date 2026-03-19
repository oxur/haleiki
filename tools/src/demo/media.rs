//! Media extraction and download from cleaned article HTML.
//!
//! Walks the DOM to find images, applies skip patterns, selects
//! appropriate resolution, and downloads to `demo/media/{slug}/`.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use globset::{Glob, GlobSet, GlobSetBuilder};
use reqwest_middleware::ClientWithMiddleware;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use super::clean::staging_clean_path;
use super::manifest::{Article, Manifest};

/// Base directory for downloaded media.
const MEDIA_DIR: &str = "demo/media";

/// Path to the media manifest file.
pub const MEDIA_MANIFEST_PATH: &str = "demo/media/manifest.json";

/// The full media manifest written to `demo/media/manifest.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaManifest {
    /// ISO 8601 timestamp when this manifest was generated.
    pub generated_at: String,

    /// Total number of images across all articles.
    pub total_images: usize,

    /// Total size of all downloaded images in bytes.
    pub total_bytes: u64,

    /// Number of articles that have media.
    pub articles_with_media: usize,

    /// Per-image metadata.
    pub images: Vec<MediaEntry>,
}

/// Metadata for a single downloaded image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaEntry {
    /// Local path relative to `demo/media/` (e.g., "dzogchen/Diagram.svg").
    pub local_path: String,

    /// Original URL from the source HTML.
    pub original_url: String,

    /// The URL we actually downloaded from (may differ for thumbnails/SVGs).
    pub download_url: String,

    /// Original filename on Wikimedia Commons (or source wiki).
    pub commons_filename: String,

    /// License for this image.
    /// Inherited from the article's license unless Commons API provides one.
    pub license: String,

    /// Author or attribution string.
    /// Best-effort: from Commons API if available, otherwise generic.
    pub author: String,

    /// Caption text from `<figcaption>` or `alt` attribute.
    pub caption: Option<String>,

    /// The article this image belongs to (by slug).
    pub source_article: String,

    /// Image format ("svg", "png", "jpg", "gif").
    pub format: String,

    /// File size in bytes on disk.
    pub size_bytes: u64,
}

impl MediaEntry {
    /// Construct a `MediaEntry` from a successfully downloaded `ExtractedImage`.
    ///
    /// Requires the article slug and the effective license for the article.
    /// Returns `None` for skipped or failed images (those without a `local_path`
    /// or `size_bytes`).
    pub fn from_extracted(
        image: &ExtractedImage,
        article_slug: &str,
        article_license: &str,
    ) -> Option<Self> {
        // Only create entries for downloaded (non-skipped) images
        let local_path = image.local_path.as_ref()?;
        let size_bytes = image.size_bytes?;

        let format = detect_format(&image.filename);

        Some(Self {
            local_path: local_path.clone(),
            original_url: image.original_src.clone(),
            download_url: image.download_url.clone(),
            commons_filename: image.filename.clone(),
            license: article_license.to_string(),
            author: "Wikimedia Commons contributor".to_string(),
            caption: image.caption.clone(),
            source_article: article_slug.to_string(),
            format,
            size_bytes,
        })
    }
}

/// Detect image format from filename extension.
fn detect_format(filename: &str) -> String {
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "svg" => "svg",
        "png" => "png",
        "jpg" | "jpeg" => "jpg",
        "gif" => "gif",
        "webp" => "webp",
        _ => "unknown",
    }
    .to_string()
}

/// Construct the Wikimedia Commons file page URL from the image source.
///
/// For Wikimedia images, the Commons page is:
///   `https://commons.wikimedia.org/wiki/File:{filename}`
///
/// For non-Wikimedia sources (e.g., Rigpa Wiki), use the original URL.
fn build_commons_url(original_src: &str, filename: &str) -> String {
    if original_src.contains("wikimedia.org") || original_src.contains("wikipedia.org") {
        format!(
            "https://commons.wikimedia.org/wiki/File:{}",
            filename.replace(' ', "_"),
        )
    } else {
        // Non-Wikimedia source — use original URL as-is
        if original_src.starts_with("//") {
            format!("https:{original_src}")
        } else {
            original_src.to_string()
        }
    }
}

/// Write the consolidated media manifest to `demo/media/manifest.json`.
///
/// Collects entries from all `MediaResult`s and writes a single JSON file.
///
/// # Errors
///
/// Returns an error if the manifest cannot be serialized or written to disk.
pub fn write_media_manifest(results: &[MediaResult], manifest: &Manifest) -> anyhow::Result<()> {
    let mut entries = Vec::new();
    let mut articles_with_media = 0;

    for result in results {
        let article = manifest.articles.iter().find(|a| a.slug == result.slug);

        let license = article.map_or(manifest.defaults.license.as_str(), |a| {
            manifest.effective_license(a)
        });

        let article_entries: Vec<MediaEntry> = result
            .images
            .iter()
            .filter_map(|img| MediaEntry::from_extracted(img, &result.slug, license))
            .collect();

        if !article_entries.is_empty() {
            articles_with_media += 1;
        }

        entries.extend(article_entries);
    }

    let total_bytes: u64 = entries.iter().map(|e| e.size_bytes).sum();

    let media_manifest = MediaManifest {
        generated_at: super::fetch::chrono_now_iso8601(),
        total_images: entries.len(),
        total_bytes,
        articles_with_media,
        images: entries,
    };

    let json = serde_json::to_string_pretty(&media_manifest)?;
    std::fs::write(MEDIA_MANIFEST_PATH, json)?;

    eprintln!(
        "Wrote media manifest: {} images from {} articles ({} bytes total)",
        media_manifest.total_images, media_manifest.articles_with_media, media_manifest.total_bytes,
    );

    Ok(())
}

/// Load an existing media manifest from disk.
///
/// Returns `Ok(None)` if the manifest file does not exist.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be read or parsed.
pub fn load_media_manifest() -> anyhow::Result<Option<MediaManifest>> {
    let path = Path::new(MEDIA_MANIFEST_PATH);
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(path)?;
    let manifest: MediaManifest = serde_json::from_str(&content)?;
    Ok(Some(manifest))
}

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

/// Build a `GlobSet` from the manifest's skip patterns (global + per-article).
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
/// - `.../commons/a/ab/Example.png` -> `Example.png`
/// - `.../thumb/a/ab/Example.png/220px-Example.png` -> `Example.png`
/// - `.../thumb/a/ab/Diagram.svg/220px-Diagram.svg.png` -> `Diagram.svg`
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
    path.split('/').next_back().map(ToString::to_string)
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
/// -> `.../commons/a/ab/Diagram.svg`
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
    // .../commons/a/ab/File.png -> .../commons/thumb/a/ab/File.png/1024px-File.png
    if let Some(pos) = url.find("/commons/") {
        let prefix = &url[..pos];
        let path_after_commons = &url[pos + "/commons/".len()..];
        return format!("{prefix}/commons/thumb/{path_after_commons}/{max_width}px-{filename}");
    }

    // Can't parse — return as-is
    url.to_string()
}

/// Extract all images from cleaned HTML for a single article.
///
/// Returns a list of `ExtractedImage` with download URLs resolved
/// and skip patterns applied.
///
/// # Errors
///
/// Returns an error if skip pattern glob compilation fails.
pub fn extract_images(
    html: &str,
    _slug: &str,
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

        let Some(filename) = extract_filename(src) else {
            continue;
        };

        // Determine caption from alt attribute; figcaption matching is done in
        // the second pass below (scraper lacks easy parent traversal).
        let caption = img.value().attr("alt").map(ToString::to_string);

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
            local_path: None, // Set after download
            size_bytes: None, // Set after download
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
        return (true, Some("matched skip pattern".to_string()));
    }

    (false, None)
}

/// Download all non-skipped images for a single article.
///
/// Images are saved to `demo/media/{slug}/{filename}`.
/// Returns a `MediaResult` summarizing what happened.
///
/// # Errors
///
/// Returns an error if the media directory cannot be created.
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
///
/// # Errors
///
/// Returns an error if the cleaned HTML file is missing or cannot be read,
/// or if image extraction or directory creation fails.
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
        result.images_downloaded, result.total_bytes, result.images_failed,
    );

    Ok(result)
}

// ─── Image source rewriting (milestone 4.3) ────────────────────────

/// Build a lookup from original src URLs to their local paths.
///
/// Returns two maps:
/// - `downloaded`: `original_src` -> local relative path (for rewriting)
/// - `skipped`: `original_src` -> skip reason (for removal)
pub fn build_image_lookup(
    result: &MediaResult,
) -> (HashMap<String, String>, HashMap<String, String>) {
    let mut downloaded = HashMap::new();
    let mut skipped_map = HashMap::new();

    for image in &result.images {
        if image.skipped {
            let reason = image
                .skip_reason
                .clone()
                .unwrap_or_else(|| "skipped".to_string());
            skipped_map.insert(image.original_src.clone(), reason);
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
    for src in skipped.keys() {
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
        (Some(open), Some(close)) => open > close,
        (Some(_), None) => true,
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

    // Not in a figure -- remove just the <img .../> tag
    if let Some(img_start) = before.rfind("<img") {
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
        let figure_selector = Selector::parse("figure").expect("valid selector");
        let img_selector = Selector::parse("img").expect("valid selector");

        let mut found_empty = false;

        for figure in document.select(&figure_selector) {
            let has_img = figure.select(&img_selector).next().is_some();
            if !has_img {
                // This figure has no images -- get its outer HTML and remove it
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
///
/// # Errors
///
/// Returns an error if the rewritten HTML file is missing or cannot be read,
/// or if the final HTML file cannot be written.
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

#[cfg(test)]
mod tests {
    use super::*;

    // --- Filename extraction tests ---

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
            extract_filename(
                "https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Example.png/220px-Example.png"
            ),
            Some("Example.png".to_string()),
        );
    }

    #[test]
    fn test_extract_filename_svg_thumbnail() {
        // SVG thumbnails are PNG renderings with .svg.png extension
        assert_eq!(
            extract_filename(
                "https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Diagram.svg/220px-Diagram.svg.png"
            ),
            Some("Diagram.svg".to_string()),
        );
    }

    #[test]
    fn test_extract_filename_with_query_params() {
        assert_eq!(
            extract_filename(
                "https://upload.wikimedia.org/wikipedia/commons/a/ab/Example.png?v=123"
            ),
            Some("Example.png".to_string()),
        );
    }

    // --- SVG detection tests ---

    #[test]
    fn test_is_svg_url_true() {
        assert!(is_svg_url(
            "https://upload.wikimedia.org/commons/a/ab/Diagram.svg"
        ));
    }

    #[test]
    fn test_is_svg_url_thumbnail_of_svg() {
        // The original filename is Diagram.svg even though the thumbnail is .svg.png
        assert!(is_svg_url(
            "https://upload.wikimedia.org/commons/thumb/a/ab/Diagram.svg/220px-Diagram.svg.png"
        ));
    }

    #[test]
    fn test_is_svg_url_false_for_png() {
        assert!(!is_svg_url(
            "https://upload.wikimedia.org/commons/a/ab/Photo.png"
        ));
    }

    // --- SVG original URL resolution ---

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

    // --- Raster thumbnail URL construction ---

    #[test]
    fn test_resolve_raster_thumbnail_from_existing_thumb() {
        let thumb =
            "https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Photo.png/220px-Photo.png";
        let resolved = resolve_raster_thumbnail(thumb, 1024);
        assert!(resolved.contains("1024px-Photo.png"), "Got: {resolved}");
    }

    #[test]
    fn test_resolve_raster_thumbnail_from_original() {
        let original = "https://upload.wikimedia.org/wikipedia/commons/a/ab/Photo.png";
        let resolved = resolve_raster_thumbnail(original, 1024);
        assert!(
            resolved.contains("/thumb/"),
            "Should add /thumb/: {resolved}"
        );
        assert!(
            resolved.contains("1024px-Photo.png"),
            "Should have width prefix: {resolved}"
        );
    }

    // --- Download URL resolution ---

    #[test]
    fn test_resolve_download_url_svg_gets_original() {
        let src =
            "//upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Diagram.svg/220px-Diagram.svg.png";
        let url = resolve_download_url(src, 1024, "en.wikipedia.org");
        assert!(
            url.ends_with(".svg"),
            "SVG should get original, not PNG: {url}"
        );
        assert!(
            !url.contains("/thumb/"),
            "SVG should not use thumb path: {url}"
        );
    }

    #[test]
    fn test_resolve_download_url_raster_gets_thumbnail() {
        let src = "//upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Photo.png/220px-Photo.png";
        let url = resolve_download_url(src, 1024, "en.wikipedia.org");
        assert!(
            url.contains("1024px-"),
            "Should request at max_width: {url}"
        );
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

    // --- Skip pattern tests ---

    #[test]
    fn test_check_skip_matching_pattern() {
        let mut builder = GlobSetBuilder::new();
        builder.add(Glob::new("Flag_of_*").unwrap());
        let patterns = builder.build().unwrap();
        let include = HashSet::new();
        let exclude = HashSet::new();

        let (skipped, reason) = check_skip(
            "Flag_of_USA.svg",
            "//upload.wikimedia.org/commons/a/ab/Flag_of_USA.svg",
            &patterns,
            &include,
            &exclude,
            "en.wikipedia.org",
        );
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

        let (skipped, _) = check_skip(
            "Flag_of_USA.svg",
            "",
            &patterns,
            &include,
            &exclude,
            "en.wikipedia.org",
        );
        assert!(!skipped, "Force-include should override skip pattern");
    }

    #[test]
    fn test_check_skip_force_exclude_always_wins() {
        let patterns = GlobSetBuilder::new().build().unwrap();
        let include = HashSet::new();
        let exclude: HashSet<String> = ["Photo.png".to_string()].into();

        let (skipped, reason) = check_skip(
            "Photo.png",
            "",
            &patterns,
            &include,
            &exclude,
            "en.wikipedia.org",
        );
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

        let (skipped, _) = check_skip(
            "Diagram.svg",
            "//upload.wikimedia.org/Diagram.svg",
            &patterns,
            &include,
            &exclude,
            "en.wikipedia.org",
        );
        assert!(!skipped);
    }

    // --- Image extraction from HTML ---

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

    // --- Format detection tests ---

    #[test]
    fn test_detect_format_svg() {
        assert_eq!(detect_format("Diagram.svg"), "svg");
    }

    #[test]
    fn test_detect_format_png() {
        assert_eq!(detect_format("Photo.png"), "png");
    }

    #[test]
    fn test_detect_format_jpg() {
        assert_eq!(detect_format("Photo.jpg"), "jpg");
    }

    #[test]
    fn test_detect_format_jpeg() {
        assert_eq!(detect_format("Photo.jpeg"), "jpg");
    }

    #[test]
    fn test_detect_format_case_insensitive() {
        assert_eq!(detect_format("PHOTO.PNG"), "png");
    }

    #[test]
    fn test_detect_format_unknown() {
        assert_eq!(detect_format("file.bmp"), "unknown");
    }

    // --- Commons URL construction tests ---

    #[test]
    fn test_build_commons_url_wikimedia_image() {
        let url = build_commons_url(
            "//upload.wikimedia.org/wikipedia/commons/a/ab/Example.png",
            "Example.png",
        );
        assert_eq!(url, "https://commons.wikimedia.org/wiki/File:Example.png");
    }

    #[test]
    fn test_build_commons_url_filename_with_spaces() {
        let url = build_commons_url(
            "//upload.wikimedia.org/wikipedia/commons/a/ab/My Image.png",
            "My Image.png",
        );
        assert_eq!(url, "https://commons.wikimedia.org/wiki/File:My_Image.png");
    }

    #[test]
    fn test_build_commons_url_non_wikimedia() {
        let url = build_commons_url("/images/a/ab/Photo.jpg", "Photo.jpg");
        assert_eq!(url, "/images/a/ab/Photo.jpg");
    }

    #[test]
    fn test_build_commons_url_protocol_relative_non_wikimedia() {
        let url = build_commons_url("//www.rigpawiki.org/images/a/ab/Photo.jpg", "Photo.jpg");
        assert_eq!(url, "https://www.rigpawiki.org/images/a/ab/Photo.jpg");
    }

    // --- MediaEntry construction tests ---

    #[test]
    fn test_media_entry_from_extracted_downloaded() {
        let image = ExtractedImage {
            original_src: "//upload.wikimedia.org/commons/a/ab/Photo.png".to_string(),
            download_url:
                "https://upload.wikimedia.org/commons/thumb/a/ab/Photo.png/1024px-Photo.png"
                    .to_string(),
            filename: "Photo.png".to_string(),
            caption: Some("A photo".to_string()),
            skipped: false,
            skip_reason: None,
            is_svg: false,
            local_path: Some("test-article/Photo.png".to_string()),
            size_bytes: Some(5000),
        };

        let entry = MediaEntry::from_extracted(&image, "test-article", "CC BY-SA 4.0");
        assert!(entry.is_some());
        let entry = entry.unwrap();

        assert_eq!(entry.local_path, "test-article/Photo.png");
        assert_eq!(entry.commons_filename, "Photo.png");
        assert_eq!(entry.license, "CC BY-SA 4.0");
        assert_eq!(entry.source_article, "test-article");
        assert_eq!(entry.format, "png");
        assert_eq!(entry.size_bytes, 5000);
        assert_eq!(entry.caption, Some("A photo".to_string()));
    }

    #[test]
    fn test_media_entry_from_extracted_skipped_returns_none() {
        let image = ExtractedImage {
            original_src: "//upload.wikimedia.org/commons/a/ab/Flag.svg".to_string(),
            download_url: String::new(),
            filename: "Flag.svg".to_string(),
            caption: None,
            skipped: true,
            skip_reason: Some("matched skip pattern".to_string()),
            is_svg: true,
            local_path: None,
            size_bytes: None,
        };

        let entry = MediaEntry::from_extracted(&image, "test", "CC BY-SA 4.0");
        assert!(
            entry.is_none(),
            "Skipped images should not produce MediaEntry"
        );
    }

    #[test]
    fn test_media_entry_from_extracted_no_local_path_returns_none() {
        let image = ExtractedImage {
            original_src: "//example.com/broken.png".to_string(),
            download_url: String::new(),
            filename: "broken.png".to_string(),
            caption: None,
            skipped: false,
            skip_reason: None,
            is_svg: false,
            local_path: None, // Download failed
            size_bytes: None,
        };

        let entry = MediaEntry::from_extracted(&image, "test", "CC BY-SA 4.0");
        assert!(
            entry.is_none(),
            "Images without local_path should not produce MediaEntry"
        );
    }

    // --- Media manifest round-trip test ---

    #[test]
    fn test_media_manifest_serialization_roundtrip() {
        let manifest = MediaManifest {
            generated_at: "2026-03-18T12:00:00Z".to_string(),
            total_images: 1,
            total_bytes: 5000,
            articles_with_media: 1,
            images: vec![MediaEntry {
                local_path: "test/Photo.png".to_string(),
                original_url: "//upload.wikimedia.org/commons/a/ab/Photo.png".to_string(),
                download_url: "https://upload.wikimedia.org/commons/a/ab/Photo.png".to_string(),
                commons_filename: "Photo.png".to_string(),
                license: "CC BY-SA 4.0".to_string(),
                author: "Wikimedia Commons contributor".to_string(),
                caption: Some("A photo".to_string()),
                source_article: "test".to_string(),
                format: "png".to_string(),
                size_bytes: 5000,
            }],
        };

        let json = serde_json::to_string_pretty(&manifest).unwrap();
        let deserialized: MediaManifest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.total_images, 1);
        assert_eq!(deserialized.images[0].local_path, "test/Photo.png");
        assert_eq!(deserialized.images[0].caption, Some("A photo".to_string()));
    }

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
        let html =
            r#"<p>Before</p><img src="//cdn.example.com/flag.svg" alt="flag" /><p>After</p>"#;

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
        assert!(
            result.contains("Before"),
            "Surrounding content should remain"
        );
        assert!(
            result.contains("After"),
            "Surrounding content should remain"
        );
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

        assert!(
            result.contains("../media/test/keep.png"),
            "First image not rewritten",
        );
        assert!(
            result.contains("../media/test/also-keep.jpg"),
            "Third image not rewritten",
        );
        assert!(!result.contains("skip.svg"), "Skipped image not removed");
        // Count remaining <img> tags
        let img_count = result.matches("<img").count();
        assert_eq!(
            img_count, 2,
            "Should have exactly 2 images remaining, got {img_count}",
        );
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
