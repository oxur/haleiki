# CC Prompt — Arc 03 / Slice 01: route map + core server

You are implementing the first slice of Arc 03 of the Haleiki tiny_http/minijinja
pivot, on branch `framework-base/tiny-http`. Work to the collaboration-framework's
ledger discipline: this slice is defined by its ledger, and every ledger row must
reach a final, evidence-backed status before you claim the slice done.

## Before you write code

1. Load the **collaboration-framework** skill and the **rust-guidelines** skill
   (anti-patterns first; **and the concurrency guide** — the server shares state
   across threads). Crate lints: `clippy::all` denied, `pedantic` warned,
   `unsafe_code` forbidden, edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, the "one renderer, two sinks"
     design, scope, and the four decisions (serving mode, asset serving, default
     bind, live-server test approach).
   - [`ledger.md`](./ledger.md) — the eight rows F-1…F-8. The contract.
   - [`../arc-plan.md`](../arc-plan.md) — the arc capability + composition rows.
   - `tools/src/export.rs` — the route-walk you refactor into `render_routes`
     (keep `export_site` byte-identical; its tests guard this), and the asset
     allowlist + content-type needs.
   - The renderers you compose: `page::render_*`, `article::render_*`, and
     `render::{build_environment, assemble_site_context, resolve_profile}`.
   - `tools/src/cli.rs` + `main.rs` for the CLI wiring pattern.
   - `CLAUDE.md` — the `&#x2f;` href-escaping gotcha note.

## What to build

- Refactor `export::export_site`'s walk into `render_routes(content_dir,
  templates_dir) -> Result<SiteRoutes>` (page routes keyed by URL path + the 404
  body). Reimplement `export_site` on top of it — **byte-identical output**.
- Add `tiny_http` to `[dependencies]`.
- New `tools/src/server.rs`:
  - `SiteRoutes` (immutable), a `resolve(path, &routes, assets_dir) ->
    ServedResponse{status, content_type, body}` **pure** router (page routes,
    404, `/assets/…` allowlist + content-type by extension, trailing-slash
    normalization).
  - `serve(content_dir, templates_dir, assets_dir, bind) -> Result<()>` — build
    `SiteRoutes` once, `Arc` it, run `tiny_http` over a fixed thread pool. The
    shared state is an **immutable `Arc<SiteRoutes>`** — lock-free; do not
    introduce a `Mutex`/`RwLock`.
- CLI `serve` subcommand (`--content-dir --templates-dir --assets-dir --bind`),
  additive alongside `build`/`export`.

## Boundaries (do not cross in this slice)

- No hardening beyond `--bind`: `HEAD`/method handling, structured logging,
  worker panic isolation, graceful shutdown, in-memory assets are **slice 02**.
- No live-render/dev mode; no search index / `search.js` behavior (Arc 04); no
  news (Arc 05); no CLI reconciliation / legacy removal (Arc 06).
- Keep `export_site` output byte-identical; do not fix unrelated pre-existing
  lint debt.
- Reuse `render::build_environment`; keep the shared state immutable + lock-free.

## Definition of done

All of F-1…F-8 reach a final status with reproducible evidence. Keep the bulk of
verification in the **pure `resolve`** unit tests; add one live ephemeral-port
smoke. Run clippy **with `--all-targets`**; concatenate the no-Liquid needle. In
the closing report, walk the ledger row by row — status + evidence, no silent
drops, no "deviations: none" prose. Name any uncertainty (the serving-mode
decision especially). Do **not** write `cdc-verification.md`. Iteration budget:
five.
