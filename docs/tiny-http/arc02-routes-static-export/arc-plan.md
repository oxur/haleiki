# Arc 02 — Routes & Static Export

> The arc plan-of-record: the full wiki route set plus the static-export
> writer that turns Arc 01's render pipeline into the pivot's default
> deliverable. Part of the tiny_http pivot ([`../project-plan.md`](../project-plan.md));
> architecture in [`../design-delta.md`](../design-delta.md). Status:
> **active** · 2026-07-28.

---

## 1. Capability

Deliver the **full wiki route set and a complete static `_site/`** from the demo
corpus: render **home**, **browse**, **per-category**, **about**,
**attribution**, **404**, and the **search shell** through minijinja, with **all
category grouping resolved in Rust** (never in the template), and write a
complete, asset-complete, deployable `_site/` — the pivot's default deliverable.

This arc composes Arc 01's article renderer with the remaining page types and
adds the export sink. After it, `haleiki` can build the entire demo wiki to
static HTML that opens in a browser with parity to the design review. (Search
*results* are wired in Arc 04; this arc renders the search page's static shell.
The tiny_http *server* sink is Arc 03; news is Arc 05.)

## 2. Slice breakdown

Four slices, in dependency order. The first three add page renderers on top of
Arc 01's `render::build_environment` + `SiteContext` pattern; the fourth walks
them all into `_site/`. Only slice 01 is planned deep here (*plan late, plan
deep*).

| Slice | Scope | Load-bearing for |
|-------|-------|------------------|
| **01 — site-index + home** | The site-index data (`site.data.stats`, `site.data.featured` reshaped from `generator`'s featured selection, `site.data.home_categories` reshaped from `generator::category_index`) and `render_home` → `pages/home.jinja`. Establishes the `category_index` → grid reshaping. | 02, 04 |
| **02 — browse + category** | Extend the reshaping into `site.data.browse_categories` (subcategory groups + article type) and per-category `page.data.category` (`groups[]`, `subcategories[]`, `source_count`/`concept_count`, article descriptions); `render_browse` + `render_category` (one per category). The grouping-in-Rust slice. | 04 |
| **03 — about + attribution + search shell + 404** | `render_about` (profile only — nearly free), `render_attribution` (reshape `generator`'s attribution map→list + `intro` + a curated `licenses[]`), `render_search` (static shell), and wire `render_404`. The simpler standalone pages. | 04 |
| **04 — static export writer** | Walk every route → render → write `_site/{path}/index.html` (and `404.html`); copy `assets/` → `_site/assets/`; wire to a CLI command. Composes the whole arc into the default deliverable, from the demo corpus. | arc close |

**Sequencing rationale.** Slice 01 introduces the `category_index` reshaping that
02 extends; 03 is independent simpler pages; 04 needs every page renderer to
exist before it can walk them. 01→02→04 is the load-bearing spine; 03 can land
any time before 04.

## 3. Dependencies

**Consumes** (from Arc 01, closed): `render::build_environment`,
`render::SiteContext` / `assemble_site_context`, `render::resolve_profile`,
`article::render_concept` / `render_source`, `markdown::render_markdown`.
**Reuses** the existing `generator` derivations — `category_index`
(`CategoryIndexEntry {name, description, sources, concepts, subcategories}`),
the featured selection (top-N concepts by typed in-degree), and the attribution
aggregation — reshaping them to the new template contracts rather than
re-deriving. **Reuses** `graph` and `parser::Corpus` (bodies now retained).

**Leaves for later arcs**: the tiny_http server sink (Arc 03); the search JSON
index + `search.js` wiring + facets (Arc 04 — this arc renders only the static
search shell); news (Arc 05); removing the legacy `_data` JSON writer and the
Cobalt/Liquid files (Arc 06 cleanup).

## 4. Arc ledger

Opens here; closes (per-row walk) in `closing-report.md` at arc close. Class (a)
= slices closed (attested by pointer); class (b) = composition, **reproduced at
arc scale** (run the export, inspect the `_site/` tree and key pages); class (c)
= bubble-up findings dispositioned.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| B-1 | Slice 01 (site-index + home) closed | ptr: `slice01-site-index-home/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 01 CDC-closed 2026-07-28, 7/7 rows |
| B-2 | Slice 02 (browse + category) closed | ptr: `slice02-browse-category/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 02 CDC-closed 2026-07-28, 7/7 rows (incl. _uncategorized fix) |
| B-3 | Slice 03 (about + attribution + search shell + 404) closed | ptr: slice03 cdc-verification | correctness | arc-plan | open | |
| B-4 | Slice 04 (static export writer) closed | ptr: slice04 cdc-verification | correctness | arc-plan | open | |
| B-5 | Static export produces a complete `_site/` from the demo corpus: `index.html` for home, browse, every category, every concept, every source, about, attribution, search, and `404.html` — each at its contract path | reproduce at arc scale: run the export, assert the file tree + spot-render key pages | serious | arc-plan | open | |
| B-6 | `assets/` (css, js, fonts, icons, vendor) copied into `_site/assets/` | reproduce: assert the asset tree under `_site/assets/` | serious | arc-plan | open | |
| B-7 | Category grouping is resolved in Rust: browse + category pages render `groups[]` with no in-template de-duplication; a spot category's groups match the corpus | reproduce: inspect a rendered category/browse page + grep templates for absence of dedup logic | correctness | arc-plan | open | |
| B-8 | Key internal links resolve within `_site/`: home → category → article paths, and category → article paths, all exist as files | reproduce: follow a sample link chain against the `_site/` tree | serious | arc-plan | open | |

_(Class-(c) bubble-up rows are added as slices surface findings.)_

## 5. Version history

### v1.0 — 2026-07-28
Initial arc plan. Capability (full wiki route set + complete static `_site/` from
the demo corpus, grouping resolved in Rust), four-slice breakdown (site-index +
home → browse + category → about/attribution/search-shell/404 → static export
writer), dependencies against Arc 01's render pipeline and the reused `generator`
derivations, and the arc ledger opened with the composition rows. Slice 01
planned deep; slices 02–04 roadmap-level.
