# Slice 02 — client wiring + facets + arc composition · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-29.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| H-1 | On load, `search.js` initializes Pagefind from `/pagefind/pagefind.js`; a `?q=<term>` in the URL seeds `.search-hero-input` and runs an initial search | headless-browser e2e loads `/search/?q=garbage`, asserts the input value + that results appear; unit test of the query-string parser via the exposed global | serious | slice-doc | open | | |
| H-2 | A query renders `.search-result` items into `.search-results-list` with the template's fields: type/category/tier badges, `.result-title > a[href=result.url]`, a `<mark>`-highlighted `.result-excerpt`, `.result-url`; non-`excerpt` fields set via `textContent` (auto-escape boundary preserved) | e2e asserts rendered items for a known term; unit test of the result→DOM mapper; grep confirms no `innerHTML` for title/url/badges | serious | slice-doc | open | | |
| H-3 | A no-match query shows `.search-empty` (not a stale list); the `.search-results-count` reflects the count; an emptied query returns to the initial state | e2e: a nonsense query → `.search-empty` visible, `.search-results-list` absent/empty | serious | slice-doc | open | | |
| H-4 | `.filter-sidebar` is populated from `pagefind.filters()` with the four groups (category / tier / page-type / trust), each option showing a `.filter-count`; slug values are humanized labels | e2e asserts the four groups render with non-zero counts against the demo corpus | serious | slice-doc | open | | |
| H-5 | Toggling a facet re-runs the search with filters — within-group **OR**, across-group **AND** — narrowing the result set; counts update live from the search response | e2e toggles a `category` facet, asserts a narrowed, correct result set + updated counts; unit test of the filter-state reducer (toggle → filters object, OR/AND composition) | serious | slice-doc | open | | |
| H-6 | `q` is reflected in the URL so a search is shareable/reloadable (reloading `/search/?q=…` reproduces the results); facet-in-URL persistence is noted (decision #5) as done or deferred | e2e: run a query, assert the URL carries `q`; reload, assert the same results | polish | slice-doc | open | | |
| H-7 | **Sink parity (D-6):** the identical query returns the identical result URL set against a live `haleiki serve` **and** a served static `_site/` (same bundle, client engine) | e2e runs the same query against both sinks, diffs the result URLs (equal) | serious | slice-doc (D-6) | open | | |
| H-8 | `search.js` is lint/format-clean per the chosen JS toolchain and is valid vanilla ES loadable **with no build step**; the Rust suite / `make check` stays green (no Rust regression); no framework dependency added to the shipped asset | run the JS lint/fmt; `make check`; grep the shipped `search.js` for a build-step/import-map requirement (none) | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 8. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
