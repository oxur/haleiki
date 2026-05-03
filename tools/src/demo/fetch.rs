//! `MediaWiki` API client for fetching article HTML.
//!
//! Supports two backends:
//! - **Wikimedia REST API** (`/api/rest_v1/page/html/{title}`) for Wikimedia
//!   projects (Wikipedia, Wikibooks, etc.)
//! - **`MediaWiki` action API** (`/api.php?action=parse`) for generic `MediaWiki`
//!   sites (e.g., rigpawiki.org)
//!
//! Both backends write identical output: raw HTML in `demo/.staging/{slug}.html`
//! and metadata in `demo/.staging/{slug}.meta.json`. The dispatcher
//! [`fetch_article_raw`] selects the correct backend based on
//! [`is_wikimedia_project`].

use std::collections::HashMap;
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

/// Known Wikimedia project domains that support the REST API.
///
/// This list covers the major English-language projects and cross-language
/// sites. Language-variant domains (e.g., `fr.wikipedia.org`) are handled
/// by suffix matching in [`is_wikimedia_project`].
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
];

/// Determines whether a project domain supports the Wikimedia REST API.
///
/// Returns `true` for domains in the [`WIKIMEDIA_DOMAINS`] list as well as
/// any domain matching a Wikimedia suffix pattern (e.g., `*.wikipedia.org`).
///
/// Wikimedia projects use: `/api/rest_v1/page/html/{title}`
/// Other `MediaWiki` sites use: `/api.php?action=parse&page={title}`
pub(super) fn is_wikimedia_project(project: &str) -> bool {
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

/// URL-encode a title for use in a query parameter.
///
/// Unreserved characters ([A-Za-z0-9\-_.~]) pass through unchanged. Spaces
/// become `+`. All other bytes (including multi-byte UTF-8 sequences) are
/// percent-encoded as `%XX`.
pub(super) fn url_encode_title(title: &str) -> String {
    title
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                String::from(b as char)
            }
            b' ' => "+".to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// Response structure from `MediaWiki` `action=parse` API.
#[derive(Debug, Deserialize)]
struct MediaWikiParseResponse {
    /// The parsed page data.
    parse: MediaWikiParse,
}

/// Inner parse result from the `MediaWiki` action API.
#[derive(Debug, Deserialize)]
struct MediaWikiParse {
    /// Page title as displayed (may contain HTML formatting).
    #[serde(default)]
    displaytitle: String,

    /// Revision ID of the parsed page.
    #[serde(default)]
    revid: Option<u64>,

    /// HTML content. The key is literally `"*"` in the JSON.
    text: HashMap<String, String>,
}

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

    /// Page revision ID (from Wikimedia `ETag` header or `MediaWiki` `revid` field).
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
pub fn build_client() -> anyhow::Result<ClientWithMiddleware> {
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

/// Fetch a single article, dispatching to the correct API based on project type.
///
/// Wikimedia projects are fetched via the REST API; generic `MediaWiki` sites
/// use the `action=parse` API. Both paths produce identical output files.
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

/// Fetch an article from a Wikimedia project using the REST API.
///
/// The response is raw HTML with the revision ID in the `ETag` header.
async fn fetch_wikimedia_article(
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

/// Fetch an article from a generic `MediaWiki` site using the `action=parse` API.
///
/// The response is JSON containing HTML in `text["*"]` and a revision ID in
/// the `revid` field. The HTML body fragment is wrapped in a full document
/// for consistency with Wikimedia REST API output.
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

    let json: MediaWikiParseResponse = response.json().await.map_err(|e| {
        anyhow::anyhow!("Failed to parse MediaWiki API response for \"{title}\": {e}")
    })?;

    // Extract HTML from the text map (key is literally "*")
    let html = json.parse.text.get("*").ok_or_else(|| {
        anyhow::anyhow!("MediaWiki API response for \"{title}\" has no text content")
    })?;

    let html_bytes = html.len();
    let revision_id = json.parse.revid.map(|id| id.to_string());

    // Ensure staging directory exists
    std::fs::create_dir_all(STAGING_DIR)?;

    // Wrap the HTML body fragment in a full document for consistency
    let full_html = format!(
        "<!DOCTYPE html>\n<html>\n<head><title>{}</title></head>\n<body>\n{}\n</body>\n</html>",
        json.parse.displaytitle, html,
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
async fn batch_fetch(
    manifest: &Manifest,
    dry_run: bool,
    force: bool,
    tier: Option<&str>,
    category: Option<&str>,
) -> anyhow::Result<()> {
    let articles: Vec<&Article> = manifest
        .articles
        .iter()
        .filter(|a| tier.is_none_or(|t| a.tier == t) && category.is_none_or(|c| a.category == c))
        .collect();
    let total = articles.len();

    if dry_run {
        println!("Dry run — would fetch {total} articles:\n");
        for article in &articles {
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
    for article in &articles {
        let html_path = staging_html_path(&article.slug);
        if html_path.exists() && !force {
            result.skipped.push(article.slug.clone());
            let bar = multi_progress.add(ProgressBar::new_spinner());
            bar.set_style(item_style.clone());
            bar.set_message(format!("{}: skipped (cached)", article.slug));
            bar.finish();
            overall_bar.inc(1);
        } else {
            to_fetch.push(*article);
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

    // Report failures but don't abort — let the pipeline process whatever succeeded
    if !result.failed.is_empty() {
        eprintln!(
            "Warning: {} article(s) failed to fetch (see above). Continuing with pipeline for fetched articles.",
            result.failed.len(),
        );
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
pub(crate) fn chrono_now_iso8601() -> String {
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

pub(crate) fn is_leap_year(y: u64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

/// Run downstream pipeline stages (clean, rewrite, media, convert, frontmatter)
/// for a single article after a successful fetch.
///
/// Runs stages up to and including `stop_stage`. Skips the `Fetch` stage itself
/// (that was already done by the caller).
async fn run_downstream(
    slug: &str,
    manifest: &Manifest,
    article: &Article,
    client: &ClientWithMiddleware,
    stop_stage: super::PipelineStage,
    pandoc: bool,
) -> anyhow::Result<()> {
    use super::PipelineStage;

    if stop_stage == PipelineStage::Fetch {
        return Ok(());
    }

    // Clean
    eprintln!("  Pipeline: cleaning {slug}...");
    super::clean::clean_article(slug)?;
    if stop_stage == PipelineStage::Clean {
        return Ok(());
    }

    // Rewrite links
    eprintln!("  Pipeline: rewriting links for {slug}...");
    super::rewrite::rewrite_article(slug, manifest)?;
    if stop_stage == PipelineStage::Rewrite {
        return Ok(());
    }

    // Media (download + rewrite)
    eprintln!("  Pipeline: processing media for {slug}...");
    let media_result = super::media::process_article_media(client, slug, manifest, article).await?;
    super::media::rewrite_article_images(slug, &media_result)?;
    if stop_stage == PipelineStage::Media {
        return Ok(());
    }

    // Convert
    eprintln!("  Pipeline: converting {slug} to Markdown...");
    super::convert::reconvert_article(slug, pandoc)?;
    if stop_stage == PipelineStage::Convert {
        return Ok(());
    }

    // Frontmatter
    eprintln!("  Pipeline: injecting frontmatter for {slug}...");
    super::frontmatter::inject_frontmatter(slug, manifest)?;

    Ok(())
}

/// Fetch a single article and optionally run downstream pipeline stages.
async fn run_single(
    slug: &str,
    manifest: &Manifest,
    dry_run: bool,
    force: bool,
    pandoc: bool,
    stage: Option<super::PipelineStage>,
) -> anyhow::Result<()> {
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
    fetch_article(&client, manifest, article).await?;

    if let Some(stop_stage) = stage {
        run_downstream(slug, manifest, article, &client, stop_stage, pandoc).await?;
    }

    eprintln!("Done.");
    Ok(())
}

/// Run downstream pipeline stages for all fetched articles in the manifest.
async fn run_batch_pipeline(
    manifest: &Manifest,
    stage: super::PipelineStage,
    pandoc: bool,
    tier: Option<&str>,
    category: Option<&str>,
) -> anyhow::Result<()> {
    let client = build_client()?;
    let mut succeeded = 0;
    let mut failed: Vec<(String, String)> = Vec::new();

    let articles: Vec<&Article> = manifest
        .articles
        .iter()
        .filter(|a| tier.is_none_or(|t| a.tier == t) && category.is_none_or(|c| a.category == c))
        .collect();

    for article in articles {
        let html_path = staging_html_path(&article.slug);
        if !html_path.exists() {
            continue;
        }
        match run_downstream(&article.slug, manifest, article, &client, stage, pandoc).await {
            Ok(()) => succeeded += 1,
            Err(e) => failed.push((article.slug.clone(), e.to_string())),
        }
    }

    eprintln!();
    eprintln!(
        "Pipeline summary: {succeeded} succeeded, {} failed",
        failed.len(),
    );
    for (slug, msg) in &failed {
        eprintln!("  {slug}: {msg}");
    }

    Ok(())
}

/// Entry point for `haleiki demo fetch`.
///
/// When `article_slug` is `Some`, fetches a single article. When `None`,
/// fetches all manifest articles with bounded concurrency and progress bars.
///
/// Runs the downstream pipeline (clean, rewrite, media, convert, frontmatter)
/// after each successful fetch. The `stage` parameter controls how far the
/// pipeline runs: default (`None`) runs the full pipeline through frontmatter.
pub async fn run(
    article_slug: Option<&str>,
    dry_run: bool,
    force: bool,
    pandoc: bool,
    stage: Option<super::PipelineStage>,
    tier: Option<&str>,
    category: Option<&str>,
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

    // Validate tier/category filters against manifest taxonomy
    if let Some(tier) = tier {
        if !manifest.taxonomy.tiers.contains(&tier.to_string()) {
            anyhow::bail!(
                "unknown tier '{tier}'. Valid tiers: {}",
                manifest.taxonomy.tiers.join(", "),
            );
        }
    }
    if let Some(category) = category {
        if !manifest.taxonomy.categories.contains(&category.to_string()) {
            anyhow::bail!(
                "unknown category '{category}'. Valid categories: {}",
                manifest.taxonomy.categories.join(", "),
            );
        }
    }

    let effective_stage = Some(stage.unwrap_or(super::PipelineStage::Frontmatter));

    if let Some(slug) = article_slug {
        run_single(slug, &manifest, dry_run, force, pandoc, effective_stage).await?;
    } else {
        batch_fetch(&manifest, dry_run, force, tier, category).await?;
        if let Some(stop_stage) = effective_stage {
            if !dry_run {
                run_batch_pipeline(&manifest, stop_stage, pandoc, tier, category).await?;
            }
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

    // --- Project type detection tests ---

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

    #[test]
    fn test_is_wikimedia_project_simple_wikipedia() {
        assert!(is_wikimedia_project("simple.wikipedia.org"));
    }

    #[test]
    fn test_is_wikimedia_project_wikiquote() {
        assert!(is_wikimedia_project("en.wikiquote.org"));
    }

    #[test]
    fn test_is_wikimedia_project_german_wikinews() {
        assert!(is_wikimedia_project("de.wikinews.org"));
    }

    // --- URL encoding tests ---

    #[test]
    fn test_url_encode_title_ascii_passthrough() {
        assert_eq!(url_encode_title("Longchenpa"), "Longchenpa");
    }

    #[test]
    fn test_url_encode_title_spaces_become_plus() {
        assert_eq!(url_encode_title("Jikme Lingpa"), "Jikme+Lingpa");
    }

    #[test]
    fn test_url_encode_title_diacritics_percent_encoded() {
        // "Chögyam Trungpa" — ö is U+00F6, UTF-8 bytes: 0xC3 0xB6
        let encoded = url_encode_title("Chögyam Trungpa");
        assert_eq!(encoded, "Ch%C3%B6gyam+Trungpa");
    }

    #[test]
    fn test_url_encode_title_unreserved_chars_passthrough() {
        assert_eq!(url_encode_title("A-B_C.D~E"), "A-B_C.D~E");
    }

    #[test]
    fn test_url_encode_title_parentheses_encoded() {
        let encoded = url_encode_title("Foo (bar)");
        assert_eq!(encoded, "Foo+%28bar%29");
    }
}
