# Milestone 4.2 — Media Metadata and Manifest Generation

**Version:** 1.0
**Depends on:** Milestone 4.1 (images extracted and downloaded)
**Produces:** `demo/media/manifest.json` with full attribution data; `haleiki demo status` shows media statistics

---

## Overview

After images are downloaded, capture metadata for each one and write a consolidated `demo/media/manifest.json`. This manifest powers:

1. The automated attribution page (milestone 7.1)
2. The `haleiki demo status` media summary
3. The `haleiki demo validate` media integrity checks (milestone 6.2)

For each downloaded image, record: original Commons URL, filename, license (inherited from article or fetched from Commons API), author/attribution, caption text, local path, format, and file size.

---

## Step 1: Define the media manifest types

### Add to `tools/src/demo/media.rs`

```rust
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

/// Path to the media manifest file.
pub const MEDIA_MANIFEST_PATH: &str = "demo/media/manifest.json";
```

---

## Step 2: Build `MediaEntry` from `ExtractedImage`

### Add to `media.rs`

```rust
impl MediaEntry {
    /// Construct a `MediaEntry` from a successfully downloaded `ExtractedImage`.
    ///
    /// Requires the article slug and the effective license for the article.
    pub fn from_extracted(
        image: &ExtractedImage,
        article_slug: &str,
        article_license: &str,
    ) -> Option<Self> {
        // Only create entries for downloaded (non-skipped) images
        let local_path = image.local_path.as_ref()?;
        let size_bytes = image.size_bytes?;

        let format = detect_format(&image.filename);
        let commons_url = build_commons_url(&image.original_src, &image.filename);

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
    let lower = filename.to_lowercase();
    if lower.ends_with(".svg") {
        "svg".to_string()
    } else if lower.ends_with(".png") {
        "png".to_string()
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "jpg".to_string()
    } else if lower.ends_with(".gif") {
        "gif".to_string()
    } else if lower.ends_with(".webp") {
        "webp".to_string()
    } else {
        "unknown".to_string()
    }
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
```

---

## Step 3: Write the media manifest

### Add to `media.rs`

```rust
use std::path::Path;

/// Write the consolidated media manifest to `demo/media/manifest.json`.
///
/// Collects entries from all `MediaResult`s and writes a single JSON file.
pub fn write_media_manifest(results: &[MediaResult], manifest: &Manifest) -> anyhow::Result<()> {
    let mut entries = Vec::new();
    let mut articles_with_media = 0;

    for result in results {
        let article = manifest
            .articles
            .iter()
            .find(|a| a.slug == result.slug);

        let license = article
            .map(|a| manifest.effective_license(a))
            .unwrap_or(&manifest.defaults.license);

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
        media_manifest.total_images,
        media_manifest.articles_with_media,
        media_manifest.total_bytes,
    );

    Ok(())
}

/// Load an existing media manifest from disk.
pub fn load_media_manifest() -> anyhow::Result<Option<MediaManifest>> {
    let path = Path::new(MEDIA_MANIFEST_PATH);
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(path)?;
    let manifest: MediaManifest = serde_json::from_str(&content)?;
    Ok(Some(manifest))
}
```

### Note on `chrono_now_iso8601`

The timestamp function was defined in `fetch.rs` as a private function. It needs to be made `pub(crate)` (or moved to a shared utility) so `media.rs` can use it. Update in `fetch.rs`:

```rust
/// ISO 8601 timestamp for "now".
pub(crate) fn chrono_now_iso8601() -> String {
```

---

## Step 4: Update `haleiki demo status` with media statistics

### Update `tools/src/demo/status.rs`

After the article table, add a media summary section:

```rust
use super::media;

/// Print media statistics if the media manifest exists.
fn print_media_summary() {
    let manifest = match media::load_media_manifest() {
        Ok(Some(m)) => m,
        Ok(None) => {
            println!("  Media: no manifest (run fetch + media pipeline first)");
            return;
        }
        Err(e) => {
            eprintln!("  Media manifest error: {e}");
            return;
        }
    };

    println!("  Media:");
    println!(
        "    Total: {} images across {} articles",
        manifest.total_images, manifest.articles_with_media,
    );
    println!(
        "    Size: {} ({})",
        format_bytes(manifest.total_bytes),
        manifest.total_bytes,
    );

    // Per-format breakdown
    let mut by_format: std::collections::HashMap<&str, (usize, u64)> = std::collections::HashMap::new();
    for entry in &manifest.images {
        let (count, bytes) = by_format.entry(&entry.format).or_insert((0, 0));
        *count += 1;
        *bytes += entry.size_bytes;
    }
    let mut formats: Vec<_> = by_format.into_iter().collect();
    formats.sort_by_key(|(_, (count, _))| std::cmp::Reverse(*count));
    for (format, (count, bytes)) in &formats {
        println!("      {format}: {count} files ({})", format_bytes(*bytes));
    }

    // Top 5 articles by media count
    let mut by_article: std::collections::HashMap<&str, (usize, u64)> = std::collections::HashMap::new();
    for entry in &manifest.images {
        let (count, bytes) = by_article.entry(&entry.source_article).or_insert((0, 0));
        *count += 1;
        *bytes += entry.size_bytes;
    }
    let mut articles: Vec<_> = by_article.into_iter().collect();
    articles.sort_by_key(|(_, (count, _))| std::cmp::Reverse(*count));

    if !articles.is_empty() {
        println!("    Top articles by image count:");
        for (slug, (count, bytes)) in articles.iter().take(5) {
            println!("      {slug}: {count} images ({})", format_bytes(*bytes));
        }
    }

    println!();
}

/// Format a byte count as a human-readable string.
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
```

Wire `print_media_summary()` into the `run()` function, called after the article table:

```rust
pub fn run() -> anyhow::Result<()> {
    // ... existing article table code ...

    print_media_summary();

    Ok(())
}
```

---

## Step 5: Write tests

### Unit tests in `media.rs`

```rust
// ─── Format detection tests ────────────────────────

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

// ─── Commons URL construction tests ─────────────────

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
    let url = build_commons_url(
        "/images/a/ab/Photo.jpg",
        "Photo.jpg",
    );
    assert_eq!(url, "/images/a/ab/Photo.jpg");
}

#[test]
fn test_build_commons_url_protocol_relative_non_wikimedia() {
    let url = build_commons_url(
        "//www.rigpawiki.org/images/a/ab/Photo.jpg",
        "Photo.jpg",
    );
    assert_eq!(url, "https://www.rigpawiki.org/images/a/ab/Photo.jpg");
}

// ─── MediaEntry construction tests ──────────────────

#[test]
fn test_media_entry_from_extracted_downloaded() {
    let image = ExtractedImage {
        original_src: "//upload.wikimedia.org/commons/a/ab/Photo.png".to_string(),
        download_url: "https://upload.wikimedia.org/commons/thumb/a/ab/Photo.png/1024px-Photo.png".to_string(),
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
    assert!(entry.is_none(), "Skipped images should not produce MediaEntry");
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
    assert!(entry.is_none(), "Images without local_path should not produce MediaEntry");
}

// ─── Media manifest round-trip test ─────────────────

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

// ─── format_bytes tests ─────────────────────────────

#[test]
fn test_format_bytes_bytes() {
    assert_eq!(format_bytes(512), "512 B");
}

#[test]
fn test_format_bytes_kilobytes() {
    assert_eq!(format_bytes(1536), "1.5 KB");
}

#[test]
fn test_format_bytes_megabytes() {
    assert_eq!(format_bytes(5_242_880), "5.0 MB");
}
```

### Integration test

```rust
#[test]
#[cfg(feature = "demo")]
fn test_demo_status_shows_media_section() {
    // This test only shows the "no manifest" message unless media has been downloaded
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "status"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(predicate::str::contains("Media:"));
}
```

---

## Step 6: JSON output format

The generated `demo/media/manifest.json` should match this structure (from design doc section 5):

```json
{
  "generated_at": "2026-03-18T14:30:00Z",
  "total_images": 47,
  "total_bytes": 3245000,
  "articles_with_media": 32,
  "images": [
    {
      "local_path": "dzogchen/Dzogchen_meditation.svg",
      "original_url": "//upload.wikimedia.org/wikipedia/commons/a/ab/Dzogchen_meditation.svg",
      "download_url": "https://upload.wikimedia.org/wikipedia/commons/a/ab/Dzogchen_meditation.svg",
      "commons_filename": "Dzogchen_meditation.svg",
      "license": "CC BY-SA 4.0",
      "author": "Wikimedia Commons contributor",
      "caption": "Illustration of Dzogchen meditation posture",
      "source_article": "dzogchen",
      "format": "svg",
      "size_bytes": 14320
    }
  ]
}
```

---

## Verification

### 6.1: Generate manifest after downloading media (requires prior pipeline)

```bash
# After running fetch + clean + media download for at least one article:
cargo test --features demo -- test_media_manifest_serialization_roundtrip
```

### 6.2: Status shows media summary

```bash
cargo run --features demo -- demo status
```

Expected output (after media pipeline has run):

```
  ...article table...

  Media:
    Total: 47 images across 32 articles
    Size: 3.1 MB (3245000)
      svg: 12 files (245.3 KB)
      png: 28 files (2.6 MB)
      jpg: 7 files (312.0 KB)
    Top articles by image count:
      quantum-mechanics: 8 images (890.2 KB)
      general-relativity: 6 images (543.1 KB)
      padmasambhava: 5 images (412.0 KB)
      johann-sebastian-bach: 4 images (234.5 KB)
      string-theory: 4 images (198.7 KB)
```

### 6.3: Manifest is valid JSON

```bash
python3 -c "import json; json.load(open('demo/media/manifest.json'))" && echo "OK"
```

### 6.4: Tests pass

```bash
cargo test --features demo
make lint
```

---

## Acceptance Criteria

- [ ] `MediaManifest` and `MediaEntry` types defined with serde derive
- [ ] `MediaEntry::from_extracted()` converts downloaded images to manifest entries
- [ ] Skipped and failed images do not appear in the manifest
- [ ] `detect_format()` correctly identifies svg, png, jpg, gif, webp
- [ ] `build_commons_url()` constructs correct Commons file page URLs
- [ ] `build_commons_url()` handles non-Wikimedia sources
- [ ] `write_media_manifest()` writes valid JSON to `demo/media/manifest.json`
- [ ] `load_media_manifest()` reads it back
- [ ] Manifest includes summary stats: total images, total bytes, articles with media
- [ ] License inherited from article (via `manifest.effective_license()`)
- [ ] Author defaults to "Wikimedia Commons contributor" (Commons API integration deferred)
- [ ] `haleiki demo status` shows media summary section
- [ ] Status shows per-format breakdown and top articles by image count
- [ ] `chrono_now_iso8601()` made `pub(crate)` for reuse
- [ ] All unit tests pass (15+ tests)
- [ ] `make lint` passes

---

## Gotchas

1. **Commons API for author/license**: The design doc mentions fetching license and author from the Commons API. This is expensive (one API call per image) and complex. For this milestone, we inherit the license from the article and use a generic author string. A future enhancement can add Commons API lookups for more accurate attribution.

2. **`chrono_now_iso8601()` visibility**: This function is defined in `fetch.rs`. To reuse it in `media.rs`, change its visibility from `fn` to `pub(crate) fn`. Alternatively, move it to a shared `util.rs` module.

3. **Manifest overwrite vs. merge**: `write_media_manifest()` overwrites the entire manifest on each run. If media is downloaded incrementally (per-article), the caller must collect all `MediaResult`s and write once at the end, not per-article. Otherwise, later writes lose earlier entries.

4. **Manifest size**: With 90 articles and potentially hundreds of images, the manifest JSON could be 100KB+. `serde_json::to_string_pretty` is fine for this size. Don't worry about streaming serialization.

5. **Empty captions**: Many Wikipedia images have no `alt` text or `<figcaption>`. The `caption` field is `Option<String>` — `None` serializes as `null` in JSON.

6. **File size accuracy**: `size_bytes` is the on-disk file size after download. For SVGs, this is the raw XML size. For rasters, it's the thumbnail at `max_width`. These sizes are accurate for attribution purposes.

7. **`.gitignore` for manifest.json**: Per milestone 1.1, `demo/media/manifest.json` is gitignored. But the design doc section 10 says demo content IS committed. Clarify: the media manifest should probably be committed (it's needed for reproducible attribution pages). Remove it from `.gitignore` if present, or verify it's not ignored.

8. **Relationship to milestone 7.1**: The attribution page generator (milestone 7.1) reads this manifest to produce the HTML attribution page. The schema here must match what 7.1 expects. The `license`, `author`, `commons_filename`, and `source_article` fields are all consumed by the attribution template.
