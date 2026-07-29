# CC Prompt — Arc 04 / Slice 02: client wiring + facets + arc composition

You are implementing the second and final slice of Arc 04 of the Haleiki
tiny_http/minijinja pivot, on branch `framework-base/tiny-http`. Closing this
slice closes the arc. Work to the collaboration-framework's ledger discipline:
every ledger row must reach a final, evidence-backed status before you claim the
slice done.

## What this slice is (and isn't)

This slice is **almost entirely client-side JavaScript** — wiring
`assets/js/search.js` to Pagefind's JS API against the `/pagefind/` bundle Slice
01 already ships and serves. Expect **little or no Rust change**. Do **not** touch
the index/bundle build, `export`, or the server's serving (Slice 01, closed) — if
you find yourself editing `search.rs`/`export.rs`/`server.rs`, stop: that's a
boundary breach.

## Before you write code

1. Load the **collaboration-framework** skill and the **javascript-deno-guidelines**
   skill (the JS is house style: classic, no-build, no-Node — see `theme.js`,
   `browse-scrollspy.js`). Load **rust-guidelines** only if you choose a Rust
   verification harness (decision #1).
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, context, the **six decisions to
     confirm at slice start**, scope, verification approach.
   - [`ledger.md`](./ledger.md) — the eight rows H-1…H-8. The contract.
   - [`../arc-plan.md`](../arc-plan.md) — the arc capability + composition rows
     D-3…D-6 this slice reproduces; the arc closes after it.
   - [`../slice01-index-and-bundle/cdc-verification.md`](../slice01-index-and-bundle/cdc-verification.md)
     — what Slice 01 delivered: the bundle (277 pages), the filters
     (`category`/`tier`/`type`/`trust`) and meta (`title`/`url`/`excerpt`) on each
     record, served at `/pagefind/` in both sinks with correct content-types.
   - `templates/pages/search.jinja` — the shell you hydrate (the exact
     `.search-result` / `.filter-*` markup your DOM must match).
   - `assets/js/search.js` — the stub you rewrite; `assets/js/theme.js` — the
     house JS style to match.

## Confirm the six decisions first (slice-doc §"Decisions to confirm")

Decision #1 (verification tooling) is **CONFIRMED (operator): Deno + Playwright** —
a Deno harness (native test runner + lint/fmt for the unit layer) driving
Playwright over a real headless Chromium against **both** sinks. **Do not add a
Node/`package.json` toolchain** (Playwright is pulled Deno-side, test-only, never
shipped). **CI wiring (`make check`) is deferred to Arc 06** — commit the harness
and reproduce it this slice; don't gate `make check` on Chromium yet. The other
five decisions still need confirming.

## What to build

- Rewrite `assets/js/search.js` (classic IIFE; dynamic `import('/pagefind/pagefind.js')`):
  init + `?q=` seeding; debounced query → `pagefind.search(q,{filters})` →
  render `.search-result` items; `.filter-sidebar` from `pagefind.filters()` with
  live counts; within-group OR / across-group AND; empty / no-results states;
  keep Cmd-K + focus. Expose pure helpers on a guarded `window.__haleikiSearch`
  for unit-level assertions in the browser harness.
- **Escaping boundary:** only `result.excerpt` is HTML (`<mark>` highlight) — set
  it via `innerHTML`; set **every** other field (title, url, badge text) via
  `textContent`. This preserves the design-delta's auto-escape correctness rule.
- Minimal `search.jinja` touch only if it keeps markup canonical (humanized
  labels; optional `<template>` fragments to clone). Prefer no structural change.
- The committed headless-browser harness exercising both sinks (query, facets,
  empty state, sink parity).

## Boundaries (do not cross in this slice)

- **No Rust change to `search.rs`/`export.rs`/`server.rs`**; no change to
  `render_search`'s empty-shell contract; no server-side `?q=` answering
  (Pagefind is client-only). Keep the Arc 03 server + Slice 01 bundle untouched.
- No default Pagefind UI; no facets beyond the four; no framework/build step for
  the shipped `search.js`; no browser storage beyond the house pattern.
- News (Arc 05); Cobalt/Liquid removal, CI wiring (Arc 06 — including whether the
  browser harness joins `make check`). Do not fix unrelated pre-existing debt.

## Definition of done

All of H-1…H-8 reach a final status with reproducible evidence. The bulk of the
proof is the **headless-browser** reproduction against **both** sinks (a live
`haleiki serve` and a served static `_site/`): query renders, facets filter with
live counts, empty state, and **sink parity** (identical query → identical result
URLs). Add unit-level assertions of the exposed pure helpers. Keep `make check`
green (Rust unchanged). In the closing report, walk the ledger row by row —
status + evidence, no silent drops, no "deviations: none" prose; record which of
the six decisions were taken; add the **bubble-up to arc-plan**: this is the last
slice, so note that D-3…D-6 are now reproducible end-to-end against both sinks,
ready for the arc close. Name any uncertainty (Pagefind API quirks; headless
flakiness). Do **not** write `cdc-verification.md` or the arc `closing-report.md`
— those are the independent reviewer's. Iteration budget: five.
