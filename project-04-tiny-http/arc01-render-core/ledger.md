# Arc ledger

> Extracted without row changes on 2026-11-11 from the August plan. Existing dispositions/evidence are retained, not freshly reproduced by the migration.

Opens here; closes (per-row walk) in `closing-report.md` at arc close. Class (a)
= slices closed (attested by pointer); class (b) = composition, **reproduced at
arc scale** (actually render real demo pages and inspect); class (c) = bubble-up
findings dispositioned. Rows accrue as slices close.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| A-1 | Slice 01 (minijinja shell) closed | ptr: `slice01-minijinja-shell/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 01 CDC-closed 2026-07-28, 8/8 rows reproduced |
| A-2 | Slice 02 (markdown + TOC) closed | ptr: `slice02-markdown-toc/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 02 CDC-closed 2026-07-28, 9/9 rows (F-9 returned+fixed) |
| A-3 | Slice 03 (wiki-article context) closed | ptr: slice03 cdc-verification | correctness | arc-plan | done | reproduced — slice 03 CDC-closed 2026-07-28, 9/9 rows (see slice03 cdc-verification.md) |
| A-4 | A real demo **concept** page renders end-to-end with full chrome (trust bar, taxonomy sidebar, see-also, provenance, breadcrumb, TOC) via the render path | reproduce at arc scale: render a named demo concept, inspect HTML against the mockup | serious | arc-plan | done | reproduced at arc scale — `test_render_concept_abelian_group_full_chrome` green in clean full-corpus build |
| A-5 | A real demo **source** page renders end-to-end with full chrome | reproduce at arc scale: render a named demo source, inspect | serious | arc-plan | done | reproduced at arc scale — `test_render_source_vimalamitra_full_chrome` green |
| A-6 | Escaping is correct: only `page.content`/`profile.about_body` (and other contract fields) are `\| safe`; all other fields auto-escape | reproduce: inject an HTML control string into an escaped field, confirm it is escaped | correctness | arc-plan | done | reproduced — `test_render_concept_escaping_holds_at_page_scale` green (injected alias escaped; content intact) |
| A-7 | The article path uses **no** Liquid/Cobalt template for rendering | `grep` the render path; Liquid article render is gone | correctness | arc-plan | done | reproduced — `test_render_path_references_no_liquid_templates` green |

_(Class-(c) bubble-up rows are added as slices surface findings.)_
