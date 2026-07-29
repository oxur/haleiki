# Slice 02 — client wiring + facets + arc composition

> Plan-of-record for the second and final slice of Arc 04 (Search / Pagefind).
> Open set: this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Closing this slice closes the arc. Status: **open** (planned, not implemented)
> · 2026-07-29.

---

## Goal

Turn the empty `/search/` shell into **working faceted search** by wiring
`assets/js/search.js` to Pagefind's **JavaScript API** against the `/pagefind/`
bundle Slice 01 ships. Then close Arc 04 by reproducing an end-to-end query +
facets + sink-parity in a real (headless) browser against **both** a live
`haleiki serve` and a built static export.

Slice 01 made the bundle exist and both sinks host it, with **no client UI**.
This slice is almost entirely **client-side JavaScript + a minimal template
touch** — little or no Rust change is expected.

## Context (what exists)

- **The shell** (`page::render_search`, Arc 02): renders `search.jinja` with
  `query=""`, `results=[]`, `facets=[]` — the empty pre-hydration frame. Arc 02
  decision stands: *do not special-case an empty query server-side*. This slice
  does **not** change `render_search`.
- **The template** (`templates/pages/search.jinja`): a `.search-layout` with a
  `.filter-sidebar` (loops `search.facets` → `.filter-group` → `.filter-option`
  with `.filter-checkbox`/`.filter-count`), a `.search-hero` form
  (`action="/search/" method="get"`, input `.search-hero-input` name `q`), a
  `.search-results-count`, an `.search-results-list` `<ol>` of `.search-result`
  (`.result-badges` with `result-badge--type-concept`/`--type`/`--category`/
  `--tier`, `.result-title > a[href]`, `.result-excerpt` printed `| safe`,
  `.result-url`), and a `.search-empty` state. It already loads
  `<script src="/assets/js/search.js" defer>`.
- **`search.js`**: the 17-line stub — focus + Cmd-K only, a classic vanilla IIFE
  (house style: see `theme.js`, `browse-scrollspy.js`; **no ES modules, no build
  step**). This slice rewrites it.
- **The bundle** (Slice 01): served at `/pagefind/` in both sinks. Pagefind's JS
  API: `await pagefind.init()`, `pagefind.search(q, {filters})` →
  `{results, filters, totalFilters, unfilteredResultCount}`; each result
  `await r.data()` → `{url, meta:{title,…}, filters, excerpt}`;
  `await pagefind.filters()` → `{group:{value:count}}`. Records carry filters
  `category`/`tier`/`type`/`trust` and meta `title`/`url`/`excerpt` (Slice 01).

## What to build

- **Rewrite `assets/js/search.js`** (classic IIFE, house style, no build step;
  dynamic `import('/pagefind/pagefind.js')` — valid in a classic script):
  - **Init + `?q=` seeding.** On load, read `?q=` from the URL, seed
    `.search-hero-input`, and run an initial search if non-empty. Keep the
    existing focus + Cmd-K behavior.
  - **Query.** Debounced input → `pagefind.search(q, {filters})`; render the
    results into `.search-results-list` matching the template's `.search-result`
    markup (type/category/tier badges, `.result-title > a[href=url]`,
    `.result-excerpt` with Pagefind's `<mark>`-highlighted excerpt, `.result-url`).
    Update `.search-results-count`. Empty query → clear to the initial state; no
    matches → show `.search-empty`.
  - **Facets.** Build the `.filter-sidebar` groups (category / tier / page-type /
    trust) from `pagefind.filters()` with per-value counts; humanize slug labels.
    Toggling a checkbox re-runs the search **within-group OR, across-group AND**;
    counts update live from the search response's `filters`/`totalFilters`.
  - **Testing seam.** Expose the pure helpers (query-string parse, result→DOM
    mapper, filter-state reducer, count formatter) on a single guarded global
    (e.g. `window.__haleikiSearch`) so the headless-browser harness can assert
    them at unit level *inside the real script*, without a separate JS toolchain.
- **Minimal template touch** (only if needed): humanized facet labels and, if it
  keeps the design team's markup canonical, `<template>` fragments for a result
  item + a facet option that `search.js` clones (decision #3). Prefer **no**
  structural change to `search.jinja` beyond this.

## Decisions to confirm at slice start

1. **Verification tooling (load-bearing — the repo has none today).** There is no
   JS/browser test tooling in the repo (no `package.json`/`deno.json`/Playwright;
   the JS assets are currently untested). Proposed: verify against a **real
   headless Chromium** (available in the build environment) driving **both** a
   live `haleiki serve` and a served static `_site/` — the harness does *both*
   e2e DOM assertions *and* direct calls to the exposed pure helpers
   (`window.__haleikiSearch.*`) for unit-level logic, so no Node/Deno toolchain is
   added to ship. Ship the harness as a **committed, runnable script**
   (recommend **Deno + Playwright** — matches the operator's stack — or a Rust
   `headless_chrome` test), reproduced at CDC. **Defer** whether it wires into
   `make check`/CI to Arc 06 (it needs Chromium in CI). Alternatives: fold a Rust
   `headless_chrome`/`chromiumoxide` test into `cargo test` now (in-language, but
   a heavy, flake-prone dev-dep + Chromium in CI immediately); or CDC-only manual
   reproduction with no committed harness. **Confirm the driver (Deno vs Rust) and
   the defer-CI call.**
2. **`search.js` stays a classic IIFE** (house style; the template keeps
   `<script … defer>`, no `type="module"`). Proposed yes — dynamic `import()`
   covers Pagefind; the guarded-global seam covers testability without a module
   refactor. Confirm (vs. converting to an ES module).
3. **Markup source of truth.** Proposed: add `<template>` fragments to
   `search.jinja` (one result item, one facet option) cloned by `search.js`, so
   the design team's markup stays canonical rather than duplicated as JS strings.
   Confirm (vs. building nodes in JS).
4. **Facet semantics.** Proposed: within-group **OR**, across-group **AND**;
   counts from Pagefind's `filters`/`totalFilters`. Confirm.
5. **URL state.** Proposed: reflect `q` in the URL (seed on load — shareable /
   reloadable). Facet state in the URL is **optional** (nice-to-have); confirm
   whether facets must persist in the URL too.
6. **No-JS behavior.** Pagefind is JS-only. Proposed: accept **JS-required**
   search; the shell's `.search-empty` copy should read sensibly without JS
   (optionally a "search needs JavaScript" note). `render_search` stays the empty
   shell (no Rust change). Confirm.

## Scope

### In

- The `search.js` rewrite (query, results render, facets + live counts, `?q=`
  seeding, empty/no-results states, the testing seam).
- Minimal `search.jinja` touch (humanized labels; optional `<template>`
  fragments).
- A committed headless-browser verification harness exercising both sinks.

### Out

- **Any index/bundle change** — Slice 01 owns record content, filters, meta, the
  `/pagefind/` bundle, and its serving. **No Rust change to `search.rs`/
  `export.rs`/`server.rs`** is expected here.
- Changing `render_search`'s empty-shell contract; adding server-side `?q=`
  answering (Pagefind is client-only — the reversal accepted this).
- The default **Pagefind UI** (we use the JS API with the bespoke markup); new
  facets beyond the four; news (Arc 05); Cobalt/Liquid removal + CI wiring
  (Arc 06 — including whether the browser harness joins `make check`).

## Verification approach

**Unit (pure helpers, via the exposed global in a headless page):** `?q=` parse;
result→DOM mapping (a Pagefind result object → the right `.search-result`
fields); the filter-state reducer (toggle → filters object; within-group OR /
across-group AND); count formatting.

**End-to-end (headless Chromium, both sinks):** build the demo and run
`haleiki serve`, and separately host the built `_site/` statically. In a real
browser: (a) load `/search/?q=garbage` → assert `.search-result` items render
with correct titles/urls and a highlighted excerpt; (b) toggle a `category`
facet → assert narrowed results + updated counts; (c) a no-match query → assert
`.search-empty`; (d) **parity**: the identical query returns the identical result
URLs on the live server and the static export.

**No regression:** the Rust suite / `make check` stays green (Rust unchanged);
`search.js` is valid vanilla ES loadable with no build step.

## Exit criteria

Ledger rows H-1…H-8 reach a final status. In short: `/search/` performs live
faceted Pagefind search against the shipped bundle — `?q=` seeding, results
render into the bespoke markup, the four facets filter with live counts, empty
state — reproduced end-to-end in a headless browser against **both** sinks; the
arc composition rows are reproducible; `search.js` is lint/fmt-clean and
build-step-free.

## Relationship to the arc close

This is Arc 04's last slice. Its browser tests are the demonstration the arc
composition rows reproduce: **D-3** (both sinks host the bundle — Slice 01,
reconfirmed live here), **D-4** (end-to-end query), **D-5** (facets + live
counts), **D-6** (sink parity). After this slice CDC-closes, the arc closes:
reproduce D-3…D-6 at arc scale against a live server **and** a built export,
write `arc04-pagefind-search/closing-report.md`, and bubble up to
`project-plan.md` (**P-4**; and P-9 — client-side faceted search — gets its
implementation, staying open for project-scale reproduction in Arc 06).

## Notes

- Load the **rust-guidelines** only if a Rust harness is chosen; otherwise the
  **javascript-deno-guidelines** skill governs the JS (house style: classic,
  no-build, no-Node). Keep `search.js` free of framework deps and browser-storage
  beyond what `theme.js` already uses.
- Pagefind must be **fetched over HTTP** — both sinks satisfy this. The bundle
  path is `/pagefind/` (Pagefind default), so `import('/pagefind/pagefind.js')`
  resolves in both sinks.
- Escaping: `result.excerpt` is the one field printed `| safe` (Pagefind returns
  `<mark>`-highlighted HTML); everything else (title, url, badge text) must be
  set via `textContent`, never `innerHTML`, to preserve the auto-escape boundary
  the design-delta names as a correctness rule.
- Keep the existing Cmd-K + focus behavior from the stub.
