# Milestone 2.2 — Batch Fetching with Progress

**Version:** 1.0
**Depends on:** Milestone 2.1 (single-article fetch works)
**Produces:** `haleiki demo fetch` downloads all 12 articles with progress bar

---

## Overview

Extend `haleiki demo fetch` (no `--article` flag) to fetch all manifest articles concurrently with bounded parallelism. Add `indicatif` progress bars showing per-article status. Implement `--dry-run` for batch mode. Add caching: skip articles whose staging HTML already exists unless `--force` is passed.

---

## Step 1: Implement batch fetch logic in `tools/src/demo/fetch.rs`

### Design Decisions

- **Bounded concurrency**: Use `tokio::sync::Semaphore` to limit concurrent HTTP requests. Default: 4 concurrent fetches. Wikimedia allows 200 req/s but we're polite.
- **Rate limiting**: Insert a small delay between request starts (500ms). Combined with bounded concurrency, this keeps us well under Wikimedia's limits.
- **Progress**: Use `indicatif::MultiProgress` with one progress bar per article. Each bar shows the article slug and transitions through states: waiting → fetching → done/failed.
- **Error handling**: Individual article failures do NOT abort the batch. Errors are collected and reported at the end. The command exits non-zero if any article failed.
- **Caching**: Skip articles where `demo/.staging/{slug}.html` exists, unless `--force` is passed. Print a "skipped" message for each cached article.

### Update the `run()` function in `fetch.rs`

Replace the `None` (batch) arm in the `run()` function:

```rust
None => {
    // Batch fetch all articles
    batch_fetch(&manifest, dry_run, force).await?;
}
```

### Add the batch fetch implementation

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Maximum number of concurrent HTTP requests.
const MAX_CONCURRENT_FETCHES: usize = 4;

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
async fn batch_fetch(
    manifest: &Manifest,
    dry_run: bool,
    force: bool,
) -> anyhow::Result<()> {
    let articles = &manifest.articles;
    let total = articles.len();

    if dry_run {
        println!("Dry run — would fetch {} articles:\n", total);
        for article in articles {
            let url = manifest.api_url(article);
            let project = manifest.effective_project(article);
            let cached = staging_html_path(&article.slug).exists();
            let cache_note = if cached && !force { " [cached, would skip]" } else { "" };
            println!("  {:<35} {}{}", article.slug, url, cache_note);
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
    let multi_progress = indicatif::MultiProgress::new();
    let overall_style = indicatif::ProgressStyle::with_template(
        "{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})"
    )
    .unwrap()
    .progress_chars("█▓░");

    let overall_bar = multi_progress.add(indicatif::ProgressBar::new(total as u64));
    overall_bar.set_style(overall_style);
    overall_bar.set_message("Fetching articles");

    let item_style = indicatif::ProgressStyle::with_template(
        "  {spinner:.green} {msg}"
    )
    .unwrap();

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
            let bar = multi_progress.add(indicatif::ProgressBar::new_spinner());
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

    // Spawn concurrent fetch tasks
    for article in &to_fetch {
        let client = Arc::clone(&client);
        let semaphore = Arc::clone(&semaphore);
        let slug = article.slug.clone();
        let title = article.title.clone();
        let api_url = manifest.api_url(article);
        let project = manifest.effective_project(article).to_string();
        let article_clone = (*article).clone();
        let manifest_defaults = manifest.defaults.clone();
        let manifest_taxonomy = manifest.taxonomy.clone();

        let bar = multi_progress.add(indicatif::ProgressBar::new_spinner());
        bar.set_style(item_style.clone());
        bar.set_message(format!("{slug}: waiting..."));
        bar.enable_steady_tick(std::time::Duration::from_millis(100));

        let overall_bar = overall_bar.clone();

        let handle = tokio::spawn(async move {
            // Acquire semaphore permit (bounds concurrency)
            let _permit = semaphore.acquire().await.unwrap();

            bar.set_message(format!("{slug}: fetching..."));

            // Rate-limit delay
            tokio::time::sleep(REQUEST_DELAY).await;

            // Reconstruct a minimal Manifest just for api_url/effective_project
            // (We can't send the full Manifest across threads easily, so we
            //  already resolved the URL above. Use fetch_article_raw instead.)
            let result = fetch_article_raw(&client, &slug, &title, &api_url, &project).await;

            match result {
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
                    bar.set_message(format!("{slug}: FAILED — {e}"));
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
        anyhow::bail!(
            "{} article(s) failed to fetch",
            result.failed.len()
        );
    }

    Ok(())
}

/// Fetch a single article given pre-resolved URL and project.
///
/// This is a lower-level version of `fetch_article` that doesn't need a
/// `Manifest` reference, making it easier to use from spawned tasks.
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
```

### Refactoring note

The original `fetch_article()` function from milestone 2.1 and the new `fetch_article_raw()` share most of their logic. Consider refactoring `fetch_article()` to delegate to `fetch_article_raw()`:

```rust
pub async fn fetch_article(
    client: &ClientWithMiddleware,
    manifest: &Manifest,
    article: &Article,
) -> anyhow::Result<FetchMeta> {
    let api_url = manifest.api_url(article);
    let project = manifest.effective_project(article).to_string();

    eprintln!("  Fetching: {} ({})", article.title, project);

    fetch_article_raw(client, &article.slug, &article.title, &api_url, &project).await
}
```

---

## Step 2: Handle `indicatif` terminal behavior

### Stderr vs stdout

Progress bars should render to stderr so they don't interfere with any data output on stdout. `indicatif` writes to stderr by default, which is correct.

### Non-interactive terminals (CI)

In CI environments, `indicatif` progress bars may produce noisy output. `indicatif` handles this gracefully — when stderr is not a TTY, it falls back to simpler output. No special handling needed.

---

## Step 3: Tests

### Unit tests — add to `fetch.rs` `#[cfg(test)]` module

```rust
// ─── Batch caching logic tests ──────────────────────

#[test]
fn test_staging_html_path_exists_check() {
    // Verify that non-existent paths return false
    let path = staging_html_path("nonexistent-article-xyz");
    assert!(!path.exists());
}
```

### Integration tests — add to `tools/tests/demo_fetch.rs`

```rust
#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_batch_dry_run() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Dry run")
                .and(predicate::str::contains("12 articles"))
                .and(predicate::str::contains("memory-management"))
                .and(predicate::str::contains("garbage-collection"))
                .and(predicate::str::contains("wikibooks-memory-management"))
                .and(predicate::str::contains("en.wikibooks.org")),
        );
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_batch_dry_run_shows_urls() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("api/rest_v1/page/html/Memory_management")
                .and(predicate::str::contains(
                    "api/rest_v1/page/html/Garbage_collection_(computer_science)",
                )),
        );
}

#[test]
#[cfg(feature = "demo")]
fn test_demo_fetch_batch_dry_run_shows_would_fetch_count() {
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch", "--dry-run"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(predicate::str::contains("Would fetch:"));
}
```

### Live batch test (optional, network-dependent)

```rust
#[test]
#[ignore] // Requires network access, takes ~10s
#[cfg(feature = "demo")]
fn test_demo_fetch_batch_live() {
    use std::path::Path;

    let staging_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("demo/.staging");

    // Clean staging directory
    if staging_dir.exists() {
        for entry in std::fs::read_dir(&staging_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "html" || e == "json") {
                let _ = std::fs::remove_file(&path);
            }
        }
    }

    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .timeout(std::time::Duration::from_secs(120))
        .assert()
        .success()
        .stdout(predicate::str::contains("Fetched: 12"));

    // Verify all 12 HTML files exist
    let expected_slugs = [
        "memory-management", "garbage-collection", "reference-counting",
        "stack-memory", "region-based-memory", "raii", "smart-pointer",
        "pointer", "dangling-pointer", "memory-safety", "type-safety",
        "wikibooks-memory-management",
    ];

    for slug in &expected_slugs {
        let html = staging_dir.join(format!("{slug}.html"));
        assert!(html.exists(), "Missing: {}", html.display());
        let meta = staging_dir.join(format!("{slug}.meta.json"));
        assert!(meta.exists(), "Missing: {}", meta.display());
    }

    // Second run should skip all (cached)
    Command::cargo_bin("haleiki")
        .unwrap()
        .args(["demo", "fetch"])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        .assert()
        .success()
        .stdout(predicate::str::contains("Skipped: 12"));

    // Clean up
    for slug in &expected_slugs {
        let _ = std::fs::remove_file(staging_dir.join(format!("{slug}.html")));
        let _ = std::fs::remove_file(staging_dir.join(format!("{slug}.meta.json")));
    }
}
```

---

## Step 4: Update `haleiki demo status` to show staging details

### Optional enhancement to `status.rs`

After batch fetch, `haleiki demo status` should show how many articles are staged. The `FetchState::Staged` variant from milestone 2.1's status update handles this automatically. No additional changes needed beyond what was specified in milestone 2.1.

Optionally, add a column for the fetch timestamp or revision ID by reading `demo/.staging/{slug}.meta.json`:

```rust
// In status.rs — optional enhancement
fn fetch_info(slug: &str) -> Option<(String, Option<String>)> {
    let meta_path = Path::new("demo/.staging").join(format!("{slug}.meta.json"));
    let content = std::fs::read_to_string(&meta_path).ok()?;
    let meta: serde_json::Value = serde_json::from_str(&content).ok()?;
    let fetched_at = meta.get("fetched_at")?.as_str()?.to_string();
    let revision_id = meta.get("revision_id").and_then(|v| v.as_str()).map(String::from);
    Some((fetched_at, revision_id))
}
```

This is a nice-to-have for this milestone — the project plan assigns full status implementation to milestone 6.4.

---

## Verification

### 5.1: Batch dry-run

```bash
cd /Users/oubiwann/lab/oxur/haleiki
cargo run --features demo -- demo fetch --dry-run
```

Expected output:
```
Dry run — would fetch 12 articles:

  memory-management                   https://en.wikipedia.org/api/rest_v1/page/html/Memory_management
  garbage-collection                  https://en.wikipedia.org/api/rest_v1/page/html/Garbage_collection_(computer_science)
  ...
  wikibooks-memory-management         https://en.wikibooks.org/api/rest_v1/page/html/Introduction_to_Computer_Science/Memory_Management

Would fetch: 12, would skip: 0
```

### 5.2: Batch live fetch (manual, requires network)

```bash
cargo run --features demo -- demo fetch
```

Expected:
- Progress bars showing each article being fetched
- All 12 `.html` and `.meta.json` files created in `demo/.staging/`
- Summary showing "Fetched: 12  Skipped: 0  Failed: 0"

### 5.3: Second run skips cached articles

```bash
cargo run --features demo -- demo fetch
```

Expected:
- All 12 articles skipped (cached)
- Summary: "Fetched: 0  Skipped: 12  Failed: 0"

### 5.4: Force re-fetch

```bash
cargo run --features demo -- demo fetch --force
```

Expected:
- All 12 articles re-fetched despite existing cache
- Summary: "Fetched: 12  Skipped: 0  Failed: 0"

### 5.5: Mixed state (some cached, some new)

```bash
# Remove one article's staging file
rm demo/.staging/garbage-collection.html
cargo run --features demo -- demo fetch
```

Expected:
- 1 article fetched, 11 skipped

### 5.6: Status after batch fetch

```bash
cargo run --features demo -- demo status
```

Expected: All 12 articles show `staged` status.

### 5.7: Tests pass

```bash
cargo test --features demo
```

### 5.8: Lint passes

```bash
make lint
```

---

## Acceptance Criteria

- [ ] `haleiki demo fetch` (no `--article`) fetches all 12 manifest articles
- [ ] Concurrent fetching with bounded parallelism (max 4 simultaneous requests)
- [ ] Rate limiting delay between requests (500ms)
- [ ] `indicatif` progress bars show per-article status during fetch
- [ ] Summary printed after batch: fetched/skipped/failed counts
- [ ] Articles with existing staging HTML are skipped unless `--force`
- [ ] `--force` re-fetches all articles regardless of cache
- [ ] `--dry-run` lists all articles with their API URLs without fetching
- [ ] `--dry-run` shows which articles would be skipped (cached)
- [ ] Individual article failures don't abort the batch
- [ ] Failed articles are listed in the summary
- [ ] Command exits non-zero if any article failed
- [ ] Wikibooks article uses `en.wikibooks.org` project domain
- [ ] All 12 `.html` and `.meta.json` files created after successful batch fetch
- [ ] `haleiki demo status` shows `staged` for all fetched articles
- [ ] All unit tests pass
- [ ] Integration tests pass (dry-run batch mode)
- [ ] `make lint` passes
- [ ] No compiler warnings

---

## Gotchas

1. **Sending `Manifest` across threads**: `Manifest` contains `String` fields and `Vec`s, so it's `Send + Sync`. However, passing it into spawned tokio tasks requires either `Arc<Manifest>` or pre-resolving the values (URLs, project, etc.) before spawning. The plan above pre-resolves URLs to avoid this complexity.

2. **`std::fs` in async tasks**: `std::fs::write` blocks the tokio runtime thread. For 12 articles writing ~50KB each, this is negligible. For larger-scale usage, switch to `tokio::fs`. Don't optimize prematurely in this milestone.

3. **`indicatif` + `eprintln!`**: Mixing `eprintln!` with `indicatif` progress bars can cause garbled output. Use `multi_progress.println()` or `bar.println()` if you need to print messages during progress display. Better: let the progress bars handle all visual feedback.

4. **`create_dir_all` race condition**: Multiple concurrent tasks calling `create_dir_all(STAGING_DIR)` is safe — the function is idempotent.

5. **Error propagation from spawned tasks**: `tokio::spawn` returns `JoinHandle<T>`. If the spawned task panics, `handle.await` returns `Err(JoinError)`. The `?` on `handle.await?` propagates this. The inner `Result` (from the fetch) is the `Ok`/`Err` of the task's return value.

6. **Semaphore fairness**: `tokio::sync::Semaphore` is fair (FIFO). Tasks acquire permits in order, so articles are fetched roughly in manifest order.

7. **Terminal width**: `indicatif` progress bar templates should work in terminals of 80+ columns. Test with narrow terminals to ensure no wrapping issues.

8. **Timeout**: Individual HTTP requests have a 30s timeout (set on the client). The overall batch operation has no timeout. If needed, add a `--timeout` flag later.

9. **Cleanup on Ctrl-C**: If the user presses Ctrl-C during a batch fetch, partial `.html` files may be left in staging. This is fine — the next `fetch` run will detect them as cached and skip (or `--force` will overwrite). No special signal handling needed in this milestone.

10. **`htmd` dependency unused**: The `htmd` crate is declared as an optional dependency but not used until milestone 5.x (Markdown conversion). Clippy may warn about unused dependencies if it checks this. The `#[cfg(feature = "demo")]` gating should prevent this, but verify.
