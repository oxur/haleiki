# CC Prompt — Arc 02 / Slice 04: static export writer

You are implementing the fourth and final slice of Arc 02 of the Haleiki
tiny_http/minijinja pivot, on branch `framework-base/tiny-http`. Closing this
slice closes the arc. Work to the collaboration-framework's ledger discipline:
every ledger row must reach a final, evidence-backed status before you claim the
slice done.

## Before you write code

1. Load the **collaboration-framework** skill and the **rust-guidelines** skill
   (anti-patterns first; also the CLI-tools guidance — this slice touches the
   CLI). Crate lints: `clippy::all` denied, `pedantic` warned, `unsafe_code`
   forbidden, edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, the route→path map, scope, and the
     four decisions to confirm (CLI command surface, asset allowlist,
     clean-before-write, templates/assets location).
   - [`ledger.md`](./ledger.md) — the eight rows F-1…F-8. The contract.
   - [`../arc-plan.md`](../arc-plan.md) — the arc capability + composition rows
     B-5…B-8 this slice reproduces.
   - The renderers you compose (do not re-render): `page::render_{home,browse,
     category,about,attribution,search,404}` and `article::render_{concept,
     source}`, plus `render::{build_environment, assemble_site_context,
     resolve_profile}`.
   - The CLI: `tools/src/cli.rs` (`Command` enum) and `tools/src/main.rs`
     (`run()` dispatch, `run_build`). The `assets/` tree (note `ai/` and
     `mockups/` are **not** site assets).

## What to build

- A new `tools/src/export.rs`: `export_site(content_dir, templates_dir,
  assets_dir, out_dir)` — parse → validate → graph → `assemble_site_context`,
  then render every route via the existing renderers and write it, then copy the
  asset allowlist. A `write_page(out, route, html)` helper (creates parent dirs;
  `index.html`, or `404.html` for the 404 route). Clean `_site/` first.
- Recursive asset copy with `std::fs` (no new dependency), allowlist
  `css/fonts/icons/images/js/vendor`, excluding `ai`/`mockups`.
- CLI wiring per decision #1 (proposed: a new `export` subcommand; confirm the
  name and additive-vs-extend-`build` with the operator).
- Deterministic output: stable iteration order (sort slugs/categories) so
  re-exports are byte-stable.

## Boundaries (do not cross in this slice)

- No tiny_http server (Arc 03); no search index / `search.js` behavior (Arc 04);
  no news (Arc 05).
- Do **not** remove the legacy `_data` JSON writer or the `build` command, and do
  not delete Cobalt/Liquid files — that's Arc 06. This slice is additive.
- Reuse the existing renderers and `render::build_environment`; do not re-render
  or construct a bare `Environment`.
- Do not fix unrelated pre-existing lint debt.

## Definition of done

All of F-1…F-8 reach a final status with reproducible evidence. Run clippy **with
`--all-targets`**; concatenate the no-Liquid needle. In the closing report, walk
the ledger row by row — status + evidence, no silent drops, no "deviations: none"
prose. Add the **bubble-up to arc-plan** section: this is the last slice, so note
that Arc 02's composition rows (B-5…B-8) are now reproducible against this
export, ready for the arc close. Name any uncertainty. Do **not** write
`cdc-verification.md` or the arc `closing-report.md` — those are the independent
reviewer's. Iteration budget: five.
