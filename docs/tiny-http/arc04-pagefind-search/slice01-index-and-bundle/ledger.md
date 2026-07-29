# Slice 01 — index build + bundle in both sinks · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-29.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| G-1 | The `pagefind` crate is a `[dependencies]` entry and an in-process indexer builds an index from the `Corpus` with **no `npx`/Node/binary shell-out**; the index has one record per concept + source (count == `|concepts| + |sources|`) | unit/integration test asserts the record count equals the demo corpus size; grep confirms no `Command::new("pagefind")`/`npx` | serious | slice-doc | open | | |
| G-2 | Each record carries the four **filters** (`category`, `tier`, `type`, `trust`) and the result **meta** (`title`, `url`, `excerpt`) the template needs | unit test asserts a sampled concept + source record's filters + meta fields are populated | serious | slice-doc | open | | |
| G-3 | **URLs** are correct: a concept record's url is `/{slug}/`, a source record's is `/source/{slug}/` | unit test asserts url for a sampled concept and source | correctness | slice-doc | open | | |
| G-4 | The **trust facet is well-formed for sources** (which have no `trust_level`): every source record has the agreed default trust bucket (decision #3), so the `trust` filter never has an empty value | unit test asserts a source record's `trust` filter == the default bucket | polish | slice-doc | open | | |
| G-5 | `haleiki export` emits `_site/pagefind/` containing the JS entry (`pagefind.js`), a `.wasm`, `pagefind-entry.json`, and index/fragment files | integration test runs `export_site` to a temp dir and asserts those bundle files exist | serious | slice-doc | open | | |
| G-6 | The tiny_http server serves `/pagefind/*` from disk with correct content-types: `pagefind.js`→`text/javascript`, `.wasm`→`application/wasm`, `.json`→`application/json`, binary index/fragment→`application/octet-stream`; all `200` | `resolve`/live test fetches each bundle file kind and asserts status + content-type | serious | slice-doc | open | | |
| G-7 | Path safety holds for the new route: a `/pagefind/…/../…` traversal → `404` (reuses the Arc 03 guard); a missing bundle file → `404` | unit/live test asserts a traversal + a missing path both `404` | serious | slice-doc | open | | |
| G-8 | The `search` subcommand is a **real in-process indexer** (writes a `/pagefind/` bundle to an output dir), not the "not yet implemented" stub; new code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean and `cargo fmt`-clean | invoke `haleiki search` → asserts a bundle is written; run clippy **with `--all-targets`** + `cargo fmt --check` scoped to new code | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 8. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
