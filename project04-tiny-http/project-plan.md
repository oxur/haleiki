---
project: project04-tiny-http
status: active
depends-on: [project03-cobalt-site-build]
blocks: []
related: [project01-demo-site, project02-concept-cards]
---

# Haleiki tiny_http / minijinja Pivot — Project Plan

> The project plan-of-record: the arc roadmap for pivoting the Haleiki wiki
> off Cobalt/Liquid onto a minijinja + tiny_http stack, on branch
> `framework-base/tiny-http`. Architecture lives in
> [`design-delta.md`](./design-delta.md); this file is the plan, with a sibling ledger.
>
> Layout: `.worktrees/planning/project04-tiny-http/` on orphan branch `planning`
> (migrated 2026-11-11). Historical status: **active** · 2026-08-21.

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
- **No Cobalt or Liquid** remaining in the branch. Pagefind is intended after
  the 2026-07-29 search reversal.

**Explicitly not in scope** (boundaries):

- AI concept extraction / merging (`extract`, `merges` remain stubs).
- Authoring new domain content beyond the working demo corpus. Arc 05 may add
  minimal news/blog demo fixtures when needed to exercise the new content type.
- The canon axis UI (demo profile sets `canon.enabled: false`).
- Reshaping the concept/source frontmatter model (news is added, not a reshape).
- Any move away from read-only (no logins, edits, comments; not a CMS).

The architecture, the reuse/change/new breakdown, and the binding context
contract (source checkout `templates/README.md`; original retained in the archived
design ZIP) are in
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
| **05 — News / blog content type** | A new content kind: news/blog frontmatter, parser, routing, and context for `news-index`/`news-post`/`news-archive`/`news-tags`/`news-categories`, incl. prev/next/related and date-grouped archives. Renders in both sinks. | Arc 01, Arc 02, Arc 03 |
| **06 — Polish, parity & deployment readiness** | Trust bar/glyph, theme-toggle and a11y parity with the review; remove all Cobalt/Liquid remnants; update `CLAUDE.md`/`README`/`Makefile`; deploy guides (static host + self-hosted server); CI. | all prior |

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
- **Arc 05 — News / blog content type:** **active** (see
  [`arc05-news-blog-content-type/arc-plan.md`](./arc05-news-blog-content-type/arc-plan.md)).
  Slice 01 is open to settle the news/blog content model and render one post
  before the broader index/archive/taxonomy surfaces are implemented.
- **Arc 06:** roadmap-level only, by design. It is planned deep when it becomes
  the active work; findings from earlier arcs bubble up and may reshape it
  before then.

## 4. Project ledger


See the sibling [ledger.md](./ledger.md). All original row IDs, criteria,
statuses and evidence are retained there.

## 5. Version history

### v1.9 — 2026-11-11 (Arc 05 execution authority)

Arc 05 Slice 01 preparation reconciled the previous migration-only resume
wording with the Operator's separate content-model and Autonomous approvals.
The roadmap and all project acceptance rows are unchanged; execution stops
at Arc 05 or earlier unresolved discussion gates. Current source-grounded
assignment and actual contributor attribution live in the arc/slice plans.

### Directory-name correction — 2026-11-11

The Operator renamed the project to `project04-tiny-http`. Current links,
project relationships, manifest destinations and the verifier were repointed;
archived payloads, issued prompts and ledger rows remain unchanged.


### Migration — 2026-11-11

Moved from `docs/tiny-http/` to orphan branch `planning`, worktree
`.worktrees/planning/project04-tiny-http/`. The embedded ledger now lives in sibling
`ledger.md`; row text and acceptance states are unchanged. See the
[byte-preserved August plan](../archive/docs/tiny-http/project-plan.md). Existing issued prompts, ledgers, close reports,
verification records and `slice-doc.md` documents retain their bytes.
Canonical `slice-plan.md` entrypoints link the original slice documents.
Historical repository-relative paths resolve via the root migration manifest;
source paths such as `tools/`, `templates/` and `demo/` refer to the implementation
checkout, not this planning-only worktree. No feature verification was rerun.

### v1.8 — 2026-08-21 (Arc 05 planned deep)
Arc 05 opened as `arc05-news-blog-content-type/`. Operator decision recorded:
News is Haleiki's blog — dated wiki-maintainer updates and related news items,
not a general CMS/editorial workflow. Slice 01 is opened for the load-bearing
news model and one rendered post, with explicit discussion gates for authoring
location, URL model, frontmatter shape, and demo/test fixtures. The stale
top-level "no Pagefind" wording was corrected; Pagefind is now intended after
the Arc 04 reversal, while Cobalt/Liquid removal remains Arc 06.

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

## Workflow, paths and resume gate

The existing CDC + CC workflow and `cdc-verification.md` records are retained.
The migration authorization alone did not start Arc 05 code. Separately, the
Operator approved Autonomous Mode through the end of Arc 05 on 2026-11-11;
see the arc plan authorization and canonical Slice 01 decision log/current
assignment. No external action or Arc 06 execution is authorized.
The directory spelling `project04-tiny-http` follows the canonical
`projectNN-<slug>` convention after the Operator corrected its name on 2026-11-11.
Arcs 01–04 remain historically closed. Arc 05 Slice 01 remains open for
implementation/independent acceptance; its authoring location, URL model,
frontmatter and fixture decisions are now approved. Later archive, taxonomy,
feed and search discussion gates remain open. Arc 06 remains roadmap-level,
with no invented directory/open set.
Future assignments must use current paths and preserve the original issued prompt.
Durable slice outputs default to that slice's `artifacts/` directory.

The original design ZIP is retained at
`../archive/workbench/Hale'iki Wiki Design Refactor.zip`; it contains the original
`templates/` tree absent from the unpacked workbench directory. Current shipped
templates are in the implementation checkout's `templates/`.
