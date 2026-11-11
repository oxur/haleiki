# Slice 03 — wiki-article context · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 9 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run. Not yet committed. This is
> Arc 01's third and final slice. 2026-07-28.

## Row-by-row walk

**F-1 — Parser retains the Markdown body.** `done`.
`cargo test parser::tests::test_parse_corpus_retains_body_for_real_demo_source_and_concept`
passes. Added `#[serde(skip)] pub body: String` to `SourcePage` and
`ConceptCard`; `parse_sources`/`parse_concepts` now assign it from
`parse_frontmatter`'s previously-discarded body return. Verified against real
demo files: `demo/sources/vimalamitra.md`'s body contains "Attributed works";
`demo/concepts/abelian-group.md`'s contains "Quick Definition". Mechanical
fallout: 7 pre-existing struct-literal test-fixture sites across `graph.rs`
(×3), `generator.rs` (×2), and `validator.rs` (×2) needed `body: String::new()`
added to keep compiling — no test logic touched; the full existing suite in
each of those files still passes.

**F-2 — `demo/profile.yaml` migrated; `parse_corpus` unaffected.** `done`.
`cargo test render::tests::test_resolve_profile_demo_yaml_migrated_and_parse_corpus_unaffected`
passes. `demo/profile.yaml` rewritten to the render `Profile` shape, mirroring
`templates/profile.sample.yaml`'s structure but with two corrections against
the *real* demo corpus rather than the sample's illustrative one:
`taxonomy.categories`/`tiers` copied from `demo/taxonomy.yaml` (10 categories,
3 tiers — the sample has fewer), and the `about_body` source-project list
corrected from the sample's "Wikipedia, Wikibooks, and Rigpa Wiki" to
"Wikipedia and Rigpa Wiki" after grepping every `demo/sources/*.md`'s
`original_source.project` and finding only `en.wikipedia.org` and
`www.rigpawiki.org` — no Wikibooks content exists in this corpus. The old
`parser::Profile` (all fields `#[serde(default)]`) deserializes the new shape
leniently, unaffected — exactly the slice-doc's sizing-note prediction #3,
now confirmed rather than assumed.

**F-3 — `site.data` assembly (`see_also`/`provenance`/`breadcrumbs`).**
`done`. `cargo test render::tests::test_assemble_site_context_shapes_for_known_demo_slug`
passes. `render::SiteContext` + `render::assemble_site_context()` reuse
`graph.see_also`/`graph.breadcrumbs` and a newly-extracted
`generator::build_provenance_map` verbatim — no reshaping, no logic fork with
the JSON-writing build path. Tested against real, specific cross-references
predicted by reading `graph.rs`'s resolution rules *before* writing the test
(concept-first resolution correctly falling back to a source page when no
concept shares that slug); all predictions matched on the first run.

**F-4 — Concept `page.data` assembly, with one disclosed enrichment.**
`done`. Three tests pass:
`test_assemble_concept_page_data_matches_abelian_group_frontmatter`,
`test_concept_facts_includes_all_three_types_in_order_when_present`,
`test_concept_title_appends_disambiguator_when_present`.

- *Enrichment, disclosed:* `ConceptPageData` carries a `subcategory` field not
  named in `templates/README.md`'s per-page summary table. `partials/
  sidebar-taxonomy.jinja` explicitly reads and conditionally renders
  `page.data.subcategory`, and the real `abelian-group.md` has one ("algebra")
  — using data the template already supports and the corpus already has
  seemed better than leaving a working row permanently blank. Flagged here
  rather than added silently.
- `page.data.type` is omitted entirely (no field on the struct at all, so
  `page.data.type` reads `Undefined` in minijinja and the sidebar's
  conditional row correctly does not render) — per decision #2's proposed
  default.
- `facts[]`: the non-empty subset of `stable_since`/`extraction_confidence`/
  `last_reviewed`, in that label order, per decision #1's proposed default.
  Verified against the real card (2 of 3 present) and a synthetic card
  (all 3 present, confirming order).

**F-5 — Real concept renders end-to-end with full chrome.** `done`.
`cargo test article::tests::test_render_concept_abelian_group_full_chrome`
passes. `article::render_concept`, built on `env: Environment` from
`render::build_environment` (reused, per arc v1.1 — no bare `Environment`
constructed). Every named chrome marker present: trust bar label, left-nav
TOC entries whose `href="#id"` match real `<h2 id="...">` anchors, breadcrumb,
title + overline, prose with `data-math-style` spans, see-also cards under a
"Related" heading, sidebar info card with alias/subject-heading text, and a
"Derived From" provenance link to the real source URL.

**F-6 — Real source renders end-to-end.** `done`.
`cargo test article::tests::test_render_source_vimalamitra_full_chrome`
passes. `SourcePageData` deliberately has no `trust_level` field — confirmed
this makes `base.jinja`'s `is defined` check fall through to
`site.data.profile.trust.default` (rendered as `trust-bar trust-bar--credible`)
rather than wrongly satisfying `is defined` with a blank value. Breadcrumb,
overline, prose (a real `<strong>` from the body), and see-also all present.

**F-7 — Escaping holds at full-page scale.** `done`.
`cargo test article::tests::test_render_concept_escaping_holds_at_page_scale`
passes. Injects `<b>x</b>` as an extra alias on a cloned `abelian-group` card
and re-renders through the complete `render_concept` pipeline (not an
isolated single-field test like slice 01/02's). The injected alias is escaped
(`&lt;b&gt;x&lt;`, no raw `<b>x</b>`) in the same render where `page.content`'s
real `<h2 id="quick-definition">` and `data-math-style="inline"` survive
unescaped — the `| safe`/auto-escape split holds under full-page composition,
not just in isolation.

**F-8 — No Liquid/Cobalt in the render path.** `done`. `cargo test
article::tests::test_render_path_references_no_liquid_templates` passes.
`include_str!`-greps `article.rs` and `render.rs` — the only two modules that
resolve templates for the article path — for the Liquid file extension; both
clean. Worth naming for whoever writes the next such check: the first version
of this test failed against *itself*, because its own assertion string
literally contained the substring being searched for (`include_str!` pulls in
the test's own source). Fixed by building the needle from concatenated
fragments rather than one literal.

**F-9 — New code clean under `--all-targets`; a new pre-existing-debt
category found and disclosed, not fixed.** `done`. `cargo fmt --all --
--check` is clean. `cargo clippy --all-targets --all-features --workspace --
-D warnings`, filtered to every file this slice touched (`article.rs`,
`render.rs`, `markdown.rs`, `parser.rs`, `graph.rs`, `validator.rs`,
`error.rs`, `main.rs`) → **0** findings. Two real findings during development
were fixed, not suppressed: a `clippy::doc_markdown` (missing backticks) and
the F-8 self-referential grep bug above.

**Finding, disclosed rather than fixed:** running clippy *with*
`--all-targets` — per the explicit slice-02 lesson, and per this slice's own
cc-prompt instruction to use it — surfaced **28 findings that predate this
slice**, entirely in `demo/{convert,media,clean,frontmatter,rewrite}.rs` and
`tools/tests/demo_fetch.rs`, plus two `assert!(x == y)` in `generator.rs`'s
own pre-existing test (not code this slice wrote — its only `generator.rs`
edit was the F-3 provenance-map extraction). Neither prior remediation round
(the earlier "fix all the problems" pass, nor arc-plan v1.2's original find)
ran clippy with `--all-targets`, so this category was invisible until now.
Per this slice's explicit boundary — "do not fix the pre-existing lint debt …
the operator is handling it separately" — left untouched and disclosed here
rather than silently fixed (scope creep past the boundary) or silently missed
(the slice-02 failure mode, repeated).

## Silent-drop check

Ledger opened with 9 rows (F-1…F-9); this report addresses 9. No missing rows.

## Bubble-up to arc-plan.md

Two findings to carry forward, added to arc-plan.md's version history (v1.5):

1. **A new pre-existing-debt category**: 28 `--all-targets`-only clippy
   findings, entirely outside this slice's code (see F-9 above). Same
   disposition as the earlier lint-debt finding: route to a cleanup pass, and
   note that `--all-targets` should be part of whatever gate the operator
   ultimately wires into CI, not just `--all-features`.
2. **Arc 01 is now composition-ready.** With slice 03 delivering the render
   pipeline, a real demo concept and a real demo source both render
   end-to-end with full chrome via minijinja (F-5/F-6), escaping holds at
   page scale (F-7), and the path uses no Liquid (F-8) — these are exactly
   arc-plan.md's A-4/A-5/A-6/A-7 composition claims, demonstrated here at
   slice scale. Per `LEDGER-DISCIPLINE.md` Section B, arc composition rows
   must still be **independently reproduced at arc scale** by whoever closes
   the arc (not inherited from this slice's attestation) — this report is
   evidence for that reproduction, not a substitute for it.

## What was NOT done (explicitly, per the "no silent drops" rule)

- No CLI/route wiring: nothing in `main.rs`/`cli.rs` calls `render_concept`/
  `render_source`. Routing is Arc 02; the server is Arc 03 (the pivot's Arc 03,
  distinct from this Arc 01's slice 03).
- No home/browse/category/about/attribution/search/news pages — Arc 02+.
- Source-page provenance display: `source.jinja` doesn't include the sidebar
  info card, so "Concepts in This Source" doesn't render on source pages
  today. This is a template-structure observation carried over verbatim from
  the slice-doc's own "Out" section — not this slice's job, not newly found.
- The pre-existing clippy debt named in F-9 (both the earlier category and
  the newly-found `--all-targets` one) was not fixed.
- Nothing has been committed to git. All changes are in the working tree.

## Independent verification status

Not yet performed. Per `LEDGER-DISCIPLINE.md`, this closing report is CC's
`attested` claim; a CDC-equivalent pass (fresh context or operator, re-running
each row's Verify command against the actual artifacts) is required before any
row's evidence strengthens to `reproduced`, before this slice can be
considered closed rather than proposed-done, and before Arc 01's own
composition rows can be reproduced and the arc closed. Per the cc-prompt's
explicit instruction, this report does **not** include `cdc-verification.md`
— that file is the independent reviewer's to write.
