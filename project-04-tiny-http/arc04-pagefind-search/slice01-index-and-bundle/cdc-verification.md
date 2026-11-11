# Slice 01 — index build + bundle in both sinks · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per `LEDGER-DISCIPLINE.md` Section A. Verdict: **CLOSED** ·
> 2026-07-29.

## Verifier and method

Independent Claude (Cowork) context, not the implementer. Clean cloud build of
the committed slice (`ef3be48`) against the full demo corpus + allowlisted
assets; `cargo 1.95.0`, `pagefind 1.5.2`. Beyond the test suite I **ran a real
`haleiki export` and `haleiki serve`** and probed the `/pagefind/` surface with
live HTTP — content-types per bundle-file kind and traversal attacks — since a
served binary bundle with wrong content-types or a weak path guard is exactly the
kind of defect a passing unit test can miss. I also **ran two independent
exports** to reproduce CC's disclosed determinism deviation first-hand.

## Row-count check

Ledger opened with **8** rows (G-1…G-8); reproduces 8. No silent drops; nothing
deferred.

## Per-row verification

Full binary suite: **113 passed, 0 failed, 1 ignored** (+12 over Arc 03's 101;
CC reports 26 new tests total across bin + integration). `cargo fmt --check`
clean. Each row maps to passing tests; the wire-visible and security rows were
additionally reproduced live.

| ID | CDC verdict | Strength | How reproduced |
|----|-------------|----------|----------------|
| G-1 | **confirmed** | reproduced | `pagefind = "1.5.2"` in `[dependencies]`. In-process only: my own grep finds **no** `Command::new`/`process::Command` anywhere in `search.rs` (the only spawns in `src` are the pre-existing `demo/*` cobalt/pandoc paths); `test_search_never_shells_out…` passes. Record count == corpus at both levels: the `build_records` unit test, **and** the real bundle's `pagefind-entry.json` reports `page_count: 277` = 186 concepts + 91 sources. |
| G-2 | **confirmed** | reproduced | `test_concept_record_has_filters_meta_and_url` + `test_source_record_has_filters_meta_and_url` — all four filters (`category`/`tier`/`type`/`trust`) and the meta (`title`/`url`/`excerpt`) populated; `test_markdown_to_plain_text_strips_tags…` confirms content is genuine plain text (no `##`/`**`/tags). |
| G-3 | **confirmed** | reproduced | Unit tests assert concept url `/{slug}/`, source url `/source/{slug}/` — matching `server::render_routes`'s own scheme (Arc 02/03). |
| G-4 | **confirmed** | reproduced | `test_source_record_trust_filter_uses_profile_default_bucket` asserts `filters["trust"] == ["credible"]`; I confirmed `demo/profile.yaml` sets `trust.default: "credible"`, and `search.rs` reads `corpus.profile.trust.default` — the *same* value `article.rs` uses for the display-time fallback, so search facets and the rendered trust bar can't disagree. |
| G-5 | **confirmed** | reproduced | A real `haleiki export` writes `_site/pagefind/` containing `pagefind.js`, `pagefind-entry.json`, `wasm.en.pagefind` + `wasm.unknown.pagefind`, and 324 binary `.pf_fragment`/`.pf_index`/`.pf_meta`/`.pf_filter` files (277 fragments). `test_build_index_writes_the_expected_bundle_files` + the CLI integration test cover the same. |
| G-6 | **confirmed** | reproduced | Live against `haleiki serve`: `/pagefind/pagefind.js`→`200 text/javascript`; `…/pagefind-entry.json`→`200 application/json`; `…/wasm.en.pagefind`→`200 application/wasm` (the `.pagefind` extension mapped correctly — CC's spike finding); a `…/fragment/*.pf_fragment`→`200 application/octet-stream`. All carry `X-Content-Type-Options: nosniff`. Unit `test_handle_request_serves_pagefind_bundle_file_kinds` + a live socket test agree. |
| G-7 | **confirmed** | reproduced | Live: `/pagefind/../../../../etc/passwd`→`404`, `/pagefind//etc/passwd` (absolute)→`404`, `/pagefind/fragment/../../../Cargo.toml`→`404`, a missing `*.pf_fragment`→`404`. The guard requires every component be `Component::Normal` — stricter than `resolve_asset` (which leans on the allowlist for the absolute-path case), a deliberate, sound difference since `/pagefind/` has no allowlist. Normal pages/assets/`/search/` still `200` (no regression). |
| G-8 | **confirmed (with a documented caveat)** | reproduced | `cargo clippy -p haleiki --all-targets -- -D warnings`: **zero findings in any Arc 04 file** (`search.rs`/`server.rs`/`export.rs`/`cli.rs`/`main.rs`); `fmt` clean; `haleiki search` is a real indexer (CLI integration test writes a bundle). Caveat below — CC's broader `--all-features` claim does not reproduce, but that is pre-existing out-of-scope demo debt, not new code. |

## The G-8 caveat (an inaccuracy in the report, over out-of-scope debt)

CC's "Verification commands run" lists `cargo clippy --all-targets --all-features
--workspace -- -D warnings` as "All green." **That command does not reproduce
clean** — it fails on pre-existing lint debt in the **`demo`-feature test code**:
`ignore_without_reason`, `map_or`, and `needless_raw_string_hashes` in
`tools/tests/demo_fetch.rs` and the `#[cfg(test)]` mods of `tools/src/demo/*.rs`
(clean/convert/frontmatter/media). **None of it is in Arc 04 code**, and it is the
same feature-gated `demo/*` backlog Arc 01's close flagged for the Arc 06 CI gate
— now shown to be larger than the "demo_status.rs unused imports" Arc 03 named,
because `--all-features` pulls in the demo *source* too.

Why the report still reads "green" elsewhere and is *partly* right: `make lint`
is `cargo clippy --all-features --workspace` **without** `--all-targets`, which
does not lint test code — I reproduced that invocation **clean**, so CC's
"`make check` passes" is true. The overbroad `--all-targets --all-features` line
is the one that doesn't hold. This does **not** block the slice: the ledger's
G-8 command is `--all-targets` (no `--all-features`), the new code is clean under
it, and the boundary explicitly forbids fixing unrelated pre-existing lint debt.
Carried forward to Arc 06 (below).

## Determinism deviation — independently reproduced

CC modified `test_export_site_is_deterministic_across_repeated_exports` to exclude
`pagefind/`. I ran **two independent `haleiki export`s** and diffed:

- **Haleiki's own output is byte-identical** across both exports (every HTML page,
  asset, and the 404) — the Arc 02 determinism guarantee holds intact.
- **Only Pagefind's internal index files vary**: the hash-named `*.pf_filter` and
  `*.pf_meta` files differ between runs (and `pagefind-entry.json`, which
  references the meta hash), because pagefind 1.5.2's filter-value `HashMap`
  iterates in per-process-random order — the crate's own
  `// TODO: Change filter indexes to BTree` in `src/index/mod.rs`.

CC's narrowing is **honest and correctly scoped**: it excludes exactly the
non-deterministic upstream artifact, keeps the byte-for-byte guarantee for
everything Haleiki produces, and still asserts the bundle exists. This is an
upstream limitation, not a Haleiki defect — worth re-checking on a future
`pagefind` upgrade.

## Boundary check

Diff is additive: new `search.rs`; `export_site` gains a final index-build step
(HTML output unchanged — proven by the untouched export-tree tests + my
byte-identical diff); `server.rs` adds `resolve_pagefind` /
`pagefind_content_type_for` / `handle_request` **alongside** `handle`/`resolve`
(both signatures verified unchanged — Arc 03's ~30 tests pass unmodified); `cli.rs`
/`main.rs` make `search` real. Shared state stays immutable + lock-free (no new
`Mutex`/`RwLock`). **No client change** — `search.js`, `render_search`, and
`search.jinja` are untouched, exactly as scoped; D-4…D-6 remain slice 02's.

## Observations / carry-forward (not defects)

- **`tokio` was promoted from an optional, `demo`-gated dependency to a required
  core dependency** (`features = ["full"]`) to host the async bridge onto
  `pagefind`'s async service API. This grows the lean core's dependency tree. It
  is a reasonable way to call one async API, but likely heavier than needed — a
  single `block_on` via `pollster` or `futures::executor` would bridge the one
  call without a full multi-threaded runtime in the core. Worth revisiting in Arc
  06's dependency hygiene; naming it so it isn't an unexamined default.
- **Arc 06 CI gate (expanded).** The gate must decide the clippy matrix. The
  strict `--all-targets --all-features` form currently fails on the pre-existing
  `demo/*` **test-code** debt characterized above; either that debt is fixed
  first, or the gate uses the `make lint` invocation. This supersedes the smaller
  "demo_status.rs unused imports" note from Arc 03.
- **Excerpt plain-texting is a hand-rolled tag stripper** (CC-disclosed), adequate
  for the demo corpus but not a full HTML parser; fine for index text, worth a
  note if a future corpus embeds unusual raw HTML.
- **`serve()` builds the index before binding**, so a doomed bind still does the
  index work first (CC-disclosed). The error still surfaces (the bad-bind test
  passes); a pure startup-ordering nicety, not a correctness issue.

## Verdict

**Slice 01 is CLOSED.** 8 rows, all reproduced; the bundle built in-process
(277 pages, no shell-out), emitted by `export`, and served by `serve` with correct
content-types and a sound (stricter) traversal guard — all audited live. Arc
ledger row **D-1** satisfied (evidence: this file). Two carry-forwards recorded
for Arc 06 (the `--all-features` clippy debt; the `tokio`-as-core-dep weight); one
upstream item to watch (pagefind bundle non-determinism). Slice 02 (client wiring
+ facets + browser composition) can proceed directly against this bundle.

## What worked (Safety-II)

- CC **spiking the `pagefind` crate source first** (per the cc-prompt) turned two
  latent browser-only bugs into design decisions *before* code: the verbatim-content
  behavior that justifies `markdown_to_plain_text`, and the real `wasm.{lang}.pagefind`
  filename that would otherwise have been a silent 404. The slice-doc had *assumed*
  `.wasm` sight-unseen; the spike corrected the plan.
- **Composing `/pagefind/` serving as a new `handle_request` beside `handle`/`resolve`**
  (not inside them) kept Arc 03's entire verified surface unchanged by construction
  — the boundary held structurally, not by careful editing of shared code.
- **Diffing two real export trees** rather than trusting the determinism failure
  message is why the upstream pagefind non-determinism was correctly attributed and
  narrowly scoped, instead of weakening the determinism guarantee wholesale.
