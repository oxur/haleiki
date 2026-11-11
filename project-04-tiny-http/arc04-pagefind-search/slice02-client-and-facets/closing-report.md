# Slice 02 — client wiring + facets + arc composition · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **CDC-closed** — all 8 rows `done`; independent verification is in
> [`cdc-verification.md`](./cdc-verification.md). This was originally planned as
> Arc 04's second and final slice, but CDC review found that the browser-JS
> tooling contract needed its own follow-up. Arc 04 now closes after Slice 03
> (`../slice03-browser-js-tooling-policy/`). 2026-08-21.
> Iterations used: 1 of 5 (two in-slice corrections — the filter-semantics
> spike and the missing-DOM-element bug — were caught and fixed within the
> single iteration via the headless-browser harness itself, not separate
> iterations).

## The six decisions, as taken

1. **Verification tooling — Deno + Playwright.** Already confirmed by the operator before this slice started; implemented as `tests/e2e/` (a small, self-contained Deno project: `deno.json`, `harness.js`, `search_test.js`). No `package.json`/Node anywhere. Playwright resolves via `npm:playwright@1.62.0` (Deno's own npm-specifier support) and is test-only — never referenced from `assets/js/search.js`.
2. **`search.js` stays a classic IIFE.** Taken as proposed. No `type="module"`, no static `import`/`export`, `var` throughout, matching `theme.js`/`browse-scrollspy.js`. The one dynamic `import('/pagefind/pagefind.js')` call is a runtime expression, not a module declaration — valid in a classic script.
3. **Markup source of truth — `<template>` fragments.** Taken as proposed. `search.jinja` gained two `<template>` elements (a result item, a facet option) that `search.js` clones via `content.firstElementChild.cloneNode(true)`, rather than duplicating the design team's markup as JS string literals. One necessary, minimal structural addition beyond the templates themselves: the facet-option template wraps the label text in a `<span data-field="label">` (the static Jinja version has it as a bare text node between two spans) — needed so JS can target it reliably via `querySelector`; it has zero visual/CSS effect (an inline span around existing text).
4. **Facet semantics — within-group OR, across-group AND.** Taken as proposed, but the *mechanism* required a correction: see "What was built" and H-5's ledger note — Pagefind's bare-array filter shape is AND, not OR; OR needs the explicit `{any:[...]}` form. Found by spiking the real bundle before writing the reducer, not discovered as a bug afterward.
5. **URL state — `q` only, facets not persisted.** Taken as proposed's minimum: `q` reflects into the URL via `history.replaceState` (shareable/reloadable, verified in H-6). Facet selections are **not** written to the URL — decision #5 marked this optional, and it's deliberately deferred, not a silent drop.
6. **No-JS behavior — accept JS-required search.** Taken as proposed; `render_search`'s empty shell is untouched (no Rust change). No "search needs JavaScript" note was added to the shell copy — a minor, disclosed gap: a user with JS disabled sees the same "No results found" text a failed search would show, which is technically accurate (Pagefind is JS-only, so no-JS genuinely can't search) but doesn't explain *why*. Named here as a nice-to-have for a future pass, not required by the ledger.

## Ledger walk

| ID | Status | Summary |
|----|--------|---------|
| H-1 | done | `?q=` seeds the input and runs an initial search on load, verified live and via the pure `parseQuery` unit test. |
| H-2 | done | Results render into the template-cloned markup with correct fields; exactly one `innerHTML` call in the whole file (the excerpt), everything else `textContent`. |
| H-3 | done | No-match shows `.search-empty`; clearing the query returns to a genuinely clean slate (not a "no results" message). |
| H-4 | done | All four facet groups render with humanized labels and live, non-zero counts from `pagefind.filters()`. |
| H-5 | done | Facet toggling narrows results correctly; the within-group-OR/across-group-AND mechanism required an in-slice correction (see below), verified against the real bundle. |
| H-6 | done | `q` is shareable/reloadable via `history.replaceState`; facet-in-URL persistence deliberately deferred per decision #5. |
| H-7 | done | Sink parity: identical `?q=garbage` result-URL sets from a live `haleiki serve` and a statically-hosted `haleiki export` output. |
| H-8 | done | The harness is Deno lint/fmt-clean; `search.js`'s browser validity is proven by real execution, not Deno's default lint/fmt rules; `make check` unchanged. The broader browser-JS tooling policy is routed to Slice 03 by operator decision. |

No silent drops. Every row reached `done` with attested evidence; the two deliberately-deferred items (facet-URL persistence, a no-JS explanatory note) are named, not hidden.

## What was built

- **`assets/js/search.js`** (full rewrite, ~380 lines): pure helpers (`parseQuery`, `mapResultData`, `toggleFilter`, `isFilterActive`, `formatCount`, `humanizeLabel`, `mergeFacetCounts`, `toPagefindFilters`) exposed on `window.__haleikiSearch`; DOM wiring that dynamically `import()`s Pagefind, seeds `?q=`, debounces live search (150ms), renders results/facets from cloned `<template>` fragments, and preserves the existing Cmd-K + focus behavior.
- **`templates/pages/search.jinja`**: two `<template>` fragments added (result item, facet option); no other structural change.
- **`assets/css/haleiki.css`**: one added rule, `.filter-option.active .filter-checkbox { background: var(--accent-primary); border-color: var(--accent-primary); }` — the facet checkbox had no "checked" visual at all before this (confirmed by research: no `:checked`/`.active` rule existed for it); this follows the site's own established `.active`-class convention (`.theme-toggle-option.active`, `.topnav-links a.active`) rather than inventing a new one.
- **`tests/e2e/`** (new): `deno.json`, `harness.js` (builds the binary, runs `haleiki export`, starts a live `haleiki serve`, and starts a small Deno static-file server for the export output — the two sinks), `search_test.js` (12 steps: 6 pure-helper unit checks + 6 end-to-end browser scenarios, one comprehensive `Deno.test` with shared setup/teardown).

## Deviations from plan

Two things were corrected **within** this slice, before commit, via the headless-browser harness itself — disclosed here in full rather than presented as "deviations: none":

1. **Facet filter semantics.** The slice-doc proposed `{category: ["a", "b"]}` as Pagefind's within-group-OR shape. Spiking the real bundle (`pagefind.search(query, {filters: {category: ["mathematics", "theoretical-physics"]}})`) returned **zero** results even though each category alone matched 32/34 — the opposite of OR. Checking Pagefind's own documentation confirmed: bare arrays default to AND ("all filtering defaults to 'AND' filtering... pages must match every filter provided"); OR requires the explicit `{any: [...]}` form. `toPagefindFilters` was written against the *correct* mechanism from the start of actual coding (the wrong assumption was caught during the pre-code spike, not shipped and later found broken).
2. **A DOM assumption bug, caught by the harness's first real run.** Every e2e step touching real results (query, facets, sink parity) initially timed out waiting for `.search-result`/`.filter-group` to appear. Root cause: `render_search`'s shell (Arc 02, untouched here) always server-renders with `query=""`/`results=[]`, so `search.jinja`'s `{% if %}` branches mean `.search-results-list`/`.search-status`/`.search-results-count` are **never** actually present in the initial DOM — only `.search-empty` (the else branch) is. The original `search.js` assumed all of these existed and just needed `hidden` toggled; `document.querySelector('.search-results-list')` returned `null`, and the first DOM operation on it (`resultsList.replaceChildren()`) threw synchronously inside `init()`, silently aborting the whole async chain before Pagefind even loaded. Fixed with a get-or-create pattern: the three missing elements are created and inserted at the position Jinja would have put them, if a query for them returns nothing.

## Named uncertainty

- **Pagefind's fuzzy matching is more permissive than the slice-doc anticipated.** Every plausible-looking ASCII "nonsense" query tried during test-writing (`zzzzznonexistentqueryzzzzz`, `xqzjklvwpq`, 100+ repeated letters, a 20-digit number) returned real, non-zero matches against this corpus — Pagefind's ranking appears to give partial credit for *any* character/substring overlap rather than requiring a token match. A CJK string with zero character overlap with the corpus (`日本語テスト`) reliably returns zero and is what the no-match test uses. This is worth knowing for anyone extending the search UI later: "the user typed gibberish" is not a reliable way to reach the empty state in manual testing.
- **`deno lint`/`deno fmt` and shipped browser assets need a policy decision.** CDC reproduced the feature but also reproduced that `deno lint assets/js/search.js` fails (52 findings: mainly `no-var`/`no-window`, plus `no-this-alias`/`no-inner-declarations`) and `deno fmt --check assets/js/search.js` would reformat the file. The operator decided this tooling mismatch should not block the functional search slice; it is the open problem for Slice 03 (`slice03-browser-js-tooling-policy`).
- **The startup-latency note Slice 01's CDC verification carried forward** (`serve()` builds the Pagefind index before binding, so a fresh `haleiki serve` takes several seconds — observed ~9s for the demo corpus — before the port opens) is directly visible to anyone testing this slice by hand: a manual `curl` immediately after starting the server will see a connection refused, not a slow response. Not a regression from this slice, but worth knowing when reproducing these results by hand.
- **The Deno harness's server ports (18080 live, 18081 static) are fixed, not dynamically allocated.** Fine for this slice's sequential, single-invocation use; would need revisiting if the harness is ever run in parallel with itself or wired into CI (Arc 06's call per the cc-prompt's explicit CI deferral).

## Bubble-up to arc-plan

This slice makes Arc 04's functional composition rows reproducible, but it is no
longer the last slice. Bubble-up for `arc-plan.md`:

- **D-3** (both sinks host the bundle): reconfirmed live by this slice's own harness setup (both a real `haleiki serve` and a real `haleiki export` + static host are stood up and exercised every run), on top of Slice 01's original verification.
- **D-4** (end-to-end query): reproducible now — `e2e (live): ?q=garbage renders results...` demonstrates exactly this against a live server with a headless browser.
- **D-5** (facets with live counts, query+facet composes): reproducible now — the facet-toggle e2e step demonstrates narrowing + count updates; H-5's ledger note documents the within-group-OR/across-group-AND mechanism precisely.
- **D-6** (sink parity): reproducible now — `e2e: identical query returns identical result urls on both sinks` is exactly this reproduction, committed and re-runnable.

Nothing here contradicts `arc-plan.md`'s functional scope for this slice. **D-2**
(this slice's own closure) should be marked `done` by the arc plan. Arc close is
now held for Slice 03, which will decide the browser-JS tooling policy before
the independent reviewer reproduces D-3…D-6 at arc scale and writes
`arc04-pagefind-search/closing-report.md`. Also worth flagging for the tooling
slice and Arc 06's CI-wiring decision: this harness needs a Chromium install
(`deno run -A npm:playwright@1.62.0 install chromium`, ~270MB) and takes ~25s to
run — both fine for a committed, on-demand-run artifact, but real inputs to
whatever CI story Arc 06 designs.

## Verification commands run

```
cargo build   (tools/, via the harness)
cd tests/e2e && deno test --allow-all # 12/12 steps pass
cd tests/e2e && deno lint .           # clean (no-window excluded, documented)
cd tests/e2e && deno fmt --check .    # clean
make check                            # Rust suite unchanged, all green
```

CDC follow-up, 2026-08-21: `make lint-e2e`, `make test-e2e`, and `make check`
all reproduced. Non-elevated localhost-binding runs fail in the Codex sandbox;
the same commands pass with localhost permissions.

Additionally hand-verified against the real `haleiki serve` binary: `curl` against `/search/?q=garbage` after a full startup (~9s, dominated by the Pagefind index build) returned `200` with both `<template>` fragments present in the served HTML.
