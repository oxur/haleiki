# Slice 04 — static export writer · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `export_site` writes a complete `_site/` from the demo corpus: `index.html` for home, browse, about, attribution, search; `category/{slug}/index.html` per category; `{slug}/index.html` per concept; `source/{slug}/index.html` per source; and `404.html` — each at its contract path | `cargo test`: export to a temp dir, assert every route's file exists (+ counts vs corpus) | serious | slice-doc (B-5) | done | attested — `cd tools && cargo test export::tests::test_export_site_writes_complete_tree_with_correct_counts` → `... ok`. | `export::export_site`. Every singleton route file asserted; every one of the corpus's real 10 categories, 186 concepts, and 91 sources gets its own `index.html`, counted against `corpus.concepts.len()`/`corpus.sources.len()`/`category_index.len()` directly (not hardcoded numbers) so the test stays correct if the demo corpus grows. `404.html` confirmed at the flat path, and confirmed **not** duplicated at `404/index.html`. Manually smoke-tested against the real demo corpus before writing any test (`cargo run -- export --content-dir demo ...`) — 186/10/91 matched by eye first. |
| F-2 | Asset allowlist copied: `_site/assets/{css,fonts,icons,images,js,vendor}` present; `assets/ai` and `assets/mockups` are **not** copied | test asserts the present + absent asset subdirs | serious | slice-doc (B-6) | done | attested — `cd tools && cargo test export::tests::test_export_site_copies_asset_allowlist_only` → `... ok`. | `export::{ASSET_ALLOWLIST, copy_assets, copy_dir_recursive}` — hand-rolled recursive `std::fs` copy, no new dependency. Confirms all 6 allowlisted subdirs present, `ai`/`mockups` absent, and spot-checks real file content made it through (`haleiki.css` exists; all 8 font files present) rather than just empty directories. |
| F-3 | Clean-before-write: a stale file planted in `_site/` before a re-export is gone afterward | test plants a file, re-exports, asserts it's absent | correctness | slice-doc | done | attested — `cd tools && cargo test export::tests::test_export_site_cleans_before_write` → `... ok`. | `export_site` calls `remove_dir_all(out_dir)` (if it exists) before writing anything. |
| F-4 | Internal links resolve within `_site/`: home → a `category/{slug}/index.html`; a category page → an article `index.html`; a concept's breadcrumb/see-also target files exist | test follows a sample link chain against the tree | serious | slice-doc (B-8) | done, **with an escaping-format surprise caught and fixed while writing the test** | attested — `cd tools && cargo test export::tests::test_export_site_internal_links_resolve_to_real_files` → `... ok`. | Real chain: home → `/category/mathematics/` → an article card → `/source/{slug}/` or `/{slug}/` → the real file; `abelian-group`'s breadcrumb → `/category/mathematics/` and see-also → `/source/group-mathematics/` + `/ring-mathematics/`, all resolved to real written files. **Finding:** `category.jinja`'s article-card `href="{{ a.url }}"` is entirely template-interpolated (unlike the partially-literal `/category/{{ slug }}/` pattern), so minijinja's attribute auto-escaping turns every `/` into `&#x2f;` — the first version of this test searched for literal `href="/source/` and found zero matches on genuinely-correct output. Same class of surprise as Arc 01 slice 01's footer links and Arc 02 slice 03's license links; fixed with a helper that checks both the literal and escaped forms, applied to every href assertion in this test rather than patched ad hoc per occurrence. |
| F-5 | Written pages match the renderers: a written concept `index.html` and the home `index.html` contain their expected chrome (trust bar / category grid, etc.) | test reads the written files and greps chrome markers | serious | slice-doc | done | attested — `cd tools && cargo test export::tests::test_export_site_written_pages_match_renderers` → `... ok`. | Reads the actual written files (not the renderer's in-memory return value) for home (`hero-title`/`category-grid`/`featured-grid`), a concept (`abelian-group`: trust bar, math span, see-also), and a source (`vimalamitra`: prose, title) — confirms the walker's write step didn't corrupt or diverge from what the renderers produced. |
| F-6 | A CLI subcommand drives the export end-to-end: running it with the demo content produces `_site/` | `assert_cmd` (or equivalent) integration test invoking the binary | serious | slice-doc | done | attested — `cd tools && cargo test --test export_cli` → `... ok`. | New `tools/tests/export_cli.rs` (matching the existing `demo_fetch.rs`/`demo_status.rs` integration-test convention), invoking the real compiled `haleiki export --content-dir demo --out-dir <temp>` binary and asserting both the process exit/stderr and the written file tree. `Command::Export` added to `cli.rs` with `--content-dir`/`--templates-dir`/`--assets-dir`/`--out-dir` (defaults: `content`/`templates`/`assets`/`_site`), dispatched via a new `run_export` in `main.rs` — additive alongside `build` per decision #1, `build`/`generator::generate` untouched. |
| F-7 | Export is deterministic and uses no Liquid: re-exporting the same corpus produces the same tree/bytes; the export path references no `.liquid` | test: two exports compared; needle-concatenated no-Liquid grep on `export.rs` | correctness | slice-doc | done | attested — `cd tools && cargo test export::tests::test_export_site_is_deterministic_across_repeated_exports export::tests::test_export_path_references_no_liquid_templates` → both `... ok`. | Determinism: exports the full demo corpus twice to two separate temp dirs, walks both trees (sorted), asserts identical relative-path lists **and** byte-for-byte identical content file-by-file (not just matching filenames) — 287+ files compared. No-Liquid: needle built by concatenation from the start, per the now-standing lesson. |
| F-8 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | done | attested — `cargo fmt --all -- --check` → exit 0. `cargo clippy --all-targets --all-features --workspace -- -D warnings` → 4 real findings in new code, all fixed: a missing-backticks doc comment, two `filter_map(\|e\| e.ok())` → `filter_map(Result::ok)`, and a nested-fn-after-statements in a test (`items_after_statements` — the two helper fns were hoisted to module scope). Remaining findings are the same two pre-existing `generator.rs` lines. | |

## What Worked

- Manually running the real CLI against the real demo corpus
  (`cargo run -- export --content-dir demo ...`) *before* writing any
  automated test caught nothing wrong — 186 concepts, 10 categories, 91
  sources, correct asset tree, all on the first invocation — but that
  confirmation is what let every subsequent test be written against known-
  good real numbers instead of guessed ones, the same discipline this whole
  arc has used for unit tests, now applied at the export-walker scale.
- The escaped-href surprise (F-4) was recognized and fixed with a reusable
  helper on first encounter this time, rather than as a repeated one-off
  patch — the third time this exact class of issue has appeared (Arc 01
  footer links, Arc 02 slice 03 license links, now here), each time
  requiring less rediscovery than the last.
- Writing the determinism test as a genuine byte-for-byte comparison across
  two full 287-file exports (not a spot-check, not just matching filenames)
  is the kind of test that would actually catch a `HashMap`-iteration-order
  regression if `export_site` ever stopped sorting its route lists —
  meaningfully stronger than "it ran twice without crashing."
- Composing the existing renderers exactly as instructed (no re-rendering,
  no new context-building logic in `export.rs` beyond directory walking and
  file I/O) kept this slice's actual new surface small despite touching the
  CLI for the first time in the pivot — the render pipeline built across
  Arc 01 and Arc 02's first three slices did the real work.

## Closure

**Independently verified (CDC): CLOSED** 2026-07-28. All 8 rows reproduced in a
clean build — `cargo test` = 81 passed / 0 failed plus the CLI integration test;
and the **real export** was run to a temp `_site/` (293 HTML files, correct
counts, asset allowlist, links resolve). `clippy --all-targets -D warnings` = 0
findings in slice-04 code; `fmt` clean. See [`cdc-verification.md`](./cdc-verification.md).
Evidence `attested → reproduced`. Verified by: independent Claude (Cowork) context.
Rows: 8. Done: 8 (reproduced). Deferred: 0. No-op: 0.
