# Slice 04 — static export writer · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `export_site` writes a complete `_site/` from the demo corpus: `index.html` for home, browse, about, attribution, search; `category/{slug}/index.html` per category; `{slug}/index.html` per concept; `source/{slug}/index.html` per source; and `404.html` — each at its contract path | `cargo test`: export to a temp dir, assert every route's file exists (+ counts vs corpus) | serious | slice-doc (B-5) | open | | |
| F-2 | Asset allowlist copied: `_site/assets/{css,fonts,icons,images,js,vendor}` present; `assets/ai` and `assets/mockups` are **not** copied | test asserts the present + absent asset subdirs | serious | slice-doc (B-6) | open | | |
| F-3 | Clean-before-write: a stale file planted in `_site/` before a re-export is gone afterward | test plants a file, re-exports, asserts it's absent | correctness | slice-doc | open | | |
| F-4 | Internal links resolve within `_site/`: home → a `category/{slug}/index.html`; a category page → an article `index.html`; a concept's breadcrumb/see-also target files exist | test follows a sample link chain against the tree | serious | slice-doc (B-8) | open | | |
| F-5 | Written pages match the renderers: a written concept `index.html` and the home `index.html` contain their expected chrome (trust bar / category grid, etc.) | test reads the written files and greps chrome markers | serious | slice-doc | open | | |
| F-6 | A CLI subcommand drives the export end-to-end: running it with the demo content produces `_site/` | `assert_cmd` (or equivalent) integration test invoking the binary | serious | slice-doc | open | | |
| F-7 | Export is deterministic and uses no Liquid: re-exporting the same corpus produces the same tree/bytes; the export path references no `.liquid` | test: two exports compared; needle-concatenated no-Liquid grep on `export.rs` | correctness | slice-doc | open | | |
| F-8 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 8. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
