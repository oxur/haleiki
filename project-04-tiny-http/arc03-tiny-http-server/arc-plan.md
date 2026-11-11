# Arc 03 — tiny_http Server

> The arc plan-of-record: the second sink. Serve the same render pipeline over
> HTTP via `tiny_http`, hardened for optional self-host — the "Both" deployment
> decision. Part of the tiny_http pivot ([`../project-plan.md`](../project-plan.md));
> architecture in [`../design-delta.md`](../design-delta.md). Status: **closed**
> · 2026-07-29.

---

## 1. Capability

`haleiki serve` runs a **hardened `tiny_http` server** that renders the full site
into memory once — via the *same* pipeline as the static export — and serves the
HTML routes and static assets over HTTP. This is the pivot's **optional
self-hosted deployment sink**: the design-delta's "Both" decision (static export
is the default deliverable; the server is hardened enough to also run as a
deployment).

The unifying idea, straight from the design-delta ("one renderer, two sinks"):
Arc 02's export walks every route and **writes** it to `_site/`; Arc 03 walks the
*same* routes and **serves** them from memory. Slice 01 extracts that shared
walk so both sinks consume one route set.

## 2. Slice breakdown

Two slices. Arc 03 is smaller than Arc 02 — it is a serving layer over the
render pipeline that already exists. Only slice 01 is planned deep here.

| Slice | Scope | Load-bearing for |
|-------|-------|------------------|
| **01 — route map + core server** | Refactor `export`'s route-walk into a shared `render_routes() -> SiteRoutes` (URL-path → html, plus the 404 body), consumed by both `export` (write) and the server. Add `tiny_http`. A **pure** `resolve(path, &routes, assets_dir) -> ServedResponse{status, content_type, body}` router (page routes, 404, `/assets/…` from the allowlist), and a `serve` loop that holds the routes in an immutable `Arc` and serves over a small thread pool. `haleiki serve`. | 02, arc close |
| **02 — hardening + arc composition** | Configurable bind/port; correct content-types + headers; `GET`/`HEAD`/unsupported-method handling; structured request logging; robust error handling (a worker never panics the server); clean startup/shutdown. The arc-scale composition: a live server against the demo, exercised across every route type + an asset + a 404. Closes the arc. | arc close |

**Sequencing rationale.** Slice 01 delivers the shared route map + a working
server (the load-bearing core); slice 02 hardens it for self-host and reproduces
the arc composition against a live server.

## 3. Dependencies

**Consumes** (from Arcs 01–02, closed): the whole render pipeline —
`render::{build_environment, assemble_site_context, resolve_profile}`, the
`page::render_*` / `article::render_*` renderers, and `export`'s route-walk
(which slice 01 refactors into the shared `render_routes`). **Adds**:
`tiny_http`.

**Leaves for later arcs**: the search JSON index + `search.js` behavior (Arc 04
— the served `/search/` is the same static shell); news (Arc 05); the CLI
reconciliation and Cobalt/Liquid removal (Arc 06). This arc does **not** change
the export's output — it shares the walk, byte-for-byte.

## 4. Arc ledger


See the sibling [ledger.md](./ledger.md). All original row IDs, criteria,
statuses and evidence are retained there.

## 5. Version history


### Migration — 2026-11-11

Moved from `docs/tiny-http/` to orphan branch `planning`, worktree
`.worktrees/planning/project-04-tiny-http/`. The embedded ledger now lives in sibling
`ledger.md`; row text and acceptance states are unchanged. See the
[byte-preserved August plan](../../archive/docs/tiny-http/arc03-tiny-http-server/arc-plan.md). Existing issued prompts, ledgers, close reports,
verification records and `slice-doc.md` documents retain their bytes.
Canonical `slice-plan.md` entrypoints link the original slice documents.
Historical repository-relative paths resolve via the root migration manifest;
source paths such as `tools/`, `templates/` and `demo/` refer to the implementation
checkout, not this planning-only worktree. No feature verification was rerun.

### v1.1 — 2026-07-29 (arc close)
Arc 03 (tiny_http server) **CLOSED and composed**
([`closing-report.md`](./closing-report.md)). Both slices CDC-closed — slice 01
2026-07-28 (route map + core server, traversal guard audited live), slice 02
2026-07-29 (hardening: GET/HEAD/405, panic isolation, access log, nosniff).
Composition rows **C-1…C-5 marked done, reproduced at arc scale** against a live
hardened `haleiki serve` over the demo corpus. Bubbled up to the project
(P-3 satisfied; P-8 gets its implementation, stays open for project-scale
reproduction). Forward finding: graceful shutdown deferred by decision — the
first hardening gap for Arc 06's deploy-readiness review.

### v1.0 — 2026-07-28
Initial arc plan. Capability (`haleiki serve`: a hardened tiny_http server that
renders the site once into memory and serves it — the self-host sink), two-slice
breakdown (route-map + core server → hardening + composition), dependencies
against the Arc 01–02 render pipeline + the shared route-walk refactor, and the
arc ledger opened with the composition rows. Slice 01 planned deep; slice 02
roadmap-level.
