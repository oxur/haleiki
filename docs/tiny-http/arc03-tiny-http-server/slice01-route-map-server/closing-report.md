# Slice 01 — route map + core server · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 8 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run (CC does not write
> `cdc-verification.md`, per the cc-prompt). Not yet committed. This is Arc
> 03's first slice. 2026-07-28. Iterations used: 1 of 5.

## Ledger walk

| ID | Status | Summary |
|----|--------|---------|
| F-1 | done | `render_routes` returns the full route set (5 singletons + one row per category/concept/source) + the 404 body, verified against an independently-derived expected key set from the parsed demo corpus. |
| F-2 | done | `export_site` reimplemented on top of `render_routes`; all 7 pre-existing export tests pass unmodified, including byte-for-byte determinism across repeated exports. |
| F-3 | done | `resolve` returns `200 text/html` with the correct body for all 8 page-route kinds. |
| F-4 | done | `resolve` returns `404` + the 404 body for an unknown path. |
| F-5 | done | `resolve` serves the asset allowlist with correct content-types, 404s misses/non-allowlisted/traversal paths, and normalizes trailing slash; cross-checked against the real running binary with `curl`. |
| F-6 | done | Live ephemeral-port smoke test exercises the real `recv()`→`resolve()`→`respond()` path over a socket; grep confirms `Arc<SiteRoutes>` and no `Mutex`/`RwLock` anywhere in `server.rs`; the actual `haleiki serve` binary was also run and curled end-to-end. |
| F-7 | done | A `resolve` body matches the corresponding file `export_site` writes, for 4 route kinds; no-Liquid grep (concatenated needle) is clean. |
| F-8 | done | `server.rs`/`export.rs`/`cli.rs`/`main.rs` are clippy(`--all-targets --all-features`)-clean and `cargo fmt`-clean; `make check` (374 tests) passes. |

No silent drops. Every row in the ledger reached `done` with attested evidence; nothing was deferred.

## What was built

- **`server::render_routes(content_dir, templates_dir) -> Result<SiteRoutes>`** — the route-walk extracted out of `export::export_site`, now the single shared walk. `SiteRoutes { pages: BTreeMap<String, String>, not_found: String }`.
- **`export::export_site`** reimplemented as a consumer of `render_routes` + `server::ASSET_ALLOWLIST`, iterating the sorted `BTreeMap` and writing each route via a new `write_route(out_dir, url_path, html)` helper. Byte-identical output, guarded by the pre-existing export test suite.
- **`server::resolve(request_path, &SiteRoutes, assets_dir) -> ServedResponse`** — a pure router: strips query strings, routes `/assets/…` through an allowlist + traversal guard + content-type-by-extension, normalizes trailing slash for page routes, falls back to the 404 body.
- **`server::serve(content_dir, templates_dir, assets_dir, bind) -> Result<()>`** — builds `SiteRoutes` once, wraps it in an immutable `Arc`, binds an `Arc<Server>`, and runs 4 worker threads each looping `recv()` → `resolve()` → `respond()`. No `Mutex`/`RwLock` — the route map is read-only for the process lifetime, so `Arc` alone is sufficient.
- **CLI**: `haleiki serve --content-dir --templates-dir --assets-dir --bind`, additive alongside `build`/`export`, defaulting to `content`/`templates`/`assets`/`127.0.0.1:8080`.
- **9 new tests** in `server.rs`: route-set, per-route-kind 200s, unknown-path 404, asset content-types + traversal/allowlist/miss guards, pure `content_type_for` coverage for extensions with no on-disk file, trailing-slash equivalence, server↔export parity, no-Liquid grep, and one live ephemeral-port smoke test.

## Deviations from plan

None. The slice-doc's four "decisions to confirm at slice start" were all taken as proposed: warm in-memory route map, on-disk asset allowlist (no in-memory asset map — that's flagged slice-02 hardening in the slice-doc itself), default bind `127.0.0.1:8080`, and a pure-`resolve`-heavy test suite with exactly one live ephemeral-port smoke test using a raw `TcpStream` GET rather than a full HTTP client dependency.

## Named uncertainty

- **Serving-mode decision (the one the cc-prompt asked to name explicitly):** this slice renders every route once at process startup and serves it read-only from memory for the life of the process. That is correct for the demo corpus's size and is what the slice-doc proposed, but it means content changes require a process restart — there is no live-render/dev mode in this slice (explicitly out of scope; the slice-doc calls it out as unaddressed here). Worth flagging at arc close as a decision the arc-plan should confirm is still wanted going into later slices, versus, e.g., a `--watch` mode.
- **F-8's `--all-targets` scope surfaced more pre-existing lint debt than prior slices reported** (`demo/*.rs`, `generator.rs`) — none of it touched by or introduced in this slice, but worth a note in case a future slice's clippy baseline looks different than expected. It's a byproduct of `--all-targets` combined with `--all-features` pulling the `demo`-gated test binaries into clippy's scope; `make lint`'s actual invocation is unaffected and clean.

## Bubble-up to arc-plan

This slice delivers the arc's foundational capability: a single shared `render_routes` walk consumed by both sinks, and a lock-free, working `tiny_http` server serving real HTTP over it. Nothing here contradicts or extends `arc-plan.md`'s scope for this slice — the arc-plan's composition expectations (shared walk, immutable `Arc`-based state, additive CLI) are all met as specified. Slice 02 (hardening: method handling, structured logging, worker panic isolation, graceful shutdown, in-memory assets) can proceed against this slice's `resolve`/`serve` surface unchanged.

## Verification commands run

```
cargo build --all-targets
cargo test --workspace
cargo clippy --all-targets --all-features --workspace -- -D warnings
cargo clippy --all-features --workspace -- -D warnings   # make lint's actual invocation
cargo fmt --all -- --check
make check
```

All green. `make check` final result: 374 tests passed, 0 failed, 8 ignored (pre-existing `#[ignore]`d live-network demo tests, unrelated to this slice).
