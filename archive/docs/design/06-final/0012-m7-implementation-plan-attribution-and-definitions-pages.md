---
number: 12
title: "Attribution and Definitions Pages"
author: "the demo"
component: All
tags: [change-me]
created: 2026-05-08
updated: 2026-05-08
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# M7 Implementation Plan: Attribution and Definitions Pages

Pre-implementation planning pass for milestone M7 of Phase 6. Same pattern as M1–M6 plans: draft the implementation plan as a design doc, Duncan and I will review, then you execute.

**This pass is plan-only — do not implement the templates yet.** Land the plan doc, report what's in it, surface design decisions you've made or want to surface for review.

---

## Reference docs

- `docs/design/06-final/0006-...site-build...project-plan.md` § M7 — milestone scope and task breakdown
- `docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md` § 7 — Definitions page (purpose, content requirements, linking, URL convention). **The authoritative reference for the about/definitions page**, including the requirement that trust-level descriptions clearly state they describe *sourcing and evidence basis* of the content, not a value judgment on the subject.
- `docs/design/05-active/0002-haleiki-demo-site-design-document.md` § 8 — Licensing & Attribution (automated attribution page). **The authoritative reference for the attribution page** structure.
- `demo/media/manifest.json` — the image manifest produced by the demo pipeline. Contains per-image `local_path`, `original_url`, `commons_filename`, `license`, `author`, `caption`, `source_article`, `format`, `size_bytes`. **Verify it exists and the shape matches the spec** before authoring.
- `demo/sources/*.md` frontmatter — each carries `original_source` block with `title`, `project`, `url`, `license`, `fetched_at`, `revision_id`. This is the per-source-page upstream attribution.
- `demo/concepts/*.md` frontmatter — concept cards may have `external_references` array (per v1.3 schema) for non-source-page citations. Rare in practice but should be aggregated into attribution if present.
- `demo/_build/_data/profile.json` — already populated with `trust.levels.<level>.{label, description}`, `trust.level_order`, `canon.enabled`, etc. The about page reads this directly.
- `assets/mockups/14-attribution-page.html` — visual contract for the attribution page
- M4's `_includes/topnav.liquid`, `_includes/footer.liquid` — already link to `/about/` and `/attribution/` as placeholders. M7 makes the targets exist.
- M6's `_includes/trust-bar.liquid` — links to `/about/#trust-levels`. M7 must include an anchor with `id="trust-levels"` on the about page.
- `assets/css/05-components.css` — class vocabulary. Confirm relevant classes (`.attribution`, `.about`, `.definitions`, etc.) exist or flag the gaps.

---

## Scope (from project plan § M7)

Goal: legally and editorially complete. Attribution page lists upstream sources and image credits. Definitions page explains trust levels.

Tasks per the project plan (M7.1–M7.6):

- **M7.1** — Implement attribution data generation in the Rust pre-build (extend `generator.rs` or add `attribution.rs`): read source-page frontmatter and `demo/media/manifest.json`, write `_data/attribution.json` (or pre-rendered markdown — see § 4 of the plan-requirements below).
- **M7.2** — Author `_layouts/attribution.liquid` per mockup 14: list every source page with title/upstream URL/license/revision_id; list every image with Commons URL/author/caption/license; CC BY-SA general notice.
- **M7.3** — Implement definitions page generation: from `profile.yaml` / `profile.json`, render the about page per addendum § 7.4. Conditional sections per axis enabled.
- **M7.4** — Author `_layouts/about.liquid` for the definitions page rendering.
- **M7.5** — Confirm Attribution and About are reachable from topnav and footer (M4 already added the links; just verify the targets resolve and the anchors work).
- **M7.6** — Verify: attribution page lists all 91 source pages and ~600 images correctly; about page explains trust levels per the encyclopedia dialect; trust bar's `/about/#trust-levels` deep-link works.

After M7, the demo is editorially complete: legal compliance via attribution, reader orientation via definitions. No more "TODO" links pointing at non-existent pages.

---

## Plan document requirements

Write the implementation plan to `docs/dev/0007-m7-implementation-plan-attribution-and-definitions-pages.md`. Mirror under `docs/design/05-active/` if that's the project's settled convention.

The plan should cover:

### 1. Files to create / modify

A clear table. Should include:

- New: `_layouts/attribution.liquid`, `_layouts/about.liquid`
- New: stub markdown files at `demo/attribution.md` (or wherever the demo orchestrator places them) and `demo/about.md` — one per layout, with appropriate frontmatter (`layout`, `permalink: /attribution/` or `/about/`, page-type metadata as needed). These are equivalent to the home/category stubs from M5: small markdown files that point at the layout, with the heavy content rendered template-side from data.
- New: `tools/src/generator.rs` extension (or new `tools/src/attribution.rs` module) to write `_data/attribution.json` aggregating source-attribution + media-manifest data. About page can read directly from `_data/profile.json` (no new data file likely needed), but verify in plan.
- Modified: `tools/src/demo/build.rs` — add `setup_attribution_stubs()` step that emits `demo/_build/attribution.md` and `demo/_build/about.md` so cobalt builds them
- Modified: `_includes/topnav.liquid` and `_includes/footer.liquid` — verify links to `/about/` and `/attribution/` are present from M4; if placeholder, no change needed.
- Updates to `assets/css/05-components.css` only if mockup 14 needs classes not already present.

### 2. Data shape verification

Before authoring, verify:

- `demo/media/manifest.json` exists and has the shape per `0002-haleiki-demo-site-design-document.md` § 5 (Media Manifest). Specifically confirm the `images` array with the named fields. If shape differs, surface as a generator-extension item.
- Source-page `original_source` frontmatter: confirm `title`, `project`, `url`, `license`, `fetched_at`, `revision_id` are present on every source page (the demo pipeline should have populated these consistently).
- `profile.json` for the about-page sections: confirm `trust.levels.<level>.{label, description}`, `trust.level_order`, `canon.enabled`. M2 fix-pass should have these.
- Concept-card `external_references`: confirm presence/absence — if no concept cards have external_references, the attribution page can omit that subsection. Spot-check a few.

If any data shape diverges, surface as a design call.

### 3. Attribution page content structure

Per `0002-haleiki-demo-site-design-document.md` § 8, the attribution page lists:

- **General notice**: CC BY-SA acknowledgement covering the demo content as a whole. Brief and clear; mentions that all source pages derive from CC BY-SA-licensed Wikimedia and Rigpa Wiki content.
- **Sources section**: every source page, with: title, upstream project (e.g., "en.wikipedia.org"), upstream URL, license version (CC BY-SA 4.0 / 3.0), revision_id (for reproducibility), fetched_at date. Suggested grouping: by upstream project (Wikipedia first, Wikibooks, Rigpa Wiki, …) then alphabetical within each. Plan should pick a grouping and justify.
- **Media section**: every image with: filename, Commons URL (or source-domain URL for non-Commons), author, license, caption, the source-page slug it came from, format, size. Probably grouped by source page (so all dzogchen images are listed together with the dzogchen entry above), since browsing 600 images flat is unwieldy.
- **External references** (optional, if any concept cards have them): list of non-source-page citations with title, URL, license.

Plan should specify the exact section ordering, grouping strategy, and what gets rendered template-side vs pre-aggregated in `attribution.json`.

### 4. Data vs pre-rendered markdown — pick one

Two implementation paths for attribution and about:

- **Path A — Data + Liquid template**: generator writes `_data/attribution.json` (aggregated source/media/external-ref data); `attribution.liquid` renders the page from that data. Same pattern as see-also/breadcrumbs/categories in M2/M5. Recommended.
- **Path B — Pre-rendered markdown**: generator emits a complete markdown file `attribution.md` with all the content baked in; the layout is just the chrome wrapper.

**Recommend Path A** for both pages. Reasons: (1) consistent with the rest of the data-driven pipeline, (2) the layout owns formatting decisions (table vs definition list, etc.) which keeps presentation in one place, (3) re-rendering is automatic when data changes.

For the about page specifically, Path A is unambiguous — it reads `_data/profile.json` (already populated by M2 fix-pass). No new data generation needed; just author the layout.

For the attribution page, Path A means the new generator step writes `_data/attribution.json`. Spec the exact JSON shape in the plan (use the addendum's structure as a guide).

### 5. About page content per addendum § 7.2

The about page MUST include (per addendum § 7.2):

- A clear statement that trust levels describe **the sourcing and evidence basis of the content, not a value judgment on the subject matter**. The Buddhism example from § 7.2 is illustrative — an article on Buddhism marked "Corroborated" means information verified by experts in Buddhist studies, NOT that the wiki endorses Buddhism as truth.
- Each trust level with profile-customized label, description, glyph, color swatch.
- **At least one concrete example per level, drawn from the deployment's own content where possible.** Implementation: read `_data/graph.json` for nodes by trust level, pick one representative example per level, link to it. Or hand-curate in `profile.yaml` if data-driven selection is awkward.
- A note that trust levels may change as content is updated and additional sources are incorporated.
- Anchor `id="trust-levels"` so M6's trust-bar link works.

Canon-status section: omitted entirely for the encyclopedia profile (canon disabled). Conditional render: `{% if site.data.profile.canon.enabled %}` wrapper.

The page should feel like a thoughtful explanation, not a spec dump. Voice: warm but precise, matching the "scholarly warmth" brand voice from the brand guidelines.

### 6. Trust-level examples sourcing

For each of the four trust levels, pick a concrete example from the corpus:

- `corroborated`: a foundational, widely-corroborated topic (e.g., `quantum-mechanics` source page, or `pythagorean-theorem` if present)
- `credible`: a well-sourced but specific topic (e.g., a programming-language source like `rust-language`, or a music-theory concept)
- `provisional`: a topic with partial evidence (none of the demo's encyclopedia-genre content is `provisional` by default — all are `credible`. So either pick the lowest-trust card if any are explicitly downgraded, or use a synthesized/illustrative example with a note "in this corpus, no entries are currently `provisional`")
- `speculative`: same caveat — encyclopedia-profile content is unlikely to have `speculative` entries. May need an illustrative-only example with an explanatory note.

Plan should address how to handle the "no real example" case for `provisional` and `speculative` in the encyclopedia-profile demo. Options:

- Hide the example section for levels with no real instances
- Show a synthesized illustrative example with a "*illustrative; no current corpus entries at this level*" note
- Pick the *lowest-trust* card available and use it even if its actual level is `credible`

Recommend the second option (synthesized illustrative). The point of the section is to give readers a feel for what each level *means*; using a real example helps but isn't strictly required.

### 7. Stub markdown files for cobalt

Per the M5 pattern: `demo/_build/attribution.md` and `demo/_build/about.md` are tiny stubs (just frontmatter + maybe a title) that point at the relevant layout. The layout reads data and renders the body.

```yaml
---
layout: attribution.liquid
permalink: /attribution/
title: Attribution
---
```

```yaml
---
layout: about.liquid
permalink: /about/
title: About
---
```

The build orchestrator's `setup_about_and_attribution_stubs()` (or whatever name) emits these into `demo/_build/`. Same pattern as M5's category stubs.

### 8. Conditional rendering / forward-compat

The about page is conditional on what axes the deployment profile enables. Encyclopedia has trust enabled, canon disabled. Writer would have both. Knowledge-site has neither (the page might be near-empty for that profile, which is fine).

Template should:

- Always render the page header (title, intro paragraph)
- Render trust section if `site.data.profile.trust.enabled` (anchor `#trust-levels`)
- Render canon section if `site.data.profile.canon.enabled` (anchor `#canon-status`)
- Render genre section if any source page has a non-default genre — forward-compat for the conversation/worldbuilding/essay genres from v1.3. Probably skip for M7 (the demo source pages don't have genre yet); flag as future M7-extension.

### 9. License-version handling

Source pages have a `license` field that can be CC BY-SA 4.0 (Wikipedia), 3.0 (Wikibooks, Rigpa Wiki), or others. Image licenses similarly vary. The attribution page should:

- Display the actual license version per source/image
- Provide a single general notice that says "this corpus is composed of CC BY-SA-licensed content from Wikimedia projects and Rigpa Wiki, with per-source license versions noted below"
- Link to the relevant license texts

Plan should specify the exact phrasing and links.

### 10. Topnav and footer verification

M4 added placeholder links to `/about/` and `/attribution/` in topnav and footer. M7 verifies:

- The links resolve (i.e., the pages exist)
- The link text reads correctly ("About" or "About Haleiki" or "Definitions"? "Attribution" or "Credits"?)
- Footer attribution notice is consistent with the page

If any link text or behavior needs adjustment, plan the include update.

### 11. Out of scope (what M7 explicitly doesn't do)

- **Math/code rendering** — that's M8.
- **Search** — separate phase.
- **Per-genre about-page sections** (conversation/worldbuilding/essay genres) — forward-compat; not in M7's encyclopedia-only scope.
- **Trust changelog / history of trust changes per page** — addendum § 12 open question; defer.
- **Image hosting / mirroring concerns** — images are already local in `demo/media/`; attribution page just lists provenance.

---

## Open questions to surface in the plan

1. **Attribution data: Path A vs Path B** — recommend A; confirm.
2. **Sources grouping** — by project, alphabetical, by tier, or other? Pick and justify.
3. **Media grouping** — under each source page (recommended) or flat alphabetical?
4. **Trust-level examples for `provisional` and `speculative`** — synthesized illustrative vs. hide-when-empty vs. lowest-real-card. Recommend synthesized illustrative.
5. **Class vocabulary check** — confirm the mockup's classes exist in `05-components.css`. Flag gaps.
6. **External references aggregation** — are there concept cards with `external_references` populated? If yes, surface in attribution. If no, omit the subsection.
7. **License notice phrasing** — propose exact text for plan review.
8. **Topnav link text and target stability** — confirm M4's link text and that it still feels right after the targets exist.

---

## Reporting

When the plan doc is landed, report:

1. **File path** — `docs/dev/0007-m7-implementation-plan-...` (or design-doc mirror).
2. **Doc structure** — section headings only.
3. **Design decisions you made** — Path A vs B, sources/media grouping, trust-level example strategy, license notice phrasing.
4. **Design decisions you surfaced for review** — open questions.
5. **Data shape verification result** — `media/manifest.json` shape, source-page `original_source` consistency, `profile.json` for about-page needs, concept-card `external_references` presence.
6. **Class vocabulary check** — gaps if any.
7. **Approximate scope** — line counts per layout, generator extension, total LOC estimate.
8. **Anything surprising** during the planning pass.

Don't commit the plan yet — Duncan will review and either approve as-is or ask for revisions before kicking off execution.

---

If you encounter a substantive design question that you can't resolve confidently from addendum 0005 § 7, demo-design 0002 § 8, the existing data files, and M4–M6 templates, surface it in the plan with a "for review" note rather than picking silently. Addendum 0005 § 7 is the authoritative spec for the about page; demo-design 0002 § 8 is authoritative for the attribution page.
