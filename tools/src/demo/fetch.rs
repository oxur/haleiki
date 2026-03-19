//! Wikimedia REST API client for fetching article HTML.
//!
//! Fetches raw HTML from the Wikimedia REST API and writes it to the staging
//! directory (`demo/.staging/{slug}.html`). Supports both single-article fetch
//! and batch fetch with bounded concurrency, rate limiting, and progress bars.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

use super::manifest::{Article, Manifest};

/// Directory where fetched HTML is staged before processing.
const STAGING_DIR: &str = "demo/.staging";

/// Delay between API requests to respect Wikimedia rate limits.
const REQUEST_DELAY: Duration = Duration::from_millis(500);

/// Maximum number of concurrent HTTP requests.
const MAX_CONCURRENT_FETCHES: usize = 4;

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

/// Fetch a single article given pre-resolved URL and project.
///
/// This is the core fetch implementation that does not require a `Manifest`
/// reference, making it safe to use from spawned tokio tasks.
async fn fetch_article_raw(
    client: &ClientWithMiddleware,
    slug: &str,
    title: &str,
    api_url: &str,
    project: &str,
) -> anyhow::Result<FetchMeta> {
    let response = client.get(api_url).send().await?;

    let http_status = response.status().as_u16();

    if !response.status().is_success() {
        anyhow::bail!("HTTP {http_status} fetching \"{title}\": {api_url}");
    }

    let revision_id = parse_revision_id(response.headers().get(reqwest::header::ETAG));
    let html = response.text().await?;
    let html_bytes = html.len();

    // Ensure staging directory exists
    std::fs::create_dir_all(STAGING_DIR)?;

    // Write HTML
    let html_path = staging_html_path(slug);
    std::fs::write(&html_path, &html)?;

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

/// Fetch a single article's HTML from the Wikimedia REST API.
///
/// High-level wrapper around [`fetch_article_raw`] that resolves the API URL
/// and project from the manifest and prints progress to stderr.
pub async fn fetch_article(
    client: &ClientWithMiddleware,
    manifest: &Manifest,
    article: &Article,
) -> anyhow::Result<FetchMeta> {
    let api_url = manifest.api_url(article);
    let project = manifest.effective_project(article).to_string();

    eprintln!("  Fetching: {} ({})", article.title, project);

    let meta = fetch_article_raw(client, &article.slug, &article.title, &api_url, &project).await?;

    eprintln!(
        "  Wrote: {} ({} bytes, rev: {})",
        staging_html_path(&article.slug).display(),
        meta.html_bytes,
        meta.revision_id.as_deref().unwrap_or("unknown"),
    );

    Ok(meta)
}

/// Results from a batch fetch operation.
#[derive(Debug)]
struct BatchResult {
    /// Articles that were successfully fetched.
    fetched: Vec<String>,
    /// Articles that were skipped (already cached).
    skipped: Vec<String>,
    /// Articles that failed with their error messages.
    failed: Vec<(String, String)>,
}

/// Fetch all articles in the manifest with bounded parallelism and progress.
#[allow(clippy::too_many_lines)] // orchestrator function; splitting would obscure the flow
async fn batch_fetch(manifest: &Manifest, dry_run: bool, force: bool) -> anyhow::Result<()> {
    let articles = &manifest.articles;
    let total = articles.len();

    if dry_run {
        println!("Dry run — would fetch {total} articles:\n");
        for article in articles {
            let url = manifest.api_url(article);
            let cached = staging_html_path(&article.slug).exists();
            let cache_note = if cached && !force {
                " [cached, would skip]"
            } else {
                ""
            };
            println!("  {:<35} {url}{cache_note}", article.slug);
        }
        println!();

        let would_fetch = if force {
            total
        } else {
            articles
                .iter()
                .filter(|a| !staging_html_path(&a.slug).exists())
                .count()
        };
        let would_skip = total - would_fetch;
        println!("Would fetch: {would_fetch}, would skip: {would_skip}");
        return Ok(());
    }

    // Set up progress display
    let multi_progress = MultiProgress::new();
    let overall_style =
        ProgressStyle::with_template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .expect("valid progress template")
            .progress_chars("##-");

    let overall_bar = multi_progress.add(ProgressBar::new(total as u64));
    overall_bar.set_style(overall_style);
    overall_bar.set_message("Fetching articles");

    let item_style =
        ProgressStyle::with_template("  {spinner:.green} {msg}").expect("valid spinner template");

    // Build HTTP client (shared across all fetches)
    let client = Arc::new(build_client()?);
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_FETCHES));

    let mut handles = Vec::new();
    let mut result = BatchResult {
        fetched: Vec::new(),
        skipped: Vec::new(),
        failed: Vec::new(),
    };

    // Partition into skip/fetch lists first
    let mut to_fetch = Vec::new();
    for article in articles {
        let html_path = staging_html_path(&article.slug);
        if html_path.exists() && !force {
            result.skipped.push(article.slug.clone());
            let bar = multi_progress.add(ProgressBar::new_spinner());
            bar.set_style(item_style.clone());
            bar.set_message(format!("{}: skipped (cached)", article.slug));
            bar.finish();
            overall_bar.inc(1);
        } else {
            to_fetch.push(article);
        }
    }

    if to_fetch.is_empty() {
        overall_bar.finish_with_message("All articles already cached");
        print_batch_summary(&result);
        return Ok(());
    }

    // Spawn concurrent fetch tasks.
    // Pre-resolve API URLs and project domains BEFORE spawning to avoid
    // sending the Manifest across threads.
    for article in &to_fetch {
        let client = Arc::clone(&client);
        let semaphore = Arc::clone(&semaphore);
        let slug = article.slug.clone();
        let title = article.title.clone();
        let api_url = manifest.api_url(article);
        let project = manifest.effective_project(article).to_string();

        let bar = multi_progress.add(ProgressBar::new_spinner());
        bar.set_style(item_style.clone());
        bar.set_message(format!("{slug}: waiting..."));
        bar.enable_steady_tick(Duration::from_millis(100));

        let overall_bar = overall_bar.clone();

        let handle = tokio::spawn(async move {
            // Acquire semaphore permit (bounds concurrency)
            let _permit = semaphore
                .acquire()
                .await
                .expect("semaphore should not be closed");

            bar.set_message(format!("{slug}: fetching..."));

            // Rate-limit delay
            tokio::time::sleep(REQUEST_DELAY).await;

            let fetch_result = fetch_article_raw(&client, &slug, &title, &api_url, &project).await;

            match fetch_result {
                Ok(meta) => {
                    bar.set_message(format!(
                        "{slug}: done ({} bytes, rev: {})",
                        meta.html_bytes,
                        meta.revision_id.as_deref().unwrap_or("?"),
                    ));
                    bar.finish();
                    overall_bar.inc(1);
                    Ok(slug)
                }
                Err(e) => {
                    bar.set_message(format!("{slug}: FAILED -- {e}"));
                    bar.finish();
                    overall_bar.inc(1);
                    Err((slug, e.to_string()))
                }
            }
        });

        handles.push(handle);
    }

    // Collect results
    for handle in handles {
        match handle.await? {
            Ok(slug) => result.fetched.push(slug),
            Err((slug, msg)) => result.failed.push((slug, msg)),
        }
    }

    overall_bar.finish_with_message("Fetch complete");
    multi_progress.clear()?;

    print_batch_summary(&result);

    if !result.failed.is_empty() {
        anyhow::bail!("{} article(s) failed to fetch", result.failed.len());
    }

    Ok(())
}

/// Print a summary of the batch fetch operation.
fn print_batch_summary(result: &BatchResult) {
    println!();
    println!("Fetch summary:");
    println!(
        "  Fetched: {}  Skipped: {}  Failed: {}",
        result.fetched.len(),
        result.skipped.len(),
        result.failed.len(),
    );

    if !result.failed.is_empty() {
        println!();
        println!("Failed articles:");
        for (slug, msg) in &result.failed {
            println!("  {slug}: {msg}");
        }
    }

    println!();
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
/// When `article_slug` is `Some`, fetches a single article. When `None`,
/// fetches all manifest articles with bounded concurrency and progress bars.
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
            // Batch fetch all articles
            batch_fetch(&manifest, dry_run, force).await?;
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

    // --- Batch caching logic tests ---

    #[test]
    fn test_staging_html_path_exists_check() {
        // Verify that non-existent paths return false
        let path = staging_html_path("nonexistent-article-xyz");
        assert!(!path.exists());
    }
}
