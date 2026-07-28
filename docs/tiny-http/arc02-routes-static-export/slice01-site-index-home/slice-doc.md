# Slice 01 — site-index + home

> Plan-of-record for the first slice of Arc 02 (Routes & Static Export). Open
> set: this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** (planned, not implemented) · 2026-07-28.

---

## Goal

Assemble the **site-index data** (`site.data.stats`, `site.data.featured`,
`site.data.home_categories`) and render the **home page** end-to-end from the
demo corpus through `pages/home.jinja`. This introduces the
`generator::category_index` → grid reshaping that slice 02 (browse + category)
extends, and gives the arc its first standalone non-article page.

## What the template needs (grounded in `pages/home.jinja`)

- `site.data.profile` — name, `hero_subtitle`, `name_short`, `taxonomy.tiers`
  (already provided by Arc 01's `SiteContext`).
- `site.data.stats {sources, concepts, categories}` — used in the hero line and
  the search placeholder (`stats.concepts + stats.sources`).
- `site.data.home_categories[] {slug, name, count, description, articles[]}`
  where each article is `{title, url}` — the category grid; the template renders
  the first three (`cat.articles[:3]`).
- `site.data.featured[] {title, category, tier, url}` (+ optional `teaser`) — the
  "Featured Articles" grid; the template guards `if item.teaser`.

## Scope

### In

- **`site.data.stats`**: `{sources: corpus.sources.len(), concepts:
  corpus.concepts.len(), categories: corpus.taxonomy.categories.len()}`.
- **`site.data.featured`**: reuse the existing featured selection (top-N
  concepts by typed in-degree) currently inlined in `generator::write_featured_json`
  — **extract it into a shared `pub(crate) build_featured` (byte-identical JSON
  output, guarded by the existing generator test)**, exactly as slice-03 did for
  `build_provenance_map`. The `teaser` field is intentionally absent (the
  template guards it); no teaser is computed in this slice.
- **`site.data.home_categories`**: built from `graph.category_index(corpus)`, in
  **`profile.taxonomy.categories` order** (the deployment doc: taxonomy drives
  the home grid order). Per category: `slug` (= `markdown::slugify(name)`),
  `name`, `count` (sources + concepts in the category), `description` (from the
  taxonomy entry), and `articles[]` — up to three `{title, url}`, with `url`
  being `/{slug}/` for concepts and `/source/{slug}/` for sources, and `title`
  resolved via the corpus (concept display title / source title).
- Extend `render::SiteContext` with `stats`, `featured`, `home_categories`, and
  build them in `assemble_site_context` (built once per site build; article
  pages simply ignore the extra fields).
- **`render_home`** (in `article.rs` or a new `page.rs` — confirm module home at
  slice start) rendering `pages/home.jinja` via `render::build_environment`.
- Tests driven from the `demo/` corpus.

### Out

- `browse` / `category` pages and the fuller `browse_categories` /
  per-category `page.data` grouping — **slice 02**.
- `about` / `attribution` / `search` shell / `404` wiring — **slice 03**.
- The static export writer and any `_site/` output — **slice 04**.
- No CLI wiring yet (render is a library capability + tests, as Arc 01).
- No teaser computation; no server; no search index.

## Decisions to confirm at slice start

- **`home_categories` article selection + order.** Proposed default: concepts
  first then sources, alphabetical by title within each, take the first three.
- **`featured` teaser.** Proposed default: omit (reuse the existing
  `FeaturedEntry` shape; the template guards `if item.teaser`). A teaser
  (e.g. the first sentence of a concept's Quick Definition) is a later
  enhancement, not this slice.
- **Module home for the page renderers.** Proposed: a new `page.rs` for
  non-article page renderers (`render_home` now; `render_browse`/`render_category`
  etc. later), keeping `article.rs` for concept/source. Confirm vs. extending
  `article.rs`.

## Verification approach

Unit + render tests reading the `demo/` corpus: `stats` counts; `featured`
reshape (top-N, order preserved, matches `build_featured`); `home_categories`
(taxonomy order, per-category count, ≤3 articles with correct title/url); a full
`render_home` chrome check (hero + stats line, category grid cards, featured
grid, tiers row, quick-links); a page-scale escaping check; and a no-Liquid
check on the home render path. `cargo clippy -p haleiki --all-targets -- -D
warnings` + `cargo fmt --check` clean on new code.

## Exit criteria

Ledger rows F-1…F-7 reach a final status. In short: `stats`, `featured`
(shared-fn extraction, byte-identical JSON), and `home_categories` (taxonomy
order, correct counts + article links) assemble from the demo corpus; the home
page renders end-to-end with its full chrome; escaping holds; the path uses no
Liquid; and new code is clippy(`--all-targets`)/fmt-clean.

## Notes

- Context contract authority: `templates/README.md` (home row) + `pages/home.jinja`.
- Carry forward the Arc 01 findings: reuse `render::build_environment` (auto-escape
  is per-`Environment`); run clippy **with `--all-targets`** (the slice-02 lesson,
  now in F-7).
- The `generator::build_featured` extraction mirrors slice-03's
  `build_provenance_map` pattern: one derivation, two consumers (the JSON writer
  and the render context), no logic fork.
