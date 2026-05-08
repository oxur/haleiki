---
number: 11
title: "Profile System — Trust Bar and Info Card Tinting"
author: "trust bar"
component: All
tags: [change-me]
created: 2026-05-08
updated: 2026-05-08
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# M6 Implementation Plan: Profile System — Trust Bar and Info Card Tinting

Pre-implementation planning pass for milestone M6 of Phase 6. Same pattern as M1–M5 plans: draft the implementation plan as a design doc, Duncan and I will review, then you execute.

**This pass is plan-only — do not implement the components yet.** Land the plan doc, report what's in it, surface design decisions you've made or want to surface for review.

---

## Reference docs

- `docs/design/06-final/0006-...site-build...project-plan.md` § M6 — milestone scope and task breakdown
- `docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md` — **the canonical reference for this milestone**:
  - § 3 — Trust Level axis (purpose, enum values, descriptions, frontmatter, visual treatment)
  - § 3.5 — Visual treatment: trust bar, article overline, info card header, glyph system
  - § 5.1 — Trust-level design tokens (8 theme × trust combinations, hue assignments, semantic token names)
  - § 7 — Definitions page (referenced by trust bar; M7 will generate but M6's link target is `/about/#trust-levels`)
  - § 8 — Cobalt integration (data flow from profile.yaml → profile.json → templates)
  - § 10 — Impact on graph & validation (forward-compat, not M6 work)
- `assets/mockups/21-trust-bar-prototype.html` — visual reference. **Note the value-space discrepancy**: the mockup uses `data-trust="established"`, but `established` is a canon-axis value per addendum § 4.2. Trust values are `corroborated | credible | provisional | speculative` per § 3.2. M6 needs to **substitute the correct trust value space** when extracting the visual approach from the mockup.
- `demo/_build/_data/profile.json` — already populated by M2 fix-pass with `trust.enabled: true`, `trust.default: credible`, `trust.levels.{corroborated,credible,provisional,speculative}.{label,description}`, `trust.level_order`. **Verify the actual shape matches what M6 templates will read** before authoring.
- M4's `_includes/topnav.liquid`, `_includes/sidebar-taxonomy.liquid`, `_layouts/source.liquid`, `_layouts/concept.liquid` — M6 updates these.
- M3's `assets/css/01-tokens.css` — already has the base token cascade (`:root`, `[data-theme="day"]`, `[data-theme="night"]`). M6 adds the compound-selector blocks (`[data-theme="day"][data-trust="..."]`).

---

## Scope (from project plan § M6)

Goal: the encyclopedia profile is enabled (already done in M2 fix-pass). Trust bar renders at the top of pages tinted by `trust_level`. Info card header tints by trust. Article overline includes trust glyph and label.

Tasks per the project plan (M6.1–M6.9):

- **M6.1** — `content/profile.yaml` (already done in M2 fix-pass at `demo/profile.yaml`; verify and skip)
- **M6.2** — `generator.rs` reads profile and writes `_data/profile.json` (already done in M2 fix-pass; verify the shape supports M6 needs)
- **M6.3** — Author `_includes/trust-bar.liquid` per mockup 21 (with corrected trust-value space)
- **M6.4** — Author `_includes/trust-glyph.liquid` — four SVG glyphs (solid, half-filled, outlined, dashed-outlined circles per § 3.5)
- **M6.5** — Update `_includes/sidebar-taxonomy.liquid` to tint info card header by trust accent token
- **M6.6** — Update `_layouts/source.liquid` and `_layouts/concept.liquid` to set `data-trust` on `<html>` (or read it via JS injected before the rest of the page) so per-trust CSS tokens apply
- **M6.7** — Update article overline in source/concept layouts to include trust glyph + label when trust is enabled
- **M6.8** — Author trust-token CSS layer per addendum § 5.1: 8 theme × trust combinations, hue assignments per the table below
- **M6.9** — Verify: a `corroborated` page and a `provisional` page render with distinct trust accents; theme toggle preserves trust tinting

After M6, the trust system is visible end-to-end: every page communicates its epistemic posture via the bar, overline, and info card. Concept cards already have `trust_level` in frontmatter (set during corpus extraction). Source pages are addressed in § 8 below.

---

## Plan document requirements

Write the implementation plan to `docs/dev/0006-m6-implementation-plan-trust-bar-and-info-card-tinting.md`. If the project's plan-doc convention has settled on `docs/design/05-active/` under ODM lifecycle, mirror there as well — pick one as canonical and note in both.

The plan should cover:

### 1. Files to create / modify

A clear table. Should include:

- New: `_includes/trust-bar.liquid`, `_includes/trust-glyph.liquid`
- Modified: `_includes/sidebar-taxonomy.liquid` (header tinting), `_layouts/source.liquid` and `_layouts/concept.liquid` (data-trust attribute, overline glyph), `_layouts/default.liquid` (if `data-trust` is set in `<html>` rather than per-layout — recommended for theme-toggle compatibility)
- New CSS: either `assets/css/07-trust-tokens.css` (new file in the layer cascade) or an extension of `01-tokens.css`. Decide and document.
- Possibly modified: `assets/css/main.css` (new `@import` for trust tokens if separate file) and `assets/css/05-components.css` (trust-bar component styles, info-card-header trust-aware tinting, overline trust treatment)
- Possibly: source-page frontmatter retrofit (see § 8 below)

### 2. Data shape verification

Before authoring, verify:

- `profile.json` — shape from M2 fix-pass. Confirm `site.data.profile.trust.enabled`, `site.data.profile.trust.default`, `site.data.profile.trust.levels.<level>.label`, `site.data.profile.trust.levels.<level>.description`, `site.data.profile.trust.level_order` are all present and correctly typed.
- `page.data.trust_level` — concept cards have this set (from corpus extraction). Source pages may or may not (see § 8). The template needs a defaulting strategy.

If the data shape diverges from what the templates need, surface as a design call rather than a template hack.

### 3. Trust-token hue assignments (per addendum § 5.1)

Reproduced from § 5.1 for the plan to lock in:

| Trust level | Day hue (OKLCH) | Night hue (OKLCH) | Color family | Visual logic |
|-------------|-----------------|-------------------|--------------|--------------|
| `corroborated` | ~45° | ~70° | Terracotta / Amber (inherits wiki accent) | Quietest — bedrock, the default posture |
| `credible` | ~165° | ~165° | Sage / Teal | Moderate — angled but trustworthy |
| `provisional` | ~295° | ~295° | Dusty violet | Noticeable — partial evidence |
| `speculative` | ~85° | ~85° | Warm ochre / Yellow | Loudest — fragmentary, furthest from bedrock |

Token names per addendum § 5.1:
- `--trust-surface` — tinted background for trust bar
- `--trust-border` — bar and component borders
- `--trust-accent` — primary accent (info card header, active states)
- `--trust-text` — text and glyph color
- `--trust-subtle` — hover/selection backgrounds
- `--trust-bar-bg` — trust bar background wash
- `--trust-on-accent` — text on accent backgrounds

For each of 4 trust × 2 themes = 8 combinations, define all 7 tokens. ~80–120 lines of CSS depending on density.

Compound selectors: `[data-theme="day"][data-trust="corroborated"]`, etc. **CRITICAL**: these must be in the tokens layer of the cascade, *after* the base theme tokens, so they correctly override the base accent on a per-trust basis.

### 4. Glyph system (per addendum § 3.5)

Four SVG glyphs, geometric, color-blind friendly. The shape encodes distance-from-bedrock:

| Level | Glyph | Shape logic |
|-------|-------|-------------|
| `corroborated` | Solid filled circle | Complete, whole, bedrock |
| `credible` | Half-filled circle | Mostly solid, slight opening |
| `provisional` | Outlined circle | Hollow — shape there, filling not |
| `speculative` | Dashed-outline circle | Fragmentary, provisional, open |

Implementation approach:

- Inline SVG in `trust-glyph.liquid`, sized at ~1em line-height
- Conditional render based on `page.data.trust_level` (with default fallback per § 8)
- `currentColor` for stroke/fill so the glyph inherits text color
- Single template file with `{% case page.data.trust_level %}` switching between four `<svg>` blocks

### 5. Trust bar placement and design (per addendum § 3.5)

A slim horizontal bar **below the top navigation**, tinted with the trust-level color. Border weight changes per level:

- `corroborated`: 1px solid (default)
- `credible`: 1px solid
- `provisional`: 2px solid
- `speculative`: 3px dashed (loudest, fragmentary)

Links to `/about/#trust-levels` (M7 will generate the about page; the link target stays valid because M4's footer also links to `/about/`).

The bar's content: trust glyph (left), label ("Corroborated" / "Credible" / etc.), short description (truncated if narrow viewport), link to definitions page.

Conditional rendering: only renders if `site.data.profile.trust.enabled` is true. When disabled, the bar is absent entirely.

### 6. Article overline trust integration

Currently the source-page overline reads "Source · {tags}" and the concept-page overline shows the category label (per arch § 4.5 and M4).

When trust is enabled, the overline becomes:

```
[trust-glyph] {Trust label} · {existing overline content}
```

Example for a corroborated `quantum-mechanics` source: `● Corroborated · theoretical-physics`. For a `provisional` concept: `○ Provisional · history-of-music`.

Glyph and label color tinted via `--trust-text` token.

### 7. Info card header tinting

The taxonomy sidebar's header (top of the info card) gets its background tinted with `--trust-accent`. Text on that header uses `--trust-on-accent`. Implementation: update `_includes/sidebar-taxonomy.liquid` to add a class or inline style that uses the trust tokens.

When trust is disabled, the header reverts to using `--accent-primary` (the base wiki accent).

### 8. Source-page trust_level handling

**Concept cards**: have `trust_level` set in frontmatter (assigned during corpus extraction; default `credible` per encyclopedia profile).

**Source pages**: do NOT have `trust_level` set. The 91 source pages produced by the demo pipeline have frontmatter without `trust_level`. Two paths:

- **Path A — Retrofit**: write a small build-orchestrator script (or one-shot Python script in `workbench/`) that adds `trust_level: credible` to every source page's frontmatter. Bulk back-fill, similar to the source-page provenance back-fill from earlier. Clean state thereafter.
- **Path B — Template default**: layouts read `page.data.trust_level | default: site.data.profile.trust.default` (Liquid `default` filter). No frontmatter retrofit; default applies wherever the field is missing.

**Recommend Path B**. Reasons: (1) no destructive corpus-touching pass needed, (2) the default is explicit in profile config (single source of truth), (3) when source pages legitimately need a non-default trust level later (e.g., conversation genre → `provisional`), authors set it explicitly and the template picks it up. Path A locks every source page at `credible` even if a future source warrants different.

Plan should specify Path B and the exact Liquid expression: `{% assign trust_level = page.data.trust_level | default: site.data.profile.trust.default %}` early in source.liquid and concept.liquid, then use `trust_level` (not `page.data.trust_level`) throughout the layout for the `data-trust` attribute and trust-bar/overline rendering.

### 9. Conditional rendering when trust is disabled

Encyclopedia profile has trust enabled, so the demo always shows trust UI. But the templates **MUST** be conditional on `site.data.profile.trust.enabled` so the same templates work for the `knowledge-site` profile (trust disabled by default).

When disabled:
- Trust bar absent (`{% if site.data.profile.trust.enabled %}` wrap)
- `data-trust` attribute not set on `<html>` (so compound selectors don't activate; base tokens win)
- Overline omits glyph + label (reverts to the M4 plain overline)
- Info card header uses base accent

### 10. Theme-toggle interaction

M3's theme.js sets `data-theme="day"` or `data-theme="night"` on `<html>`. M6 sets `data-trust="<level>"` on the same element. Compound selectors `[data-theme][data-trust]` activate based on both attributes. The theme toggle changes `data-theme` only — `data-trust` is set once per page based on the page's trust level and doesn't change.

Verify in plan: switching between day/night should preserve trust tinting (both attributes still set, compound selectors still match the new theme + same trust). This is the architecture's intent; it should "just work" with the cascade structure, but worth explicit confirmation in the plan.

### 11. Mockup-vs-spec value-space reconciliation

Mockup 21 uses `data-trust="established"`. Per addendum § 3.2, `established` is a *canon* value (canon-axis = `established | proposed | developing | seed`). Trust values are `corroborated | credible | provisional | speculative` per § 3.2.

**M6 implementation must use the correct trust values.** Don't just copy the mockup's `data-trust="established"` literal. The mockup demonstrates the visual approach — extract that, apply it to the four real trust levels with the hue assignments from § 5.1.

The mockup itself should arguably be updated to use trust values, but that's an `assets/mockups/` cleanup task, not part of M6's implementation scope. Flag it in the plan as a follow-up nice-to-have.

### 12. Out of scope (what M6 explicitly doesn't do)

- **Canon system / canon badge** — writer profile only; encyclopedia disables canon. Forward-compat: the implementation should leave room for canon (don't structurally exclude it), but M6 doesn't build any canon UI.
- **Definitions / about page generation** — that's M7. M6's trust-bar link to `/about/#trust-levels` is forward-compat; the target page comes in M7.
- **Search facet filtering by trust level** — separate phase (search).
- **Trust-aware validation rules** (per addendum § 10 — e.g., warn when a `corroborated` page references a `speculative` prerequisite) — forward-compat for `validator.rs`; not M6 work.
- **Trust-level transitions / changelog** — addendum § 12 open question; defer.

---

## Open questions to surface in the plan

1. **Mockup-vs-spec discrepancy**: confirm the resolution per § 11 above. Recommend M6 ignores the mockup's literal `data-trust="established"` and uses correct trust values.
2. **Source-page trust default strategy**: Path A (retrofit) vs Path B (template default). Recommend Path B per § 8.
3. **CSS file organization**: new `07-trust-tokens.css` file in the cascade, or extend `01-tokens.css`. Plan should decide.
4. **`data-trust` on `<html>` vs `<body>`**: per addendum § 8.3, `<html>` is canonical. Confirm in plan.
5. **Mockup follow-up**: the trust-bar mockup should be updated to use a real trust value. Flag for post-M6 cleanup task; not blocking.
6. **Border-weight per level**: per § 5 above, addendum specifies 1px solid (corroborated), 1px solid (credible), 2px solid (provisional), 3px dashed (speculative). Confirm in plan or surface for review if mockup suggests different.
7. **Class vocabulary check**: confirm trust-bar styling, info-card-header trust class, overline-trust treatment have classes in `05-components.css` or flag the gaps.

---

## Reporting

When the plan doc is landed, report:

1. **File path** — `docs/dev/0006-m6-implementation-plan-...` (or design-doc mirror).
2. **Doc structure** — section headings only.
3. **Design decisions you made** — source-page trust default (Path A vs B), CSS file organization, glyph SVG approach, trust-bar placement, etc.
4. **Design decisions you surfaced for review** — any open questions you couldn't resolve confidently.
5. **Data shape verification result** — `profile.json` shape; `page.data.trust_level` presence on concepts vs sources; flag mismatches.
6. **Class vocabulary check** — did the necessary classes exist in `05-components.css` from M3 forward-compat? List gaps if any.
7. **Approximate scope** — line counts per new file, modifications, total LOC estimate.
8. **Anything surprising** during the planning pass.

Don't commit the plan yet — Duncan will review and either approve as-is or ask for revisions before kicking off execution.

---

If you encounter a substantive design question that you can't resolve confidently from the addendum, mockup, and existing M3/M4 code, surface it in the plan with a "for review" note rather than picking silently. The deployment-profiles addendum (0005) is the authoritative spec for this milestone — when in doubt, defer to its language.
