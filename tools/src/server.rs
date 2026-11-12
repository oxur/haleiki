//! The `tiny_http` server sink — Arc 03's second sink over the same render
//! pipeline `export.rs` walks and writes. "One renderer, two sinks"
//! (design-delta): [`render_routes`] does the one walk; `export::export_site`
//! writes its output to disk, [`serve`] here serves it from memory over HTTP.

use std::collections::BTreeMap;
use std::panic::{self, UnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

use anyhow::{Context, Result};
use tiny_http::{Header, Method, Response, Server};

use crate::article;
use crate::graph::HaleikiGraph;
use crate::news;
use crate::page;
use crate::parser;
use crate::render;
use crate::validator;

/// Site-asset subdirectories served (and, in `export.rs`, copied). Shared
/// between the two sinks so the allowlist never drifts between them.
pub(crate) const ASSET_ALLOWLIST: [&str; 6] = ["css", "fonts", "icons", "images", "js", "vendor"];

/// Number of worker threads handling requests concurrently. `tiny_http`'s
/// `Server` is documented for exactly this pattern: several threads calling
/// `recv()` on the same `Arc<Server>` (see its crate-level docs).
const WORKER_THREADS: usize = 4;

/// The rendered site: every page route keyed by URL path (`"/"`,
/// `"/browse/"`, `"/category/{slug}/"`, `"/{slug}/"`, `"/source/{slug}/"`,
/// `"/about/"`, `"/attribution/"`, `"/search/"`), plus the 404 body.
///
/// Immutable once built. Shared across worker threads via `Arc` only — no
/// `Mutex`/`RwLock`, no interior mutability. The site is read-only for the
/// lifetime of the process (live-render-per-request is explicitly out of
/// scope this slice), so there is nothing to synchronize.
#[derive(Debug, Clone)]
pub struct SiteRoutes {
    pub pages: BTreeMap<String, String>,
    pub not_found: String,
}

/// Renders every route from `content_dir`/`templates_dir` into a
/// [`SiteRoutes`]. This is the shared walk: `export::export_site` calls it
/// and writes the result to disk; [`serve`] calls it and holds the result in
/// memory. Composes the existing `page::render_*` / `article::render_*`
/// renderers — it does not re-render per request or per file.
///
/// # Errors
///
/// Returns an error if the corpus fails to parse or validate, the profile
/// fails to resolve, or a template fails to render.
pub fn render_routes(content_dir: &Path, templates_dir: &Path) -> Result<SiteRoutes> {
    let corpus = parser::parse_corpus(content_dir)
        .with_context(|| format!("failed to parse corpus from {}", content_dir.display()))?;

    let report = validator::validate(&corpus, content_dir);
    if report.has_errors() {
        report.print();
        anyhow::bail!("Validation failed with {} error(s).", report.errors.len());
    }

    let graph = HaleikiGraph::build(&corpus);
    let profile_path = content_dir.join("profile.yaml");
    let profile = render::resolve_profile(&profile_path).with_context(|| {
        format!(
            "failed to resolve site.data.profile from {}",
            profile_path.display()
        )
    })?;
    let site = render::assemble_site_context(&corpus, &graph, profile);
    let env = render::build_environment(templates_dir);

    let mut pages = BTreeMap::new();
    pages.insert("/".to_string(), page::render_home(&env, &site)?);
    pages.insert("/browse/".to_string(), page::render_browse(&env, &site)?);
    pages.insert("/about/".to_string(), page::render_about(&env, &site)?);
    pages.insert(
        "/attribution/".to_string(),
        page::render_attribution(&env, &site)?,
    );
    pages.insert("/search/".to_string(), page::render_search(&env, &site)?);

    let category_index = graph.category_index(&corpus);
    let mut category_names: Vec<&String> = category_index.keys().collect();
    category_names.sort();
    for name in category_names {
        let html = page::render_category(&env, &site, &corpus, &graph, name)?;
        pages.insert(format!("/category/{name}/"), html);
    }

    let mut concept_slugs: Vec<&String> = corpus.concepts.keys().collect();
    concept_slugs.sort();
    for slug in concept_slugs {
        let html = article::render_concept(&env, &site, &corpus, slug)?;
        pages.insert(format!("/{slug}/"), html);
    }

    let mut source_slugs: Vec<&String> = corpus.sources.keys().collect();
    source_slugs.sort();
    for slug in source_slugs {
        let html = article::render_source(&env, &site, &corpus, slug)?;
        pages.insert(format!("/source/{slug}/"), html);
    }

    let mut news_slugs: Vec<&String> = corpus.news.keys().collect();
    news_slugs.sort();
    for slug in news_slugs {
        let post = &corpus.news[slug];
        let route = post.url().to_owned();
        if pages.contains_key(&route) {
            anyhow::bail!(
                "news route collision at {route:?} for news slug {slug:?}; an existing page already uses this route"
            );
        }
        let html = news::render_post(&env, &site, &corpus, slug)?;
        pages.insert(route, html);
    }

    let not_found = page::render_404(&env, &site)?;

    Ok(SiteRoutes { pages, not_found })
}

/// The result of resolving a request: what to send back.
///
/// `content_length` is carried separately from `body.len()` so a `HEAD`
/// response can declare the same `Content-Length` a `GET` on the same route
/// would, while `body` itself is empty (see [`handle`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServedResponse {
    pub status: u16,
    pub content_type: &'static str,
    pub body: Vec<u8>,
    pub content_length: usize,
    pub allow: Option<&'static str>,
}

/// Resolves a request path against the route map and the on-disk asset
/// allowlist. **Pure** apart from the asset-file read: no other I/O, no
/// mutation, exhaustively unit-testable without a socket.
///
/// - `/assets/{allowlisted-subdir}/...` → the file's bytes, `200`, and a
///   content-type by extension; anything else under `/assets/` (missing
///   file, non-allowlisted subdir, `..` traversal) → `404`.
/// - A known page route → `200 text/html` with its body. Trailing slash is
///   normalized (`/browse` and `/browse/` both resolve; `/` is exempt, it
///   has no non-slash form). A query string (e.g. `/search/?q=x`) is
///   stripped before matching.
/// - Anything else → `404 text/html` with the site's 404 body.
#[must_use]
pub fn resolve(request_path: &str, routes: &SiteRoutes, assets_dir: &Path) -> ServedResponse {
    let path = request_path.split('?').next().unwrap_or(request_path);

    if let Some(asset_path) = path.strip_prefix("/assets/") {
        return resolve_asset(asset_path, assets_dir);
    }

    let normalized = normalize_page_path(path);
    if let Some(html) = routes.pages.get(&normalized) {
        let body = html.clone().into_bytes();
        return ServedResponse {
            status: 200,
            content_type: "text/html",
            content_length: body.len(),
            body,
            allow: None,
        };
    }

    let body = routes.not_found.clone().into_bytes();
    ServedResponse {
        status: 404,
        content_type: "text/html",
        content_length: body.len(),
        body,
        allow: None,
    }
}

/// Normalizes a page path to the canonical form every route key uses: a
/// trailing slash. `/` is already canonical; `/browse` and `/browse/` both
/// normalize to `/browse/`.
fn normalize_page_path(path: &str) -> String {
    if path == "/" {
        return "/".to_string();
    }
    format!("{}/", path.trim_end_matches('/'))
}

/// Resolves `/assets/{rel_path}` against the allowlist. `rel_path` is
/// everything after `/assets/`, e.g. `css/haleiki.css`.
fn resolve_asset(rel_path: &str, assets_dir: &Path) -> ServedResponse {
    let not_found = ServedResponse {
        status: 404,
        content_type: "text/html",
        body: Vec::new(),
        content_length: 0,
        allow: None,
    };

    let rel = Path::new(rel_path);

    // Reject traversal outright, before ever touching the filesystem.
    if rel
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return not_found;
    }

    let Some(top) = rel.components().next().and_then(|c| c.as_os_str().to_str()) else {
        return not_found;
    };
    if !ASSET_ALLOWLIST.contains(&top) {
        return not_found;
    }

    match std::fs::read(assets_dir.join(rel)) {
        Ok(body) => ServedResponse {
            status: 200,
            content_type: content_type_for(rel_path),
            content_length: body.len(),
            body,
            allow: None,
        },
        Err(_) => not_found,
    }
}

/// Content-type by file extension, for the asset allowlist's actual file
/// kinds (arc-03 slice-01 scope). Unrecognized extensions fall back to
/// `application/octet-stream` rather than guessing.
fn content_type_for(path: &str) -> &'static str {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match ext.as_str() {
        "css" => "text/css",
        "js" => "application/javascript",
        "woff2" => "font/woff2",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "json" => "application/json",
        "html" => "text/html",
        _ => "application/octet-stream",
    }
}

/// Resolves `/pagefind/{rel_path}` against `pagefind_dir` — the Pagefind
/// bundle `export::export_site` and [`serve`] both build via
/// [`crate::search::build_index`] (Arc 04 slice 01).
///
/// The traversal guard is **stricter** than [`resolve_asset`]'s: `/pagefind/`
/// is a single dedicated directory with no allowlist to fall back on, so
/// every path component must be [`std::path::Component::Normal`] — rejecting
/// not just `..` but also a leading `/` (`Path::join` with an absolute
/// operand *replaces* the base entirely, ignoring `pagefind_dir`, so an
/// unguarded `/pagefind//etc/passwd` would otherwise escape the bundle
/// directory the same way an unguarded `/assets/` request would if
/// `ASSET_ALLOWLIST` didn't happen to reject `RootDir`).
fn resolve_pagefind(rel_path: &str, pagefind_dir: &Path) -> ServedResponse {
    let not_found = ServedResponse {
        status: 404,
        content_type: "text/html",
        body: Vec::new(),
        content_length: 0,
        allow: None,
    };

    let rel = Path::new(rel_path);
    if rel
        .components()
        .any(|c| !matches!(c, std::path::Component::Normal(_)))
    {
        return not_found;
    }

    match std::fs::read(pagefind_dir.join(rel)) {
        Ok(body) => ServedResponse {
            status: 200,
            content_type: pagefind_content_type_for(rel_path),
            content_length: body.len(),
            body,
            allow: None,
        },
        Err(_) => not_found,
    }
}

/// Content-type by file extension for the Pagefind bundle's file kinds
/// (`pagefind.js`, `pagefind-entry.json`; the binary index/fragment/filter
/// files fall back to `application/octet-stream`, matching what they are).
///
/// **Spike finding (this slice):** the crate's actual WASM binary is named
/// `wasm.{lang}.pagefind` (confirmed against a real build and against the
/// crate's own `service::api` test) — extension `pagefind`, not the
/// `.wasm` the slice-doc assumed before this was spiked. Mapping `pagefind`
/// → `application/wasm` is correct for the real bundle; `wasm` is also
/// mapped for forward-compatibility, in case a future Pagefind version
/// changes the on-disk name back to a literal `.wasm`.
fn pagefind_content_type_for(path: &str) -> &'static str {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match ext.as_str() {
        "js" => "text/javascript",
        "wasm" | "pagefind" => "application/wasm",
        "json" => "application/json",
        _ => "application/octet-stream",
    }
}

/// Routes a request across pages/assets ([`handle`]) and the Pagefind bundle
/// served from `pagefind_dir` under `/pagefind/*`. Composed *alongside*
/// [`handle`] rather than folded into it or into [`resolve`] — neither's
/// existing signature or behavior changes for this slice, per the slice-doc's
/// boundary. Pure (bar the bundle-file read), method-aware the same way
/// [`handle`] is: `GET` serves the body, `HEAD` serves the same headers with
/// an empty body, anything else is `405` + `Allow: GET, HEAD`.
#[must_use]
pub fn handle_request(
    method: &Method,
    path: &str,
    routes: &SiteRoutes,
    assets_dir: &Path,
    pagefind_dir: &Path,
) -> ServedResponse {
    let request_path = path.split('?').next().unwrap_or(path);
    let Some(rel_path) = request_path.strip_prefix("/pagefind/") else {
        return handle(method, path, routes, assets_dir);
    };

    match method {
        Method::Get => resolve_pagefind(rel_path, pagefind_dir),
        Method::Head => {
            let full = resolve_pagefind(rel_path, pagefind_dir);
            ServedResponse {
                body: Vec::new(),
                ..full
            }
        }
        _ => {
            let body = b"405 Method Not Allowed".to_vec();
            ServedResponse {
                status: 405,
                content_type: "text/plain",
                content_length: body.len(),
                body,
                allow: Some("GET, HEAD"),
            }
        }
    }
}

/// Method-aware wrapper around [`resolve`]: `GET` serves the body; `HEAD`
/// serves the same status, `Content-Type`, and `Content-Length` a `GET`
/// would, with an **empty** body; anything else is `405 Method Not Allowed`
/// with `Allow: GET, HEAD`. Pure — unit-testable without a socket.
#[must_use]
pub fn handle(
    method: &Method,
    path: &str,
    routes: &SiteRoutes,
    assets_dir: &Path,
) -> ServedResponse {
    match method {
        Method::Get => resolve(path, routes, assets_dir),
        Method::Head => {
            let full = resolve(path, routes, assets_dir);
            ServedResponse {
                body: Vec::new(),
                ..full
            }
        }
        _ => {
            let body = b"405 Method Not Allowed".to_vec();
            ServedResponse {
                status: 405,
                content_type: "text/plain",
                content_length: body.len(),
                body,
                allow: Some("GET, HEAD"),
            }
        }
    }
}

/// Formats one access-log line: `"{method} {path} {status} {content_length}"`
/// (e.g. `"GET /abelian-group/ 200 18446"`). Pure, so it's directly
/// unit-testable; the serve loop prints it to stderr per request.
#[must_use]
pub fn access_log_line(method: &str, path: &str, status: u16, content_length: usize) -> String {
    format!("{method} {path} {status} {content_length}")
}

/// Runs `f` behind [`std::panic::catch_unwind`]: on a panic, returns `500`
/// instead of letting the panic unwind into the caller. Used to isolate a
/// single request's failure from the worker thread handling it — a panic
/// serving one request must never shrink the worker pool.
fn guarded<F>(f: F) -> ServedResponse
where
    F: FnOnce() -> ServedResponse + UnwindSafe,
{
    if let Ok(response) = panic::catch_unwind(f) {
        response
    } else {
        let body = b"Internal Server Error".to_vec();
        ServedResponse {
            status: 500,
            content_type: "text/plain",
            content_length: body.len(),
            body,
            allow: None,
        }
    }
}

/// A directory under the OS temp root, unique to this call (pid + a
/// nanosecond timestamp — no two `serve()` invocations, even started in the
/// same process, collide).
fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()))
}

/// Renders the site once, then serves it from memory over `bind` until the
/// process is killed. Holds the route map in an immutable `Arc<SiteRoutes>`
/// shared (lock-free) across a small fixed thread pool, following
/// `tiny_http`'s own documented multi-threading pattern: several worker
/// threads calling `server.recv()` on a shared `Arc<Server>`.
///
/// Also builds the Pagefind search index once at startup into a fresh temp
/// directory (Arc 04 slice 01, decision #2: disk, not in-memory — mirrors
/// Arc 03's "assets from disk") and serves `/pagefind/*` from it.
///
/// Each request is handled behind [`guarded`] (panic isolation), logged via
/// [`access_log_line`], and answered with an `X-Content-Type-Options:
/// nosniff` header alongside its `Content-Type`/`Content-Length`/`Allow`.
///
/// # Errors
///
/// Returns an error if the site or the search index fails to build, or the
/// server cannot bind `bind`.
pub fn serve(
    content_dir: &Path,
    templates_dir: &Path,
    assets_dir: &Path,
    bind: &str,
) -> Result<()> {
    let routes = Arc::new(render_routes(content_dir, templates_dir)?);
    let assets_dir = assets_dir.to_path_buf();

    let pagefind_dir = unique_temp_dir("haleiki-pagefind");
    crate::search::build_index(content_dir, &pagefind_dir)
        .context("failed to build the pagefind search index")?;

    let server = Server::http(bind).map_err(|e| anyhow::anyhow!("failed to bind {bind}: {e}"))?;
    let server = Arc::new(server);

    let workers: Vec<_> = (0..WORKER_THREADS)
        .map(|_| {
            let server = Arc::clone(&server);
            let routes = Arc::clone(&routes);
            let assets_dir = assets_dir.clone();
            let pagefind_dir = pagefind_dir.clone();
            thread::spawn(move || serve_loop(&server, &routes, &assets_dir, &pagefind_dir))
        })
        .collect();

    for worker in workers {
        let _ = worker.join();
    }

    Ok(())
}

/// Builds the `tiny_http` response for a [`ServedResponse`]: `Content-Type`,
/// an explicit `Content-Length` (so a `HEAD`'s empty body still declares the
/// length the matching `GET` would have sent), `X-Content-Type-Options:
/// nosniff` on every response, and `Allow` when set (405s).
fn build_response(resolved: ServedResponse) -> Response<std::io::Cursor<Vec<u8>>> {
    let content_type = Header::from_bytes("Content-Type", resolved.content_type)
        .expect("content-type values are always valid ASCII header values");
    let content_length = Header::from_bytes("Content-Length", resolved.content_length.to_string())
        .expect("a formatted usize is always a valid ASCII header value");
    let nosniff = Header::from_bytes("X-Content-Type-Options", "nosniff")
        .expect("nosniff is a valid ASCII header value");

    let mut response = Response::from_data(resolved.body)
        .with_status_code(resolved.status)
        .with_header(content_type)
        .with_header(content_length)
        .with_header(nosniff);

    if let Some(allow) = resolved.allow {
        let allow_header =
            Header::from_bytes("Allow", allow).expect("allow is a valid ASCII header value");
        response = response.with_header(allow_header);
    }

    response
}

/// One worker's request loop: `recv()` → [`handle_request`] (behind
/// [`guarded`]) → `respond`, until the server's listener is closed (`recv()`
/// returns an error). Logs each request via [`access_log_line`].
fn serve_loop(server: &Server, routes: &SiteRoutes, assets_dir: &Path, pagefind_dir: &Path) {
    loop {
        let Ok(request) = server.recv() else { return };

        let method = request.method().clone();
        let path = request.url().to_string();
        let resolved = guarded(|| handle_request(&method, &path, routes, assets_dir, pagefind_dir));

        eprintln!(
            "{}",
            access_log_line(
                method.as_str(),
                &path,
                resolved.status,
                resolved.content_length
            )
        );

        let response = build_response(resolved);
        let _ = request.respond(response);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::path::PathBuf;
    use std::sync::OnceLock;

    fn demo_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    fn templates_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates")
    }

    fn assets_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets")
    }

    /// The demo corpus's Pagefind bundle, built once and shared across every
    /// test in this module that needs to serve `/pagefind/*` — a real build
    /// takes several seconds, so tests share one rather than each building
    /// their own.
    fn demo_pagefind_dir() -> &'static Path {
        static DIR: OnceLock<PathBuf> = OnceLock::new();
        DIR.get_or_init(|| {
            let dir = std::env::temp_dir().join("haleiki-server-test-pagefind-bundle");
            let _ = std::fs::remove_dir_all(&dir);
            crate::search::build_index(&demo_dir(), &dir)
                .expect("pagefind bundle builds for the demo corpus");
            dir
        })
    }

    fn demo_routes() -> SiteRoutes {
        render_routes(&demo_dir(), &templates_dir()).expect("demo corpus renders")
    }

    // F-1: render_routes returns the full route set keyed by URL path.
    #[test]
    fn test_render_routes_returns_full_route_set() {
        let corpus = crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses");
        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let category_index = graph.category_index(&corpus);

        let routes = demo_routes();

        let mut expected: HashSet<String> = [
            "/".to_string(),
            "/browse/".to_string(),
            "/about/".to_string(),
            "/attribution/".to_string(),
            "/search/".to_string(),
        ]
        .into_iter()
        .collect();
        expected.extend(
            category_index
                .keys()
                .map(|name| format!("/category/{name}/")),
        );
        expected.extend(corpus.concepts.keys().map(|slug| format!("/{slug}/")));
        expected.extend(corpus.sources.keys().map(|slug| format!("/source/{slug}/")));
        expected.extend(corpus.news.values().map(|post| post.url().to_owned()));

        let actual: HashSet<String> = routes.pages.keys().cloned().collect();
        assert_eq!(actual, expected);
        assert!(!routes.not_found.is_empty());
    }

    // Arc 03 Slice 01, F-3: resolve returns 200 text/html with the correct
    // body for every kind of page route.
    #[test]
    fn test_resolve_returns_200_for_every_page_route_kind() {
        let routes = demo_routes();
        let assets = assets_dir();

        for path in [
            "/",
            "/browse/",
            "/about/",
            "/attribution/",
            "/search/",
            "/category/mathematics/",
            "/abelian-group/",
            "/source/vimalamitra/",
            "/news/search-lands-in-the-demo-wiki/",
        ] {
            let expected_body = routes
                .pages
                .get(path)
                .unwrap_or_else(|| panic!("{path} should be a real route"));
            let served = resolve(path, &routes, &assets);
            assert_eq!(served.status, 200, "path {path} should resolve 200");
            assert_eq!(served.content_type, "text/html");
            assert_eq!(
                served.body,
                expected_body.as_bytes(),
                "path {path} body should match the rendered route"
            );
        }
    }

    #[test]
    fn test_render_routes_rejects_news_collision_with_legacy_concept_path() {
        let dir = assert_fs::TempDir::new().expect("temporary directory");
        let demo = demo_dir();
        std::fs::copy(demo.join("taxonomy.yaml"), dir.path().join("taxonomy.yaml"))
            .expect("copy taxonomy");
        std::fs::copy(demo.join("profile.yaml"), dir.path().join("profile.yaml"))
            .expect("copy profile");
        let news_dir = dir.path().join("news");
        std::fs::create_dir_all(&news_dir).expect("news directory");
        std::fs::write(
            news_dir.join("search-lands-in-the-demo-wiki.md"),
            "---\ntype: news\ntitle: Search Lands\ndate: '2026-08-21'\ncategory: Project Updates\ntags: []\n---\nBody\n",
        )
        .expect("news post");
        let concepts_dir = dir.path().join("concepts");
        std::fs::create_dir_all(&concepts_dir).expect("concept directory");
        let mut legacy = std::fs::read_to_string(demo.join("concepts/abelian-group.md"))
            .expect("read a complete concept fixture");
        legacy = legacy.replace(
            "slug: \"abelian-group\"",
            "slug: \"news/search-lands-in-the-demo-wiki\"",
        );
        legacy = legacy.replace(
            "derived_from:\n  - slug: \"group-mathematics\"\n    title: \"Group (mathematics)\"\n  - slug: \"abstract-algebra\"\n    title: \"Abstract algebra\"",
            "derived_from: []",
        );
        std::fs::write(concepts_dir.join("legacy.md"), legacy).expect("legacy concept");

        let error = render_routes(dir.path(), &templates_dir())
            .expect_err("route collision must be rejected")
            .to_string();
        assert!(
            error.contains("/news/search-lands-in-the-demo-wiki/"),
            "{error}"
        );
        assert!(error.contains("news route collision"), "{error}");
    }

    #[test]
    fn test_news_route_coexists_with_concept_and_source_sharing_bare_slug() {
        let dir = assert_fs::TempDir::new().expect("temporary directory");
        let demo = demo_dir();
        std::fs::copy(demo.join("taxonomy.yaml"), dir.path().join("taxonomy.yaml"))
            .expect("copy taxonomy");
        std::fs::copy(demo.join("profile.yaml"), dir.path().join("profile.yaml"))
            .expect("copy profile");

        let news_dir = dir.path().join("news");
        std::fs::create_dir_all(&news_dir).expect("news directory");
        std::fs::write(
            news_dir.join("search-lands-in-the-demo-wiki.md"),
            "---\ntype: news\ntitle: Search Lands\ndate: '2026-08-21'\ncategory: Project Updates\ntags: []\n---\nBody\n",
        )
        .expect("news post");

        let concepts_dir = dir.path().join("concepts");
        std::fs::create_dir_all(&concepts_dir).expect("concept directory");
        let mut concept = std::fs::read_to_string(demo.join("concepts/abelian-group.md"))
            .expect("read a complete concept fixture");
        concept = concept.replace(
            "slug: \"abelian-group\"",
            "slug: \"search-lands-in-the-demo-wiki\"",
        );
        concept = concept.replace(
            "derived_from:\n  - slug: \"group-mathematics\"\n    title: \"Group (mathematics)\"\n  - slug: \"abstract-algebra\"\n    title: \"Abstract algebra\"",
            "derived_from: []",
        );
        std::fs::write(concepts_dir.join("same-slug.md"), concept)
            .expect("concept with bare news slug");

        let sources_dir = dir.path().join("sources");
        std::fs::create_dir_all(&sources_dir).expect("source directory");
        std::fs::write(
            sources_dir.join("same-slug.md"),
            "---\ntitle: Same Slug Source\nslug: search-lands-in-the-demo-wiki\npage_type: source\ncategory: mathematics\nsubcategory: algebra\ntier: intermediate\nkeywords: []\ntags: []\nextraction_status: complete\nconcepts_generated: []\nstatus: published\n---\nSource body\n",
        )
        .expect("source with bare news slug");

        let routes = render_routes(dir.path(), &templates_dir()).expect("routes render");
        for path in [
            "/news/search-lands-in-the-demo-wiki/",
            "/search-lands-in-the-demo-wiki/",
            "/source/search-lands-in-the-demo-wiki/",
        ] {
            assert!(routes.pages.contains_key(path), "missing route {path}");
        }
    }

    // F-4: resolve returns 404 + the 404 body for an unknown page path.
    #[test]
    fn test_resolve_returns_404_for_unknown_path() {
        let routes = demo_routes();
        let assets = assets_dir();

        let served = resolve("/this-page-does-not-exist/", &routes, &assets);
        assert_eq!(served.status, 404);
        assert_eq!(served.content_type, "text/html");
        assert_eq!(served.body, routes.not_found.as_bytes());
    }

    // F-5: /assets/... serves real allowlisted files with the correct
    // content-type; trailing-slash normalization; misses and non-allowlisted
    // paths 404.
    #[test]
    fn test_resolve_serves_assets_with_correct_content_type() {
        let routes = demo_routes();
        let assets = assets_dir();

        let css = resolve("/assets/css/haleiki.css", &routes, &assets);
        assert_eq!(css.status, 200);
        assert_eq!(css.content_type, "text/css");
        assert!(!css.body.is_empty());

        let js = resolve("/assets/js/browse-scrollspy.js", &routes, &assets);
        assert_eq!(js.status, 200);
        assert_eq!(js.content_type, "application/javascript");

        let font = resolve(
            "/assets/fonts/RedHatDisplay-Variable.woff2",
            &routes,
            &assets,
        );
        assert_eq!(font.status, 200);
        assert_eq!(font.content_type, "font/woff2");

        let icon = resolve("/assets/icons/favicon.ico", &routes, &assets);
        assert_eq!(icon.status, 200);
        assert_eq!(icon.content_type, "image/x-icon");

        let png = resolve("/assets/icons/android-chrome-192x192.png", &routes, &assets);
        assert_eq!(png.status, 200);
        assert_eq!(png.content_type, "image/png");

        // Missing file within an allowlisted dir.
        let missing = resolve("/assets/css/does-not-exist.css", &routes, &assets);
        assert_eq!(missing.status, 404);

        // Non-allowlisted top-level asset dir (ai/mockups are excluded, same
        // allowlist export.rs uses).
        let excluded = resolve("/assets/ai/SKILL.md", &routes, &assets);
        assert_eq!(excluded.status, 404);

        // Path traversal.
        let traversal = resolve("/assets/css/../../../../etc/passwd", &routes, &assets);
        assert_eq!(traversal.status, 404);
    }

    #[test]
    fn test_content_type_for_covers_every_allowlisted_extension() {
        assert_eq!(content_type_for("x.css"), "text/css");
        assert_eq!(content_type_for("x.js"), "application/javascript");
        assert_eq!(content_type_for("x.woff2"), "font/woff2");
        assert_eq!(content_type_for("x.svg"), "image/svg+xml");
        assert_eq!(content_type_for("x.png"), "image/png");
        assert_eq!(content_type_for("x.ico"), "image/x-icon");
        assert_eq!(content_type_for("x.json"), "application/json");
        assert_eq!(content_type_for("x.html"), "text/html");
        assert_eq!(content_type_for("x.unknownext"), "application/octet-stream");
    }

    #[test]
    fn test_resolve_trailing_slash_both_forms_resolve_the_same_route() {
        let routes = demo_routes();
        let assets = assets_dir();

        let with_slash = resolve("/browse/", &routes, &assets);
        let without_slash = resolve("/browse", &routes, &assets);
        assert_eq!(with_slash, without_slash);
        assert_eq!(with_slash.status, 200);

        // A query string on a page route is stripped before matching.
        let with_query = resolve("/search/?q=abelian", &routes, &assets);
        assert_eq!(with_query.status, 200);
        assert_eq!(with_query.body, routes.pages["/search/"].as_bytes());
    }

    // Arc 03 Slice 01, F-7 (parity half): a resolve() body equals the
    // corresponding file export_site writes for that route — one renderer,
    // two sinks, verified to actually agree.
    #[test]
    fn test_resolve_body_matches_exported_file_for_same_route() {
        let routes = demo_routes();
        let assets = assets_dir();

        let out = std::env::temp_dir().join("haleiki-server-parity-test");
        let _ = std::fs::remove_dir_all(&out);
        crate::export::export_site(&demo_dir(), &templates_dir(), &assets, &out)
            .expect("export succeeds");

        for (path, exported_rel) in [
            ("/", "index.html"),
            ("/category/mathematics/", "category/mathematics/index.html"),
            ("/abelian-group/", "abelian-group/index.html"),
            ("/source/vimalamitra/", "source/vimalamitra/index.html"),
            (
                "/news/search-lands-in-the-demo-wiki/",
                "news/search-lands-in-the-demo-wiki/index.html",
            ),
        ] {
            let served = resolve(path, &routes, &assets);
            let exported = std::fs::read(out.join(exported_rel)).unwrap();
            assert_eq!(
                served.body, exported,
                "{path} served body should equal the exported file {exported_rel}"
            );
        }

        let _ = std::fs::remove_dir_all(&out);
    }

    // F-7 (no-Liquid half): needle built by concatenation, not a literal —
    // include_str! pulls in this test's own source.
    #[test]
    fn test_server_path_references_no_liquid_templates() {
        let liquid_extension = format!(".{}{}", "liqu", "id");
        let server_src = include_str!("server.rs");

        assert!(
            !server_src.contains(&liquid_extension),
            "server.rs must not reference any Liquid template"
        );
    }

    // Arc 03 Slice 01, F-6: haleiki serve binds and serves live over an
    // immutable, lock-free Arc<SiteRoutes>. This test binds an ephemeral
    // port directly (rather than calling the blocking `serve()`, which loops
    // forever joining its workers) so it can shut down cleanly when the test
    // process exits.
    #[test]
    fn test_live_server_smoke_ephemeral_port() {
        let routes = Arc::new(demo_routes());
        let assets = assets_dir();

        let server = Server::http("127.0.0.1:0").expect("bind an ephemeral port");
        let port = server
            .server_addr()
            .to_ip()
            .expect("an IP listen address")
            .port();
        let server = Arc::new(server);

        {
            let server = Arc::clone(&server);
            let routes = Arc::clone(&routes);
            thread::spawn(move || {
                // Two requests is exactly what this test sends.
                for _ in 0..2 {
                    let Ok(request) = server.recv() else { break };
                    let resolved = resolve(request.url(), &routes, &assets);
                    let header = Header::from_bytes("Content-Type", resolved.content_type).unwrap();
                    let response = Response::from_data(resolved.body)
                        .with_status_code(resolved.status)
                        .with_header(header);
                    let _ = request.respond(response);
                }
            });
        }

        let (status, body) = http_get(port, "/");
        assert_eq!(status, 200);
        assert!(body.contains("hero-title"));

        let (status, _) = http_get(port, "/this-does-not-exist/");
        assert_eq!(status, 404);
    }

    /// A minimal, dependency-light raw HTTP/1.1 GET over `std::net`, per the
    /// slice-doc's explicit preference over pulling a full HTTP client into
    /// non-demo deps. Returns `(status_code, full_response_text)`.
    fn http_get(port: u16, path: &str) -> (u16, String) {
        use std::io::{Read, Write};

        let mut stream = std::net::TcpStream::connect(("127.0.0.1", port))
            .expect("connect to the live test server");
        write!(
            stream,
            "GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        )
        .unwrap();
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();

        let status = buf
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|code| code.parse().ok())
            .expect("a valid HTTP status line");
        (status, buf)
    }

    /// Like [`http_get`] but for an arbitrary method, returning
    /// `(status, headers, body)` split at the header/body blank line.
    fn http_request(port: u16, method: &str, path: &str) -> (u16, String, String) {
        use std::io::{Read, Write};

        let mut stream = std::net::TcpStream::connect(("127.0.0.1", port))
            .expect("connect to the live test server");
        write!(
            stream,
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        )
        .unwrap();
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();

        let status = buf
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|code| code.parse().ok())
            .expect("a valid HTTP status line");

        let (headers, body) = buf.split_once("\r\n\r\n").unwrap_or((buf.as_str(), ""));
        (status, headers.to_string(), body.to_string())
    }

    // Arc 03 Slice 02, F-1: HEAD serves the same status/headers as GET, with
    // an empty body.
    #[test]
    fn test_handle_head_matches_get_status_and_content_length_with_empty_body() {
        let routes = demo_routes();
        let assets = assets_dir();

        for path in ["/", "/browse/", "/abelian-group/"] {
            let get = handle(&Method::Get, path, &routes, &assets);
            let head = handle(&Method::Head, path, &routes, &assets);

            assert_eq!(head.status, get.status);
            assert_eq!(head.content_type, get.content_type);
            assert_eq!(head.content_length, get.content_length);
            assert_eq!(head.content_length, get.body.len());
            assert!(head.body.is_empty(), "HEAD body should be empty for {path}");
        }
    }

    // F-2: unsupported methods 405 with an Allow header.
    #[test]
    fn test_handle_rejects_unsupported_methods_with_405_and_allow_header() {
        let routes = demo_routes();
        let assets = assets_dir();

        for method in [Method::Post, Method::Put, Method::Delete] {
            let response = handle(&method, "/", &routes, &assets);
            assert_eq!(response.status, 405, "{method:?} should be 405");
            assert_eq!(response.allow, Some("GET, HEAD"));
            assert_eq!(response.content_length, response.body.len());
            assert!(!response.body.is_empty());
        }

        // GET and HEAD are not rejected.
        assert_eq!(handle(&Method::Get, "/", &routes, &assets).status, 200);
        assert_eq!(handle(&Method::Head, "/", &routes, &assets).status, 200);
    }

    // F-4: access_log_line is a pure formatter.
    #[test]
    fn test_access_log_line_formats_method_path_status_length() {
        assert_eq!(
            access_log_line("GET", "/abelian-group/", 200, 18446),
            "GET /abelian-group/ 200 18446"
        );
        assert_eq!(access_log_line("POST", "/", 405, 23), "POST / 405 23");
    }

    // F-3: the catch_unwind wrapper returns 500 on a panicking closure and
    // does not propagate the panic to the caller.
    #[test]
    fn test_guarded_returns_500_on_panic_and_does_not_propagate() {
        let resolved = guarded(|| panic!("deliberately triggered by this test"));

        assert_eq!(resolved.status, 500);
        assert_eq!(resolved.content_length, resolved.body.len());
        assert!(!resolved.body.is_empty());
    }

    // guarded() must be transparent for the non-panicking path.
    #[test]
    fn test_guarded_passes_through_a_normal_response_unchanged() {
        let routes = demo_routes();
        let assets = assets_dir();

        let direct = handle(&Method::Get, "/", &routes, &assets);
        let via_guard = guarded(|| handle(&Method::Get, "/", &routes, &assets));
        assert_eq!(direct, via_guard);
    }

    // F-5 (nosniff half): build_response attaches Content-Type,
    // Content-Length, and X-Content-Type-Options: nosniff.
    #[test]
    fn test_build_response_sets_content_type_length_and_nosniff_headers() {
        let resolved = ServedResponse {
            status: 200,
            content_type: "text/css",
            body: b"body".to_vec(),
            content_length: 4,
            allow: None,
        };
        let response = build_response(resolved);
        let headers = response.headers();
        let value_of = |name: &'static str| {
            headers
                .iter()
                .find(|h| h.field.equiv(name))
                .map(|h| h.value.as_str().to_string())
        };

        assert_eq!(response.status_code().0, 200);
        assert_eq!(value_of("Content-Type").as_deref(), Some("text/css"));
        // `tiny_http` tracks an explicitly-set `Content-Length` as
        // `data_length` internally rather than a plain header entry (it
        // only becomes a header at `raw_print` time).
        assert_eq!(response.data_length(), Some(4));
        assert_eq!(
            value_of("X-Content-Type-Options").as_deref(),
            Some("nosniff")
        );
        assert!(value_of("Allow").is_none());
    }

    #[test]
    fn test_build_response_sets_allow_header_for_405() {
        let resolved = ServedResponse {
            status: 405,
            content_type: "text/plain",
            body: Vec::new(),
            content_length: 0,
            allow: Some("GET, HEAD"),
        };
        let response = build_response(resolved);
        let value_of = |name: &'static str| {
            response
                .headers()
                .iter()
                .find(|h| h.field.equiv(name))
                .map(|h| h.value.as_str().to_string())
        };

        assert_eq!(value_of("Allow").as_deref(), Some("GET, HEAD"));
    }

    // F-5 (bad-bind half): an invalid --bind is a clear Err, not a panic.
    #[test]
    fn test_serve_returns_a_clear_error_on_an_invalid_bind_address() {
        let result = serve(
            &demo_dir(),
            &templates_dir(),
            &assets_dir(),
            "not-a-valid-address",
        );
        let err = result.expect_err("an invalid bind address should be a clear Err, not a panic");
        assert!(
            err.to_string().contains("failed to bind"),
            "error message should name the failure: {err}"
        );
    }

    // Arc 03 Slice 02, F-6 (= arc-plan C-3/C-5 at arc scale): a live,
    // hardened server (the real serve_loop) serves every route type, an
    // asset, 404, 405 + Allow, and HEAD with an empty body + nosniff header.
    #[test]
    fn test_live_hardened_server_serves_every_route_asset_404_405_and_head() {
        let routes = Arc::new(demo_routes());
        let assets = assets_dir();

        let server = Server::http("127.0.0.1:0").expect("bind an ephemeral port");
        let port = server
            .server_addr()
            .to_ip()
            .expect("an IP listen address")
            .port();
        let server = Arc::new(server);

        {
            let server = Arc::clone(&server);
            let routes = Arc::clone(&routes);
            let assets = assets.clone();
            let pagefind = demo_pagefind_dir().to_path_buf();
            thread::spawn(move || serve_loop(&server, &routes, &assets, &pagefind));
        }

        for path in [
            "/",
            "/browse/",
            "/about/",
            "/attribution/",
            "/search/",
            "/category/mathematics/",
            "/abelian-group/",
            "/source/vimalamitra/",
            "/news/search-lands-in-the-demo-wiki/",
        ] {
            let (status, headers, _) = http_request(port, "GET", path);
            assert_eq!(status, 200, "GET {path}");
            let lower = headers.to_ascii_lowercase();
            assert!(
                lower.contains("content-type: text/html"),
                "GET {path} headers: {headers}"
            );
            assert!(lower.contains("x-content-type-options: nosniff"));
        }

        let (status, headers, _) = http_request(port, "GET", "/assets/css/haleiki.css");
        assert_eq!(status, 200);
        assert!(
            headers
                .to_ascii_lowercase()
                .contains("content-type: text/css")
        );

        let (status, _, body) = http_request(port, "GET", "/this-page-does-not-exist/");
        assert_eq!(status, 404);
        assert!(!body.is_empty());

        let (status, headers, body) = http_request(port, "HEAD", "/");
        assert_eq!(status, 200);
        assert!(body.is_empty(), "HEAD body should be empty over the wire");
        assert!(headers.to_ascii_lowercase().contains("content-length:"));

        let (status, headers, _) = http_request(port, "POST", "/");
        assert_eq!(status, 405);
        assert!(headers.to_ascii_lowercase().contains("allow: get, head"));
    }

    // F-7: server↔export parity still holds after hardening (reconfirms the
    // slice-01 guarantee against the hardened `handle` path, not just
    // `resolve`), and the no-Liquid property still holds.
    #[test]
    fn test_handle_body_matches_exported_file_for_same_route() {
        let routes = demo_routes();
        let assets = assets_dir();

        let out = std::env::temp_dir().join("haleiki-server-hardening-parity-test");
        let _ = std::fs::remove_dir_all(&out);
        crate::export::export_site(&demo_dir(), &templates_dir(), &assets, &out)
            .expect("export succeeds");

        let served = handle(&Method::Get, "/abelian-group/", &routes, &assets);
        let exported = std::fs::read(out.join("abelian-group/index.html")).unwrap();
        assert_eq!(served.body, exported);

        let _ = std::fs::remove_dir_all(&out);
    }

    // Arc 03 Slice 02, F-3 (live half): a request that panics is answered
    // 500 and the same worker keeps serving the next, normal request. Uses
    // the real `guarded`/`handle`/`build_response` production functions; the
    // panic trigger itself is test-local (a magic path this test's own
    // closure recognizes), not a backdoor in production code.
    #[test]
    fn test_live_server_survives_a_panicking_request_and_serves_the_next() {
        let routes = Arc::new(demo_routes());
        let assets = assets_dir();

        let server = Server::http("127.0.0.1:0").expect("bind an ephemeral port");
        let port = server
            .server_addr()
            .to_ip()
            .expect("an IP listen address")
            .port();
        let server = Arc::new(server);

        {
            let server = Arc::clone(&server);
            let routes = Arc::clone(&routes);
            let assets = assets.clone();
            thread::spawn(move || {
                for _ in 0..2 {
                    let Ok(request) = server.recv() else { break };
                    let path = request.url().to_string();
                    let resolved = guarded(|| {
                        assert!(
                            path != "/__test_induced_panic__",
                            "deliberately triggered by this test"
                        );
                        handle(&Method::Get, &path, &routes, &assets)
                    });
                    let response = build_response(resolved);
                    let _ = request.respond(response);
                }
            });
        }

        let (status, _, _) = http_request(port, "GET", "/__test_induced_panic__");
        assert_eq!(status, 500);

        let (status, _, body) = http_request(port, "GET", "/");
        assert_eq!(status, 200);
        assert!(body.contains("hero-title"));
    }

    // Arc 04 Slice 01, G-6: resolve_pagefind (via handle_request) serves the
    // real bundle's file kinds with the correct content-types.
    #[test]
    fn test_handle_request_serves_pagefind_bundle_file_kinds() {
        let routes = Arc::new(demo_routes());
        let assets = assets_dir();
        let pagefind = demo_pagefind_dir();

        let js = handle_request(
            &Method::Get,
            "/pagefind/pagefind.js",
            &routes,
            &assets,
            pagefind,
        );
        assert_eq!(js.status, 200);
        assert_eq!(js.content_type, "text/javascript");
        assert!(!js.body.is_empty());

        let entry = handle_request(
            &Method::Get,
            "/pagefind/pagefind-entry.json",
            &routes,
            &assets,
            pagefind,
        );
        assert_eq!(entry.status, 200);
        assert_eq!(entry.content_type, "application/json");

        // The real wasm binary's on-disk name is `wasm.{lang}.pagefind`
        // (see `pagefind_content_type_for`'s doc comment) — find it rather
        // than hardcoding the language-tagged filename.
        let wasm_name = std::fs::read_dir(pagefind)
            .unwrap()
            .filter_map(std::result::Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .find(|name| name.ends_with(".pagefind"))
            .expect("a wasm.*.pagefind file exists in the bundle");
        let wasm = handle_request(
            &Method::Get,
            &format!("/pagefind/{wasm_name}"),
            &routes,
            &assets,
            pagefind,
        );
        assert_eq!(wasm.status, 200);
        assert_eq!(wasm.content_type, "application/wasm");
        assert!(!wasm.body.is_empty());

        let fragment_name = std::fs::read_dir(pagefind.join("fragment"))
            .unwrap()
            .filter_map(std::result::Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .next()
            .expect("at least one fragment file exists");
        let fragment = handle_request(
            &Method::Get,
            &format!("/pagefind/fragment/{fragment_name}"),
            &routes,
            &assets,
            pagefind,
        );
        assert_eq!(fragment.status, 200);
        assert_eq!(fragment.content_type, "application/octet-stream");
        assert!(!fragment.body.is_empty());
    }

    // G-7: a /pagefind/… traversal and a missing bundle file both 404;
    // HEAD/POST compose correctly with pagefind routing too.
    #[test]
    fn test_handle_request_pagefind_traversal_and_missing_file_404() {
        let routes = Arc::new(demo_routes());
        let assets = assets_dir();
        let pagefind = demo_pagefind_dir();

        let traversal = handle_request(
            &Method::Get,
            "/pagefind/../../../../etc/passwd",
            &routes,
            &assets,
            pagefind,
        );
        assert_eq!(traversal.status, 404);

        // An absolute-path operand would make `Path::join` discard
        // `pagefind_dir` entirely if unguarded (see the doc comment on
        // `resolve_pagefind`) — assert it doesn't escape the bundle root.
        let absolute = handle_request(
            &Method::Get,
            "/pagefind//etc/passwd",
            &routes,
            &assets,
            pagefind,
        );
        assert_eq!(absolute.status, 404);

        let missing = handle_request(
            &Method::Get,
            "/pagefind/does-not-exist.js",
            &routes,
            &assets,
            pagefind,
        );
        assert_eq!(missing.status, 404);

        let head = handle_request(
            &Method::Head,
            "/pagefind/pagefind.js",
            &routes,
            &assets,
            pagefind,
        );
        assert_eq!(head.status, 200);
        assert!(head.body.is_empty());
        assert!(head.content_length > 0);

        let post = handle_request(
            &Method::Post,
            "/pagefind/pagefind.js",
            &routes,
            &assets,
            pagefind,
        );
        assert_eq!(post.status, 405);
        assert_eq!(post.allow, Some("GET, HEAD"));
    }

    // G-6 (live half): a real server serves /pagefind/* over a socket.
    #[test]
    fn test_live_server_serves_pagefind_js_and_404s_a_traversal() {
        let routes = Arc::new(demo_routes());
        let assets = assets_dir();
        let pagefind = demo_pagefind_dir().to_path_buf();

        let server = Server::http("127.0.0.1:0").expect("bind an ephemeral port");
        let port = server
            .server_addr()
            .to_ip()
            .expect("an IP listen address")
            .port();
        let server = Arc::new(server);

        {
            let server = Arc::clone(&server);
            let routes = Arc::clone(&routes);
            thread::spawn(move || serve_loop(&server, &routes, &assets, &pagefind));
        }

        let (status, headers, _) = http_request(port, "GET", "/pagefind/pagefind.js");
        assert_eq!(status, 200);
        assert!(
            headers
                .to_ascii_lowercase()
                .contains("content-type: text/javascript")
        );

        let (status, _, _) = http_request(port, "GET", "/pagefind/../../../../../../etc/passwd");
        assert_eq!(status, 404);
    }
}
