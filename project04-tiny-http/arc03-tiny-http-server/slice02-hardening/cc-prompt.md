# CC Prompt — Arc 03 / Slice 02: hardening + arc composition

You are implementing the second and final slice of Arc 03 of the Haleiki
tiny_http/minijinja pivot, on branch `framework-base/tiny-http`. Closing this
slice closes the arc. Work to the collaboration-framework's ledger discipline:
every ledger row must reach a final, evidence-backed status before you claim the
slice done.

## Before you write code

1. Load the **collaboration-framework** skill and the **rust-guidelines** skill
   (anti-patterns; **CLI-tools, concurrency, and error-handling** guides —
   relevant to a hardened server). Crate lints: `clippy::all` denied, `pedantic`
   warned, `unsafe_code` forbidden, edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, the hardening scope, and the three
     decisions (logging approach, graceful shutdown, security headers).
   - [`ledger.md`](./ledger.md) — the eight rows F-1…F-8. The contract.
   - [`../arc-plan.md`](../arc-plan.md) — the arc capability + composition rows
     C-3…C-5 this slice reproduces.
   - [`../slice01-route-map-server/cdc-verification.md`](../slice01-route-map-server/cdc-verification.md)
     — what slice 01 delivered (`render_routes`, `resolve`, `serve`/`serve_loop`,
     the lock-free `Arc<SiteRoutes>`, the raw-`std::net` `http_get` test helper).
   - `tools/src/server.rs` — the code you harden (`serve_loop`, `ServedResponse`).

## What to build (in `server.rs`)

- Method-aware handling: factor a pure `handle(method, path, &routes,
  assets_dir) -> ServedResponse` (extend response building with `HEAD` → empty
  body + `Content-Length`; other-than-GET/HEAD → `405` + `Allow: GET, HEAD`).
  The serve loop calls it.
- `catch_unwind` per request → `500` on panic, worker survives (do not let a
  panic escape the loop).
- A pure `access_log_line(method, path, status, len) -> String`, emitted per
  request to stderr.
- `X-Content-Type-Options: nosniff` on responses; a clear error (not a panic) on
  a bad `--bind`.
- Extend the slice-01 live-test helper with `HEAD`/`POST` variants for the
  composition tests.

## Boundaries (do not cross in this slice)

- Keep the shared state **immutable + lock-free** (`Arc<SiteRoutes>`); do not add
  a `Mutex`/`RwLock`.
- Do not change `resolve`'s routing semantics or the export output.
- Graceful shutdown / signal handling is **out** unless the operator confirms
  decision #2 (proposed: defer). No search behavior (Arc 04), news (Arc 05), CLI
  reconciliation/legacy removal (Arc 06).
- Do not fix unrelated pre-existing lint debt.

## Definition of done

All of F-1…F-8 reach a final status with reproducible evidence. Keep the bulk of
verification in pure unit tests (`handle`, `access_log_line`, the panic wrapper);
prove the rest with live ephemeral-port integration tests. Run clippy **with
`--all-targets`**. In the closing report, walk the ledger row by row — status +
evidence, no silent drops, no "deviations: none" prose. Add the **bubble-up to
arc-plan**: this is the last slice, so note that C-3…C-5 are now reproducible
against the hardened live server, ready for the arc close. Name any uncertainty.
Do **not** write `cdc-verification.md` or the arc `closing-report.md` — those are
the independent reviewer's. Iteration budget: five.
