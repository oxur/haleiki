# Slice 01 — route map + core server · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per `LEDGER-DISCIPLINE.md` Section A. Verdict: **CLOSED** ·
> 2026-07-28.

## Verifier and method

Independent Claude (Cowork) context, not the implementer. Clean cloud build
against the full demo corpus + allowlisted assets; `cargo 1.95.0`. Beyond the
test suite, I **ran a real `haleiki serve`** and probed it with live HTTP —
including path-traversal attacks — since the asset server is the security-
sensitive surface this slice introduces.

## Row-count check

Ledger opened with **8** rows (F-1…F-8); reproduces 8. No silent drops.

## Per-row verification

Full suite (default features): **90 passed, 0 failed, 1 ignored** (+9 server
tests over Arc 02's 81). Each row maps to a passing test, and the security path
was additionally probed live:

| ID | CDC verdict | Strength | How reproduced |
|----|-------------|----------|----------------|
| F-1 | **confirmed** | reproduced | `test_render_routes_returns_full_route_set` — exact key set (home/browse/about/attribution/search + every category/concept/source URL path) + non-empty 404. |
| F-2 | **confirmed** | reproduced | The export suite (determinism + tree + counts) passes unchanged after `export_site` was reimplemented on `render_routes` — byte-identical output. |
| F-3 | **confirmed** | reproduced | `test_resolve_returns_200_for_every_page_route_kind` — 200 `text/html`, body equals the rendered route. Live: `/category/mathematics/`, `/browse` → 200. |
| F-4 | **confirmed** | reproduced | `test_resolve_returns_404_for_unknown_path` + live `/nope/` → 404 with the 404 body. |
| F-5 | **confirmed** | reproduced | `test_resolve_serves_assets_with_correct_content_type` + live: `.css`→`text/css`, `.woff2`→`font/woff2`, `.png`, `.ico`; missing/non-allowlisted/traversal → 404; trailing-slash + query-string normalization. |
| F-6 | **confirmed** | reproduced | `test_live_server_smoke_ephemeral_port` (raw `std::net` GET) + my own `haleiki serve` run. Shared state is `Arc<SiteRoutes>` immutable across a 4-thread pool — **no `Mutex`/`RwLock`** (read the code + grep). |
| F-7 | **confirmed** | reproduced | `test_resolve_body_matches_exported_file_for_same_route` — served body == exported file for `/`, category, concept, source (one renderer, two sinks, verified to agree); no-Liquid test (needle concatenated). |
| F-8 | **confirmed** | reproduced | `clippy --all-targets -D warnings`: 0 findings in `server.rs`/`export.rs`/`cli.rs`; `fmt` clean; only pre-existing demo-test imports remain. |

## Security audit — asset path traversal (the sensitive surface)

The guard in `resolve_asset` is **sound**, and I confirmed it against a live
server, not just the unit test:

- Rejects any `Component::ParentDir` (`..`) before touching the filesystem, and
  requires the first path component to be in the allowlist. The allowlist check
  also defuses the absolute-path `Path::join` footgun: for `/assets//etc/passwd`
  the first component is `RootDir` (`/`), which isn't allowlisted → 404 *before*
  the join.
- tiny_http's `request.url()` is not percent-decoded, so `%2e%2e` arrives literal
  (a non-existent filename → 404); and were it ever decoded, the `..` guard
  catches it.

Live probes against `haleiki serve` (demo corpus), all **404** as required:
`/assets/css/../../../../etc/passwd` (curl-normalized **and** raw `--path-as-is`),
encoded `/assets/css/%2e%2e%2f…/etc/passwd`, `/assets/ai/SKILL.md` (non-allowlisted),
`/assets//etc/passwd` (absolute). Legitimate assets and pages served 200. No
traversal escapes the asset root.

## Boundary check

Changed `Cargo.toml` (+`tiny_http`), `cli.rs` (+`serve`), `main.rs` (dispatch),
`export.rs` (refactored onto `render_routes`, byte-identical), new `server.rs`.
Additive; the `build`/`export` behavior is unchanged. `ASSET_ALLOWLIST` is now
shared (`server.rs`, referenced by `export.rs`) so the two sinks can't drift.

## Minor observations (not defects)

- **Startup window:** `serve` renders all 287 routes *then* binds, so there's a
  brief pre-bind window (I saw a couple of connection-refused responses when I
  probed too eagerly). Fine for a self-host one-time startup; slice 02's logging
  already prints "Rendering…" then "Serving on…".
- **Symlinked assets** would be followed by `std::fs::read` (export's copy skips
  them). First-party asset tree, so not a practical risk; a `canonicalize`
  prefix-check is a possible slice-02 hardening nicety.

## Verdict

**Slice 01 is CLOSED.** 8 rows, all reproduced; the traversal guard independently
audited live. Arc ledger row **C-1** satisfied (evidence: this file). Slice 02
(hardening + arc composition) is next, then Arc 03 closes.

## What worked (Safety-II)

- The `render_routes` extraction makes "one renderer, two sinks" literally
  checkable — and CC wrote the parity test that proves the served body equals the
  exported file, so the two sinks can't silently diverge.
- The traversal test was written against a real attack string (`../../../../etc/
  passwd`), not a toy — which is why the live probe confirmed rather than
  surprised.
