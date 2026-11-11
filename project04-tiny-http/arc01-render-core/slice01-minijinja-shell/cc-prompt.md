# CC Prompt — Arc 01 / Slice 01: minijinja shell

You are implementing one slice of the Haleiki tiny_http/minijinja pivot, on
branch `framework-base/tiny-http`. Work to the collaboration-framework's ledger
discipline: this slice is defined by its ledger, and every ledger row must reach
a final, evidence-backed status before you claim the slice done.

## Before you write code

1. Load the **collaboration-framework** skill (ledger discipline, peer frame,
   anti-degradation) and the **rust-guidelines** skill (`assets/ai/rust/SKILL.md`
   and guides — especially anti-patterns). This crate lints hard: `clippy::all`
   is denied and `pedantic` is warned; `unsafe_code` is forbidden. Edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, scope in/out, exit criteria.
   - [`ledger.md`](./ledger.md) — the eight rows F-1…F-8 you are closing. This is
     the contract. If a row is wrong or impossible, raise an amendment request;
     do not quietly ignore it.
   - [`../arc-plan.md`](../arc-plan.md) and
     [`../../design-delta.md`](../../design-delta.md) — the arc and the pivot.
   - **`workbench/Hale'iki Wiki Design Refactor/templates/README.md`** — the
     binding context contract. Also read `templates/base.jinja`,
     `templates/partials/topnav.jinja`, `templates/partials/footer.jinja`,
     `templates/pages/404.jinja`, and `templates/profile.sample.yaml`.

## What to build

Stand up the minijinja rendering spine in the `haleiki` crate (`tools/`):

- Add `minijinja` (with the `loader` feature for `path_loader`). Introduce a
  `render` module owning the `Environment`, the `path_loader("templates")`
  loader, and the context types.
- **Confirm with the operator** where `templates/` and the refactor `assets/`
  should live in the repo before moving them (proposed default: repo-root
  `templates/`, consolidated `assets/`). Then relocate them there.
- Resolve `profile.yaml` into a typed context that serializes to the exact
  `site.data.profile` shape in the README / `profile.sample.yaml`
  (`name`, `name_short`, `tagline`, `hero_subtitle`, `about_intro`,
  `about_body[]`, `content_notice`, `trust{…}`, `canon{enabled}`,
  `taxonomy{…}`, `footer{columns[],copyright,meta[]}`).
- Wire the asset set: unified `haleiki.css` in; old modular CSS out of this path;
  add `search.js`, `toc-scrollspy.js`, `browse-scrollspy.js`; keep `theme.js`
  (+ `math.js`/KaTeX). Confirm the 8 refactor woff2 are present and referenced.
- Establish the `| safe` vs auto-escape discipline (only pre-rendered HTML
  fields are `| safe`; here that is `profile.about_body`) and the `active_nav`
  block-capture that `base.jinja` uses to reach the included topnav.
- Render `pages/404.jinja` as the smoke target and cover it with tests.

## Boundaries (do not cross in this slice)

- Do **not** modify `parser.rs`, `graph.rs`, or `validator.rs`.
- Do **not** implement markdown rendering (slice 02), article `page.data` /
  see-also / provenance (slice 03), any non-404 route (Arc 02), the server or
  static export (Arcs 02–03), search (Arc 04), or news (Arc 05).
- Do **not** invent context shapes — satisfy `templates/README.md`.
- Do **not** delete the Cobalt/Liquid files repo-wide yet (a later arc); this
  slice only stops *using* Liquid for the smoke route.

## Definition of done

All of F-1…F-8 in the ledger reach a final status with reproducible evidence
(the `Verify` column is a command a reviewer can re-run). In the closing report,
walk the ledger **row by row** — status + evidence for each — with no silent
drops and no prose "deviations: none" summary. Name any uncertainty honestly.
Iteration budget: five. If you hit it without convergence, stop and flag the
slice for re-scoping rather than grinding a sixth pass.
