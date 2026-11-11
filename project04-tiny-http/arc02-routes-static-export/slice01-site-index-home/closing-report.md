# Slice 01 — site-index + home · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 7 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run. Not yet committed. This is
> Arc 02's first slice. 2026-07-28.

## Row-by-row walk

**F-1 — `site.data.stats`.** `done`. `cargo test
render::tests::test_assemble_site_context_stats_match_demo_corpus_counts`
passes. `render::{Stats, build_stats}` computes the three counts directly
from `Corpus`; the test independently recomputes them and additionally
sanity-checks against the real demo corpus (10 categories, matching the
`demo/profile.yaml` migrated in Arc 01 slice 03).

**F-2 — `site.data.featured` via extracted `build_featured`, byte-identical.**
`done`. `cargo test
generator::tests::test_build_featured_real_demo_corpus_ranked_by_typed_indegree`
and the pre-existing `test_generate_creates_all_files` both pass. Extracted
`build_featured` out of `write_featured_json`, mirroring slice 03's
`build_provenance_map` pattern exactly — same fn, now called by both the JSON
writer and (via `assemble_site_context`) the render path. Unlike
`build_provenance_map`, there was no pre-existing content-checking test to
lean on for regression confidence (`test_generate_creates_all_files` only
asserts `featured.json` exists), so this slice added one: it independently
recomputes each returned entry's typed in-degree by walking the graph's
incoming edges directly (a different code path than `build_featured`'s own
counting loop) and asserts the result is genuinely ranked, then separately
confirms the JSON writer and the in-memory function agree field-by-field.

**F-3 — `site.data.home_categories`.** `done`. `cargo test
render::tests::test_assemble_site_context_home_categories_order_and_shape`
passes. `render::{HomeCategory, ArticleLink, build_home_categories}`, built
from `graph.category_index` reshaped into `profile.taxonomy.categories`
order. Verified against real data: the "mathematics" category has 24 concepts
+ 9 sources = 33 (hand-counted via `grep -l` against the real corpus before
writing the assertion — matched on the first run), and its top-3 articles are
all concepts (more than 3 concepts alone exist in that category, so sources
never appear in the truncated slice — confirms decision #1's "concepts
first" ordering is real, not accidental). The test also explicitly asserts
taxonomy order is *not* alphabetical, closing a real gap: `category_index`
returns a `BTreeMap`, so a bug that silently used its (alphabetical)
iteration order instead of `profile.taxonomy.categories`' order would only
be caught if the two orders are known to differ.

**F-4 — `render_home` full-chrome render.** `done`. `cargo test
page::tests::test_render_home_full_chrome` passes. New module
`tools/src/page.rs` (see the module-home note below), `render_home` built on
`render::build_environment` (reused, per Arc 01 v1.1). Every named chrome
section confirmed present: hero (title, subtitle, stats line with real
source/concept counts), category grid (a real `/category/mathematics/` card
with count and description), featured grid (a "Read article" link), tiers
row ("Foundational"), and quick-links (`/attribution/`).

**F-5 — Escaping holds at page scale.** `done`. `cargo test
page::tests::test_render_home_escaping_holds_at_page_scale` passes. Injects
`<b>x</b>` into a real, fully-assembled `SiteContext`'s first
`home_categories[].description` and re-renders through the complete
`render_home` pipeline (not an isolated fixture) — escaped correctly.

**F-6 — No Liquid/Cobalt in the home render path.** `done`. `cargo test
page::tests::test_render_home_path_references_no_liquid_templates` passes.
`include_str!`-greps `page.rs` — the module unique to the home path;
`render.rs` was already covered by Arc 01 slice 03's equivalent test, so
re-checking it here would be redundant, not additional coverage. Applied the
slice-02 self-referential-grep lesson (concatenated needle, not a literal)
proactively — didn't have to rediscover it by watching the test fail against
itself, the way slice 02 did.

**F-7 — New code clean under `--all-targets`.** `done`. `cargo fmt --all --
--check` clean. `cargo clippy --all-targets --all-features --workspace --
-D warnings` reports 30 findings — all pre-existing, confirmed by reading
each one's actual content, not just filtering by filename: 28 unchanged from
Arc 01's close (`demo/*.rs`, `tools/tests/demo_fetch.rs`) and 2 in
`generator.rs`'s pre-existing `test_provenance_json_bidirectional`
(`assert!(x == y)`, the same pair CDC found at Arc 01 close — now at lines
759/761 instead of 752/754 purely because this slice's unrelated edits above
them in the file shifted line numbers). Ran with `--all-targets
--all-features` from the first invocation, not added after a gap was found.

## Silent-drop check

Ledger opened with 7 rows (F-1…F-7); this report addresses 7. No missing rows.

## Decisions made (per slice-doc's "confirm at slice start" list)

All three followed the slice-doc's proposed defaults, disclosed here rather
than silently assumed:

1. **`home_categories` article selection + order**: concepts first, then
   sources, alphabetical by title within each group, first three. Confirmed
   real (see F-3 above — math's top 3 are genuinely all concepts because
   there are more than 3 to choose from, not because sources happen to sort
   after by coincidence).
2. **`featured` teaser**: omitted. `generator::FeaturedEntry` carries no
   teaser field; `pages/home.jinja`'s `{% if item.teaser %}` guard correctly
   reads it as undefined and skips the teaser paragraph.
3. **Module home**: a new `tools/src/page.rs`, not an extension of
   `article.rs`. Rationale (matching the slice-doc's own framing): `article.rs`
   holds concept/source renderers, which have real, page-type-specific
   `page.data`; home's `page` context is minimal (home.jinja reads almost
   entirely from `site.data`, overriding `{% block title %}` unconditionally
   and touching no other `page.*` field) — different enough in shape that a
   separate module for "pages that mostly just read `site.data`" reads
   cleaner than folding it into the article-specific module, and leaves room
   for `render_browse`/`render_category`/etc. to land in the same place
   without renaming `article.rs` later.

## Bubble-up to arc-plan.md

Nothing new to add to `arc-plan.md`'s open findings. The featured-selection
extraction pattern (`build_featured`, mirroring `build_provenance_map`) is
now precedent twice over; if slice 02's `browse_categories`/per-category
`page.data` reshaping surfaces something in `generator.rs` worth extracting
the same way, that's an expected continuation of the pattern, not a new
finding to flag.

## What was NOT done (explicitly, per the "no silent drops" rule)

- No `browse`/`category` pages or the fuller subcategory-grouping — slice 02.
- No `about`/`attribution`/`search` shell/`404` wiring — slice 03.
- No static export writer, no `_site/` output — slice 04.
- No CLI wiring: nothing in `main.rs`/`cli.rs` calls `render_home`. Render
  remains a library capability with tests, matching Arc 01's pattern.
- No teaser computation for featured articles (explicitly deferred per
  decision #2 above, a later enhancement per the slice-doc, not a gap).
- The pre-existing clippy debt named in F-7 was not fixed (out of this
  slice's boundary).
- Nothing has been committed to git. All changes are in the working tree.

## Independent verification status

Not yet performed. Per `LEDGER-DISCIPLINE.md`, this closing report is CC's
`attested` claim; a CDC-equivalent pass (fresh context or operator, re-running
each row's Verify command against the actual artifacts) is required before any
row's evidence strengthens to `reproduced` and before this slice can be
considered closed rather than proposed-done. Per the cc-prompt's explicit
instruction, this report does **not** include `cdc-verification.md` — that
file is the independent reviewer's to write.
