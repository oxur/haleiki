# Slice 03 — wiki-article context

> Plan-of-record for the third and final slice of Arc 01 (Render Core). Open
> set: this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** (planned, not implemented) · 2026-07-28.

---

## Goal

Assemble the full per-page context and render a **fully-chromed concept page and
source page end-to-end from the real demo corpus**, through minijinja. This is
the slice that ties slice 01 (the `Environment` + resolved `site.data.profile`)
and slice 02 (`render_markdown` → `page.content` + `toc`) into a working
article-render pipeline, and it produces the evidence the arc's composition rows
(A-4…A-7) close against.

After this slice, Arc 01 is composition-ready: a real concept and a real source
render with their complete chrome — trust bar, TOC, breadcrumb, see-also,
provenance, taxonomy info card, math — from `demo/` alone.

## Sizing note (why this is one slice, not two)

Three findings during planning collapsed the risk that made this look like it
might split:

1. **The derived-data shapes already match the template contracts.**
   `graph::SeeAlsoEntry {slug,title,relationship,url}`, `graph::Breadcrumb
   {label,url}`, and the provenance links `{slug,title,url}` are exactly what
   `see-also.jinja` / `breadcrumb.jinja` / `provenance.jinja` consume. So this
   slice *assembles* existing graph outputs into the render context; it does not
   reshape them.
2. **Demo sources carry no `trust_level`** (0 of 91) → they fall back to the
   profile default via `base.jinja`. No `SourcePage` trust field is needed.
3. **The validator ignores the profile's `personality`/`page_types`/
   `relationships`** (verified), so migrating `demo/profile.yaml` to the render
   `Profile` shape does not disturb validation — the parser deserializes it
   leniently and the validator doesn't read those fields.

Escape hatch (kept, per "if it won't fit, it was two slices"): if implementation
shows this exceeds one context with iteration headroom, split the **source**
render into a follow-on slice and close the arc after it — record the split as an
arc-plan change.

## Scope

### In

- **Parser body retention.** Add `#[serde(skip)] pub body: String` to
  `SourcePage` and `ConceptCard`; populate it from `parse_frontmatter`'s body
  return (currently discarded as `_body`) in `parse_sources`/`parse_concepts`.
- **`demo/profile.yaml` migration.** Rewrite it from the Cobalt-era parser shape
  to the render `Profile` shape (mirror `templates/profile.sample.yaml`:
  `name`, `name_short`, `tagline`, `hero_subtitle`, `about_*`, `content_notice`,
  `trust{enabled,default,level_order,levels}`, `canon{enabled:false}`,
  `taxonomy` from `demo/taxonomy.yaml`, `footer`). Keep `trust.enabled: true`,
  `trust.default: credible`. Guard with a test that `parse_corpus("demo")` still
  succeeds and `resolve_profile("demo/profile.yaml")` yields a valid `Profile`.
- **Site-context assembly.** A function that builds the `site.data` the article
  templates read, once, from `Corpus` + `HaleikiGraph` + resolved `Profile`:
  `profile`, `see_also` (map slug→`Vec<SeeAlsoEntry>`), `provenance`
  (map slug→`{links:[…]}`), `breadcrumbs` (map slug→`Vec<Breadcrumb>`). Reuse
  `graph.see_also` / `graph.breadcrumbs`; extract the provenance-map builder out
  of `generator.rs`'s `write_provenance_json` into a shared fn returning the map
  (both the JSON writer and this consume it — no logic fork).
- **Concept `page.data` assembly** per the README contract: `concept`,
  `disambiguator`, `category`, `category_slug` (slugified category),
  `tier`, `page_type:"concept"`, `trust_level`, `subject_headings[]`,
  `aliases[]`, `facts[]{label,value}` (see decision below), `toc[]{id,label}`
  (from `render_markdown(card.body)`), plus `page.content` (its html) and
  `page.title`.
- **Source `page.data` assembly** (subset `source.jinja` uses): `category`,
  `category_slug`, `tier`, `page_type:"source"`, `trust_level` (omitted →
  profile default), `page.content` (`render_markdown(source.body)`),
  `page.title`.
- **The render pipeline.** `render_concept` / `render_source` (or one
  `render_article`) that, given the assembled `site` context and a slug, builds
  the `page` context and renders `pages/concept.jinja` / `pages/source.jinja`
  via `render::build_environment` (reuse it — auto-escape is per-`Environment`,
  arc v1.1). Tests drive it from the `demo/` corpus.

### Out

- No CLI/route wiring, no server, no static export (Arcs 02–03). This slice
  delivers the render pipeline as a library capability with tests, matching
  slices 01–02.
- No home/browse/category/about/attribution/search/news pages (Arc 02+).
- No fix to source-page provenance display (source.jinja doesn't include the
  info card, so "Concepts in This Source" doesn't render on sources today — a
  template observation, not this slice's job).

## Decisions to confirm at slice start

- **`facts[]` composition.** Which concept-card frontmatter becomes info-card
  fact rows. Proposed default: the non-empty subset of `stable_since` →
  "Stable since", `extraction_confidence` → "Confidence", `last_reviewed` →
  "Last reviewed". (Values render `| safe`; they are plain strings.)
- **`page.data.type`.** The sidebar renders a "Type" row only if present.
  Proposed default: omit it (no row) unless a meaningful value is identified.
- **`category_slug`.** Demo categories are already kebab slugs; proposed:
  `slugify(category)` (idempotent) — reuse `markdown::slugify` (make it
  `pub(crate)`) rather than duplicating.

## Verification approach

Unit + render tests reading the `demo/` corpus: body retention; profile-migration
guard (`parse_corpus` + `resolve_profile` both succeed); site-context shapes for a
known slug; concept `page.data` correctness; full concept render (chrome
assertions); full source render; page-scale escaping; and a no-Liquid check on
the render path. `cargo clippy -p haleiki --all-targets -- -D warnings` (note the
`--all-targets` — the slice-02 lesson) + `cargo fmt --check` clean on new code.

## Exit criteria

Ledger rows F-1…F-9 reach a final status. In short: the parser retains bodies;
`demo/profile.yaml` is migrated and both profile consumers cope; `site.data`
assembles in the contract shapes; concept and source `page.data` are correct; a
real demo concept and a real demo source each render end-to-end with full chrome
via minijinja; escaping holds at page scale; the render path uses no Liquid; and
new code is clippy(`--all-targets`)/fmt-clean.

## Relationship to the arc close

This is Arc 01's last slice. Its concept/source renders are the demonstrations
the arc composition rows reproduce: **A-4** (concept end-to-end), **A-5** (source
end-to-end), **A-6** (escaping at arc scale), **A-7** (no Liquid in the article
path). After this slice CDC-closes, the arc closes: reproduce A-4…A-7 via this
render path, write `arc01-render-core/closing-report.md`, and bubble up to
`project-plan.md`.

## Notes

- Context contract authority: `templates/README.md` (concept/source rows).
- Carried-forward items now addressed here (arc-plan v1.2/v1.3): parser body
  retention and `demo/profile.yaml` migration.
