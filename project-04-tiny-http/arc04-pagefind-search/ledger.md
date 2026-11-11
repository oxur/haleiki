# Arc ledger

> Extracted without row changes on 2026-11-11 from the August plan. Existing dispositions/evidence are retained, not freshly reproduced by the migration.

Opens here; closes (per-row walk) in `closing-report.md` at arc close. Class (a)
= slices closed; class (b) = composition, **reproduced at arc scale** (build the
bundle in both sinks and run a real query, browser-driven); class (c) = bubble-up
findings dispositioned.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| D-1 | Slice 01 (index build + bundle in both sinks) closed | ptr: `slice01-index-and-bundle/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 01 CDC-closed 2026-07-29, 8/8 rows; bundle built in-process (277 pages), served live with correct content-types + traversal guard |
| D-2 | Slice 02 (client wiring + facets + composition) closed | ptr: `slice02-client-and-facets/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 02 CDC-closed 2026-08-21, 8/8 rows; functional browser search/facets/sink parity reproduced, browser-JS tooling policy routed to slice 03 |
| D-3 | Both sinks host the bundle: a built `_site/` contains `_site/pagefind/` and a live `haleiki serve` serves `/pagefind/pagefind.js`, the `.wasm`, and the index/fragment files `200` with correct content-types | reproduce at arc scale: build the export + start the server; fetch each bundle file kind | serious | arc-plan | done | reproduced at arc close — fresh export produced `_site/pagefind/` with 336 files; static and live sinks returned `200` for `pagefind.js` (`text/javascript`), `wasm.en.pagefind` (`application/wasm`), `pagefind-entry.json` (`application/json`), `index/en_13a9ba1.pf_index` and `fragment/en_1174e95.pf_fragment` (`application/octet-stream`). |
| D-4 | End-to-end query: in a real browser against a live server, a known term (e.g. "garbage") returns the expected result set with correct title/url/type/category/tier and a highlighted excerpt | reproduce: headless browser drives `/search/?q=garbage`, assert rendered `.search-result` items | serious | arc-plan | done | reproduced at arc close — Chromium against live `/search/?q=garbage` rendered 14 results; first result `Garbage Collection` at `/garbage-collection/`, type `Concept`, category `programming-languages`, tier `foundational`, with Pagefind `<mark>` highlights. |
| D-5 | Facets work with live counts: selecting a category/tier/page-type/trust facet narrows the results and the counts reflect `pagefind.filters()`; combining a query with a facet composes (AND) | reproduce: headless browser toggles a facet, asserts narrowed results + updated counts | serious | arc-plan | done | reproduced at arc close — live facet groups `Category`, `Tier`, `Type`, `Trust` populated with query-specific counts for `garbage`; selecting `Mathematics` narrowed to 0 and showed `.search-empty`, selecting `Programming Languages` restored 14 programming-language results. |
| D-6 | Sink parity: the same query returns the same result set against the static export and the tiny_http server (same bundle, client-side engine) | reproduce: run the identical browser query against both sinks, diff the result URLs | correctness | arc-plan | done | reproduced at arc close — identical browser query `garbage` against static and live sinks returned the same sorted 14-result URL set (see `closing-report.md`). |
| D-7 | Slice 03 (browser JS tooling policy) closed | ptr: `slice03-browser-js-tooling-policy/cdc-verification.md` | correctness | CDC review | done | reproduced — slice 03 CDC-closed 2026-08-21, 7/7 rows; browser-JS policy settled as `make check-browser-js`, Deno lint/fmt scoped to `tests/e2e/`, no Node/npm workflow introduced |

_(Class-(c) bubble-up rows are added as slices surface findings — e.g. the
in-process-crate vs. binary indexing decision, or any content-type gaps.)_
