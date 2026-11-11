# Slice 02 — hardening + arc composition · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **closed** · 2026-07-29.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `GET` on a known route serves `200` + body; `HEAD` on the same route serves `200` with the same `Content-Type` and a `Content-Length` equal to the would-be body length, but an **empty body** | unit test on the pure `handle(method, …)` + a live `HEAD` integration test | serious | slice-doc | done | reproduced — `handle` unit tests (GET body; HEAD → same status/`Content-Type`/`content_length`, empty `body`) + the live composition test. CDC live: `HEAD /` → `200`, `Content-Type: text/html`, `Content-Length: 16756`, body **0 bytes** (`curl size_download=0`); `GET /` `size_download=16756` — HEAD's `Content-Length` equals the true GET body length while no body is sent. | `ServedResponse.content_length` is decoupled from `body` so the struct is self-describing; tiny_http's own `do_not_send_body` is a second, independent net. |
| F-2 | An unsupported method (`POST`/`PUT`/`DELETE`) → `405` with an `Allow: GET, HEAD` header | unit test on `handle` + a live `POST` → 405 integration test | serious | slice-doc | done | reproduced — `handle` unit test for `POST`/`PUT`/`DELETE` → `405` + `Allow`. CDC live: `POST /` → `HTTP/1.1 405 Method Not Allowed`, `Allow: GET, HEAD` (with `nosniff` also present). | |
| F-3 | Per-request panic isolation: the request wrapper returns `500` when handling panics and does **not** propagate; a live server keeps serving a subsequent request after one that panics | unit test of the `catch_unwind` wrapper on a panicking closure; live test: panic-path request then a normal `200` | serious | slice-doc | done | reproduced — unit test of `guarded` on a panicking closure → `500`, no propagation; pass-through transparent. Live single-worker integration test serves a normal request on the same thread immediately after one that panicked. | Trigger is test-local by design — no real `resolve`/`handle` input panics today; the wrapper's generic isolation is what's proven. |
| F-4 | Each served request emits a structured access-log line (method, path, status, body length) to stderr; the line is produced by a pure `access_log_line(...)` | unit test on `access_log_line` formatting | polish | slice-doc | done | reproduced — `access_log_line` unit test on the `"{method} {path} {status} {len}"` shape. CDC live: the running binary's stderr shows exactly that, e.g. `GET /assets/css/haleiki.css 200 88227`, `GET /nope/ 404 5608`, `GET /source/vimalamitra/ 200 9483`. | |
| F-5 | Responses carry `X-Content-Type-Options: nosniff`; an invalid `--bind` returns a clear error (no panic) | live test asserts the header; unit/integration test asserts a bad bind → `Err` with a helpful message | serious | slice-doc | done | reproduced — `build_response` unit test asserts `nosniff`; bad-`--bind` test asserts an `Err` (not a panic). CDC live: `nosniff` present on GET, HEAD **and** the 405. | Bad-bind was already `Err` since slice-01's `Server::http(...).map_err(...)`; this slice adds the first test proving it. |
| F-6 | Live composition (C-3/C-5 at arc scale): a running server serves every route type (home, browse, category, concept, source, about, attribution, search) `200`, an asset `200` + content-type, unknown `404`, `POST` `405`, `HEAD` `200`-empty | live ephemeral-port integration test walking each case | serious | slice-doc (C-3/C-5) | done | reproduced — `test_live_hardened_server_serves_every_route_asset_404_405_and_head` walks all cases against the real `serve_loop`. CDC live sweep: `/ /browse/ /about/ /attribution/ /search/ /category/mathematics/ /abelian-group/ /source/vimalamitra/` + `/assets/css/haleiki.css` all `200`; `/nope/` `404`; `POST` `405`; `HEAD` `200`-empty; traversal `404`. | |
| F-7 | Server ↔ export parity still holds (C-4): a served body equals the exported file for a sample route; the server path references no `.liquid` | reuse/extend the slice-01 parity + no-Liquid tests (needle concatenated) | correctness | slice-doc (C-4) | done | reproduced — slice-01 parity + no-Liquid tests pass unmodified; a new test reconfirms parity through the hardened `handle()` entry point. Served bodies still equal exported files. | |
| F-8 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | done | reproduced — CDC `cargo clippy -p haleiki --all-targets -- -D warnings`: **0 findings** in `server.rs`; `cargo fmt --check` clean. Only the two pre-existing `tests/demo_status.rs` unused-import findings remain — untouched, out of scope. `cargo test -p haleiki --bin haleiki` = 101 passed / 0 failed / 1 ignored (20 server tests). | |

## What Worked

- Making `ServedResponse` **self-describing** (`content_length` decoupled from
  `body`) is why HEAD is correct regardless of who calls `respond()` — the struct
  carries the truth rather than depending on a caller remembering tiny_http's HEAD
  behavior. Naming tiny_http's own suppression as a *second, independent* net
  (read from `respond_impl` source) is the belt-and-braces reasoning that makes a
  future regression here loud, not silent.
- Keeping the hardening in **pure functions** (`handle`, `guarded`,
  `access_log_line`, `build_response`) put the bulk of the contract in socket-free
  unit tests, leaving the live tests to prove only what needs a wire — which is
  why the CDC live probe confirmed rather than surprised.
- The shared state stayed an **immutable `Arc<SiteRoutes>`** — hardening added
  zero synchronization primitives, so panic isolation is a clean per-request
  wrapper, not a poisoned-lock recovery problem.

## Closure

**Independently verified (CDC): CLOSED** 2026-07-29. All 8 rows reproduced in a
clean build (`cargo 1.95.0`, commit `8348dc0`) — `cargo test -p haleiki --bin
haleiki` = 101 passed / 0 failed / 1 ignored (20 server tests) — plus a **live
`haleiki serve`** run probed with real HTTP: `HEAD /` → `200` empty body with
`Content-Length: 16756` (== the `16756`-byte GET body); `POST /` → `405` +
`Allow: GET, HEAD`; `nosniff` on GET/HEAD/405; a full route sweep all `200`,
`/nope/` and a `../` traversal both `404`; correctly-shaped stderr access-log
lines. `clippy --all-targets -D warnings` = 0 findings in `server.rs`; `fmt`
clean (only pre-existing `demo_status.rs` import debt remains). See
[`cdc-verification.md`](./cdc-verification.md). Evidence `attested → reproduced`.
Verified by: independent Claude (Cowork) context. This was Arc 03's final slice —
the arc closes with it (see [`../closing-report.md`](../closing-report.md)).
Rows: 8. Done: 8 (reproduced). Deferred: 0. No-op: 0.
