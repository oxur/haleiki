# Arc 03 — tiny_http Server · Closing Report

> Per-row walk of the arc ledger (`arc-plan.md` §4), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section B (class-a slices closed, class-b
> composition **reproduced at arc scale**, class-c bubble-ups dispositioned).
> Independent (CDC) close, not the implementer. Verdict: **CLOSED** ·
> 2026-07-29.

---

## Capability verdict

Arc 03's capability — *`haleiki serve` runs a hardened `tiny_http` server that
renders the full site into memory once, via the same pipeline as the static
export, and serves the HTML routes and static assets over HTTP* — is
**delivered and reproduced at arc scale**. A real `haleiki serve` against the
demo corpus serves every route type and asset, rejects unknown paths and
unsupported methods correctly, answers HEAD with true HTTP semantics, isolates
per-request panics, logs every request, and carries a `nosniff` header — and its
served bodies are byte-identical to what `haleiki export` writes. The pivot's
"Both" deployment decision now has its second sink: static export (Arc 02) is
the default deliverable, and this server is hardened enough to also run as an
optional self-host.

## Slice walk (class a)

Two slices, both CDC-closed with reproduced (not merely attested) evidence:

- **Slice 01 — route map + core server** — CLOSED 2026-07-28
  ([`slice01-route-map-server/cdc-verification.md`](./slice01-route-map-server/cdc-verification.md)).
  Extracted `export`'s route-walk into the shared `render_routes() ->
  SiteRoutes` (byte-identical export preserved), added `tiny_http`, a **pure**
  `resolve(path, &routes, assets_dir)` router, and a lock-free `Arc<SiteRoutes>`
  serve loop. The asset path-traversal guard was independently audited against a
  live server. 8/8 rows reproduced.
- **Slice 02 — hardening + arc composition** — CLOSED 2026-07-29
  ([`slice02-hardening/cdc-verification.md`](./slice02-hardening/cdc-verification.md)).
  Added method-aware handling (`GET`/`HEAD`/`405`+`Allow`), per-request
  `catch_unwind` panic isolation (`500`, worker survives), a pure
  `access_log_line`, and the `X-Content-Type-Options: nosniff` header — all with
  the shared state still immutable + lock-free. 8/8 rows reproduced; the hardened
  surface audited live.

Both slice ledgers reached final status on every row; nothing was silently
dropped. The one recurring in-scope decision (dependency-light stderr logging
over `tracing`/`log`, graceful shutdown deferred, nosniff-only) was taken as
proposed and recorded — a legitimate outcome, not a gap.

## Composition — reproduced at arc scale (class b)

The arc ledger's composition rows were reproduced by running a live `haleiki
serve` against the demo corpus in a clean cloud build (`cargo 1.95.0`, commit
`8348dc0`) and exercising it over the wire — not inherited from the slices'
attestations.

| ID | Criterion | Verdict | Arc-scale reproduction |
|----|-----------|---------|------------------------|
| C-1 | Slice 01 closed | **done** | reproduced — slice 01 CDC-closed 2026-07-28, 8/8 rows; traversal guard audited live. |
| C-2 | Slice 02 closed | **done** | reproduced — slice 02 CDC-closed 2026-07-29, 8/8 rows; hardened surface audited live. |
| C-3 | Live server serves every route type `200`+`text/html`, assets `200`+correct type, unknown `404` | **done** | reproduced — live sweep of `/ /browse/ /about/ /attribution/ /search/ /category/mathematics/ /abelian-group/ /source/vimalamitra/` + `/assets/css/haleiki.css` all `200`; `/nope/` `404`. |
| C-4 | Server ↔ export parity (one pipeline, two sinks) | **done** | reproduced — the parity test (served body == exported file) passes, including through the hardened `handle()` path; the export suite is unchanged and green. |
| C-5 | Hardened for self-host: configurable bind; a per-request error doesn't crash the server; requests logged | **done** | reproduced — `--bind 127.0.0.1:<port>` honored; live panic-isolation (worker survives a panicking request); every request logged to stderr in the `{method} {path} {status} {len}` shape; `HEAD` empty-body/`Content-Length`, `POST` `405`+`Allow`, and `nosniff` all confirmed on the wire. |

All five rows **done**, reproduced at arc scale.

## Bubble-up to the project (class c)

- **P-3 satisfied.** Arc 03 delivered its capability as scoped; the project
  roadmap is **unchanged** by this close — a valid, recorded outcome. `haleiki
  serve` is the DoD's "servable process," giving project ledger row **P-8** its
  implementation (P-8 itself stays open until reproduced at project scale in Arc
  06's recomposition).
- **Carried forward, unchanged:** the Arc 06 CLI reconciliation note (static
  export landed as an additive `export` subcommand while `build` still writes
  the legacy `_data` JSON; Cobalt/Liquid removal also lands there). Arc 03 added
  a third subcommand (`serve`) additively alongside both — no reconciliation
  attempted here, by boundary.
- **New forward finding (hardening backlog):** graceful shutdown / signal
  handling was deferred by decision this arc. It is the first hardening gap to
  reconsider **if** self-hosted `haleiki serve` ever moves beyond a
  personal/low-stakes deployment. Not on the critical path to the DoD; noted for
  Arc 06's deploy-readiness review.
- **No scope reshape** to Arcs 04–06 is forced by this close.

## What worked (Safety-II)

- The slice-01 `render_routes` extraction made "one renderer, two sinks"
  literally checkable, and slice 02's hardening rode entirely on **pure
  functions** (`handle`, `guarded`, `access_log_line`, `build_response`) — so the
  arc-scale composition needed the live server only to confirm wire behavior the
  unit tests had already pinned, and it confirmed rather than surprised.
- Keeping the shared state an **immutable `Arc<SiteRoutes>`** across both slices
  meant hardening added zero synchronization primitives — no lock to reason
  about, and panic isolation is a clean per-request wrapper rather than a
  poisoned-lock recovery problem.

## Verdict

**Arc 03 (tiny_http server) is CLOSED and composed.** Both slices CDC-closed;
composition rows C-1…C-5 reproduced at arc scale against a live hardened server.
Project ledger row **P-3** is satisfied (evidence: this file). Next: **Arc 04 —
custom search**, to plan deep on the operator's go-ahead.
