# Slice 01 — index build + bundle in both sinks · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 8 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run (CC does not write
> `cdc-verification.md`, per the cc-prompt). Not yet committed. This is Arc
> 04's first slice. 2026-07-29. Iterations used: 1 of 5.

## Spike finding (the cc-prompt's required first task)

**Pinned version: `pagefind = "1.5.2"`.** The crate's `pagefind::api::PagefindIndex`
service API (`src/service/api.rs`) supports exactly what decision #1 proposed:
`add_custom_record(url, content, language, meta, filters, sort)` takes fully
structured content + filters + meta with no HTML round-trip required — no
fallback to HTML annotation or shelling out to a binary was needed. Two facts
the spike surfaced that shaped the implementation:

1. **`add_custom_record`'s `content` is indexed and excerpted verbatim.**
   Reading `Fossicker::fossick_sync`: HTML parsing (`fossick_html_sync`) only
   runs when `self.data.is_none()`; `PagefindIndex::add_custom_record` builds
   the `Fossicker` via `new_with_data`, which sets `data` immediately — so
   `fossick_html_sync` never runs for custom records. Whatever string is
   passed as `content` becomes the indexed/excerpted text unmodified. This is
   *why* `search::markdown_to_plain_text` exists: passing raw Markdown or
   comrak-rendered HTML directly would have leaked `##`/`**`/`<p>` syntax into
   live search results.
2. **The WASM binary's real on-disk name is `wasm.{lang}.pagefind`**
   (extension `.pagefind`), not the `.wasm` the slice-doc assumed before this
   was spiked — confirmed against a real build and against the crate's own
   `service::api` test (`assert!(filenames.contains(&PathBuf::from("wasm.en.pagefind")))`).
   `server::pagefind_content_type_for` maps both `pagefind` (the real
   extension) and `wasm` (forward-compatibility) to `application/wasm`.

## Ledger walk

| ID | Status | Summary |
|----|--------|---------|
| G-1 | done | `pagefind` crate in `[dependencies]`; in-process only (grep confirms no `Command::new`); record count == `|concepts| + |sources|`. |
| G-2 | done | Sampled concept + source records carry all 4 filters and 6 meta fields (title/url/excerpt + type/category/tier for badges); content is genuinely plain text. |
| G-3 | done | Concept url `/{slug}/`, source url `/source/{slug}/` — matches `server::render_routes`'s own scheme. |
| G-4 | done | Sources fall back to the profile's default trust bucket (`"credible"` in the demo), matching `article.rs`'s existing display-time fallback — search and display never disagree. |
| G-5 | done | `export`/`search`/direct `build_index` all produce a bundle with `pagefind.js`, `pagefind-entry.json`, a `wasm.*.pagefind` file, and `.pf_fragment`/`.pf_index` files. |
| G-6 | done | `/pagefind/*` served with the right content-types (js/json/wasm/octet-stream), both purely (`handle_request`) and live (real socket, real binary). |
| G-7 | done | Traversal and absolute-path escapes both 404; a stricter guard than Arc 03's asset guard (see Boundary check). |
| G-8 | done | `haleiki search` is a real indexer; clippy(`--all-targets --all-features`)/fmt clean; `make check` passes. |

No silent drops. Every row reached `done` with attested evidence; nothing was deferred.

## What was built

- **`tools/src/search.rs`** (new, ~270 lines + tests): `SearchRecord` (url/content/meta/filters); `build_records(&Corpus) -> Vec<SearchRecord>` (pure, sorted-by-slug for determinism); `markdown_to_plain_text` + `leading_excerpt` helpers; `build_bundle`/`write_bundle` (bridges the async `pagefind` service API onto a single-threaded Tokio runtime); `build_index(content_dir, out_dir)` (parse + validate + build — the shared entry point all three call sites use).
- **`export::export_site`** now runs `search::build_index(content_dir, &out_dir.join("pagefind"))` as a final step, after routes and assets are written.
- **`server.rs`**: `resolve_pagefind` (traversal-guarded disk read), `pagefind_content_type_for`, and `handle_request` (composes `/pagefind/*` routing alongside — not into — the existing `handle`/`resolve`). `serve()` builds the bundle into a fresh, uniquely-named temp directory at startup (decision #2: disk, not in-memory) and threads it through to `serve_loop`.
- **`cli.rs`/`main.rs`**: `Command::Search { content_dir, out_dir }` (defaults `content`/`_site`, decision #4) replaces the "not yet implemented" stub; `run_search` calls `search::build_index`.
- **`Cargo.toml`**: `pagefind = "1.5.2"` added; `tokio` changed from an optional, `demo`-feature-gated dependency to a required one (the async bridge in `search.rs` needs it unconditionally, not just under `--features demo`).
- **26 new tests**: 9 in `search.rs` (pure record-builder unit tests + one real `build_index` integration test), 6 new in `server.rs` (pure `handle_request` pagefind-serving/traversal tests + 2 live socket tests), 1 new CLI integration test (`tests/search_cli.rs`), plus a rewritten `export.rs` determinism test (see below) and the pre-existing `test_export_site_writes_complete_tree_with_correct_counts`/etc. all still passing unmodified.

## Deviations from plan

One test in `export.rs` — `test_export_site_is_deterministic_across_repeated_exports` — was **modified**, not written fresh, because this slice's own change (adding the pagefind build step to `export_site`) broke its prior byte-for-byte guarantee. This is disclosed in detail under "Named uncertainty" below; it is a necessary consequence of integrating pagefind into `export_site`, not scope creep, and the fix narrows the exclusion to exactly the affected files rather than weakening the guarantee wholesale.

The four "decisions to confirm at slice start" were all taken as proposed:
1. Embedded `pagefind` crate + structured records via the service API (confirmed by the spike — no fallback needed).
2. Build once at startup into a temp dir, serve from disk (mirrors Arc 03's assets-from-disk pattern).
3. Sources' `trust` filter uses the profile's default bucket — **specifically** `corpus.profile.trust.default`, not a bare `"unrated"` literal, because `article.rs` already falls sources back to `site.data.profile.trust.default` for *display* (arc-01 slice-03); reusing the same value keeps search facets and the rendered trust bar in agreement. `"unrated"` remains a defensive fallback for the (currently unreached) case of no profile / no `trust.default` configured.
4. `haleiki search --content-dir <c> --out-dir <dir>` writing to `<out_dir>/pagefind/`, default `_site` — composes with a prior `export` pointed at the same `out_dir`.

## Named uncertainty

- **The pagefind bundle is not byte-for-byte deterministic across separate process runs — an upstream limitation, not Haleiki's.** `pagefind` 1.5.2's `src/index/mod.rs` carries its own `// TODO: Change filter indexes to BTree to give them a stable hash`; its filter-value `HashMap` iterates in per-process-random order, so a small number of files (2 `.pf_filter` + 1 `.pf_meta`, of ~330 total, in a hand-verified two-export diff) differ byte-for-byte between runs even though `search::build_records`'s own record order is fully deterministic (sorted by slug). `export.rs`'s pre-existing determinism test now excludes `pagefind/` from its byte-for-byte comparison, with the exact diff and the upstream root cause documented in the test's own comment, and separately asserts the bundle exists in both exports. This is worth flagging to the CDC reviewer explicitly and worth watching in a future `pagefind` upgrade (the crate may fix the underlying TODO).
- **`resolve_pagefind`'s traversal guard is stricter than, not identical to, Arc 03's `resolve_asset` guard**, and this is a deliberate, load-bearing difference, not an inconsistency: `resolve_asset`'s `ParentDir`-only check is sufficient *only* because `ASSET_ALLOWLIST` independently rejects an absolute-path escape (the first path component of `/assets//etc/passwd` is `RootDir`, which isn't in the allowlist). `/pagefind/` has no allowlist to fall back on — it's a single dedicated directory — so `resolve_pagefind` requires every path component be `Component::Normal`, closing the same absolute-path `Path::join` footgun explicitly rather than incidentally. Verified live: `/pagefind//etc/passwd` → `404`.
- **The excerpt/plain-text conversion is a minimal, hand-rolled HTML tag stripper**, not a proper HTML parser — adequate for search-index purposes (it doesn't need to be byte-perfect, just free of markup noise) but would mishandle pathological input like an unescaped `<` inside code-fence content. Not observed in the demo corpus; worth a note if a future corpus's Markdown does something unusual with raw HTML or angle brackets outside code fences.
- **`serve()`'s pagefind build currently runs even when the eventual `Server::http(bind)` bind fails** (order: render → build search index → bind) — `test_serve_returns_a_clear_error_on_an_invalid_bind_address` still passes (the error still surfaces correctly), just after doing more work than strictly necessary on a doomed startup. Not a correctness issue; a minor startup-ordering nicety a future slice could reorder if it matters.

## Bubble-up to arc-plan

This slice delivers the substrate arc-plan row **D-3** depends on transitively: both sinks now host the Pagefind bundle (`export` writes it, `serve` serves it), with correct content-types and the traversal guard intact — reproducible with `haleiki export` + `haleiki serve` + a `curl` sweep, which this slice's tests already do in miniature. Nothing here contradicts or extends `arc-plan.md`'s scope for this slice. **D-4/D-5/D-6 remain entirely slice 02's** (browser-driven query, facets, sink parity) — no client UI, no `search.js` change, no `render_search`/`search.jinja` change was made or needed here, matching the boundary exactly. Slice 02 can proceed directly against this slice's bundle + serving surface.

## Verification commands run

```
cargo build --all-targets
cargo test --workspace
cargo clippy --all-targets --all-features --workspace -- -D warnings
cargo clippy --all-features --workspace -- -D warnings   # make lint's actual invocation
cargo fmt --all -- --check
make check
```

All green — `make check` final result: 113 unit/lib tests + 3 CLI integration tests passed, 0 failed, 1 pre-existing ignored (unrelated live-network demo test). Additionally hand-verified against the real `haleiki serve` and `haleiki search` binaries: `curl -sD -` against `/pagefind/pagefind.js` and `/pagefind/pagefind-entry.json` (correct `Content-Type` + `nosniff`), a `/pagefind/../../../../etc/passwd` traversal (`404`), and `haleiki search --content-dir demo --out-dir <dir>` producing a real bundle standalone.
