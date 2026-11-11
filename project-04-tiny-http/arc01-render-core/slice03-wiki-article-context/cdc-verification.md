# Slice 03 — wiki-article context · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per the collaboration-framework's `LEDGER-DISCIPLINE.md`
> Section A. Verdict: **CLOSED** · 2026-07-28. (Arc 01's composition reproduction
> and close are in [`../closing-report.md`](../closing-report.md).)

## Verifier and method

Verified by an independent Claude context (Cowork), not the implementing
context. Reproduced in a clean cloud build against the **full demo corpus** (91
sources + 186 concepts + taxonomy + the migrated profile), extracted from a
device tarball into a fresh container, `comrak 0.54` and the rest resolved from
crates.io, `cargo 1.95.0`. The full corpus matters here: the concept/source
renders exercise real cross-reference resolution (see-also, provenance,
breadcrumbs), which a fixture subset would not.

## Row-count check

Ledger opened with **9** rows (F-1…F-9); the closing report walks **9**; this
verification reproduces **9**. No silent drops.

## Per-row verification

The full `cargo test -p haleiki --bin haleiki` suite passed in the clean build:
**53 passed, 0 failed, 1 ignored** (the pre-existing `#[ignore]` demo-corpus
parser test). Each slice-03 row maps to a passing test:

| ID | CDC verdict | Strength | How reproduced |
|----|-------------|----------|----------------|
| F-1 | **confirmed** | reproduced | `parser::…test_parse_corpus_retains_body_for_real_demo_source_and_concept` — `vimalamitra`/`abelian-group` bodies retained and non-empty. |
| F-2 | **confirmed** | reproduced | `render::…test_resolve_profile_demo_yaml_migrated_and_parse_corpus_unaffected` — migrated `demo/profile.yaml` resolves (name, 4 trust levels, level_order, footer, 10 categories / 3 tiers) **and** `parse_corpus` still succeeds. |
| F-3 | **confirmed** | reproduced | `render::…test_assemble_site_context_shapes_for_known_demo_slug` — see_also / provenance / breadcrumbs in the contract shapes. |
| F-4 | **confirmed** | reproduced | `article::…test_assemble_concept_page_data_matches_abelian_group_frontmatter` (+ facts-ordering + title tests) — every field checked against the real card. |
| F-5 | **confirmed** | reproduced | `article::…test_render_concept_abelian_group_full_chrome` — trust bar, TOC (ids match anchors), breadcrumb, overline, prose with `data-math-style` spans, see-also grouped, sidebar (category/tier/subject-headings/aliases), and the provenance link `/source/group-mathematics/` all present. (Arc A-4.) |
| F-6 | **confirmed** | reproduced | `article::…test_render_source_vimalamitra_full_chrome` — breadcrumb, trust bar (profile default), overline, prose, see-also. (Arc A-5.) |
| F-7 | **confirmed** | reproduced | `article::…test_render_concept_escaping_holds_at_page_scale` — injected `<b>x</b>` alias renders as `&lt;b&gt;`, while `page.content` stays `\| safe` (real `<h2>`/math spans intact). (Arc A-6.) |
| F-8 | **confirmed** | reproduced | `article::…test_render_path_references_no_liquid_templates` — no Liquid extension in the render path (the self-grep needle is split to avoid matching itself). (Arc A-7.) |
| F-9 | **confirmed** | reproduced | `cargo clippy -p haleiki --all-targets -- -D warnings`: **0** findings in `article.rs` / `render.rs` / any slice-03-touched file; `cargo fmt --check` clean. (See F-9 note below.) |

## Boundary check

Slice 03 touched more files than the slice-doc named, but every extra touch is
benign and necessary — no scope creep:

- `validator.rs` (+2), `graph.rs` (+3): `body: String::new()` added to **test
  fixtures**, required by the new `SourcePage`/`ConceptCard` `body` field.
- `error.rs` (+4): a clean `Error::Render(#[from] minijinja::Error)` variant.
- `markdown.rs` (+5): `slugify` made `pub(crate)` for reuse by
  `article::category_slug` (one slugify convention — the slice-doc's decision #3).
- `generator.rs` (+25): `build_provenance_map` extracted as a `pub(crate)`
  shared fn; `write_provenance_json` now calls it. **Byte-identical output
  verified** — the existing `generator::…test_provenance_json_bidirectional`
  passes unchanged in my build.

Architecture spot-check: `render::SiteContext` reuses `graph::SeeAlsoEntry` /
`graph::Breadcrumb` / `generator::ProvenanceEntry` and
`assemble_site_context` reuses `graph.see_also` / `graph.breadcrumbs` /
`build_provenance_map` — no derivation fork. `SourcePageData` deliberately omits
`trust_level` so `base.jinja`'s `is defined` check falls through to the profile
default (a present-but-empty field would wrongly select an empty level) — correct
and subtle.

## Disclosures reviewed

- **`ConceptPageData.subcategory`** (beyond the README's literal field list):
  justified — `sidebar-taxonomy.jinja` renders a Subcategory row when present,
  and the real cards carry it. Enrichment, not spec drift.
- **The `--all-targets` pre-existing-debt category (CC's "28").** Reproduced the
  shape, not the full count: `cargo clippy -p haleiki --all-targets -- -D
  warnings` (default features) shows **0** findings in slice-03 code and **2**
  pre-existing unused-import findings in `tools/tests/demo_fetch.rs`. The larger
  set lives in the **feature-gated `demo/*` pipeline** (only compiled under
  `--all-features`), which this slice did not touch. Pre-existing, out of scope,
  correctly bubbled to arc-plan v1.5. I did not run the heavy `--all-features`
  compile to enumerate all 28 — the debt is definitionally outside slice-03 code
  and the operator is remediating it separately.

## Verdict

**Slice 03 is CLOSED.** 9 rows, all reproduced. Arc ledger row **A-3** is
satisfied (evidence: this file). This was Arc 01's final slice; its
concept/source renders reproduced A-4…A-7 at arc scale, so the arc closes — see
[`../closing-report.md`](../closing-report.md).

## What worked (Safety-II)

- Slice 03 landed a large surface (parser change, profile migration, shared-fn
  extraction, a new module, two end-to-end renders) in one clean pass. The
  planning payoff was concrete: the three sizing findings (shapes already match,
  no source `trust_level`, validator ignores the profile) all held, so nothing
  in the slice fought its plan.
- The `--all-targets` discipline (carried from slice 02 into F-9 and the prompt)
  worked exactly as intended — it kept the lint gate honest and surfaced the
  feature-gated debt as an explicit bubble-up instead of a latent surprise.
