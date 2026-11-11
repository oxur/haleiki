# Slice 02 — hardening + arc composition

> Plan-of-record for the second and final slice of Arc 03 (tiny_http Server).
> Open set: this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** (planned, not implemented) · 2026-07-28.

---

## Goal

Harden slice 01's core server into something fit to run as an optional
**self-hosted deployment**, and demonstrate the arc's capability end-to-end
against a live server. Slice 01 shipped `recv → resolve → respond` with no method
handling, no logging, and no panic isolation; this slice adds those, then closing
it closes Arc 03.

## What "hardened" means here (scope)

- **HTTP methods.** `GET` serves the body. `HEAD` serves the same status and
  headers (`Content-Type`, `Content-Length`) with an **empty body**. Any other
  method → `405 Method Not Allowed` with an `Allow: GET, HEAD` header. (Today the
  loop serves the resolved body regardless of method.)
- **Per-request panic isolation.** Wrap each request's handling in
  `std::panic::catch_unwind` so a panic serving one request returns `500` and the
  **worker keeps serving** — a bad request can never shrink the pool to zero.
- **Structured access logging.** One line per request to stderr — method, path,
  status, body length (e.g. `GET /abelian-group/ 200 18446`). Factor the line as
  a pure `access_log_line(...) -> String` so it is unit-testable; the loop prints
  it.
- **Robust bind.** `--bind addr:port` already exists (slice 01); ensure an
  invalid bind returns a clear error (no panic), and the startup messages
  ("Rendering…", "Serving on…") remain.
- **A safe default header.** Add `X-Content-Type-Options: nosniff` to responses
  (cheap defense-in-depth for a server that returns user-visible HTML + assets).

## Scope

### In

- Method-aware handling (`GET`/`HEAD`/405) in the serve loop; factor a pure
  `handle(method, path, &routes, assets_dir) -> ServedResponse` (extend
  `ServedResponse`/response building with method + the `Allow` header for 405)
  so it stays unit-testable without a socket.
- `catch_unwind` per request → `500` on panic, worker survives.
- `access_log_line` (pure) + emit per request.
- `nosniff` header on responses.
- Live integration tests exercising the hardened server (methods, 404/405, a
  HEAD, an asset) — the arc-composition demonstration.

### Out

- **Graceful shutdown / signal handling** — see decision #2 (proposed: defer;
  rely on the process manager / `SIGTERM`, since the site is read-only and
  stateless). No `ctrlc`-style dependency this slice unless confirmed.
- Live-render / dev mode; in-memory assets (slice-01 decision stands: assets from
  disk). Search behavior (Arc 04); news (Arc 05); CLI reconciliation / legacy
  removal (Arc 06).
- No change to `resolve`'s routing semantics or the export output.

## Decisions to confirm at slice start

- **Logging approach.** Proposed: **dependency-light stderr access log** (a pure
  `access_log_line` + `eprintln!`). Alternative: adopt `tracing`/`log` (the
  rust-guidelines default) — heavier, but structured/filterable. Given the
  lightweight-server goal, plain stderr is proposed; confirm.
- **Graceful shutdown.** Proposed: **defer** (out of scope) — the server is
  read-only and stateless, so `SIGTERM`/`SIGKILL` from the process manager is a
  safe stop; a `catch_unwind`-protected pool doesn't need drain. Alternative: a
  minimal Ctrl+C handler (adds a signal dep). Confirm defer vs minimal.
- **Security headers.** Proposed: add `X-Content-Type-Options: nosniff` only.
  Confirm (and whether any others — e.g. a permissive `Cache-Control` — are
  wanted).

## Verification approach

Unit tests on the pure pieces: `handle` returns 200 for `GET` on a route, the
same headers with empty body for `HEAD`, `405` + `Allow` for `POST`/`PUT`;
`access_log_line` formats correctly; the `catch_unwind` wrapper returns `500` on
a panicking closure and does not propagate. Live integration tests (ephemeral
port): every route type + an asset → `200`; unknown → `404`; a `POST` → `405`; a
`HEAD` → `200` with empty body + correct `Content-Length`; and — the panic-
isolation proof — after a request that would panic, a subsequent normal request
still succeeds. `cargo clippy -p haleiki --all-targets -- -D warnings` + `cargo
fmt --check` clean.

## Exit criteria

Ledger rows F-1…F-8 reach a final status. In short: methods handled
(GET/HEAD/405), panics isolated (500, worker survives), requests logged, the
nosniff header present, bind robust — all demonstrated against a live server; new
code is clippy(`--all-targets`)/fmt-clean.

## Relationship to the arc close

This is Arc 03's last slice. Its live-server tests are the demonstration the arc
composition rows reproduce: **C-3** (every route type + assets + 404 served),
**C-4** (server↔export parity, from slice 01, reconfirmed), **C-5** (hardened:
configurable bind, per-request error doesn't crash the server, requests logged).
After this slice CDC-closes, the arc closes: reproduce C-3…C-5 at arc scale
against a live server, write `arc03-tiny-http-server/closing-report.md`, and
bubble up to `project-plan.md` (P-3).

## Notes

- Load the **rust-guidelines** CLI-tools + concurrency + error-handling guides.
  Keep the shared state immutable + lock-free (slice 01's `Arc<SiteRoutes>`).
- `catch_unwind` requires the caught closure be `UnwindSafe` — the immutable
  `&SiteRoutes` / `&Path` are fine; keep the boundary clean.
- Run clippy **with `--all-targets`**; concatenate any no-Liquid needle. Reuse
  the raw-`std::net` `http_get` test helper pattern from slice 01 for the live
  tests (add a HEAD/POST variant).
