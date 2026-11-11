# CC Prompt — Arc 01 / Slice 03: wiki-article context

You are implementing the third and final slice of Arc 01 of the Haleiki
tiny_http/minijinja pivot, on branch `framework-base/tiny-http`. Work to the
collaboration-framework's ledger discipline: this slice is defined by its
ledger, and every ledger row must reach a final, evidence-backed status before
you claim the slice done.

## Before you write code

1. Load the **collaboration-framework** skill (ledger discipline, peer frame,
   anti-degradation) and the **rust-guidelines** skill (`assets/ai/rust/SKILL.md`
   + guides; anti-patterns first). Crate lints: `clippy::all` denied,
   `pedantic` warned, `unsafe_code` forbidden, edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, scope, the sizing note, and the
     three decisions to confirm at start (`facts[]`, `page.data.type`,
     `category_slug`).
   - [`ledger.md`](./ledger.md) — the nine rows F-1…F-9. The contract. Raise an
     amendment request if a row is wrong; don't silently ignore it.
   - [`../arc-plan.md`](../arc-plan.md) (esp. v1.1 auto-escape, v1.2/v1.3
     carried items: body retention, `demo/profile.yaml` migration) and the two
     prior `cdc-verification.md` files (what slices 01–02 delivered:
     `render::build_environment`, `render::Profile`/`resolve_profile`,
     `markdown::render_markdown`).
   - The consumers and their contracts: `templates/README.md`, and
     `templates/pages/concept.jinja`, `templates/pages/source.jinja`, and the
     partials they include — `partials/{trust-bar,left-nav-toc,breadcrumb,
     see-also,provenance,sidebar-taxonomy}.jinja` and `macros.jinja`.
   - The existing derivations you reuse: `tools/src/graph.rs`
     (`see_also`, `breadcrumbs`, the `SeeAlsoEntry`/`Breadcrumb` shapes) and
     `tools/src/generator.rs` (`write_provenance_json` — the provenance map
     logic to extract-and-share).
   - Real targets: `demo/concepts/abelian-group.md`,
     `demo/sources/vimalamitra.md`, `demo/taxonomy.yaml`, `demo/profile.yaml`.

## What to build

The article-render pipeline (see slice-doc for the full list):

- Parser: retain the body (`#[serde(skip)] pub body: String` on `SourcePage` /
  `ConceptCard`, populated from the discarded `_body`).
- Migrate `demo/profile.yaml` to the render `Profile` shape (mirror
  `templates/profile.sample.yaml`, taxonomy from `demo/taxonomy.yaml`); keep a
  guard test that `parse_corpus` and `resolve_profile` both still succeed.
- Assemble `site.data` (`profile`, `see_also`, `provenance`, `breadcrumbs`) once
  from `Corpus` + `HaleikiGraph` + `Profile`. Reuse `graph.see_also`/
  `graph.breadcrumbs`; extract the provenance-map builder out of
  `generator.rs` into a shared fn (no logic fork with the JSON writer).
- Assemble concept and source `page.data` per the README contract; render
  `pages/concept.jinja` / `pages/source.jinja` via `render::build_environment`
  from the `demo/` corpus.

## Boundaries (do not cross in this slice)

- No CLI/route wiring, no server, no static export (Arcs 02–03). Deliver the
  pipeline as library functions + tests, as slices 01–02 did.
- No other page types (home/browse/category/about/attribution/search/news).
- Reuse `render::build_environment` for any `Environment` (auto-escape is
  per-`Environment`, arc v1.1). Do **not** construct a bare `Environment`.
- Do not fix the pre-existing lint debt (arc v1.2) — the operator is handling it
  separately; keep your new code clean and leave the rest.
- When you touch `generator.rs` to extract the provenance builder, keep its JSON
  output byte-identical (the extraction is a refactor, not a behavior change) —
  the existing `generator.rs` tests must still pass.

## Definition of done

All of F-1…F-9 reach a final status with reproducible evidence (each `Verify` is
a command a reviewer can re-run). Run clippy **with `--all-targets`** — the
slice-02 miss was a lint hiding in test code that only `--all-targets` surfaces.
In the closing report, walk the ledger row by row — status + evidence, no silent
drops, no "deviations: none" prose. Name any uncertainty. Do **not** write
`cdc-verification.md`. Iteration budget: five; if you hit it, stop and flag for
re-scoping (and consider the source-render split named in the slice-doc).
