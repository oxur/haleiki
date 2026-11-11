# CC Prompt — Arc 02 / Slice 01: site-index + home

You are implementing the first slice of Arc 02 of the Haleiki tiny_http/minijinja
pivot, on branch `framework-base/tiny-http`. Work to the collaboration-framework's
ledger discipline: this slice is defined by its ledger, and every ledger row must
reach a final, evidence-backed status before you claim the slice done.

## Before you write code

1. Load the **collaboration-framework** skill (ledger discipline, peer frame,
   anti-degradation) and the **rust-guidelines** skill (`assets/ai/rust/SKILL.md`
   + guides; anti-patterns first). Crate lints: `clippy::all` denied, `pedantic`
   warned, `unsafe_code` forbidden, edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, the template contract, scope, and
     the three decisions to confirm at start (article selection/order, teaser,
     module home).
   - [`ledger.md`](./ledger.md) — the seven rows F-1…F-7. The contract; raise an
     amendment request if a row is wrong.
   - [`../arc-plan.md`](../arc-plan.md) — the arc capability and slice breakdown.
   - Arc 01 (closed), which you build on: `../../arc01-render-core/closing-report.md`
     and the code it delivered — `render::build_environment` /
     `render::SiteContext` / `assemble_site_context` / `resolve_profile`
     (`tools/src/render.rs`), `article::render_concept` / `render_source`
     (`tools/src/article.rs`), `markdown::{render_markdown, slugify}`.
   - The consumers/contracts: `templates/README.md` (home row) and
     `templates/pages/home.jinja`.
   - The derivations you reuse: `tools/src/graph.rs` (`category_index`,
     `CategoryIndexEntry`) and `tools/src/generator.rs` (`write_featured_json` —
     the featured selection to extract-and-share).
   - Real targets: `demo/taxonomy.yaml`, `demo/profile.yaml`, and a couple of
     `demo/concepts` / `demo/sources` to eyeball.

## What to build

- `site.data.stats` = `{sources, concepts, categories}` counts.
- `site.data.featured`: extract the top-N-by-in-degree selection out of
  `generator::write_featured_json` into a `pub(crate) build_featured` (keep the
  JSON output byte-identical — the existing generator test must still pass), and
  feed it into the render context. No teaser this slice.
- `site.data.home_categories`: from `graph.category_index`, in
  `profile.taxonomy.categories` order; per category `{slug, name, count,
  description, articles[≤3]{title, url}}` with concept/source url + title
  resolution.
- Extend `render::SiteContext` + `assemble_site_context` with these three, built
  once. Add `render_home` (confirm module home — proposed `page.rs`) rendering
  `pages/home.jinja` via `render::build_environment`.

## Boundaries (do not cross in this slice)

- No `browse`/`category` pages or their fuller grouping (slice 02); no
  `about`/`attribution`/`search`/`404` (slice 03); no static export / `_site/`
  (slice 04).
- No CLI wiring, no server, no search index.
- Reuse `render::build_environment` for any `Environment` (auto-escape is
  per-`Environment`, Arc 01 v1.1). Do not construct a bare `Environment`.
- The `build_featured` extraction is a **refactor** — keep `featured.json`
  byte-identical (the existing `generator` test guards it), exactly as slice-03's
  `build_provenance_map` extraction did.
- Do not fix unrelated pre-existing lint debt.

## Definition of done

All of F-1…F-7 reach a final status with reproducible evidence (each `Verify` is
a command a reviewer can re-run). Run clippy **with `--all-targets`** (the recurring
lesson — a lint can hide in test code otherwise). In the closing report, walk the
ledger row by row — status + evidence, no silent drops, no "deviations: none"
prose. Name any uncertainty. Do **not** write `cdc-verification.md`. Iteration
budget: five; if you hit it, stop and flag for re-scoping.
