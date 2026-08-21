# Arc 04 — Search (Pagefind)

> The arc plan-of-record: client-side faceted search over the rendered site,
> via **Pagefind**. Part of the tiny_http pivot
> ([`../project-plan.md`](../project-plan.md)); architecture + the reversed
> search decision in [`../design-delta.md`](../design-delta.md) (§ Addendum A,
> 2026-07-29). Status: **active** · 2026-08-21.

---

## 0. Direction note (read first)

The pivot's original decision #3 was a *custom* Rust-built JSON index (no
Pagefind). On 2026-07-29 the operator **reversed that decision** on learning the
design team's `search.js` ships as a documented stub (an integration point, not
a shortfall): the branch will use **Pagefind** after all. The reversal is
recorded in `../design-delta.md` (Addendum A) and `../project-plan.md` (DoD, P-9,
P-11, v1.4). Two downstream consequences the operator accepted when choosing
Pagefind:

- **Search is client-only.** Pagefind's query engine is a browser WASM client;
  there is no server-side `?q=` answer path. Search therefore behaves
  **identically in both sinks** (the earlier "server-side `?q=` in tiny_http"
  idea is dropped). The server sink's only added duty is to *serve the bundle*.
- **The index is Pagefind's own bundle, not a route-map JSON.** It is not folded
  into `render_routes` as a page; instead it is a `/pagefind/` directory
  (WASM + JS + binary index/fragment files) built from the rendered site. The
  "fold into the build" intent is honored differently: `haleiki export` emits it
  into `_site/pagefind/` as a build step, and `haleiki serve` serves `/pagefind/*`
  from disk.

## 1. Capability

The demo wiki gains **working client-side search with facets**. `haleiki` builds
a **Pagefind** index over the rendered corpus (concepts + sources), emits the
`/pagefind/` bundle, and both sinks host it (static export writes it; the
tiny_http server serves it). The design team's bespoke `search.jinja` shell is
wired — via `search.js` — to Pagefind's **JavaScript API** (not the default
Pagefind UI): a query renders results into `.search-results-list`, and the
`.filter-sidebar` drives facets over **category / tier / page-type / trust** with
live result counts. This replaces Arc 02's empty pre-hydration shell (`query=""`,
`results=[]`, `facets=[]`) with real search, and turns the Pagefind-stub `search`
subcommand into a real indexer.

The unifying idea stays the pivot's "one renderer, two sinks": the index is built
from the *same* rendered HTML both sinks emit, so search over the static export
and search over the running server are the same search.

## 2. Slice breakdown

Three slices. Slice 01 is the load-bearing core (the index exists and both sinks
host it); slice 02 is the client experience + functional arc-scale composition;
slice 03 closes the browser-JS tooling-policy gap surfaced by CDC review before
the arc itself closes.

| Slice | Scope | Load-bearing for |
|-------|-------|------------------|
| **01 — index build + bundle in both sinks** | Build the Pagefind index in-process from the corpus (records carry searchable content + the four filters + result meta: title/url/type/category/tier/trust/excerpt), emitting the `/pagefind/` bundle. `haleiki export` writes it under `_site/pagefind/`; `haleiki serve` serves `/pagefind/*` from disk with correct content-types (`.wasm`→`application/wasm`, `.js`, `.json`, binary index/fragment→`application/octet-stream`). Replace the Pagefind-wrapper `search` stub with a real in-process build. **No client UI yet.** | 02, arc close |
| **02 — client wiring + facets + arc composition** | Wire `search.js` to the Pagefind JS API: live query → results rendered into `search.jinja`'s `.search-results-list`; the `?q=` URL param seeds the query on load; empty / no-results states. The `.filter-sidebar` facets (category/tier/page-type/trust) drive `pagefind.search(q,{filters})` using `pagefind.filters()` counts, updated live as the query narrows. Functional composition: a **headless-browser** query end-to-end against both a live `haleiki serve` and a built static export. | arc close |
| **03 — browser JS tooling policy** | Define and implement the canonical quality gate for shipped browser JS (`assets/js/*.js`), separating browser-asset policy from the Deno/Playwright harness policy. Keep Node out, keep browser/e2e default-CI wiring deferred to Arc 06, and confirm Slice 02's browser evidence still reproduces. | arc close |

**Sequencing rationale.** The index + bundle-hosting is the substrate; nothing
searches without it, and it is fully verifiable in Rust (build the bundle, assert
its contents; serve `/pagefind/*` and assert content-types) with no browser.
Slice 02 layers the browser experience on top and is where a headless-browser
reproduction earns its keep.

## 3. Dependencies

**Consumes** (from Arcs 01–03, closed): the render pipeline and `render_routes`
(the rendered per-page HTML that gets indexed), `export::export_site` (writes
`_site/`, now also invokes the index build), and the Arc 03 asset server
(`resolve_asset` + the allowlist — extended to serve `/pagefind/*`). **Adds**:
the **`pagefind` crate** (in-process indexer — the `api`/`options`/`runner`
modules) and the vendored Pagefind browser bundle it emits.

**Leaves for later arcs**: news (Arc 05); the CLI reconciliation and Cobalt/Liquid
removal + CI/deploy (Arc 06). Note the founding "no Pagefind" boundary is
**withdrawn** for this arc by the operator's reversal — Pagefind is now intended,
so Arc 06's remnant-removal (P-11) targets **Cobalt/Liquid only**.

## 4. Arc ledger

Opens here; closes (per-row walk) in `closing-report.md` at arc close. Class (a)
= slices closed; class (b) = composition, **reproduced at arc scale** (build the
bundle in both sinks and run a real query, browser-driven); class (c) = bubble-up
findings dispositioned.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| D-1 | Slice 01 (index build + bundle in both sinks) closed | ptr: `slice01-index-and-bundle/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 01 CDC-closed 2026-07-29, 8/8 rows; bundle built in-process (277 pages), served live with correct content-types + traversal guard |
| D-2 | Slice 02 (client wiring + facets + composition) closed | ptr: `slice02-client-and-facets/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 02 CDC-closed 2026-08-21, 8/8 rows; functional browser search/facets/sink parity reproduced, browser-JS tooling policy routed to slice 03 |
| D-3 | Both sinks host the bundle: a built `_site/` contains `_site/pagefind/` and a live `haleiki serve` serves `/pagefind/pagefind.js`, the `.wasm`, and the index/fragment files `200` with correct content-types | reproduce at arc scale: build the export + start the server; fetch each bundle file kind | serious | arc-plan | open | |
| D-4 | End-to-end query: in a real browser against a live server, a known term (e.g. "garbage") returns the expected result set with correct title/url/type/category/tier and a highlighted excerpt | reproduce: headless browser drives `/search/?q=garbage`, assert rendered `.search-result` items | serious | arc-plan | open | |
| D-5 | Facets work with live counts: selecting a category/tier/page-type/trust facet narrows the results and the counts reflect `pagefind.filters()`; combining a query with a facet composes (AND) | reproduce: headless browser toggles a facet, asserts narrowed results + updated counts | serious | arc-plan | open | |
| D-6 | Sink parity: the same query returns the same result set against the static export and the tiny_http server (same bundle, client-side engine) | reproduce: run the identical browser query against both sinks, diff the result URLs | correctness | arc-plan | open | |
| D-7 | Slice 03 (browser JS tooling policy) closed | ptr: `slice03-browser-js-tooling-policy/cdc-verification.md` | correctness | CDC review | done | reproduced — slice 03 CDC-closed 2026-08-21, 7/7 rows; browser-JS policy settled as `make check-browser-js`, Deno lint/fmt scoped to `tests/e2e/`, no Node/npm workflow introduced |

_(Class-(c) bubble-up rows are added as slices surface findings — e.g. the
in-process-crate vs. binary indexing decision, or any content-type gaps.)_

## 5. Version history

### v1.2 — 2026-08-21 (Slice 03 CDC close)
Slice 03 (browser JS tooling policy) CDC-closed against commits `367799e`,
`384c046`, and `e194a0a`: `make check-browser-js` reproduced on all tracked
`assets/js/*.js` files, `make lint-e2e` kept the Deno/Playwright harness clean,
`make test-e2e` reproduced Slice 02's browser evidence with localhost
permissions, and `make check` passed with localhost permissions. No Node/npm
project infrastructure was introduced. Arc ledger D-7 marked done; D-3...D-6
remain open for arc-scale close.

### v1.1 — 2026-08-21 (Slice 02 CDC close + Slice 03 opened)
Slice 02 (client wiring + facets + composition) CDC-closed against commit
`ae82a12`: the Deno/Playwright harness under `tests/e2e/` reproduced query
seeding, result rendering, empty/clear states, facets with live counts, URL
state, and live/static sink parity; `make lint-e2e`, `make test-e2e`, and
`make check` all pass with localhost permissions where needed. The review found
one real plan defect: the original H-8 wording treated Deno's default lint/fmt
rules as the authority for shipped browser assets, but `assets/js/search.js` is a
classic browser script and direct `deno lint`/`deno fmt --check` does not
reproduce cleanly. Operator decision: close Slice 02 functionally and open
Slice 03 (`slice03-browser-js-tooling-policy/`) to define the canonical
browser-JS quality gate before Arc 04 closes. Arc ledger D-2 marked done; D-7
added.

### v1.0 — 2026-07-29
Initial arc plan, authored **after the operator reversed pivot decision #3**
(custom JSON index → Pagefind; see `../design-delta.md` Addendum A). Capability
(client-side faceted search via Pagefind; index built in-process over the
rendered corpus; both sinks host the `/pagefind/` bundle; the bespoke
`search.jinja` wired to Pagefind's JS API for query + category/tier/page-type/
trust facets), two-slice breakdown (index+bundle → client+facets+composition),
dependencies on the Arc 01–03 pipeline + asset server + the `pagefind` crate, and
the arc ledger opened with the composition rows D-3…D-6. Recorded consequences of
the reversal: search is client-only (no server `?q=`; identical in both sinks),
and the index is Pagefind's bundle (a build step + a served directory, not a
route-map page). Slice 01 planned deep; slice 02 roadmap-level.
