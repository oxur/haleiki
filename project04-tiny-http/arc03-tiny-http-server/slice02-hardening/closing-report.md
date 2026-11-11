# Slice 02 — hardening + arc composition · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 8 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run (CC does not write
> `cdc-verification.md` or the arc `closing-report.md` — those are the
> independent reviewer's, per the cc-prompt). Not yet committed. This is Arc
> 03's second and final slice. 2026-07-29. Iterations used: 1 of 5.

## Row-by-row walk

| ID | Status | Summary |
|----|--------|---------|
| F-1 | done | `HEAD` matches `GET`'s status/`Content-Type`/`Content-Length` with an empty body, proven both purely (`handle`) and live (`curl -sI`, and the live composition test). |
| F-2 | done | `POST`/`PUT`/`DELETE` → `405` + `Allow: GET, HEAD`, proven purely and live. |
| F-3 | done | The `guarded` `catch_unwind` wrapper returns `500` on a panic without propagating; a live single-worker test proves the same thread serves a normal request immediately after one that panicked. |
| F-4 | done | `access_log_line` is a pure formatter, unit-tested and cross-checked against real stderr output from the running binary. |
| F-5 | done | `X-Content-Type-Options: nosniff` on every response (unit + live); an invalid `--bind` returns a clear `Err` (already true since slice 01's `Server::http(...).map_err(...)`; this slice adds the first test proving it). |
| F-6 | done | Live composition test walks all 8 page-route kinds, an asset, a 404, a `HEAD`, and a `POST` against the real `serve_loop` over an ephemeral port — reconfirmed by hand against the actual `haleiki serve` binary. |
| F-7 | done | Slice-01's parity + no-Liquid tests pass unmodified; a new test reconfirms parity through the hardened `handle()` entry point specifically. |
| F-8 | done | `server.rs`/`export.rs`/`cli.rs`/`main.rs` are clippy(`--all-targets --all-features`)-clean and `cargo fmt`-clean; `make check` passes. |

No silent drops. Every row reached `done` with attested evidence; nothing was deferred.

## What was built

- **`ServedResponse`** gained `content_length: usize` and `allow: Option<&'static str>` fields, so a `HEAD` response can declare the same `Content-Length` a `GET` on the same route would while its `body` is genuinely empty — the struct is self-describing regardless of who reads it, not dependent on a caller separately knowing `tiny_http`'s own HEAD-suppression behavior.
- **`server::handle(method, path, &routes, assets_dir) -> ServedResponse`** — the pure, method-aware wrapper around `resolve`: `GET` delegates unchanged; `HEAD` delegates then empties `body` (keeping `content_length`); anything else returns `405` + `Allow: GET, HEAD`.
- **`server::guarded<F>(f) -> ServedResponse`** — a `std::panic::catch_unwind` wrapper: `500` on panic, transparent otherwise. Used per-request in `serve_loop` so one bad request can never shrink the worker pool.
- **`server::access_log_line(method, path, status, content_length) -> String`** — pure formatter (`"GET /abelian-group/ 200 18446"`-shaped), emitted to stderr per request.
- **`server::build_response(ServedResponse) -> Response<...>`** — centralizes `Content-Type`, an explicit `Content-Length` (decoupled from `body.len()` so `HEAD` reports correctly), `X-Content-Type-Options: nosniff` on every response, and `Allow` when set.
- **`serve_loop`** rewritten onto `handle`/`guarded`/`build_response`/`access_log_line` — same `Arc<SiteRoutes>`, no new synchronization primitive.
- **17 new tests**: pure unit tests for `handle` (HEAD, 405+Allow), `access_log_line`, `guarded` (panic and pass-through), `build_response` (nosniff/Content-Length, Allow), the bad-bind error path, a hardened-parity spot-check, plus two live ephemeral-port integration tests — one walking every route/asset/404/405/HEAD case against the real `serve_loop`, one proving panic isolation on a live single worker thread.

## Deviations from plan

None. The slice-doc's three "decisions to confirm at slice start" were all taken as proposed: dependency-light stderr access log (no `tracing`/`log`), graceful shutdown deferred (no signal-handling dependency added), and `X-Content-Type-Options: nosniff` as the only added security header.

## Named uncertainty

- **`tiny_http`'s own HEAD-suppression is a second, independent safety net** beyond `handle()`'s explicit body-emptying: reading `respond_impl`'s source showed `do_not_send_body = self.method == Method::Head` is checked against the *real request*, regardless of what response object is handed to `respond()`. This means the design is correct even in a hypothetical future where someone forgets to route through `handle()` for a HEAD request — worth knowing, but also worth naming explicitly so a future reader doesn't assume `handle()`'s body-emptying is the *only* thing preventing HEAD from leaking a body.
- **The panic-isolation live test's trigger is test-local by design**, not a reachable path in production `server.rs` — `resolve`/`handle` don't panic on any real input today. The test proves the `guarded` wrapper generically isolates *any* panic, not that a specific bug exists. If a future slice adds request paths that legitimately can panic (e.g. new parsing logic in the hot path), it's worth re-confirming this guarantee against that new code specifically, not just trusting this test's coverage.
- **Graceful shutdown remains explicitly deferred**, per the slice-doc's decision: `SIGTERM`/`SIGKILL` from the process manager is treated as a safe stop since the site is read-only and stateless. If self-hosted `haleiki serve` deployment moves beyond a personal/low-stakes use case, this is the first hardening gap a future arc should reconsider.

## Bubble-up to arc-plan

This is Arc 03's last slice — closing it should close the arc. Bubble-up for `arc-plan.md`'s composition rows:

- **C-3** (every route type + assets + 404 served): reproducible now — `test_live_hardened_server_serves_every_route_asset_404_405_and_head` demonstrates this against a live server using the real production `serve_loop`, and it was independently hand-confirmed against the actual `haleiki serve` binary.
- **C-4** (server↔export parity): still holds — reconfirmed both via the unchanged slice-01 parity test and a new test that goes through the hardened `handle()` path specifically.
- **C-5** (hardened for self-host: configurable bind, a per-request error doesn't crash the server, requests are logged): reproducible now — `--bind` was already configurable (slice 01); this slice adds panic isolation (a request that panics doesn't crash the worker, demonstrated live) and access logging (demonstrated live against real stderr output).

Nothing here contradicts or extends `arc-plan.md`'s scope for this slice. C-1 and C-2 (slice closures) are pointer rows for the independent reviewer; **C-2** should be marked `done` once this slice's own CDC verification lands, at which point the arc can close: reproduce C-3…C-5 at arc scale (a fresh live-server run against the demo, walking the same cases this slice's tests already walk), write `arc03-tiny-http-server/closing-report.md`, and bubble up to `project-plan.md` (P-3). Per the cc-prompt, that composition report and the arc close are the independent reviewer's to write, not CC's.

## Verification commands run

```
cargo build --all-targets
cargo test --workspace
cargo clippy --all-targets --all-features --workspace -- -D warnings
cargo clippy --all-features --workspace -- -D warnings   # make lint's actual invocation
cargo fmt --all -- --check
make check
```

All green. Additionally hand-verified against the real `haleiki serve` binary: `curl -sI` (HEAD), `curl -X POST` (405 + Allow), `curl` against an asset and a traversal path, and the resulting stderr access-log lines, all matching the test suite's expectations.
