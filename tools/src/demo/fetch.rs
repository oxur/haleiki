//! Wikimedia REST API client for fetching article HTML.
//!
//! Fetches raw HTML from the Wikimedia REST API and writes it to the staging
//! directory (`demo/.staging/{slug}.html`).

use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use serde::{Deserialize, Serialize};

use super::manifest::{Article, Manifest};

/// Directory where fetched HTML is staged before processing.
const STAGING_DIR: &str = "demo/.staging";

/// Delay between API requests to respect Wikimedia rate limits.
#[allow(dead_code)] // used in batch fetch (milestone 2.2)
const REQUEST_DELAY: Duration = Duration::from_millis(500);

/// User-Agent string -- Wikimedia API requires a descriptive User-Agent.
/// See: <https://meta.wikimedia.org/wiki/User-Agent_policy>
#[allow(dead_code)] // used in build_client via from_static with a literal
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

    /// Wikipedia revision ID (from `ETag` header).
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
/// starting at 500ms (500ms -> ~1s -> ~2s).
fn build_client() -> anyhow::Result<ClientWithMiddleware> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Haleiki/0.1.0 (https://github.com/oxur/haleiki; haleiki@oxur.net)",
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
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

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

/// Extract the revision ID from the Wikimedia REST API `ETag` header.
///
/// The `ETag` format is typically: `"<revision_id>/<hash>"` or just `"<revision_id>"`.
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

    // Extract revision ID from `ETag` header before consuming the response body
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
/// Batch fetching (no `--article`) is milestone 2.2.
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
            // Batch fetch -- stub for now, implemented in milestone 2.2
            eprintln!(
                "Batch fetch not yet implemented. Use --article <slug> to fetch a single article."
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    // --- ETag parsing tests ---

    #[test]
    fn test_parse_revision_id_standard_etag() {
        let val = HeaderValue::from_static("\"1234567890/abcdef\"");
        assert_eq!(
            parse_revision_id(Some(&val)),
            Some("1234567890".to_string())
        );
    }

    #[test]
    fn test_parse_revision_id_numeric_only() {
        let val = HeaderValue::from_static("\"9876543210\"");
        assert_eq!(
            parse_revision_id(Some(&val)),
            Some("9876543210".to_string())
        );
    }

    #[test]
    fn test_parse_revision_id_none() {
        assert_eq!(parse_revision_id(None), None);
    }

    #[test]
    fn test_parse_revision_id_unquoted() {
        let val = HeaderValue::from_static("1234567890");
        assert_eq!(
            parse_revision_id(Some(&val)),
            Some("1234567890".to_string())
        );
    }

    // --- Path construction tests ---

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

    // --- Timestamp tests ---

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

    // --- FetchMeta serialization tests ---

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
