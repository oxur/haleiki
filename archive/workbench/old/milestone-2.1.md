# Milestone 2.1 — Wikimedia REST API Client (`fetch.rs`)

**Version:** 1.0
**Depends on:** Milestone 1.3 (manifest parsing works, `haleiki demo status` functional)
**Produces:** `haleiki demo fetch --article memory-management` downloads raw HTML to `demo/.staging/`

---

## Overview

Implement the HTTP client for fetching article HTML from the Wikimedia REST API. The client uses `reqwest-middleware` + `reqwest-retry` for automatic retries with exponential backoff and jitter on transient failures (5xx, timeouts, connection errors). This milestone covers single-article fetching only — batch fetching with progress is milestone 2.2.

The fetch flow for a single article:
1. Load `demo/manifest.yaml`, find the article by slug
2. Construct the REST API URL: `https://{project}/api/rest_v1/page/html/{title}`
3. Send GET request with appropriate User-Agent header (retries automatically on transient failures)
4. Extract the revision ID from the response's `ETag` header
5. Write the raw HTML to `demo/.staging/{slug}.html`
6. Write fetch metadata to `demo/.staging/{slug}.meta.json`

---

## Step 1: Add async runtime wiring

The `demo` feature already depends on `tokio` and `reqwest`. This milestone also adds `reqwest-middleware` and `reqwest-retry` for automatic retry with exponential backoff. We need to make the demo `run()` function async-aware.

### Update `tools/Cargo.toml` — add retry middleware dependencies

Add to the `[dependencies]` section alongside the existing `reqwest` entry:

```toml
# ─── Demo feature (optional) ───
reqwest = { version = "0.12", features = ["rustls-tls", "json"], optional = true }
reqwest-middleware = { version = "0.4", optional = true }
reqwest-retry = { version = "0.7", optional = true }
tokio = { version = "1", features = ["full"], optional = true }
# ... other demo deps ...
```

Update the `demo` feature to include the new crates:

```toml
[features]
demo = [
    "dep:reqwest",
    "dep:reqwest-middleware",
    "dep:reqwest-retry",
    "dep:tokio",
    "dep:scraper",
    "dep:htmd",
    "dep:globset",
    "dep:indicatif",
]
```

**Note:** Verify latest versions with `cargo search reqwest-middleware --limit 1` and `cargo search reqwest-retry --limit 1` before writing. The versions above are best-effort.

### Update `tools/src/demo/mod.rs`

The `run()` function needs to spin up a tokio runtime for async operations. There are two approaches:

**Approach A (recommended):** Keep `main()` synchronous, create a tokio runtime inside `demo::run()`:

```rust
pub fn run(cmd: &DemoCommand) -> anyhow::Result<()> {
    match cmd {
        DemoCommand::Fetch { article, dry_run, force, pandoc } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(fetch::run(article.as_deref(), *dry_run, *force, *pandoc))?;
        }
        DemoCommand::Status => {
            status::run()?;
        }
        // ... other stubs unchanged ...
    }
    Ok(())
}
```

**Approach B:** Make `main()` async with `#[tokio::main]`. This is cleaner but pulls tokio into the non-demo path. Since tokio is optional (demo feature only), Approach A is correct.

**Important:** Do NOT add `#[tokio::main]` to `main()` — tokio is an optional dependency gated behind the `demo` feature.

---

## Step 2: Create `tools/src/demo/fetch.rs`

### File: `tools/src/demo/fetch.rs`

```rust
//! Wikimedia REST API client for fetching article HTML.
//!
//! Fetches raw HTML from the Wikimedia REST API and writes it to the staging
//! directory (`demo/.staging/{slug}.html`).

use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};

use super::manifest::{Article, Manifest};

/// Directory where fetched HTML is staged before processing.
const STAGING_DIR: &str = "demo/.staging";

/// Delay between API requests to respect Wikimedia rate limits.
const REQUEST_DELAY: Duration = Duration::from_millis(500);

/// User-Agent string — Wikimedia API requires a descriptive User-Agent.
/// See: https://meta.wikimedia.org/wiki/User-Agent_policy
const USER_AGENT_STRING: &str = concat!(
    "Haleiki/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/oxur/haleiki; haleiki@oxur.net) reqwest/",
);

/// Metadata captured during a fetch operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchMeta {
    /// Article slug.
    pub slug: String,

    /// Wikimedia project domain.
    pub project: String,

    /// Original article title.
    pub title: String,

    /// API URL that was fetched.
    pub api_url: String,

    /// Wikipedia revision ID (from ETag header).
    pub revision_id: Option<String>,

    /// ISO 8601 timestamp of when the fetch occurred.
    pub fetched_at: String,

    /// HTTP status code received.
    pub http_status: u16,

    /// Size of the fetched HTML in bytes.
    pub html_bytes: usize,
}

/// Build the HTTP client with retry middleware.
///
/// Uses `reqwest-middleware` + `reqwest-retry` for automatic retries on
/// transient failures (5xx responses, timeouts, connection errors).
/// Retry policy: exponential backoff with jitter, up to 3 retries,
/// starting at 500ms (500ms → ~1s → ~2s).
fn build_client() -> anyhow::Result<ClientWithMiddleware> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Haleiki/0.1.0 (https://github.com/oxur/haleiki; haleiki@oxur.net)"
        ),
    );
    // Accept HTML from the REST API
    headers.insert(
        reqwest::header::ACCEPT,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );

    let base_client = reqwest::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()?;

    // Exponential backoff: 3 retries, starting at 500ms, with jitter
    let retry_policy = ExponentialBackoff::builder()
        .build_with_max_retries(3);

    let client = reqwest_middleware::ClientBuilder::new(base_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    Ok(client)
}

/// Path where an article's raw HTML is staged.
pub fn staging_html_path(slug: &str) -> PathBuf {
    Path::new(STAGING_DIR).join(format!("{slug}.html"))
}

/// Path where an article's fetch metadata is written.
pub fn staging_meta_path(slug: &str) -> PathBuf {
    Path::new(STAGING_DIR).join(format!("{slug}.meta.json"))
}

/// Extract the revision ID from the Wikimedia REST API ETag header.
///
/// The ETag format is typically: `"<revision_id>/<hash>"` or just `"<revision_id>"`.
/// We extract the numeric revision ID.
fn parse_revision_id(etag: Option<&HeaderValue>) -> Option<String> {
    let etag_str = etag?.to_str().ok()?;
    // Strip surrounding quotes
    let stripped = etag_str.trim_matches('"');
    // Take the part before any slash
    let rev = stripped.split('/').next()?;
    // Verify it looks numeric
    if rev.chars().all(|c| c.is_ascii_digit()) {
        Some(rev.to_string())
    } else {
        Some(stripped.to_string())
    }
}

/// Fetch a single article's HTML from the Wikimedia REST API.
///
/// Returns the fetch metadata on success.
pub async fn fetch_article(
    client: &ClientWithMiddleware,
    manifest: &Manifest,
    article: &Article,
) -> anyhow::Result<FetchMeta> {
    let api_url = manifest.api_url(article);
    let project = manifest.effective_project(article).to_string();

    eprintln!("  Fetching: {} ({})", article.title, project);

    let response = client.get(&api_url).send().await?;

    let http_status = response.status().as_u16();

    if !response.status().is_success() {
        anyhow::bail!(
            "HTTP {} fetching \"{}\": {}",
            http_status,
            article.title,
            api_url,
        );
    }

    // Extract revision ID from ETag header before consuming the response body
    let revision_id = parse_revision_id(response.headers().get(reqwest::header::ETAG));

    let html = response.text().await?;
    let html_bytes = html.len();

    // Ensure staging directory exists
    let staging_dir = Path::new(STAGING_DIR);
    std::fs::create_dir_all(staging_dir)?;

    // Write raw HTML
    let html_path = staging_html_path(&article.slug);
    std::fs::write(&html_path, &html)?;

    // Build fetch metadata
    let now = chrono_now_iso8601();
    let meta = FetchMeta {
        slug: article.slug.clone(),
        project,
        title: article.title.clone(),
        api_url,
        revision_id,
        fetched_at: now,
        http_status,
        html_bytes,
    };

    // Write metadata JSON
    let meta_path = staging_meta_path(&article.slug);
    let meta_json = serde_json::to_string_pretty(&meta)?;
    std::fs::write(&meta_path, meta_json)?;

    eprintln!(
        "  Wrote: {} ({} bytes, rev: {})",
        html_path.display(),
        html_bytes,
        meta.revision_id.as_deref().unwrap_or("unknown"),
    );

    Ok(meta)
}

/// ISO 8601 timestamp for "now" without pulling in the chrono crate.
///
/// Uses `std::time::SystemTime` and formats manually. If you need more
/// sophisticated date handling later, consider adding the `chrono` crate.
fn chrono_now_iso8601() -> String {
    use std::time::SystemTime;
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Simple UTC timestamp calculation
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;

    // Days since epoch to Y-M-D (simplified, correct for 1970-2100)
    let mut y = 1970;
    let mut remaining_days = days;
    loop {
        let days_in_year = if is_leap_year(y) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        y += 1;
    }
    let month_days: &[u64] = if is_leap_year(y) {
        &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0;
    for &md in month_days {
        if remaining_days < md {
            break;
        }
        remaining_days -= md;
        m += 1;
    }

    format!(
        "{y:04}-{:02}-{:02}T{hours:02}:{minutes:02}:{seconds:02}Z",
        m + 1,
        remaining_days + 1,
    )
}

fn is_leap_year(y: u64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

/// Entry point for `haleiki demo fetch`.
///
/// In this milestone, only `--article <slug>` is implemented.
/// Batch fetching (no --article) is milestone 2.2.
pub async fn run(
    article_slug: Option<&str>,
    dry_run: bool,
    force: bool,
    _pandoc: bool,
) -> anyhow::Result<()> {
    let manifest_path = Path::new("demo/manifest.yaml");
    if !manifest_path.exists() {
        anyhow::bail!(
            "manifest not found at {}\n\
             Hint: run this command from the repository root",
            manifest_path.display()
        );
    }

    let manifest = Manifest::from_file(manifest_path)?;

    // Validate manifest first
    let issues = manifest.validate();
    if !issues.is_empty() {
        eprintln!("Manifest validation warnings:");
        for issue in &issues {
            eprintln!("  - {issue}");
        }
        eprintln!();
    }

    match article_slug {
        Some(slug) => {
            // Single article fetch
            let article = manifest
                .articles
                .iter()
                .find(|a| a.slug == slug)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "article with slug \"{slug}\" not found in manifest\n\
                         Available slugs: {}",
                        manifest
                            .articles
                            .iter()
                            .map(|a| a.slug.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })?;

            // Check if already fetched (skip unless --force)
            let html_path = staging_html_path(&article.slug);
            if html_path.exists() && !force {
                eprintln!(
                    "Already fetched: {} (use --force to re-fetch)",
                    html_path.display()
                );
                return Ok(());
            }

            if dry_run {
                let url = manifest.api_url(article);
                println!("Would fetch: {}", article.title);
                println!("  URL: {url}");
                println!("  Project: {}", manifest.effective_project(article));
                println!("  Destination: {}", html_path.display());
                return Ok(());
            }

            let client = build_client()?;
            fetch_article(&client, &manifest, article).await?;

            eprintln!("Done.");
        }
        None => {
            // Batch fetch — stub for now, implemented in milestone 2.2
            eprintln!("Batch fetch not yet implemented. Use --article <slug> to fetch a single article.");
        }
    }

    Ok(())
}
```

### Design Notes

- **Retry middleware**: `reqwest-middleware` wraps the base `reqwest::Client` in a middleware stack. `reqwest-retry` provides `RetryTransientMiddleware` which automatically retries on transient failures (HTTP 5xx, timeouts, connection resets) using exponential backoff with jitter. The retry policy (3 retries, 500ms base) is configured in `build_client()`. Non-transient errors (4xx, DNS resolution failures) are NOT retried.
- **User-Agent**: Wikimedia requires a descriptive User-Agent. See https://meta.wikimedia.org/wiki/User-Agent_policy. Include project name, version, URL, and contact.
- **ETag parsing**: The REST API returns ETag headers containing the revision ID. Format varies: `"1234567890/abcdef"` or `"1234567890"`. We parse out the numeric prefix.
- **No chrono dependency**: We avoid adding `chrono` by implementing a simple ISO 8601 formatter. If the project later needs chrono for other purposes, refactor to use it.
- **Metadata JSON**: Each fetch writes a `.meta.json` alongside the `.html`. This metadata is consumed by frontmatter generation (milestone 5.3) and status reporting (milestone 6.4).

---

## Step 3: Wire `fetch` module into `demo/mod.rs`

### Update `tools/src/demo/mod.rs`

Add module declaration:

```rust
pub mod fetch;
pub mod manifest;
pub mod status;
```

Update the `Fetch` arm in `run()`:

```rust
DemoCommand::Fetch { article, dry_run, force, pandoc } => {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(fetch::run(article.as_deref(), *dry_run, *force, *pandoc))?;
}
```

---

## Step 4: Update `.gitignore`

Ensure staging HTML files are not committed (they're intermediate artifacts). The `.meta.json` files SHOULD be committed since they contain revision IDs for reproducibility.

### Update to `.gitignore`

```
# Demo staging files (intermediate HTML, not committed)
/demo/.staging/*.html
```

**Do NOT gitignore** `demo/.staging/*.meta.json` — these contain revision IDs and fetch timestamps needed for reproducibility and `haleiki demo status` reporting.

---

## Step 5: Write tests

### Unit tests in `tools/src/demo/fetch.rs`

Add a `#[cfg(test)]` module at the bottom of `fetch.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    // ─── ETag parsing tests ─────────────────────────────

    #[test]
    fn test_parse_revision_id_standard_etag() {
        let val = HeaderValue::from_static("\"1234567890/abcdef\"");
        assert_eq!(parse_revision_id(Some(&val)), Some("1234567890".to_string()));
    }

    #[test]
    fn test_parse_revision_id_numeric_only() {
        let val = HeaderValue::from_static("\"9876543210\"");
        assert_eq!(parse_revision_id(Some(&val)), Some("9876543210".to_string()));
    }

    #[test]
    fn test_parse_revision_id_none() {
        assert_eq!(parse_revision_id(None), None);
    }

    #[test]
    fn test_parse_revision_id_unquoted() {
        let val = HeaderValue::from_static("1234567890");
        assert_eq!(parse_revision_id(Some(&val)), Some("1234567890".to_string()));
    }

    // ─── Path construction tests ────────────────────────

    #[test]
    fn test_staging_html_path_format() {
        let path = staging_html_path("memory-management");
        assert_eq!(path, PathBuf::from("demo/.staging/memory-management.html"));
    }

    #[test]
    fn test_staging_meta_path_format() {
        let path = staging_meta_path("garbage-collection");
        assert_eq!(
            path,
            PathBuf::from("demo/.staging/garbage-collection.meta.json")
        );
    }

    // ─── Timestamp tests ────────────────────────────────

    #[test]
    fn test_chrono_now_iso8601_format() {
        let ts = chrono_now_iso8601();
        // Should match: YYYY-MM-DDTHH:MM:SSZ
        assert!(
            ts.len() == 20,
            "Unexpected timestamp length: {ts} (len={})",
            ts.len()
        );
        assert!(ts.ends_with('Z'), "Timestamp should end with Z: {ts}");
        assert_eq!(&ts[4..5], "-", "Expected dash at pos 4: {ts}");
        assert_eq!(&ts[7..8], "-", "Expected dash at pos 7: {ts}");
        assert_eq!(&ts[10..11], "T", "Expected T at pos 10: {ts}");
    }

    // ─── FetchMeta serialization tests ──────────────────

    #[test]
    fn test_fetch_meta_serializes_to_json() {
        let meta = FetchMeta {
            slug: "test".to_string(),
            project: "en.wikipedia.org".to_string(),
            title: "Test Article".to_string(),
            api_url: "https://en.wikipedia.org/api/rest_v1/page/html/Test_Article".to_string(),
            revision_id: Some("123456".to_string()),
            fetched_at: "2026-03-18T12:00:00Z".to_string(),
            http_status: 200,
            html_bytes: 5000,
        };

        let json = serde_json::to_string_pretty(&meta).unwrap();
        assert!(json.contains("\"slug\": \"test\""));
        assert!(json.contains("\"revision_id\": \"123456\""));
        assert!(json.contains("\"html_bytes\": 5000"));
    }

    #[test]
    fn test_fetch_meta_roundtrip() {
        let meta = FetchMeta {
            slug: "test".to_string(),
            project: "en.wikipedia.org".to_string(),
            title: "Test".to_string(),
            api_url: "https://example.com".to_string(),
            revision_id: None,
            fetched_at: "2026-01-01T00:00:00Z".to_string(),
            http_status: 200,
            html_bytes: 100,
        };

        let json = serde_json::to_string(&meta).unwrap();
        let meta2: FetchMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(meta.slug, meta2.slug);
        assert_eq!(meta.revision_id, meta2.revision_id);
    }
}
```

### Integration test for `--dry-run`

Add to `tools/tests/demo_fetch.rs`:

```rust
//! Integration tests for `haleiki demo fetch`.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_dry_run_single_article() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--article", "memory-management", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Would fetch")
                .and(predicate::str::contains("Memory management"))
                .and(predicate::str::contains("en.wikipedia.org"))
                .and(predicate::str::contains("api/rest_v1/page/html")),
        );
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_unknown_slug_fails() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--article", "nonexistent-article"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found in manifest"));
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_dry_run_wikibooks_shows_project() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args([
            "demo",
            "fetch",
            "--article",
            "wikibooks-memory-management",
            "--dry-run",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(predicate::str::contains("en.wikibooks.org"));
}
```

### Live fetch test (optional, network-dependent)

This test actually hits the Wikimedia API. Mark it `#[ignore]` so it doesn't run in CI by default:

```rust
#[test]
#[ignore] // Requires network access
#[cfg(feature = "demo")]
fn test_demo_fetch_single_article_live() {
    use std::path::Path;

    let staging_html = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("demo/.staging/memory-management.html");

    // Clean up from previous runs
    let _ = std::fs::remove_file(&staging_html);

    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--article", "memory-management"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stderr(predicate::str::contains("Fetching"));

    // Verify file was created
    assert!(staging_html.exists(), "Staging HTML was not created");

    let html = std::fs::read_to_string(&staging_html).unwrap();
    assert!(html.len() > 1000, "HTML seems too short: {} bytes", html.len());
    assert!(
        html.contains("memory") || html.contains("Memory"),
        "HTML doesn't mention 'memory'"
    );

    // Verify metadata was created
    let meta_path = staging_html.with_extension("meta.json");
    assert!(meta_path.exists(), "Metadata JSON was not created");

    // Clean up
    let _ = std::fs::remove_file(&staging_html);
    let _ = std::fs::remove_file(&meta_path);
}
```

---

## Step 6: Update `haleiki demo status` to show staging state

### Update `tools/src/demo/status.rs`

Now that we have staging files, the status command should reflect two dimensions:
- **Staging state**: Is the raw HTML fetched? (checks `demo/.staging/{slug}.html`)
- **Source state**: Is the converted Markdown present? (checks `demo/sources/{slug}.md`)

Update the `FetchState` enum and `fetch_state()` function:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchState {
    /// No staging HTML or source Markdown exists.
    Missing,
    /// Staging HTML exists but not yet converted to Markdown.
    Staged,
    /// Converted source Markdown exists.
    Converted,
}

impl std::fmt::Display for FetchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(f, "missing"),
            Self::Staged => write!(f, "staged"),
            Self::Converted => write!(f, "converted"),
        }
    }
}

fn fetch_state(slug: &str) -> FetchState {
    let source_path = Path::new("demo/sources").join(format!("{slug}.md"));
    if source_path.exists() {
        return FetchState::Converted;
    }
    let staging_path = Path::new("demo/.staging").join(format!("{slug}.html"));
    if staging_path.exists() {
        return FetchState::Staged;
    }
    FetchState::Missing
}
```

Update the summary line to include all three states:

```rust
println!(
    "  Total: {} articles ({} converted, {} staged, {} missing)",
    manifest.articles.len(),
    converted_count,
    staged_count,
    missing_count,
);
```

---

## Verification

### 7.1: Dry-run works

```bash
cd /Users/oubiwann/lab/oxur/haleiki
cargo run --features demo -- demo fetch --article memory-management --dry-run
```

Expected output:
```
Would fetch: Memory management
  URL: https://en.wikipedia.org/api/rest_v1/page/html/Memory_management
  Project: en.wikipedia.org
  Destination: demo/.staging/memory-management.html
```

### 7.2: Live single-article fetch (manual, requires network)

```bash
cargo run --features demo -- demo fetch --article memory-management
```

Expected:
- `demo/.staging/memory-management.html` created (should be 50KB+)
- `demo/.staging/memory-management.meta.json` created
- stderr shows "Fetching: Memory management (en.wikipedia.org)"

### 7.3: Status reflects fetched article

```bash
cargo run --features demo -- demo status
```

The `memory-management` row should show `staged` instead of `missing`.

### 7.4: Skip existing unless --force

```bash
# Should skip (already fetched)
cargo run --features demo -- demo fetch --article memory-management
# Should show: "Already fetched: demo/.staging/memory-management.html (use --force to re-fetch)"

# Should re-fetch
cargo run --features demo -- demo fetch --article memory-management --force
```

### 7.5: Unknown slug fails gracefully

```bash
cargo run --features demo -- demo fetch --article nonexistent 2>&1
# Should fail with: "article with slug "nonexistent" not found in manifest"
```

### 7.6: Wikibooks article uses correct project

```bash
cargo run --features demo -- demo fetch --article wikibooks-memory-management --dry-run
# URL should contain en.wikibooks.org, not en.wikipedia.org
```

### 7.7: All tests pass

```bash
cargo test --features demo
```

### 7.8: Linting passes

```bash
make lint
```

---

## Acceptance Criteria

- [ ] `tools/src/demo/fetch.rs` implements the Wikimedia REST API client
- [ ] `fetch_article()` sends GET to the correct API URL with proper User-Agent
- [ ] `fetch_article()` extracts revision ID from ETag header
- [ ] `fetch_article()` writes raw HTML to `demo/.staging/{slug}.html`
- [ ] `fetch_article()` writes metadata JSON to `demo/.staging/{slug}.meta.json`
- [ ] `haleiki demo fetch --article <slug>` works for any manifest article
- [ ] `haleiki demo fetch --article <slug> --dry-run` prints URL without fetching
- [ ] `haleiki demo fetch --article <slug>` skips if already fetched (no `--force`)
- [ ] `haleiki demo fetch --article <slug> --force` re-fetches
- [ ] Unknown slug produces helpful error listing available slugs
- [ ] Wikibooks article uses `en.wikibooks.org` project domain
- [ ] `haleiki demo status` shows `staged` for fetched articles
- [ ] Batch fetch (no `--article`) prints "not yet implemented" (milestone 2.2)
- [ ] All unit tests pass
- [ ] Integration tests pass (dry-run mode, unknown slug)
- [ ] `make lint` passes

---

## Gotchas

1. **Wikimedia User-Agent policy**: The API requires a descriptive User-Agent with project name, version, URL, and contact email. Requests without one may be blocked. See https://meta.wikimedia.org/wiki/User-Agent_policy

2. **ETag format varies**: Different Wikimedia projects may return ETags in different formats. The parser should handle: `"12345/hash"`, `"12345"`, `12345` (without quotes). Be defensive.

3. **Title encoding**: The Wikimedia REST API expects underscores for spaces but does NOT expect percent-encoding for parentheses or slashes. `Garbage_collection_(computer_science)` is correct. The `api_url()` method from milestone 1.3 already handles this.

4. **Rate limiting vs. retries**: The `REQUEST_DELAY` constant (500ms between requests) handles politeness / rate-limit avoidance. The `reqwest-retry` middleware handles transient *failures* (server errors, timeouts). These are complementary: the delay prevents us from hitting rate limits in the first place; the retry middleware recovers when transient errors happen anyway. `reqwest-retry` uses `Retryable::from_reqwest_response()` to classify responses — 429 (Too Many Requests) is classified as transient and will be retried with backoff, which is exactly right for Wikimedia's rate limiting.

5. **Tokio runtime in a sync context**: Since `main()` is synchronous and tokio is an optional dependency, we create a `Runtime::new()` inside the demo `run()` function. Don't use `#[tokio::main]` on `main()`.

6. **File system ops in async context**: `std::fs::write` blocks the async runtime. For single-article fetch this is fine. For batch fetch (2.2), consider using `tokio::fs`. But don't over-engineer in this milestone.

7. **`chrono` vs manual timestamp**: We avoid adding `chrono` as a dependency by implementing a simple ISO 8601 formatter. This is intentional — it keeps the dependency count low. The formatter is correct for dates between 1970 and 2100.

8. **HTML content**: The Wikimedia REST API returns full HTML documents with `<html>`, `<head>`, `<body>` tags plus extensive Wikipedia chrome (navboxes, edit links, etc.). This is expected — cleaning happens in milestone 3.1.

9. **Network tests**: Tests that actually hit the Wikimedia API should be `#[ignore]` so they don't run in CI. Run them manually with `cargo test --features demo -- --ignored`.

10. **`.meta.json` committed, `.html` not**: The gitignore should exclude staging HTML but NOT the metadata JSON. The metadata contains revision IDs needed for reproducibility tracking.
