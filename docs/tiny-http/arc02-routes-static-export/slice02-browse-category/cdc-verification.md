# Slice 02 — browse + category · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per the collaboration-framework's `LEDGER-DISCIPLINE.md`
> Section A. Verdict: **CLOSED** · 2026-07-28.

## Verifier and method

Verified by an independent Claude context (Cowork), not the implementer.
Reproduced in a clean cloud build against the **full demo corpus**, deps from
crates.io, `cargo 1.95.0`.

## Row-count check

Ledger opened with **7** rows (F-1…F-7); reproduces 7. No silent drops.

## Per-row verification

Full suite: **66 passed, 0 failed, 1 ignored**. Each row maps to a passing test:

| ID | CDC verdict | Strength | How reproduced |
|----|-------------|----------|----------------|
| F-1 | **confirmed** | reproduced | `render::…test_assemble_site_context_browse_categories_grouped_by_subcategory` — taxonomy order, subcategory groups, article `type`, incl. the `_uncategorized` recovery (below). |
| F-2 | **confirmed** | reproduced | `render::…test_build_category_page_data_mathematics_grouped_by_tier` — tier groups in taxonomy order, counts, `subcategories[]` excluding `_uncategorized`. |
| F-3 | **confirmed** | reproduced | `page::…test_render_browse_full_chrome` — left-nav category list + counts, subcategory groups, article links + `type` badges. |
| F-4 | **confirmed** | reproduced | `page::…test_render_category_mathematics_full_chrome` — header, tier sections with `--modifier`, article cards/badges, sidebar info card. |
| F-5 | **confirmed** | reproduced | `render_browse` + `render_category` escaping tests — `<b>x</b>` in a group/description field renders `&lt;b&gt;`. |
| F-6 | **confirmed** | reproduced | `page::…test_render_browse_category_path_references_no_liquid_templates` — needle split to avoid self-match. |
| F-7 | **confirmed** | reproduced | `clippy --all-targets -D warnings`: **0** findings in `page.rs`/`render.rs`; `fmt` clean. The `bool_to_int_with_if` CC fixed is gone; only the same pre-existing `demo_status.rs` imports remain. |

## The `_uncategorized` bug + fix (independently confirmed)

`graph::category_index` deliberately excludes `"_uncategorized"` from
`entry.subcategories`, so a naïve browse grouping over `subcategories` alone
drops uncategorized articles (while still counting them). CC caught this with an
**exact-count** test on a real category (`philosophy-of-mind`) rather than a
loose shape check, and fixed it with a set difference:
`entry.concepts ∪ entry.sources` minus every slug present in any
`entry.subcategories[*]`, surfaced as the unlabeled group. I read the fix and it
is correct; the exact-count test passes in my build. This is the ledger
discipline working — an exact assertion on real data caught a silent drop a
shape check would have missed.

## Boundary check

Only `page.rs` and `render.rs` changed. `resolve_article_ref` was factored as
the shared concept/source→`{title,url,type}` resolver, and slice 01's
`build_home_categories` was refactored to use it — **behavior-preserving**: the
slice-01 `test_assemble_site_context_home_categories_order_and_shape` is intact
and green in my build. No scope creep.

## Latent robustness note (not a defect for the demo)

`build_category_page_data` groups articles by tier and iterates only the
taxonomy's tiers, so an article whose `tier` is **not** in the taxonomy (or
empty) would be counted in `count`/`source_count`/`concept_count` but omitted
from the displayed tier groups — the same shape of gap as the `_uncategorized`
bug, one axis over. **For this corpus it does not fire:** all 277 demo articles
carry a tier in `{foundational, intermediate, advanced}` (0 missing), so the
tier-group article sum equals the category count. It is also guarded upstream —
the validator errors on unknown tiers. The only unguarded residue is an *empty*
tier, which no demo article has. Recorded as a low-priority robustness item for a
future arc (an "other" bucket or a non-empty-tier validator rule), not a slice-02
defect.

## Verdict

**Slice 02 is CLOSED.** 7 rows, all reproduced. Arc ledger row **B-2** is
satisfied (evidence: this file). Arc 02 continues — slice 03 (about /
attribution / search shell / 404) is next, then slice 04 (static export) closes
the arc.

## What worked (Safety-II)

- The exact-count test on real data (`philosophy-of-mind`, 8 uncategorized) is
  why the `_uncategorized` silent drop was caught in development rather than at
  CDC or later. Loose shape checks would have passed the buggy version.
- CC continues to internalize the corrections: `--all-targets` clean first pass,
  concatenated no-Liquid needle, shared-helper extraction with a re-run of the
  refactored caller's tests to prove behavior-preservation.
