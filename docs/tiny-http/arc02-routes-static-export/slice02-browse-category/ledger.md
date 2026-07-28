# Slice 02 — browse + category · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `site.data.browse_categories` built from `category_index` in taxonomy order, grouped **by subcategory**: each `{slug, name, count, description, groups[]}`; each group `{name, articles[]{title, url, type}}` with `"_uncategorized"` → `name = ""`; `type` correct (`concept`/`source`); articles concepts-then-sources, alphabetical | unit test asserting order, a known category's groups, and article `type`/url shapes | serious | slice-doc | open | | |
| F-2 | `build_category_page_data` for a known category groups **by tier** in taxonomy tier order (non-empty tiers only): each group `{name, modifier, articles[]{title, url, type}}`; `subcategories[]` excludes `"_uncategorized"`; `count`/`source_count`/`concept_count` correct | unit test on a known category (e.g. `mathematics`) asserting tier groups, counts, subcategories | serious | slice-doc | open | | |
| F-3 | `render_browse` renders `pages/browse.jinja` end-to-end from the demo corpus: left-nav category list with counts, per-category sections with subcategory groups, article links + `type` badges, header stats | render test grepping the output for each marker | serious | slice-doc | open | | |
| F-4 | `render_category` renders `pages/category.jinja` end-to-end for a real category: header (name, description, count, group count), tier sections with `tier-header--{modifier}`, article cards + badges, sidebar info card (`count`/`source_count`/`concept_count`/`subcategories`) | render test grepping the output | serious | slice-doc | open | | |
| F-5 | Escaping holds at page scale: an HTML control string in an auto-escaped browse/category field (e.g. a group name or category description) is escaped in the rendered page | render test injecting `<b>x</b>` and asserting `&lt;b&gt;` | correctness | slice-doc | open | | |
| F-6 | The browse/category render path uses **no** Liquid/Cobalt template — only `render::build_environment` + `pages/{browse,category}.jinja` | `grep`/test: no `.liquid` referenced by the render path (needle built by concatenation) | correctness | slice-doc | open | | |
| F-7 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 7. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
