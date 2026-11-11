# CC Prompt — Arc 02 / Slice 02: browse + category

You are implementing the second slice of Arc 02 of the Haleiki tiny_http/minijinja
pivot, on branch `framework-base/tiny-http`. Work to the collaboration-framework's
ledger discipline: this slice is defined by its ledger, and every ledger row must
reach a final, evidence-backed status before you claim the slice done.

## Before you write code

1. Load the **collaboration-framework** skill and the **rust-guidelines** skill
   (anti-patterns first). Crate lints: `clippy::all` denied, `pedantic` warned,
   `unsafe_code` forbidden, edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, the two groupings, data sources,
     scope, and the decisions to confirm (grouping semantics, description,
     empty-tier handling, ordering).
   - [`ledger.md`](./ledger.md) — the seven rows F-1…F-7. The contract.
   - [`../arc-plan.md`](../arc-plan.md) and slice 01
     (`../slice01-site-index-home/`) — the `SiteContext` extension pattern and
     `build_home_categories` / `page::render_home` you build on.
   - The consumers/contracts: `templates/README.md` (browse / category rows) and
     `templates/pages/browse.jinja` / `templates/pages/category.jinja`.
   - The data: `tools/src/graph.rs` (`category_index`, `CategoryIndexEntry`,
     `SubcategoryEntry`, the `"_uncategorized"` bucketing) and `demo/taxonomy.yaml`.

## What to build

- **`site.data.browse_categories`** (extend `SiteContext` + `assemble_site_context`),
  grouped **by subcategory**, taxonomy order, `"_uncategorized"` → unlabeled
  group, articles `{title, url, type}` concepts-then-sources alphabetical.
- **`CategoryPageData`** + `build_category_page_data(corpus, entry, profile)`,
  grouped **by tier** (taxonomy tier order, non-empty only, `modifier` = tier
  slug), `subcategories[]` (excluding `"_uncategorized"`), and the three counts.
  Article `description` omitted this slice.
- **`page::render_browse`** and **`page::render_category`** (one render per
  category), via `render::build_environment`. Set `page.title` so the breadcrumb
  fallback is correct.
- Factor a shared article-ref resolver rather than a second title/url convention
  (reuse slice 01's logic).

## Boundaries (do not cross in this slice)

- No `about`/`attribution`/`search`/`404` (slice 03); no static export /
  enumerate-and-write `_site/` (slice 04); no CLI, server, or search index.
- Reuse `render::build_environment` (auto-escape per-`Environment`, Arc 01 v1.1).
- Do not add article `description` text or fix unrelated pre-existing lint debt.
- Keep the grouping in **Rust** — the templates must stay declarative (no
  in-template de-duplication), per the README.

## Definition of done

All of F-1…F-7 reach a final status with reproducible evidence. Run clippy **with
`--all-targets`**. Build the no-Liquid test's needle by concatenation (don't write
the literal extension in the test source). In the closing report, walk the ledger
row by row — status + evidence, no silent drops, no "deviations: none" prose. Name
any uncertainty (the grouping semantics especially, if the templates surprise
you). Do **not** write `cdc-verification.md`. Iteration budget: five.
