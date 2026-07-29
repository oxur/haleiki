# Slice 02 — hardening + arc composition · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `GET` on a known route serves `200` + body; `HEAD` on the same route serves `200` with the same `Content-Type` and a `Content-Length` equal to the would-be body length, but an **empty body** | unit test on the pure `handle(method, …)` + a live `HEAD` integration test | serious | slice-doc | open | | |
| F-2 | An unsupported method (`POST`/`PUT`/`DELETE`) → `405` with an `Allow: GET, HEAD` header | unit test on `handle` + a live `POST` → 405 integration test | serious | slice-doc | open | | |
| F-3 | Per-request panic isolation: the request wrapper returns `500` when handling panics and does **not** propagate; a live server keeps serving a subsequent request after one that panics | unit test of the `catch_unwind` wrapper on a panicking closure; live test: panic-path request then a normal `200` | serious | slice-doc | open | | |
| F-4 | Each served request emits a structured access-log line (method, path, status, body length) to stderr; the line is produced by a pure `access_log_line(...)` | unit test on `access_log_line` formatting | polish | slice-doc | open | | |
| F-5 | Responses carry `X-Content-Type-Options: nosniff`; an invalid `--bind` returns a clear error (no panic) | live test asserts the header; unit/integration test asserts a bad bind → `Err` with a helpful message | serious | slice-doc | open | | |
| F-6 | Live composition (C-3/C-5 at arc scale): a running server serves every route type (home, browse, category, concept, source, about, attribution, search) `200`, an asset `200` + content-type, unknown `404`, `POST` `405`, `HEAD` `200`-empty | live ephemeral-port integration test walking each case | serious | slice-doc (C-3/C-5) | open | | |
| F-7 | Server ↔ export parity still holds (C-4): a served body equals the exported file for a sample route; the server path references no `.liquid` | reuse/extend the slice-01 parity + no-Liquid tests (needle concatenated) | correctness | slice-doc (C-4) | open | | |
| F-8 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 8. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
