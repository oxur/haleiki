# Slice 01 — route map + core server

> Plan-of-record for the first slice of Arc 03 (tiny_http Server). Open set:
> this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** (planned, not implemented) · 2026-07-28.

---

## Goal

Extract the export's route-walk into a **shared route map** that both the static
export and the server consume, add `tiny_http`, and stand up a working
`haleiki serve`: a **pure** request router plus a serve loop that holds the
rendered site immutably in memory and serves HTML routes + static assets. This
is the load-bearing core of the server sink; slice 02 hardens it.

## Design (one renderer, two sinks — realized)

- **`render_routes(content_dir, templates_dir) -> SiteRoutes`** — refactored out
  of `export::export_site`'s walk. `SiteRoutes` holds the page routes keyed by
  **URL path** (`"/"`, `"/browse/"`, `"/about/"`, `"/attribution/"`,
  `"/search/"`, `"/category/{slug}/"`, `"/{slug}/"`, `"/source/{slug}/"`) → the
  rendered HTML, plus the `404` body. `export_site` is reimplemented on top of it
  (walk → write), so its output stays **byte-identical** (the export tests guard
  this).
- **`resolve(path, &routes, assets_dir) -> ServedResponse{status, content_type,
  body}`** — a **pure** function (no I/O beyond reading an asset file), so it is
  exhaustively unit-testable without binding a socket:
  - a page route → `200 text/html` with its body;
  - an unknown page path → `404 text/html` with the `404` body;
  - `/assets/…` → `200` with the content-type by extension, scoped to the asset
    allowlist (`css/fonts/icons/images/js/vendor`); missing/other → `404`.
- **`serve(content_dir, templates_dir, assets_dir, bind) -> Result<()>`** — build
  `SiteRoutes` once, wrap in `Arc` (immutable, so **lock-free** sharing), create
  a `tiny_http::Server`, and run a small fixed thread pool where each worker
  loops `recv()` → `resolve` → `respond`.
- **CLI**: `haleiki serve --content-dir demo --templates-dir templates
  --assets-dir assets --bind 127.0.0.1:8080` (additive alongside `build` /
  `export`).

## Scope

### In

- The `render_routes` extraction + `export_site` reimplemented on it (byte-identical).
- `tiny_http` dependency.
- A new `server.rs`: `SiteRoutes`, `resolve` (pure), the content-type map, and
  the `serve` loop (`Arc<SiteRoutes>`, fixed thread pool over `Arc<Server>`).
- Trailing-slash normalization: canonical URLs have a trailing slash; `/browse`
  and `/browse/` both resolve to the same route (root `/` and `/assets/…` exempt).
- Asset serving from the on-disk allowlist with content-type by extension
  (`css`→`text/css`, `js`→`application/javascript`, `woff2`→`font/woff2`,
  `svg`→`image/svg+xml`, `png`→`image/png`, `ico`→`image/x-icon`,
  `json`→`application/json`, `html`→`text/html`).
- CLI `serve` subcommand.
- Tests: `render_routes` route set + export byte-identity; exhaustive `resolve`
  unit tests; one **live** ephemeral-port smoke test; server↔export parity spot.

### Out

- Hardening — configurable-beyond-`--bind`, `HEAD`/method handling, structured
  logging, worker panic isolation, graceful shutdown, in-memory assets — **slice
  02**.
- Live-render / dev mode (re-render per request). This slice renders once at
  startup (read-only site).
- The search JSON index / `search.js` behavior (Arc 04); news (Arc 05); CLI
  reconciliation + legacy `_data`/Cobalt removal (Arc 06). No change to export
  output.

## Decisions to confirm at slice start

- **Serving mode.** Proposed: **warm in-memory route map** — render every route
  once at startup, serve from memory (fast, robust, read-only-friendly; realizes
  "the server is the deployment" without a separate build step). Alternatives:
  serve a pre-built `_site/` (static file server) or live-render per request.
- **Asset serving.** Proposed: from the on-disk `assets/` allowlist, content-type
  by extension. An in-memory asset map (fully self-contained server) is a slice-02
  hardening option.
- **Default bind.** Proposed: `127.0.0.1:8080`, overridable via `--bind`.
- **Live-server test.** Proposed: keep the bulk of verification in the **pure**
  `resolve` unit tests; add **one** integration smoke that binds an ephemeral
  port and does a real HTTP GET. Keep it dependency-light (raw `std::net`
  `TcpStream` GET, or a minimal client) rather than pulling a heavy HTTP client
  into non-demo deps.

## Verification approach

Unit tests: `render_routes` returns the expected route set (all page URL paths +
404) from the demo corpus; `export_site` output byte-identical (existing export
tests pass); `resolve` — a `200 text/html` for each page route, `404` + 404 body
for a miss, assets `200` + content-type, asset miss `404`, trailing-slash both
resolve; server↔export parity (a `resolve` body equals the corresponding
exported file). One live smoke: ephemeral port, GET `/` → 200 + home body, GET a
miss → 404. `cargo clippy -p haleiki --all-targets -- -D warnings` + `cargo fmt
--check` clean.

## Exit criteria

Ledger rows F-1…F-8 reach a final status. In short: the shared `render_routes`
feeds a byte-identical export and a working server; `resolve` routes pages, 404s,
and assets correctly; `haleiki serve` binds and serves live from an immutable
shared route map; server output matches the export; no Liquid; new code is
clippy(`--all-targets`)/fmt-clean.

## Notes

- Load the **rust-guidelines** concurrency guide: the shared state is an
  immutable `Arc<SiteRoutes>` across a fixed thread pool — lock-free by
  construction (`Send`/`Sync` via `Arc`, no interior mutability). Keep it that way.
- Reuse `render::build_environment` (auto-escape per-`Environment`). Run clippy
  **with `--all-targets`**; concatenate the no-Liquid needle. Watch the `&#x2f;`
  href-escaping gotcha (see `CLAUDE.md`) if any test greps served hrefs.
- Context/renderer authority: the existing `page::render_*` / `article::render_*`
  and `export`'s walk — the server composes them, it does not re-render per
  request.
