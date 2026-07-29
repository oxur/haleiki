# Arc 03 — tiny_http Server

> The arc plan-of-record: the second sink. Serve the same render pipeline over
> HTTP via `tiny_http`, hardened for optional self-host — the "Both" deployment
> decision. Part of the tiny_http pivot ([`../project-plan.md`](../project-plan.md));
> architecture in [`../design-delta.md`](../design-delta.md). Status: **active**
> · 2026-07-28.

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

Opens here; closes (per-row walk) in `closing-report.md` at arc close. Class (a)
= slices closed; class (b) = composition, **reproduced at arc scale** (run a live
server and exercise it); class (c) = bubble-up findings dispositioned.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| C-1 | Slice 01 (route map + core server) closed | ptr: `slice01-route-map-server/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 01 CDC-closed 2026-07-28, 8/8 rows; traversal guard audited live |
| C-2 | Slice 02 (hardening + composition) closed | ptr: slice02 cdc-verification | correctness | arc-plan | open | |
| C-3 | A live `haleiki serve` against the demo serves every route type (home, browse, category, concept, source, about, attribution, search) with `200` + `text/html`, assets with `200` + correct content-type, and unknown paths with `404` + the 404 body | reproduce at arc scale: start the server, HTTP-GET a sample of each | serious | arc-plan | open | |
| C-4 | Server ↔ export parity: a served route's body is identical to the file the export writes for that route (one pipeline, two sinks) | reproduce: diff a served body against the exported file | serious | arc-plan | open | |
| C-5 | Hardened for self-host: configurable bind/port; a per-request error does not crash the server; requests are logged | reproduce: bind a chosen port; induce a bad request; observe the server survives + logs | serious | arc-plan | open | |

_(Class-(c) bubble-up rows are added as slices surface findings.)_

## 5. Version history

### v1.0 — 2026-07-28
Initial arc plan. Capability (`haleiki serve`: a hardened tiny_http server that
renders the site once into memory and serves it — the self-host sink), two-slice
breakdown (route-map + core server → hardening + composition), dependencies
against the Arc 01–02 render pipeline + the shared route-walk refactor, and the
arc ledger opened with the composition rows. Slice 01 planned deep; slice 02
roadmap-level.
