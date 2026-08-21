# Haleiki tiny_http / minijinja Pivot — Project Plan

> The project plan-of-record: the arc roadmap for pivoting the Haleiki wiki
> off Cobalt/Liquid/Pagefind onto a minijinja + tiny_http stack, on branch
> `framework-base/tiny-http`. Architecture lives in
> [`design-delta.md`](./design-delta.md); this file is the plan and the ledger.
>
> Layout: framework arc/slice tree, no version directory (operator override,
> per `docs/PROJECT-MANAGEMENT.md` Part VI). Status: **active** · 2026-08-21.

---

## 1. Definition of done and boundaries

**Done when** the `framework-base/tiny-http` branch renders and serves the
Haleiki demo wiki **and** news site from the existing Markdown corpus, entirely
through a Rust-native minijinja + tiny_http stack, with:

- **Static export** as the default deliverable: `haleiki build` renders the full
  site to `_site/` (static HTML/CSS/JS) deployable to any static host.
- **A servable process**: `haleiki serve` runs a hardened `tiny_http` server
  rendering the same site, usable as an optional self-hosted deployment.
- **Client-side search** via **Pagefind** (index built in-process over the
  rendered site), faceted by category / tier / page-type / trust.
- **The news/blog surface** rendering from real posts, alongside the wiki.
- **Visual and functional parity** with Claude Design's design review, using the
  refactor's `templates/` set, the unified `haleiki.css`, and the Red Hat
  Display / Newsreader / IBM Plex Mono fonts.
- **No Cobalt, Liquid, or Pagefind** remaining in the branch.

**Explicitly not in scope** (boundaries):

- AI concept extraction / merging (`extract`, `merges` remain stubs).
- Authoring new content — the existing demo corpus is the working set.
- The canon axis UI (demo profile sets `canon.enabled: false`).
- Reshaping the concept/source frontmatter model (news is added, not a reshape).
- Any move away from read-only (no logins, edits, comments; not a CMS).

The architecture, the reuse/change/new breakdown, and the binding context
contract (`workbench/Hale'iki Wiki Design Refactor/templates/README.md`) are in
[`design-delta.md`](./design-delta.md).

## 2. The arc roadmap

Six arcs in dependency order. Each delivers one coherent capability. Detailed
arc-plans are written **late** — only Arc 01 is planned deep here; the rest are
roadmap-level until they become the active work (per *plan late, plan deep*).

| Arc | Capability | Depends on |
|-----|------------|------------|
| **01 — Render core** | Render a fully-chromed wiki article (concept **and** source) to correct HTML from the real demo corpus, via minijinja over the refactor templates + unified CSS. Replaces the Liquid renderer for the article path. | existing `parser`/`graph`/`validator` |
| **02 — Site routes & static export** | The full wiki route set (home, browse, category, about, attribution, 404, search shell), with grouping resolved in Rust, plus the static-export writer (`_site/` + asset copy). | Arc 01 |
| **03 — tiny_http server** | `haleiki serve`: routing, static-asset serving, configurable bind/port, structured errors + logging, hardened for optional self-host. Same render output as export. | Arc 01 (composes with Arc 02 routes) |
| **04 — Search (Pagefind)** | Pagefind index built in-process over the rendered site + `search.jinja` wired to `search.js` via Pagefind's JS API, faceted by category / tier / page-type / trust. Both sinks host the `/pagefind/` bundle. | Arc 02, Arc 01 |
| **05 — News / blog content type** | A new content kind: news frontmatter, parser, routing, and context for `news-index`/`news-post`/`news-archive`/`news-tags`/`news-categories`, incl. prev/next/related and date-grouped archives. Renders in both sinks. | Arc 01, Arc 02, Arc 03 |
| **06 — Polish, parity & deployment readiness** | Trust bar/glyph, theme-toggle and a11y parity with the review; remove all Cobalt/Liquid/Pagefind remnants; update `CLAUDE.md`/`README`/`Makefile`; deploy guides (static host + self-hosted server); CI. | all prior |

**Sequencing rationale.** Arc 01 is the load-bearing render spine — nothing
paints without it. Arcs 02 (static sink) and 03 (server sink) are the two
outputs of that one renderer and can be built in either order after 01; 02 comes
first because static export is the default deliverable and gives every later arc
a concrete artifact to diff against the mockups. Search (04) needs the routes and
export in place. News (05) is a whole content type and rides on the render +
export + serve machinery, so it lands after them. Polish/parity/deploy (06) is
the recomposition arc that closes the branch as a clean Cobalt replacement.

## 3. Current status

- **Arc 01 — Render core:** **CLOSED** 2026-07-28 (see
  [`arc01-render-core/closing-report.md`](./arc01-render-core/closing-report.md)).
  All three slices CDC-closed; composition rows A-4…A-7 reproduced at arc scale.
- **Arc 02 — Routes & static export:** **CLOSED** 2026-07-28 (see
  [`arc02-routes-static-export/closing-report.md`](./arc02-routes-static-export/closing-report.md)).
  All four slices CDC-closed; a real 293-file `_site/` export reproduces B-5…B-8.
- **Arc 03 — tiny_http server:** **CLOSED** 2026-07-29 (see
  [`arc03-tiny-http-server/closing-report.md`](./arc03-tiny-http-server/closing-report.md)).
  Both slices CDC-closed; composition rows C-1…C-5 reproduced at arc scale against
  a live hardened `haleiki serve` (route sweep, HEAD/405/nosniff, panic isolation,
  access log, export parity).
- **Arc 04 — Search (Pagefind):** **CLOSED** 2026-08-21 (see
  [`arc04-pagefind-search/closing-report.md`](./arc04-pagefind-search/closing-report.md)).
  All three slices CDC-closed; composition rows D-3...D-6 reproduced at arc
  scale against a fresh export, a live `haleiki serve`, and a real browser.
  **Note:** the operator reversed pivot decision #3
  (custom JSON index → Pagefind) on 2026-07-29 — see v1.4 below.
- **Arcs 05–06:** roadmap-level only, by design. Each is planned deep when it
  becomes the active work; findings from earlier arcs bubble up and may reshape
  the later entries before they are planned.

## 4. Project ledger

Opens here; closes (per-row walk) in the project's `closing-report.md` at project
close. Rows are the definition-of-done composition criteria (class **b**) and the
arcs-closed roll-up (class **a**), per the collaboration-framework's
`LEDGER-DISCIPLINE.md` Section C. Composition rows are **reproduced at project
scale** (an end-to-end demonstration), never inherited from arc attestations.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| P-1 | Arc 01 (render core) closed + composed | ptr: `arc01-render-core/closing-report.md` | correctness | project-plan | done | reproduced at arc scale — Arc 01 closed 2026-07-28; A-4…A-7 green in clean build |
| P-2 | Arc 02 (routes & static export) closed + composed | ptr: `arc02-routes-static-export/closing-report.md` | correctness | project-plan | done | reproduced at arc scale — Arc 02 closed 2026-07-28; real 293-file `_site/` export, B-5…B-8 green |
| P-3 | Arc 03 (tiny_http server) closed + composed | ptr: `arc03-tiny-http-server/closing-report.md` | correctness | project-plan | done | reproduced at arc scale — Arc 03 closed 2026-07-29; C-1…C-5 green against a live hardened `haleiki serve` (route sweep, HEAD/405/nosniff, panic isolation, export parity) |
| P-4 | Arc 04 (search / Pagefind) closed + composed | ptr: `arc04-pagefind-search/closing-report.md` | correctness | project-plan | done | reproduced at arc scale — Arc 04 closed 2026-08-21; D-3...D-6 green against a fresh export, live `haleiki serve`, and Chromium (`garbage` query, live facets, static/live parity) |
| P-5 | Arc 05 (news content type) closed + composed | ptr: arc05 closing-report | correctness | project-plan | open | |
| P-6 | Arc 06 (polish/parity/deploy) closed + composed | ptr: arc06 closing-report | correctness | project-plan | open | |
| P-7 | `haleiki build` emits a static `_site/` that renders the demo **wiki** with parity to the design review | reproduce: build, open in browser, diff vs review mockups | serious | DoD | open | |
| P-8 | `haleiki serve` serves the same site over tiny_http with configurable bind/port, structured errors + logging | reproduce: serve, curl routes + assets, kill/restart | serious | DoD | open | implemented in Arc 03 (`haleiki serve`: bind/port, HEAD/405, nosniff, access log, panic isolation — audited live); stays open pending project-scale reproduction in Arc 06 |
| P-9 | Client-side search returns faceted results via Pagefind | reproduce: query in browser, exercise each facet | serious | DoD | open | implemented in Arc 04 and reproduced at arc scale (`garbage` query returned 14 highlighted metadata-bearing results; live Category/Tier/Type/Trust counts rendered; query+category composition narrowed/restored results; static/live URL parity held). Stays open pending project-scale recomposition in Arc 06/final close rather than inheriting arc evidence into a DoD row. |
| P-10 | The **news** section renders end-to-end in both static export and server | reproduce: build + serve, walk index/post/archive/tags/categories | serious | DoD | open | |
| P-11 | No Cobalt / Liquid references remain on the branch (Pagefind is now intended — decision #3 reversed, v1.4) | `grep -ri -e cobalt -e liquid` returns only historical/doc mentions | correctness | DoD | open | |
| P-12 | Visual + functional parity with the design review accepted by the operator | operator acceptance judgment, recorded | serious | DoD | open | |

## 5. Version history

### v1.7 — 2026-08-21 (bubble-up from Arc 04 close)
Arc 04 (Search/Pagefind) closed and composed
(`arc04-pagefind-search/closing-report.md`): all three slices CDC-closed, and
D-3...D-6 reproduced at arc scale against a fresh export, a live `haleiki serve`,
and Chromium. The close verified the `/pagefind/` bundle in both sinks
(JS/WASM/JSON/index/fragment file kinds and content types), a live `garbage`
query with 14 metadata-bearing highlighted results, live facet counts and
query+category composition, and exact static/live result URL parity. Project
ledger **P-4** marked done. **P-9** is annotated with this implementation
evidence but remains open for project-scale recomposition in Arc 06/final close;
the roadmap is otherwise unchanged. Browser/e2e CI/default-gate wiring remains an
Arc 06 decision, with `make check-browser-js`, `make lint-e2e`, and
`make test-e2e` now available as named gates.

### v1.6 — 2026-08-21 (Arc 04 Slice 03 CDC close)
Arc 04 Slice 03 (`browser-js-tooling-policy`) CDC-closed: shipped browser JS is
now governed by `make check-browser-js`; Deno lint/fmt remains scoped to the
`tests/e2e/` harness; no Node/npm project infrastructure was introduced.
Project ledger P-4/P-9 remain open until Arc 04 is composed and closed at arc
scale.

### v1.5 — 2026-08-21 (Arc 04 functional search closed; tooling slice opened)
Arc 04 Slice 02 CDC-closed: client-side Pagefind search now reproduces in a
real browser against both sinks via `tests/e2e/` (`make test-e2e`), and
`make check` remains green. CDC review found a tooling-contract mismatch rather
than a product defect: Deno's default lint/fmt rules do not currently reproduce
cleanly on the shipped classic browser asset `assets/js/search.js`. Operator
decision: keep Slice 02 closed on functional evidence and open Arc 04 Slice 03
(`browser-js-tooling-policy`) to define the canonical browser-JS quality gate.
Project ledger P-4/P-9 remain open until Arc 04 is composed and closed.

### v1.4 — 2026-07-29 (operator decision reversal: custom index → Pagefind)
Planning Arc 04 surfaced that the design team's `search.js` ships as a documented
17-line **stub** (an integration point, exactly as the design-delta's "custom
search is *new* work" framing implied — not a shortfall). On learning this, the
operator **reversed pivot decision #3**: the branch will use **Pagefind** after
all, rather than a hand-built custom JSON index. Recorded consequences, accepted
by the operator: (a) search is **client-only** (Pagefind's query engine is a
browser WASM client — no server-side `?q=`), so it behaves identically in both
sinks and the server's only added duty is to serve the bundle; (b) the index is
Pagefind's **own `/pagefind/` bundle** (a build step over the rendered site + a
served directory), not a route-map JSON. Doc changes: DoD search bullet →
Pagefind; roadmap Arc 04 → "Search (Pagefind)"; **P-9** reworded (via Pagefind);
**P-11** now targets **Cobalt/Liquid only** (Pagefind is intended, not a remnant
to remove). Design record: `design-delta.md` Addendum A. Arc 04 planned deep as
`arc04-pagefind-search/` (index+bundle → client+facets+composition); slice 01
open set written. This is a valid, recorded reversal, not a silent drift — the
founding decision is superseded with its rationale on the record.

### v1.3 — 2026-07-29 (bubble-up from Arc 03 close)
Arc 03 (tiny_http server) closed and composed
(`arc03-tiny-http-server/closing-report.md`): both slices CDC-closed (slice 01
route map + core server 2026-07-28; slice 02 hardening 2026-07-29), and C-1…C-5
reproduced at arc scale against a live hardened `haleiki serve` over the demo —
every route type + asset `200`, unknown `404`, `HEAD` `200`-empty with a true
`Content-Length`, `POST` `405`+`Allow`, `nosniff`, per-request panic isolation,
stderr access logging, and server↔export byte parity. Project ledger **P-3**
marked done; **P-8** ("a servable process") now has its implementation and is
annotated, staying open until reproduced at project scale in Arc 06. Roadmap
unchanged. Forward findings for later arcs: (a) **graceful shutdown / signal
handling** was deferred by decision this arc — the first hardening gap for Arc
06's deploy-readiness review; (b) the CLI reconciliation note stands — `serve`
landed additively alongside `build`/`export`, and `build` still writes legacy
`_data` JSON (Arc 06). Status advanced: Arc 04 (custom search) is next to plan
deep.

### v1.2 — 2026-07-28 (bubble-up from Arc 02 close)
Arc 02 (routes & static export) closed and composed
(`arc02-routes-static-export/closing-report.md`): all four slices CDC-closed, and
B-5…B-8 reproduced at arc scale by running the real `haleiki export` — a 293-file
`_site/` (6 singletons + 10 categories + 91 sources + 186 concepts) with assets
copied and links resolving. Project ledger **P-2** marked done. Roadmap unchanged.
Forward findings for later arcs: (a) **Arc 06** should reconcile the CLI — the
static export landed as an additive `export` subcommand while `build` still writes
the legacy `_data` JSON; (b) a project-level note on the `&#x2f;` href-escaping
gotcha was added to `CLAUDE.md`. Status advanced: Arc 03 (tiny_http server) is next.

### v1.1 — 2026-07-28 (bubble-up from Arc 01 close)
Arc 01 (render core) closed and composed (`arc01-render-core/closing-report.md`):
all three slices CDC-closed, composition rows A-4…A-7 reproduced at arc scale in a
clean independent build. Project ledger **P-1** marked done. The roadmap is
unchanged — Arc 01 delivered its capability as defined, so no restructure is
forced (a valid, recorded bubble-up outcome). Two forward-looking findings for
later arcs: (a) the CI lint gate should run `clippy --all-targets --all-features
-D warnings` (Arc 06), with the feature-gated `demo/*` debt as its backlog;
(b) source-page provenance display is an Arc 02 template decision. Status
advanced: Arc 02 is next to plan deep.

### v1.0 — 2026-07-28
Initial roadmap for the tiny_http/minijinja pivot. Six arcs (render core →
routes & static export → tiny_http server → custom search → news content type →
polish/parity/deploy), the definition of done and boundaries reflecting the four
operator decisions (both-deployment, news in scope, custom search, arc/slice
layout), and the project ledger opened with the DoD composition rows. Arc 01
planned deep; Arcs 02–06 roadmap-level pending *plan late, plan deep*.
