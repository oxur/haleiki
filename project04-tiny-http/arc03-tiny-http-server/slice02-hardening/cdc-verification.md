# Slice 02 — hardening + arc composition · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per `LEDGER-DISCIPLINE.md` Section A. Verdict: **CLOSED** ·
> 2026-07-29.

## Verifier and method

Independent Claude (Cowork) context, not the implementer. Clean cloud build of
the committed slice (`8348dc0`) against the full demo corpus + allowlisted
assets; `cargo 1.95.0`. Beyond the test suite I **ran a real `haleiki serve`**
(bound to an ephemeral local port) and probed the hardened surface with live
HTTP — HEAD/POST/GET headers, the full route sweep, a 404, and a traversal
attack — reading the process's own stderr access log back. The hardening claims
(HEAD semantics, method rejection, security header, logging) are exactly the
kind that a passing unit test can assert without the wire agreeing, so each was
reconfirmed on the wire.

## Row-count check

Ledger opened with **8** rows (F-1…F-8); reproduces 8. No silent drops; nothing
deferred.

## Per-row verification

Full binary suite: **101 passed, 0 failed, 1 ignored** (20 server tests, +11
over slice 01's 90-test line). Each row maps to a passing test; the wire-visible
rows were additionally reproduced live.

| ID | CDC verdict | Strength | How reproduced |
|----|-------------|----------|----------------|
| F-1 | **confirmed** | reproduced | `handle` unit tests (GET body; HEAD → same status/`Content-Type`/`content_length`, empty `body`). Live: `HEAD /` → `200`, `Content-Type: text/html`, `Content-Length: 16756`, **body 0 bytes** (`curl` `size_download=0`); `GET /` `size_download=16756` — the HEAD `Content-Length` equals the real GET body length while no body is sent. |
| F-2 | **confirmed** | reproduced | `handle` unit test for `POST`/`PUT`/`DELETE` → `405` + `Allow`. Live: `POST /` → `HTTP/1.1 405 Method Not Allowed`, `Allow: GET, HEAD` (and `nosniff` also present). |
| F-3 | **confirmed** | reproduced | Unit test of `guarded` on a panicking closure → `500`, no propagation; the pass-through case is transparent. Live single-worker panic-isolation integration test serves a normal request on the same thread immediately after one that panicked. The trigger is test-local by design (no real `resolve`/`handle` input panics today) — the wrapper's generic isolation is what's proven, correctly. |
| F-4 | **confirmed** | reproduced | `access_log_line` unit test on the `"{method} {path} {status} {len}"` shape. Live: the running binary's stderr shows exactly that, e.g. `GET /assets/css/haleiki.css 200 88227`, `GET /nope/ 404 5608`, `GET /source/vimalamitra/ 200 9483`. |
| F-5 | **confirmed** | reproduced | `build_response` unit test asserts `X-Content-Type-Options: nosniff`; bad-`--bind` test asserts an `Err` (not a panic). Live: `nosniff` present on GET, HEAD **and** the 405. |
| F-6 | **confirmed** | reproduced | `test_live_hardened_server_serves_every_route_asset_404_405_and_head` walks all cases against the real `serve_loop`. Live sweep (my own run): `/ /browse/ /about/ /attribution/ /search/ /category/mathematics/ /abelian-group/ /source/vimalamitra/` and `/assets/css/haleiki.css` all `200`; `/nope/` `404`; `POST` `405`; `HEAD` `200`-empty. |
| F-7 | **confirmed** | reproduced | Slice-01 parity + no-Liquid tests pass unmodified; a new test reconfirms parity through the hardened `handle()` entry point. Served bodies still equal exported files (one renderer, two sinks). |
| F-8 | **confirmed** | reproduced | `cargo clippy -p haleiki --all-targets -- -D warnings`: **0 findings** in `server.rs`; `cargo fmt --check` clean. Only the two pre-existing `tests/demo_status.rs` unused-import findings remain — untouched, correctly out of scope for this slice. |

## Live hardening audit (the wire-visible surface)

Ran `haleiki serve --content-dir demo --templates-dir templates --assets-dir
assets --bind 127.0.0.1:<port>` against the demo corpus and probed it directly:

- **HEAD semantics** — the subtle one. `HEAD /` returns `Content-Length: 16756`
  with a genuinely empty body (`size_download=0`), and `GET /` returns a
  `16756`-byte body. So the `Content-Length` a client sees for HEAD is the true
  GET length, not `body.len()` of an empty vec. This works because
  `ServedResponse` carries `content_length` independently of `body`, and
  `build_response` emits it explicitly — and, as the closing report names,
  tiny_http's own `do_not_send_body = method == HEAD` is a second, independent
  net checked against the real request. Both belts confirmed on the wire.
- **Method rejection** — `POST /` → `405` + `Allow: GET, HEAD`.
- **Security header** — `X-Content-Type-Options: nosniff` on GET, HEAD, and 405.
- **Traversal still guarded** (slice-01 property, unregressed) —
  `--path-as-is /assets/css/../../../../etc/passwd` → `404`, logged as
  `... 404 0`.
- **Access log** — one correctly-shaped line per request on stderr, including
  the 404 and traversal attempts.

## Boundary check

Diff is confined to `server.rs` (+444 lines): the new `handle`, `guarded`,
`build_response`, `access_log_line`, the `ServedResponse` `content_length` /
`allow` fields, and the rewritten `serve_loop`. Shared state stays an immutable
`Arc<SiteRoutes>` — **no `Mutex`/`RwLock` added**. `resolve`'s routing semantics
and the export output are unchanged (F-7 parity + the unmodified export suite
prove it). The three slice-doc decisions were all taken as proposed
(dependency-light stderr log, graceful shutdown deferred, nosniff-only) — a
legitimate, recorded outcome, not a silent drop.

## Minor observations (not defects)

- **Graceful shutdown is deferred**, by decision — a `SIGTERM`/`SIGKILL` stop is
  safe for a read-only stateless server. Carried forward as the first hardening
  gap to reconsider if `haleiki serve` ever becomes a higher-stakes deployment.
- **The panic-isolation test's trigger is test-local**: it proves `guarded`
  isolates *any* panic, not that a reachable panic exists. If a later arc adds
  fallible logic to the hot path (e.g. search-query parsing in Arc 04), re-confirm
  isolation against that specific code, not just this generic test.

## Verdict

**Slice 02 is CLOSED.** 8 rows, all reproduced; the hardened surface
independently audited live (HEAD empty-body/Content-Length, 405+Allow, nosniff,
route sweep, traversal, access log). Arc ledger row **C-2** satisfied (evidence:
this file). This was Arc 03's final slice — the arc now closes (see
[`../closing-report.md`](../closing-report.md)).

## What worked (Safety-II)

- Making `ServedResponse` **self-describing** — `content_length` decoupled from
  `body` — is why HEAD is correct regardless of who calls `respond()`; the
  struct carries the truth rather than depending on a caller remembering
  tiny_http's HEAD behavior. The closing report naming tiny_http's own
  suppression as a *second, independent* net (read from `respond_impl` source)
  is exactly the kind of belt-and-braces reasoning that makes a future
  regression here loud instead of silent.
- Keeping the hardening in **pure functions** (`handle`, `guarded`,
  `access_log_line`, `build_response`) put the bulk of the contract in
  socket-free unit tests, leaving the live tests to prove only what genuinely
  needs a wire — which is why the live probe confirmed rather than surprised.
