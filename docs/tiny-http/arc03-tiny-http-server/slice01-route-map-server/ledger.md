# Slice 01 — route map + core server · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `render_routes(content_dir, templates_dir)` returns the full route set keyed by URL path — `"/"`, `"/browse/"`, `"/about/"`, `"/attribution/"`, `"/search/"`, one `"/category/{slug}/"` per category, one `"/{slug}/"` per concept, one `"/source/{slug}/"` per source — plus the 404 body | `cargo test` from the demo corpus: assert the key set + counts | serious | slice-doc | open | | |
| F-2 | The `export_site` refactor onto `render_routes` is behavior-preserving — export output is byte-identical (the existing export determinism + tree tests still pass) | `cargo test` export suite green after the refactor | serious | slice-doc | open | | |
| F-3 | `resolve(path, &routes, assets_dir)` returns `200` `text/html` with the correct body for every page route (home, browse, a category, a concept, a source, about, attribution, search) | unit test on `resolve` for each page route | serious | slice-doc | open | | |
| F-4 | `resolve` returns `404` with the 404 body for an unknown page path | unit test | serious | slice-doc | open | | |
| F-5 | `resolve` serves `/assets/…` from the allowlist with the correct content-type (`css`/`js`/`woff2`/`svg`/`png`/`ico`); a missing or non-allowlisted asset → `404`. Trailing-slash: `/browse` and `/browse/` both resolve | unit tests covering content-types + trailing-slash | serious | slice-doc | open | | |
| F-6 | `haleiki serve` binds and serves live: an integration test binds an ephemeral port, GETs `/` → `200` + the home body and a missing path → `404`; the route map is shared as an immutable `Arc` across worker threads (no locks / no interior mutability) | live ephemeral-port integration test + a grep confirming `Arc<SiteRoutes>` and no `Mutex`/`RwLock` around it | serious | slice-doc | open | | |
| F-7 | Server ↔ export parity + no-Liquid: a `resolve` page body equals the corresponding exported file's body; the server path references no `.liquid` (needle concatenated) | unit test comparing a served body to an exported file; no-Liquid grep test | correctness | slice-doc | open | | |
| F-8 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 8. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
