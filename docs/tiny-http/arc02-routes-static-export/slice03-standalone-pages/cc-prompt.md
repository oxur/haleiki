# CC Prompt — Arc 02 / Slice 03: about + attribution + search shell + 404

You are implementing the third slice of Arc 02 of the Haleiki tiny_http/minijinja
pivot, on branch `framework-base/tiny-http`. Work to the collaboration-framework's
ledger discipline: this slice is defined by its ledger, and every ledger row must
reach a final, evidence-backed status before you claim the slice done.

## Before you write code

1. Load the **collaboration-framework** skill and the **rust-guidelines** skill
   (anti-patterns first). Crate lints: `clippy::all` denied, `pedantic` warned,
   `unsafe_code` forbidden, edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, per-template contracts, scope, and
     the three decisions to confirm (content home for `intro`/`licenses`,
     `general_notice` home, search-shell empty-state).
   - [`ledger.md`](./ledger.md) — the eight rows F-1…F-8. The contract.
   - [`../arc-plan.md`](../arc-plan.md) and slices 01–02
     (`../slice01-site-index-home/`, `../slice02-browse-category/`) — the
     `SiteContext` extension + `page::render_*` patterns you build on.
   - Contracts: `templates/README.md` (about / attribution / search rows) and
     `templates/pages/{about,404,search,attribution}.jinja`.
   - The data to reuse: `tools/src/generator.rs` (`write_attribution_json` —
     the sources-by-project grouping to extract-and-share; `AttributionSourceEntry`),
     and `tools/src/render.rs` (`Profile`, `resolve_profile`, `SiteContext`).

## What to build

- `page::render_about`, `page::render_404`, `page::render_search` (empty
  `search` context: `query=""`, `results=[]`, `facets=[]`).
- `site.data.attribution` (extend `SiteContext` + `assemble_site_context`):
  extract the sources-by-project grouping out of `write_attribution_json` into a
  `pub(crate)` shared fn (keep `attribution.json` byte-identical — the existing
  generator test guards it), reshape map→list; take `source_count` +
  `general_notice` from it; take `intro` + `licenses[]` from a new
  `attribution:` section in `profile.yaml` + `render::Profile`.
- `page::render_attribution`.
- Add `attribution: {intro, licenses[]}` to `demo/profile.yaml` (the parser's
  older `Profile` ignores unknown fields, so `parse_corpus` is unaffected —
  confirm with a test, as slice-03 of Arc 01 did for its migration).

## Boundaries (do not cross in this slice)

- No static export / `_site/` output (slice 04); no real search — facets, JSON
  index, `search.js` behavior are **Arc 04** (render only the empty shell); no
  CLI, server; no template edits.
- Reuse `render::build_environment` (auto-escape per-`Environment`, Arc 01 v1.1).
- The attribution grouping extraction is a **refactor** — keep `attribution.json`
  byte-identical (existing generator test guards it), same as the three prior
  `build_*` extractions.
- Do not fix unrelated pre-existing lint debt.

## Definition of done

All of F-1…F-8 reach a final status with reproducible evidence. Run clippy **with
`--all-targets`**; build the no-Liquid needle by concatenation. In the closing
report, walk the ledger row by row — status + evidence, no silent drops, no
"deviations: none" prose. Name any uncertainty. Do **not** write
`cdc-verification.md`. Iteration budget: five.
