# Slice 01 — site-index + home · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `site.data.stats` = `{sources, concepts, categories}` with the correct demo-corpus counts (`sources.len()`, `concepts.len()`, `taxonomy.categories.len()`) | `cargo test` reading `demo/`, asserting the three counts | serious | slice-doc | open | | |
| F-2 | `site.data.featured` reuses the existing top-N-by-in-degree selection via an extracted `pub(crate) build_featured`; the shared fn produces output byte-identical to the current `featured.json` (the existing `generator` featured test still passes) | `cargo test`: featured list matches; `generator` featured/JSON test green | serious | slice-doc | open | | |
| F-3 | `site.data.home_categories` built from `category_index` in `profile.taxonomy.categories` order: each `{slug, name, count, description, articles[]}`; `count` = sources+concepts in the category; `articles[]` are `≤3` `{title, url}` with `url` = `/{slug}/` (concept) or `/source/{slug}/` (source) and titles resolved from the corpus | unit test asserting order, a known category's count, and its article title/url shapes | serious | slice-doc | open | | |
| F-4 | `render_home` renders `pages/home.jinja` end-to-end from the demo corpus with full chrome: hero (name, subtitle, `stats` line), category grid (cards with title, count, description, ≤3 article links), featured grid, tiers row, quick-links | render test grepping the output for each section marker | serious | slice-doc | open | | |
| F-5 | Escaping holds at page scale: an HTML control string placed in an auto-escaped home field (e.g. a `home_categories` name/description) is escaped in the rendered page | render test injecting `<b>x</b>` and asserting `&lt;b&gt;` | correctness | slice-doc | open | | |
| F-6 | The home render path uses **no** Liquid/Cobalt template — only `render::build_environment` + `pages/home.jinja` | `grep`/test: no `.liquid` referenced by the home render path | correctness | slice-doc | open | | |
| F-7 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 7. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
