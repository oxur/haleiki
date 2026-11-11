# Slice 03 — about + attribution + search shell + 404

> Plan-of-record for the third slice of Arc 02 (Routes & Static Export). Open
> set: this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** (planned, not implemented) · 2026-07-28.

---

## Goal

Render the four **standalone pages** end-to-end from the demo corpus: **about**,
**attribution**, the **search shell**, and **404**. Three are simple
(profile-only / near-static); attribution is the real work (reshape the
corpus-derived attribution data + add curated `intro`/`licenses`). After this
slice, every page type the site has exists as a renderer, leaving only slice 04
(the static-export walker) to compose them into `_site/`.

## What the templates need (grounded in the page templates)

- **`pages/about.jinja`** — entirely `site.data.profile`: `name_short`,
  `about_intro`, `trust.{enabled, level_order, levels[level]{label, description,
  note}}`, and `about_body[]` (printed `| safe`). Already in `SiteContext`; the
  render is essentially free.
- **`pages/404.jinja`** — only `site.data.profile.name_short`. Already
  smoke-renders (Arc 01 slice 01). Free.
- **`pages/search.jinja`** — `search {query, facets[]{title, options[]{label,
  count}}, results[]{type, category, tier, url, title, excerpt}}`. This is the
  **static shell**: real search is client-side (`search.js`, already in the
  template). Render with an **empty** search context (`query = ""`,
  `results = []`, `facets = []`); the page's structure (search input, filter
  sidebar frame, empty state) renders and `search.js` hydrates it. Facets,
  results, and the JSON index are **Arc 04**.
- **`pages/attribution.jinja`** — `site.data.attribution {intro, general_notice,
  source_count, sources_by_project[]{name, sources[]{slug, title, license, url}},
  licenses[]{title, points[], links[]{label, url}}}`. The reshape work.

## Scope

### In

- **`render_about`**, **`render_404`**, **`render_search`** (in `page.rs`),
  rendering their templates via `render::build_environment`. `render_search`
  builds a minimal empty `search` context.
- **`site.data.attribution`** (extend `SiteContext` + `assemble_site_context`):
  - Reuse the corpus-derived aggregation currently in
    `generator::write_attribution_json` — **extract the sources-by-project
    grouping into a shared `pub(crate)` fn (byte-identical JSON, guarded by the
    existing attribution test)**, mirroring slice-01's `build_featured` /
    slice-03's `build_provenance_map`. Reshape its `BTreeMap<project, …>` into
    the template's `sources_by_project[]` list (`{name, sources[]}`),
    project-alphabetical, each source `{slug, title, license, url}`.
  - `source_count` and `general_notice` from that aggregation.
  - `intro` and `licenses[]` are **curated content** → add an `attribution:
    {intro, licenses[]}` section to `profile.yaml` + `render::Profile` (content
    belongs in config, not code, for a framework). `licenses[]` =
    `{title, points[], links[]{label, url}}` for this deployment's licences
    (CC BY-SA 4.0 content; Apache-2.0 + MIT framework).
  - **`render_attribution`** rendering `pages/attribution.jinja`.
- Tests from the `demo/` corpus.

### Out

- The static export writer / `_site/` output — **slice 04** (this slice renders
  each page on demand; it does not walk/write them).
- The real search: facets, the JSON index, `search.js` wiring/behavior, result
  ranking — **Arc 04**. This slice only renders the empty shell.
- No CLI, server. No template edits.

## Decisions to confirm at slice start

- **Content home for `intro` + `licenses[]`.** Proposed: **profile-sourced**
  (add `attribution:` to `profile.yaml` + `render::Profile`). The parser's older
  `Profile` shape ignores unknown fields, so this does not disturb
  `parse_corpus` (same lenient-parse property slice-03's migration relied on).
  Alternative: Rust-curated for now, move to profile later.
- **`general_notice` home.** It is currently a hardcoded string in
  `generator.rs`. Proposed: reuse it as-is this slice; note that for consistency
  it should eventually move to `profile.attribution` too (a cleanup item, not
  required here).
- **Search shell empty-state.** With an empty query, `search.jinja` renders its
  "No results found" block (the template shows it whenever `results` is empty).
  Proposed: accept this as the pre-hydration shell (`search.js` manages the live
  UX in Arc 04); do **not** edit the template to special-case an empty query.

## Verification approach

Render tests from the `demo/` corpus: `render_about` (header, trust-level cards
from `level_order`, about-body paragraphs), `render_404` (error chrome +
`name_short`), `render_search` (empty shell: input, filter frame, `search.js`
tag), attribution assembly (sources-by-project reshape byte-identical to the JSON
path; `source_count`; `intro`/`licenses` from profile), `render_attribution`
(intro, licence callout, per-project source tables, licence-text cards); a
page-scale escaping check; and no-Liquid checks. `cargo clippy -p haleiki
--all-targets -- -D warnings` + `cargo fmt --check` clean on new code.

## Exit criteria

Ledger rows F-1…F-8 reach a final status. In short: about / 404 / search-shell /
attribution all render end-to-end from the demo corpus; the attribution reshape
is byte-identical to the JSON path with `intro`/`licenses` sourced from the
profile; escaping holds; the paths use no Liquid; new code is
clippy(`--all-targets`)/fmt-clean.

## Notes

- Context contract authority: `templates/README.md` (about / attribution /
  search rows) + the four page templates.
- Reuse `render::build_environment`; run clippy **with `--all-targets`** (F-8).
  Build the no-Liquid needle by concatenation.
- The `build_attribution_*` extraction is the same one-derivation-two-consumers
  pattern used three times already; keep the `attribution.json` output identical.
