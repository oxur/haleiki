# Slice 02 — browse + category

> Plan-of-record for the second slice of Arc 02 (Routes & Static Export). Open
> set: this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** (planned, not implemented) · 2026-07-28.

---

## Goal

Render the **browse index** and the **per-category pages** end-to-end from the
demo corpus, with **all grouping resolved in Rust** (the README's explicit
requirement: "grouping is done in Rust, not the template"). This is the
grouping-heavy slice — it extends slice 01's `category_index` reshaping into two
distinct grouping schemes and adds the first *parameterised* page (one render per
category).

## What the templates need (grounded in the page templates)

Two different groupings — confirmed from the template markup, not assumed:

- **`pages/browse.jinja` → `site.data.browse_categories[]`** grouped **by
  subcategory** (`.subcategory-group` / `.subcategory-label`):
  `{slug, name, count, description, groups[]{name, articles[]{title, url, type}}}`.
  The left-nav also lists each category with its count.
- **`pages/category.jinja` → `page.data.category`** grouped **by tier**
  (`.tier-section` / `.tier-header--{modifier}`):
  `{name, description, count, source_count, concept_count, subcategories[],
  groups[]{name, modifier, articles[]{title, url, type, description}}}`. The
  sidebar info card reads `count` / `source_count` / `concept_count` /
  `subcategories[]`.

`type` is `"concept"` | `"source"` (drives the badge). The category page's
article `description` is guarded (`if a.description`).

## Data sources (reuse, don't re-derive)

`graph.category_index(corpus)` returns
`BTreeMap<category, CategoryIndexEntry {name, description, sources:Vec<slug>,
concepts:Vec<slug>, subcategories: BTreeMap<subcat, SubcategoryEntry
{sources, concepts}>}>`, with `"_uncategorized"` as the empty-subcategory
bucket. Browse groups come straight from `subcategories`; category tier-groups
come from looking up each article slug's `tier` in the corpus
(`corpus.concepts[slug].tier` / `corpus.sources[slug].tier`). Article title/url
resolution is the same as slice 01's `home_categories` (concept `/{slug}/`,
source `/source/{slug}/`) — factor a shared article-ref helper.

## Scope

### In

- **`site.data.browse_categories`**: extend `SiteContext` + `assemble_site_context`,
  in `profile.taxonomy.categories` order. Per category, `groups[]` = one group
  per subcategory (BTreeMap order), `"_uncategorized"` → a group with `name = ""`
  (the template hides the label when empty); each group's `articles[]` are
  `{title, url, type}`, concepts-then-sources, alphabetical within.
- **`CategoryPageData`** + `build_category_page_data(corpus, entry, profile)`:
  `groups[]` = one per **tier** present, in taxonomy tier order, `name` = tier
  label, `modifier` = tier slug; `articles[]` = `{title, url, type, description}`
  (description omitted this slice — see decisions); `subcategories[]` = the
  subcategory names excluding `"_uncategorized"`; `count` / `source_count` /
  `concept_count` from the entry.
- **`render_browse(env, site)`** → `pages/browse.jinja`; **`render_category(env,
  site, category)`** → `pages/category.jinja` (one per category), in `page.rs`.
  `page.title` set so the breadcrumb fallback reads correctly
  (`"Browse"` / the category name).
- Tests from the `demo/` corpus.

### Out

- `about` / `attribution` / `search` shell / `404` — **slice 03**.
- The static export writer and any `_site/` output (walking the per-category
  routes) — **slice 04**. This slice renders a category page on demand; it does
  not yet enumerate/write them all.
- No CLI, server, or search index. Article `description` text (a later
  enhancement, like slice 01's featured teaser).

## Decisions to confirm at slice start

- **Grouping semantics** (proposed default, from the template markup): **browse
  groups by subcategory, category groups by tier**. This is the load-bearing
  interpretation — confirm it before building.
- **Category-page article `description`.** Proposed: omit (the template guards
  it). Deriving a per-article blurb (e.g. a concept's Quick Definition first
  sentence) is a later enhancement.
- **Empty tiers / subcategories.** Proposed: category tier-groups include only
  **non-empty** tiers, in taxonomy tier order; browse includes the
  `"_uncategorized"` bucket as an unlabeled group.
- **Article ordering within a group.** Proposed: concepts-then-sources,
  alphabetical by title (matching slice 01's `home_categories`).

## Verification approach

Unit + render tests from the `demo/` corpus: `browse_categories` (taxonomy
order; a known category's subcategory groups; article `type` correct);
`build_category_page_data` (tier grouping in tier order; counts;
`subcategories[]`; article shapes) for a known category (e.g. `mathematics`); a
full `render_browse` chrome check and a full `render_category` chrome check
(header, tier sections with `--modifier`, article cards/badges, sidebar info
card); a page-scale escaping check; and a no-Liquid check. `cargo clippy -p
haleiki --all-targets -- -D warnings` + `cargo fmt --check` clean on new code.

## Exit criteria

Ledger rows F-1…F-7 reach a final status. In short: `browse_categories` (by
subcategory) and `CategoryPageData` (by tier) assemble from the demo corpus with
correct counts, order, and article types; browse and a real category page render
end-to-end with full chrome and all grouping resolved in Rust; escaping holds;
the path uses no Liquid; new code is clippy(`--all-targets`)/fmt-clean.

## Notes

- Context contract authority: `templates/README.md` (browse / category rows) +
  `pages/browse.jinja` / `pages/category.jinja`.
- Reuse `render::build_environment` (auto-escape per-`Environment`); run clippy
  **with `--all-targets`** (now in F-7). Reuse slice 01's article-link resolution
  rather than duplicating a second convention.
