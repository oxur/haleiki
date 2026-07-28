# Haleiki tiny_http / minijinja Pivot — Project Plan

> The project plan-of-record: the arc roadmap for pivoting the Haleiki wiki
> off Cobalt/Liquid/Pagefind onto a minijinja + tiny_http stack, on branch
> `framework-base/tiny-http`. Architecture lives in
> [`design-delta.md`](./design-delta.md); this file is the plan and the ledger.
>
> Layout: framework arc/slice tree, no version directory (operator override,
> per `docs/PROJECT-MANAGEMENT.md` Part VI). Status: **active** · 2026-07-28.

---

## 1. Definition of done and boundaries

**Done when** the `framework-base/tiny-http` branch renders and serves the
Haleiki demo wiki **and** news site from the existing Markdown corpus, entirely
through a Rust-native minijinja + tiny_http stack, with:

- **Static export** as the default deliverable: `haleiki build` renders the full
  site to `_site/` (static HTML/CSS/JS) deployable to any static host.
- **A servable process**: `haleiki serve` runs a hardened `tiny_http` server
  rendering the same site, usable as an optional self-hosted deployment.
- **Client-side search** over a Rust-built JSON index (no Pagefind).
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
| **04 — Custom search** | Rust-built JSON search index + `search.jinja` wired to `search.js`, faceted by category / tier / page-type / trust. Pagefind removed. | Arc 02, Arc 01 |
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

- **Arc 01 — Render core:** planned in detail (see
  [`arc01-render-core/arc-plan.md`](./arc01-render-core/arc-plan.md)). Slice 01
  open set written; not yet implemented.
- **Arcs 02–06:** roadmap-level only, by design. Each is planned deep when it
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
| P-1 | Arc 01 (render core) closed + composed | ptr: `arc01-render-core/closing-report.md` | correctness | project-plan | open | |
| P-2 | Arc 02 (routes & static export) closed + composed | ptr: arc02 closing-report | correctness | project-plan | open | |
| P-3 | Arc 03 (tiny_http server) closed + composed | ptr: arc03 closing-report | correctness | project-plan | open | |
| P-4 | Arc 04 (custom search) closed + composed | ptr: arc04 closing-report | correctness | project-plan | open | |
| P-5 | Arc 05 (news content type) closed + composed | ptr: arc05 closing-report | correctness | project-plan | open | |
| P-6 | Arc 06 (polish/parity/deploy) closed + composed | ptr: arc06 closing-report | correctness | project-plan | open | |
| P-7 | `haleiki build` emits a static `_site/` that renders the demo **wiki** with parity to the design review | reproduce: build, open in browser, diff vs review mockups | serious | DoD | open | |
| P-8 | `haleiki serve` serves the same site over tiny_http with configurable bind/port, structured errors + logging | reproduce: serve, curl routes + assets, kill/restart | serious | DoD | open | |
| P-9 | Client-side search returns faceted results over the Rust-built JSON index | reproduce: query in browser, exercise each facet | serious | DoD | open | |
| P-10 | The **news** section renders end-to-end in both static export and server | reproduce: build + serve, walk index/post/archive/tags/categories | serious | DoD | open | |
| P-11 | No Cobalt / Liquid / Pagefind references remain on the branch | `grep -ri -e cobalt -e liquid -e pagefind` returns only historical/doc mentions | correctness | DoD | open | |
| P-12 | Visual + functional parity with the design review accepted by the operator | operator acceptance judgment, recorded | serious | DoD | open | |

## 5. Version history

### v1.0 — 2026-07-28
Initial roadmap for the tiny_http/minijinja pivot. Six arcs (render core →
routes & static export → tiny_http server → custom search → news content type →
polish/parity/deploy), the definition of done and boundaries reflecting the four
operator decisions (both-deployment, news in scope, custom search, arc/slice
layout), and the project ledger opened with the DoD composition rows. Arc 01
planned deep; Arcs 02–06 roadmap-level pending *plan late, plan deep*.
