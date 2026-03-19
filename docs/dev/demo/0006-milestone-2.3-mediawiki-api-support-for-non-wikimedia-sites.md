# Milestone 2.3 — MediaWiki API Support for Non-Wikimedia Sites

**Version:** 1.0
**Depends on:** Milestone 2.1 (Wikimedia REST API client works), Milestone 2.2 (batch fetch works)
**Produces:** `haleiki demo fetch` successfully fetches articles from rigpawiki.org alongside Wikipedia articles

---

## Overview

The manifest contains 6 articles from `www.rigpawiki.org`, a MediaWiki site that is NOT part of the Wikimedia ecosystem. Wikimedia sites use the REST API (`/api/rest_v1/page/html/{title}`), but standard MediaWiki instances use a different API.

This milestone adds a second fetch backend that detects non-Wikimedia projects and uses the MediaWiki API instead. Both backends produce the same output: raw HTML in `demo/.staging/{slug}.html` + metadata in `demo/.staging/{slug}.meta.json`.

### The 6 Rigpa Wiki articles

| Slug | Title |
|------|-------|
| yangthang-rinpoche | Yangthang Rinpoche |
| gyatrul-rinpoche | Gyatrul Rinpoche |
| chogyam-trungpa | Chögyam Trungpa |
| longchen-nyingtik | Longchen Nyingtik |
| jikme-lingpa | Jikme Lingpa |
| longchenpa | Longchenpa |

---

## MediaWiki API Options

Standard MediaWiki sites support several ways to get page HTML:

### Option A: `action=parse` API (recommended)

```
GET https://www.rigpawiki.org/api.php?action=parse&page={title}&format=json&prop=text|revid|displaytitle
```

Returns JSON with:
- `parse.text["*"]` — rendered HTML of the page body (no site chrome)
- `parse.revid` — revision ID
- `parse.displaytitle` — formatted title

**Pros:** Clean HTML without site navigation/chrome. Structured JSON response. Revision ID available. Widely supported across MediaWiki versions.

**Cons:** Requires URL-encoding the title. Response is JSON-wrapped.

### Option B: `action=render`

```
GET https://www.rigpawiki.org/index.php?title={title}&action=render
```

Returns raw HTML of the page body (no JSON wrapping).

**Pros:** Simpler — just HTML, no JSON parsing. Same URL pattern as the links in the manifest.

**Cons:** No revision ID in the response. Fewer metadata fields. Less control over what's included.

### Recommendation: Option A (`action=parse`)

Use `action=parse` because it provides the revision ID (important for reproducibility) and gives us control over which properties to request. The JSON wrapper is trivial to handle.

---

## Step 1: Detect project type

Add a function to determine whether a project domain is a Wikimedia site or a generic MediaWiki site.

### Add to `tools/src/demo/fetch.rs`

```rust
/// Known Wikimedia project domains that support the REST API.
const WIKIMEDIA_DOMAINS: &[&str] = &[
    "en.wikipedia.org",
    "en.wikibooks.org",
    "en.wikiversity.org",
    "en.wikisource.org",
    "en.wiktionary.org",
    "en.wikinews.org",
    "en.wikiquote.org",
    "commons.wikimedia.org",
    "simple.wikipedia.org",
    "meta.wikimedia.org",
    // Add other language editions as needed:
    // "fr.wikipedia.org", "de.wikipedia.org", etc.
];

/// Determines whether a project domain supports the Wikimedia REST API.
///
/// Wikimedia projects use: `/api/rest_v1/page/html/{title}`
/// Other MediaWiki sites use: `/api.php?action=parse&page={title}`
fn is_wikimedia_project(project: &str) -> bool {
    WIKIMEDIA_DOMAINS.contains(&project)
        || project.ends_with(".wikipedia.org")
        || project.ends_with(".wikibooks.org")
        || project.ends_with(".wikiversity.org")
        || project.ends_with(".wikisource.org")
        || project.ends_with(".wiktionary.org")
        || project.ends_with(".wikimedia.org")
        || project.ends_with(".wikinews.org")
        || project.ends_with(".wikiquote.org")
}
```

---

## Step 2: Update API URL construction

### Update `Manifest::api_url()` in `tools/src/demo/manifest.rs`

The current `api_url()` always constructs a Wikimedia REST API URL. Update it to dispatch based on project type:

```rust
impl Manifest {
    /// Build the API URL for fetching an article's HTML.
    ///
    /// Wikimedia projects: REST API `/api/rest_v1/page/html/{title}`
    /// Other MediaWiki sites: Parse API `/api.php?action=parse&page={title}&format=json&prop=text|revid|displaytitle`
    pub fn api_url(&self, article: &Article) -> String {
        let project = self.effective_project(article);
        let encoded_title = article.title.replace(' ', "_");

        if is_wikimedia_project(project) {
            format!("https://{project}/api/rest_v1/page/html/{encoded_title}")
        } else {
            // MediaWiki action API — URL-encode the title for query parameter
            let url_title = url_encode_title(&article.title);
            format!(
                "https://{project}/api.php?action=parse&page={url_title}&format=json&prop=text%7Crevid%7Cdisplaytitle"
            )
        }
    }
}

/// URL-encode a title for use in a query parameter.
/// Spaces → +, special characters → %XX.
fn url_encode_title(title: &str) -> String {
    title
        .chars()
        .map(|c| match c {
            ' ' => "+".to_string(),
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
```

Alternatively, keep `api_url()` as Wikimedia-only and add a separate `mediawiki_api_url()`, dispatching in `fetch_article`. Either approach works — the key is that `fetch_article_raw` (used by batch fetch) receives the correct URL.

**Note:** Move `is_wikimedia_project` to `manifest.rs` (or a shared location) so both `manifest.rs` and `fetch.rs` can use it.

---

## Step 3: Implement MediaWiki fetch in `fetch.rs`

### Add a MediaWiki response parser

```rust
/// Response structure from MediaWiki `action=parse` API.
#[derive(Debug, Deserialize)]
struct MediaWikiParseResponse {
    parse: MediaWikiParse,
}

#[derive(Debug, Deserialize)]
struct MediaWikiParse {
    /// Page title as displayed.
    #[serde(default)]
    displaytitle: String,

    /// Revision ID.
    #[serde(default)]
    revid: Option<u64>,

    /// HTML content. The key is literally "*" in the JSON.
    text: std::collections::HashMap<String, String>,
}

/// Fetch an article from a generic MediaWiki site using the action=parse API.
async fn fetch_mediawiki_article(
    client: &ClientWithMiddleware,
    slug: &str,
    title: &str,
    api_url: &str,
    project: &str,
) -> anyhow::Result<FetchMeta> {
    let response = client.get(api_url).send().await?;

    let http_status = response.status().as_u16();

    if !response.status().is_success() {
        anyhow::bail!("HTTP {http_status} fetching \"{title}\" from {project}: {api_url}");
    }

    let json: MediaWikiParseResponse = response.json().await
        .map_err(|e| anyhow::anyhow!(
            "Failed to parse MediaWiki API response for \"{title}\": {e}"
        ))?;

    // Extract HTML from the text map (key is "*")
    let html = json.parse.text.get("*")
        .ok_or_else(|| anyhow::anyhow!(
            "MediaWiki API response for \"{title}\" has no text content"
        ))?;

    let html_bytes = html.len();
    let revision_id = json.parse.revid.map(|id| id.to_string());

    // Ensure staging directory exists
    std::fs::create_dir_all(STAGING_DIR)?;

    // Write HTML (wrap in basic HTML structure since API returns body fragment)
    let full_html = format!(
        "<!DOCTYPE html>\n<html>\n<head><title>{}</title></head>\n<body>\n{}\n</body>\n</html>",
        json.parse.displaytitle,
        html,
    );
    let html_path = staging_html_path(slug);
    std::fs::write(&html_path, &full_html)?;

    // Build metadata
    let meta = FetchMeta {
        slug: slug.to_string(),
        project: project.to_string(),
        title: title.to_string(),
        api_url: api_url.to_string(),
        revision_id,
        fetched_at: chrono_now_iso8601(),
        http_status,
        html_bytes,
    };

    // Write metadata
    let meta_path = staging_meta_path(slug);
    let meta_json = serde_json::to_string_pretty(&meta)?;
    std::fs::write(&meta_path, meta_json)?;

    Ok(meta)
}
```

### Key differences from Wikimedia fetch

| Aspect | Wikimedia REST API | MediaWiki action=parse |
|--------|-------------------|----------------------|
| Response format | Raw HTML | JSON with HTML in `text["*"]` |
| Revision ID | ETag header | `revid` field in JSON |
| Content | Full HTML document | HTML body fragment (needs wrapping) |
| Accept header | `text/html` | `application/json` (default) |

---

## Step 4: Update `fetch_article_raw` to dispatch

### Update `fetch_article_raw` in `fetch.rs`

```rust
/// Fetch a single article, dispatching to the correct API based on project type.
async fn fetch_article_raw(
    client: &ClientWithMiddleware,
    slug: &str,
    title: &str,
    api_url: &str,
    project: &str,
) -> anyhow::Result<FetchMeta> {
    if is_wikimedia_project(project) {
        fetch_wikimedia_article(client, slug, title, api_url, project).await
    } else {
        fetch_mediawiki_article(client, slug, title, api_url, project).await
    }
}
```

Rename the existing `fetch_article_raw` to `fetch_wikimedia_article` (it already does the Wikimedia REST API flow).

---

## Step 5: Handle MediaWiki HTML differences in cleaning

Rigpa Wiki HTML will differ from Wikipedia HTML. The `clean.rs` module (milestone 3.1) strips Wikipedia-specific chrome. MediaWiki sites have different chrome patterns.

### Add to `clean.rs`

Add selectors for common generic MediaWiki chrome:

```rust
/// Additional selectors for generic MediaWiki sites (non-Wikimedia).
const MEDIAWIKI_REMOVE_SELECTORS: &[&str] = &[
    "#toc",
    ".toc",
    ".mw-editsection",
    ".catlinks",
    ".printfooter",
    "#siteSub",
    "#contentSub",
    ".mw-jump-link",
];
```

Since these overlap with the existing `REMOVE_SELECTORS`, no additional code is needed — the existing selectors already handle the common MediaWiki patterns. The Rigpa Wiki content is simpler than Wikipedia (fewer navboxes, no amboxes), so less stripping is needed.

**Test this empirically:** After fetching a Rigpa Wiki article, inspect the HTML to see what chrome needs stripping. Add selectors as needed.

---

## Step 6: Write tests

### Unit tests in `fetch.rs`

```rust
// ─── Project type detection ─────────────────────────

#[test]
fn test_is_wikimedia_project_wikipedia() {
    assert!(is_wikimedia_project("en.wikipedia.org"));
}

#[test]
fn test_is_wikimedia_project_wikibooks() {
    assert!(is_wikimedia_project("en.wikibooks.org"));
}

#[test]
fn test_is_wikimedia_project_french_wikipedia() {
    assert!(is_wikimedia_project("fr.wikipedia.org"));
}

#[test]
fn test_is_wikimedia_project_commons() {
    assert!(is_wikimedia_project("commons.wikimedia.org"));
}

#[test]
fn test_is_wikimedia_project_rigpawiki_is_not() {
    assert!(!is_wikimedia_project("www.rigpawiki.org"));
}

#[test]
fn test_is_wikimedia_project_generic_mediawiki_is_not() {
    assert!(!is_wikimedia_project("wiki.example.com"));
}
```

### Unit tests in `manifest.rs`

```rust
#[test]
fn test_api_url_wikimedia_uses_rest_api() {
    let m = sample_manifest_with_rigpawiki();
    let wp_article = m.articles.iter().find(|a| a.slug == "dzogchen").unwrap();
    let url = m.api_url(wp_article);
    assert!(url.contains("/api/rest_v1/page/html/"), "Expected REST API URL: {url}");
}

#[test]
fn test_api_url_rigpawiki_uses_action_parse() {
    let m = sample_manifest_with_rigpawiki();
    let rw_article = m.articles.iter().find(|a| a.slug == "longchenpa").unwrap();
    let url = m.api_url(rw_article);
    assert!(url.contains("api.php?action=parse"), "Expected action=parse URL: {url}");
    assert!(url.contains("www.rigpawiki.org"), "Expected rigpawiki domain: {url}");
}
```

### Integration test (live, network-dependent)

```rust
#[test]
#[ignore] // Requires network access
#[cfg(feature = "demo")]
fn test_demo_fetch_rigpawiki_article_live() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--article", "longchenpa"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success();

    let staging_html = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("demo/.staging/longchenpa.html");
    assert!(staging_html.exists(), "Rigpa Wiki HTML was not created");

    let html = std::fs::read_to_string(&staging_html).unwrap();
    assert!(html.len() > 500, "HTML seems too short: {} bytes", html.len());

    // Clean up
    let _ = std::fs::remove_file(&staging_html);
    let _ = std::fs::remove_file(staging_html.with_extension("meta.json"));
}
```

---

## Verification

### 7.1: Dry-run shows correct URLs per project type

```bash
cargo run --features demo -- demo fetch --dry-run
```

Wikipedia articles should show `/api/rest_v1/page/html/` URLs.
Rigpa Wiki articles should show `/api.php?action=parse&page=` URLs.

### 7.2: Fetch a Rigpa Wiki article (manual, network)

```bash
cargo run --features demo -- demo fetch --article longchenpa
```

Should create `demo/.staging/longchenpa.html` and `demo/.staging/longchenpa.meta.json`.

### 7.3: Batch fetch includes both project types

```bash
cargo run --features demo -- demo fetch --force
```

All 90 articles should fetch — 84 from Wikipedia, 6 from Rigpa Wiki.

### 7.4: Status shows all articles

```bash
cargo run --features demo -- demo status
```

All 90 articles should show `staged`.

### 7.5: Tests pass

```bash
cargo test --features demo
```

---

## Acceptance Criteria

- [ ] `is_wikimedia_project()` correctly identifies Wikimedia vs. non-Wikimedia domains
- [ ] `api_url()` generates REST API URLs for Wikimedia, action=parse URLs for others
- [ ] `fetch_mediawiki_article()` handles the `action=parse` JSON response
- [ ] Revision ID extracted from JSON `revid` field (not ETag)
- [ ] MediaWiki HTML body fragment wrapped in `<html><body>` for consistency
- [ ] All 6 Rigpa Wiki articles fetch successfully
- [ ] Metadata JSON written for MediaWiki articles (same format as Wikimedia)
- [ ] `--dry-run` shows correct API URLs per project type
- [ ] Batch fetch handles mixed project types
- [ ] HTML cleaning works on Rigpa Wiki content (may need additional selectors)
- [ ] All unit tests pass
- [ ] `make lint` passes

---

## Gotchas

1. **Rigpa Wiki availability**: Unlike Wikipedia, Rigpa Wiki may have downtime or rate limiting that's more aggressive. The retry middleware from 2.1 helps, but be prepared for slower fetches.

2. **MediaWiki API response format**: The `text` field in the parse response is a map with a single key `"*"` (literally an asterisk). This is a MediaWiki convention, not a wildcard. Access it as `text.get("*")`.

3. **HTML fragment vs. full document**: The action=parse API returns an HTML fragment (the page body), not a full document. We wrap it in `<html><body>` so the cleaning and conversion stages have consistent input.

4. **Character encoding in titles**: Rigpa Wiki titles may contain Tibetan script or diacritics (e.g., "Chögyam Trungpa"). These need proper URL encoding in the `page=` query parameter.

5. **HTTPS support**: Verify that `www.rigpawiki.org` supports HTTPS. If not, fall back to HTTP (and update the URL construction). As of the manifest, we assume HTTPS.

6. **MediaWiki version**: Different MediaWiki versions may return slightly different JSON structures. The `action=parse` API is stable across versions, but field names or nesting could vary. Test against the actual Rigpa Wiki response.

7. **API rate limits**: Rigpa Wiki may not document its rate limits. Be conservative — the existing 500ms delay between requests should suffice. If you get 429s, increase the delay for non-Wikimedia sites.

8. **Redirects**: MediaWiki articles may redirect. The `action=parse` API follows redirects automatically and returns the final page content. The title in the response (`displaytitle`) will be the canonical title after redirect resolution.
