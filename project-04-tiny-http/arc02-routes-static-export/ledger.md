# Arc ledger

> Extracted without row changes on 2026-11-11 from the August plan. Existing dispositions/evidence are retained, not freshly reproduced by the migration.

Opens here; closes (per-row walk) in `closing-report.md` at arc close. Class (a)
= slices closed (attested by pointer); class (b) = composition, **reproduced at
arc scale** (run the export, inspect the `_site/` tree and key pages); class (c)
= bubble-up findings dispositioned.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| B-1 | Slice 01 (site-index + home) closed | ptr: `slice01-site-index-home/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 01 CDC-closed 2026-07-28, 7/7 rows |
| B-2 | Slice 02 (browse + category) closed | ptr: `slice02-browse-category/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 02 CDC-closed 2026-07-28, 7/7 rows (incl. _uncategorized fix) |
| B-3 | Slice 03 (about + attribution + search shell + 404) closed | ptr: `slice03-standalone-pages/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 03 CDC-closed 2026-07-28, 8/8 rows |
| B-4 | Slice 04 (static export writer) closed | ptr: slice04 cdc-verification | correctness | arc-plan | done | reproduced — slice 04 CDC-closed 2026-07-28, 8/8 rows |
| B-5 | Static export produces a complete `_site/` from the demo corpus: `index.html` for home, browse, every category, every concept, every source, about, attribution, search, and `404.html` — each at its contract path | reproduce at arc scale: run the export, assert the file tree + spot-render key pages | serious | arc-plan | done | reproduced at arc scale — real export: 293-file `_site/` (6 singletons + 10 cat + 91 src + 186 concepts) at contract paths |
| B-6 | `assets/` (css, js, fonts, icons, vendor) copied into `_site/assets/` | reproduce: assert the asset tree under `_site/assets/` | serious | arc-plan | done | reproduced — `_site/assets/{css,fonts,icons,images,js,vendor}` present; `ai`/`mockups` excluded |
| B-7 | Category grouping is resolved in Rust: browse + category pages render `groups[]` with no in-template de-duplication; a spot category's groups match the corpus | reproduce: inspect a rendered category/browse page + grep templates for absence of dedup logic | correctness | arc-plan | done | reproduced — grouping delivered slice 02; present in the written browse/category pages |
| B-8 | Key internal links resolve within `_site/`: home → category → article paths, and category → article paths, all exist as files | reproduce: follow a sample link chain against the `_site/` tree | serious | arc-plan | done | reproduced — home→category→article + concept breadcrumb/see-also targets all exist as files |

_(Class-(c) bubble-up rows are added as slices surface findings.)_
