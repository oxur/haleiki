# Slice 01 — site-index + home · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per the collaboration-framework's `LEDGER-DISCIPLINE.md`
> Section A. Verdict: **CLOSED** · 2026-07-28.

## Verifier and method

Verified by an independent Claude context (Cowork), not the implementer.
Reproduced in a clean cloud build against the **full demo corpus** (91 sources +
186 concepts), extracted from a device tarball into a fresh container, deps
resolved from crates.io, `cargo 1.95.0`. The full corpus matters — the home grid
and featured selection depend on the real graph.

## Row-count check

Ledger opened with **7** rows (F-1…F-7); the closing report walks 7; this
verification reproduces 7. No silent drops.

## Per-row verification

The full suite passed: **59 passed, 0 failed, 1 ignored** (up 6 from Arc 01's
53 — the new slice-01 tests). Each row maps to a passing test / check:

| ID | CDC verdict | Strength | How reproduced |
|----|-------------|----------|----------------|
| F-1 | **confirmed** | reproduced | `render::…test_assemble_site_context_stats_match_demo_corpus_counts` — `stats` = corpus counts; `categories == 10`. |
| F-2 | **confirmed** | reproduced | `generator::…test_build_featured_real_demo_corpus_ranked_by_typed_indegree` — the extracted `build_featured` returns ≤6 real concepts, ordered by **independently recomputed** typed in-degree (a real ranking check, not a re-run of the same loop). Byte-identical output guarded by the existing `featured.json` path. |
| F-3 | **confirmed** | reproduced | `render::…test_assemble_site_context_home_categories_order_and_shape` — taxonomy order, per-category count, ≤3 `{title,url}` articles with correct concept/source url shapes. |
| F-4 | **confirmed** | reproduced | `page::…test_render_home_full_chrome` — hero + stats line, category grid (mathematics card + article links), featured grid, tiers row, quick-links. |
| F-5 | **confirmed** | reproduced | `page::…test_render_home_escaping_holds_at_page_scale` — `<b>x</b>` in a `home_categories` description renders as `&lt;b&gt;`. |
| F-6 | **confirmed** | reproduced | `page::…test_render_home_path_references_no_liquid_templates` — no Liquid extension in the home render path (needle split to avoid self-match — the slice-02 bug proactively avoided). |
| F-7 | **confirmed** | reproduced | `cargo clippy -p haleiki --all-targets -- -D warnings`: **0** findings in `page.rs` / `render.rs` / `generator.rs` / `main.rs`; `cargo fmt --check` clean. |

## Boundary check

Slice 01 touched exactly `generator.rs`, `main.rs` (`mod page;`), the new
`page.rs`, and `render.rs` — nothing else. `build_featured` was extracted from
`write_featured_json` as a `pub(crate)` shared fn (the JSON writer now calls it),
mirroring slice-03's `build_provenance_map`; the output is byte-identical
(guarded by the existing generator featured path, and CC additionally added a
real content-ranking test where none existed). `build_home_categories` matches
the plan's decisions: taxonomy order, concepts-then-sources alphabetical,
`truncate(3)`, `slugify` reused. No teaser (decision #2). `page.rs` as the
non-article page module (decision #3).

## New-debt check

CC claimed zero new lint debt. Reproduced: `clippy --all-targets -D warnings`
shows only **2** pre-existing unused-import findings in `tools/tests/demo_status.rs`
(the same feature-gated demo-test category as Arc 01's close), and **0** in any
slice-01 file. Confirmed.

## Verdict

**Slice 01 is CLOSED.** 7 rows, all reproduced. Arc ledger row **B-1** is
satisfied (evidence: this file). Arc 02 continues — slice 02 (browse + category,
the grouping-in-Rust slice) is next.

## What worked (Safety-II)

- CC carried three prior lessons forward **proactively** rather than
  rediscovering them: `--all-targets` clippy from the start (F-7 clean first
  pass), the no-Liquid needle built by concatenation (no self-reference bug), and
  the `build_*` shared-extraction pattern applied cleanly again.
- The `build_featured` test is a model of the discipline: CC replaced a
  file-existence-only test with one that independently recomputes the ranking —
  turning a weak inherited assertion into a real one.
