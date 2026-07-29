//! The `tiny_http` server sink — Arc 03's second sink over the same render
//! pipeline `export.rs` walks and writes. "One renderer, two sinks"
//! (design-delta): [`render_routes`] does the one walk; `export::export_site`
//! writes its output to disk, [`serve`] here serves it from memory over HTTP.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::thread;

use anyhow::{Context, Result};
use tiny_http::{Header, Response, Server};

use crate::article;
use crate::graph::HaleikiGraph;
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

    let not_found = page::render_404(&env, &site)?;

    Ok(SiteRoutes { pages, not_found })
}

/// The result of resolving a request path: what to send back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServedResponse {
    pub status: u16,
    pub content_type: &'static str,
    pub body: Vec<u8>,
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
        return ServedResponse {
            status: 200,
            content_type: "text/html",
            body: html.clone().into_bytes(),
        };
    }

    ServedResponse {
        status: 404,
        content_type: "text/html",
        body: routes.not_found.clone().into_bytes(),
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
            body,
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

/// Renders the site once, then serves it from memory over `bind` until the
/// process is killed. Holds the route map in an immutable `Arc<SiteRoutes>`
/// shared (lock-free) across a small fixed thread pool, following
/// `tiny_http`'s own documented multi-threading pattern: several worker
/// threads calling `server.recv()` on a shared `Arc<Server>`.
///
/// No graceful shutdown, request logging, or panic isolation yet (arc-03
/// slice 02 hardens this).
///
/// # Errors
///
/// Returns an error if the site fails to render or the server cannot bind
/// `bind`.
pub fn serve(
    content_dir: &Path,
    templates_dir: &Path,
    assets_dir: &Path,
    bind: &str,
) -> Result<()> {
    let routes = Arc::new(render_routes(content_dir, templates_dir)?);
    let assets_dir = assets_dir.to_path_buf();

    let server = Server::http(bind).map_err(|e| anyhow::anyhow!("failed to bind {bind}: {e}"))?;
    let server = Arc::new(server);

    let workers: Vec<_> = (0..WORKER_THREADS)
        .map(|_| {
            let server = Arc::clone(&server);
            let routes = Arc::clone(&routes);
            let assets_dir = assets_dir.clone();
            thread::spawn(move || serve_loop(&server, &routes, &assets_dir))
        })
        .collect();

    for worker in workers {
        let _ = worker.join();
    }

    Ok(())
}

/// One worker's request loop: `recv()` → `resolve` → `respond`, until the
/// server's listener is closed (`recv()` returns an error).
fn serve_loop(server: &Server, routes: &SiteRoutes, assets_dir: &Path) {
    loop {
        let Ok(request) = server.recv() else { return };

        let resolved = resolve(request.url(), routes, assets_dir);
        let header = Header::from_bytes("Content-Type", resolved.content_type)
            .expect("content-type values are always valid ASCII header values");
        let response = Response::from_data(resolved.body)
            .with_status_code(resolved.status)
            .with_header(header);
        let _ = request.respond(response);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::path::PathBuf;

    fn demo_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    fn templates_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates")
    }

    fn assets_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets")
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
}
