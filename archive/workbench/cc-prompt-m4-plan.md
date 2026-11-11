# M4 Implementation Plan: Source and Concept Page Templates with Full Chrome

Pre-implementation planning pass for milestone M4 of Phase 6. Same pattern as M1, M2, and M3 plans: draft the implementation plan as a design doc, Duncan and I will review, then you execute.

**This pass is plan-only — do not implement the templates yet.** Land the plan doc, report what's in it, and surface the design decisions you've made or want to surface for review. We'll iterate on the plan if needed before kicking off execution.

---

## Reference docs

- `docs/design/06-final/0006-...site-build...project-plan.md` § M4 — milestone scope and task breakdown
- `docs/design/01-draft/0001-haleiki-architecture-document.md` § 4.5 — page-type visual differentiation (Concept vs Source page chrome differences). **Read this carefully** — source.liquid and concept.liquid are *not* identical; they have meaningful differences in overline content, body-structure expectations, provenance direction, and see-also content.
- `docs/design/06-final/0007-m2-implementation-plan-haleiki-build-core.md` — what shape the data files take. Templates read `site.data.graph`, `site.data.categories`, `site.data.breadcrumbs[page.slug]`, `site.data.see-also[page.slug]`, `site.data.provenance[page.slug]`, `site.data.profile`. **Verify the data shapes match what the templates need before authoring** — if a shape doesn't match, surface it as a design call rather than papering over.
- `docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md` (v1.3) — concept-card schema, so the sidebar info card and provenance rendering reference the right frontmatter fields
- `assets/mockups/` — visual contract:
  - **`13-concept-page.html`** — primary reference for concept-page chrome (left-nav, article header, sidebar info card, see-also, footer)
  - **`17-news-post.html`** — primary reference for source-page chrome (post-layout, post-header, post-nav, footer)
  - **`08-footer.html`** — footer component reference
  - **`10-home-page.html`** — topnav reference (the home-page mockup has the most complete topnav)
- `assets/css/05-components.css` — **the existing class vocabulary M3 produced**. The M4 templates **MUST** use these existing class names (e.g., `.topnav`, `.site-footer`, `.article`, `.source-page`, `.concept-card`, `.article-overline`, `.article-title`, `.article-meta`, `.tag`, `.prose`, `.sidebar`, `.see-also`, etc.). Do not invent new class names that aren't already in the components stylesheet — if the mockup needs a class that isn't defined, flag it as a gap rather than silently adding it.

---

## Scope (from project plan § M4)

Goal: source pages and concept pages render with full chrome matching mockups 13 (concept) and 17 (source/news-post). After M4, dzogchen and ownership-rust pages should look like the mockups end-to-end (modulo any deferred-to-M5/M6/M7 components like home/category/browse pages, trust bar, attribution).

Tasks per the project plan (M4.1–M4.9):

- **M4.1** — Author `_includes/topnav.liquid` (site logo/wordmark, primary nav, theme toggle button)
- **M4.2** — Author `_includes/footer.liquid` (per mockup 8: brand, links, attribution notice)
- **M4.3** — Author `_includes/sidebar-taxonomy.liquid` (info card: category, subcategory, tier, subject_headings, aliases, derived_from / concepts_generated, status)
- **M4.4** — Author `_includes/breadcrumb.liquid` (reads `site.data.breadcrumbs[page.slug]`)
- **M4.5** — Author `_includes/see-also.liquid` (reads `site.data.see-also[page.slug]`, renders typed-relationship groups)
- **M4.6** — Author `_includes/provenance.liquid` (renders source ↔ concept cross-links from `site.data.provenance[page.slug]`)
- **M4.7** — Update `_layouts/source.liquid` (full source-page layout: topnav, breadcrumb, header with overline, body, post-nav, sidebar, footer)
- **M4.8** — Update `_layouts/concept.liquid` (full concept-page layout: topnav, breadcrumb, header, body sections, sidebar, footer; left-nav TOC if mockup 13 specifies)
- **M4.9** — Verify: dzogchen source page and ownership-rust concept page render with full mockup-equivalent chrome

---

## Plan document requirements

Write the implementation plan to `docs/dev/0004-m4-implementation-plan-source-and-concept-page-templates.md` (continuing the dev-plan numbering). If your project convention has shifted to landing implementation plans in `docs/design/05-active/` under ODM lifecycle, mirror there as well — but do not duplicate-and-diverge: pick one as canonical and clearly note in both that they are mirrors.

The plan should cover:

### 1. Files to create / modify

A clear table of all new and modified files, like the M1/M2/M3 plans. Approximate line counts where reasonable. Should include the six includes, the two layout updates, and any small additions to default.liquid (e.g., wiring the theme-toggle button into the topnav, exposing site config for the topnav).

### 2. Data shape verification

Before authoring includes, walk through each `site.data.*` source the includes will read, and verify the actual JSON shape (look at `demo/_build/_data/*.json`) matches what the include needs. Specifically:

- `breadcrumbs.json`: confirm shape is `{ "<slug>": [{ "label": "...", "url": "..." }, ...] }`. Does the breadcrumb include current-page in the trail (last element with no link), or does the template add the current-page label separately?
- `see-also.json`: confirm shape is `{ "<slug>": [{ "slug": ..., "title": ..., "relationship": ..., "url": ... }, ...] }`. Are entries pre-sorted by relationship type, or does the include need to group them?
- `provenance.json`: confirm shape and bidirectional structure (source pages have `concepts_generated`, concept cards have `derived_from`).
- `profile.json`: confirm `trust.enabled`, `trust.levels`, `trust.level_order` for forward-compat (M6 will use these; M4 just shouldn't break the path).

If any data shape diverges from what the include needs to render the mockup faithfully, **surface it as a design call**, not as a silent template hack. We can fix the generator if the data shape is wrong, or we can adjust the template approach.

### 3. Page-type differentiation

Source.liquid and concept.liquid are *not* the same template with different titles. Per arch doc § 4.5:

| Element | Concept Page | Source Page |
|---------|-------------|-------------|
| Overline | Category label | "Source · {tags}" |
| Body structure | Standardized sections (Quick Definition, Core Definition, Prerequisites, etc.) | Freeform authored prose |
| Provenance section | "Derived from" → source pages | "Concepts in this source" → concept pages |
| See Also | Graph-derived typed relationships | Related sources + extracted concepts |

Spell out in the plan how source.liquid and concept.liquid each handle these four elements, and what shared chrome they have via includes.

### 4. Include composition strategy

Each include reads either:
- **Page-local frontmatter** via `page.data.*` (M1's Cobalt-injected frontmatter)
- **Build-derived data** via `site.data.*` (M2's `_data/` files)
- **Profile config** via `site.data.profile.*` (M2's `profile.json`, used for forward-compat with M6)

For each of the six includes, specify which data source(s) it reads and how it composes them. Includes should be parameter-free where possible — they read `page` and `site.data` directly. If an include needs explicit parameters (e.g., to render a typed-relationship subset), document that.

### 5. Topnav design decisions

The topnav appears on every page. Decisions to make:

- **Brand/logo**: text wordmark or the H-mark from `assets/icons/`? Brand guidelines § 1 has the H-mark spec — probably this for top-left of nav.
- **Primary nav**: what links does it include? Likely Home, Browse, About (M7), maybe Categories (a dropdown listing all 10 categories from `site.data.categories`?). Source: home-page mockup `10-home-page.html`.
- **Theme toggle**: button placement (right side?), icon (sun/moon SVG?), behavior (calls `window.haleikiToggleTheme()` from M3's theme.js, then updates the icon to reflect new state).
- **Search input** (deferred): mockup 10 has a search input in the topnav. Search is a separate phase per project plan, but the topnav structure should leave room (a placeholder slot) for M4's structure to not need topnav rework when search lands.

### 6. Sidebar taxonomy info card

Per mockup 13 right side. Shows:

- Category (with link to category page — even though category pages don't exist yet, the link should resolve to `/category/<category>/` so M5's category-page work doesn't have to touch templates)
- Subcategory (if present)
- Tier
- Subject headings (if any)
- Aliases (concept cards only)
- Derived from / concepts generated (provenance — but wait, that's the provenance include's job? Decide whether the sidebar duplicates a summary or whether provenance lives only in the provenance section below the article body)
- Status (if not "published")
- Stable since (concept cards, optional)
- Last reviewed / last extracted (cosmetic — show if useful)

Trust-level field is intentionally NOT rendered in M4 — that's the trust bar in M6.

### 7. Concept-page TOC (left-nav)

Mockup 13 has a `<aside class="left-nav">` element. Decisions:

- **Source**: auto-generate from headings in the rendered body (h2, h3) via Liquid, or static? Auto-generate is harder in pure Liquid (no rich AST access), but possible by parsing the body content. Static (one TOC per concept-card section name from the v1.3 schema's body sections) is simpler and more reliable since concept cards have a standardized body structure.
- **Recommendation**: static, listing the canonical concept-card sections (Quick Definition, Core Definition, Prerequisites, Key Properties, Construction/Recognition, Context & Application, Examples, Relationships, Common Errors, Common Confusions, Source Reference, Verification Notes). Hide sections that aren't present in the body (a small JS post-render check, or just always show all entries and let users click into not-found anchors).
- Source pages don't get a left-nav (their bodies are freeform; no canonical TOC structure). Source-page mockup 17 confirms this.

### 8. See-also rendering

Reads `site.data.see-also[page.slug]`. Should:

- Group entries by relationship type (Prerequisites first, then Extends, Related, Contrasts With; Derived From shown only on concept pages — and that's actually the provenance include's job, so skip in see-also)
- For each group, render a heading + list of cards (title + disambiguator + url)
- If a group is empty, skip the heading
- Treat "concepts_generated" the same way for source pages (renders as see-also when on a source page, since the source-page see-also is "concepts that came out of this article")

### 9. Provenance rendering

Reads `site.data.provenance[page.slug]`. On concept pages, renders "Derived from" with each source-page link plus external references (if present). On source pages, renders "Concepts in this source" with each concept-card link.

This is rendered as a *section* in the article body (not the sidebar), typically near the end, per architecture.

### 10. Footer

Per mockup 8. Includes:

- Brand
- License notice (CC BY-SA 4.0 attribution for the demo content; demo's source pages have per-page upstream attribution which is rendered elsewhere — the footer's attribution is the *general* notice)
- Link to attribution page (M7) — for now, link to `/attribution/` even though the page doesn't exist; M7 will create it
- Link to about page (M7)
- Maybe a small "Built with Haleiki" mark linking back to the framework repo

### 11. Theme toggle button

A small UI piece in the topnav. Calls `window.haleikiToggleTheme()` from M3. SVG icon changes between sun (day mode) and moon (night mode) — small inline SVGs in the topnav include.

### 12. Out of scope (what M4 explicitly doesn't do)

- **Home / category / browse pages** — that's M5.
- **Trust bar / canon badge / info-card trust tinting** — that's M6.
- **Definitions / about / attribution pages** (auto-generated) — that's M7.
- **Math/code rendering** — that's M8.
- **Search** — separate phase entirely.
- **Per-page comments, social, anything dynamic** — never (Haleiki is read-only static).

After M4, dzogchen and ownership-rust pages should look like a real, navigable wiki with full chrome. Home/categories/browse stay as M1's bare index page until M5.

---

## Open questions to surface in the plan

1. **Mock and CSS class alignment** — confirm every CSS class the templates use is defined in M3's `05-components.css`. If any are missing (e.g., the mockup uses `.left-nav` but components.css doesn't have it), flag for either CSS-layer follow-up or template-side substitution with an existing class.
2. **TOC strategy** — static vs auto-generated (recommend static per § 7 above; confirm in plan).
3. **Sidebar vs provenance section overlap** — does the sidebar duplicate provenance summary or strictly defer to the provenance section? (Recommend defer.)
4. **Topnav nav structure** — Home / Browse / Categories / About / search? Specify in plan.
5. **Brand mark in topnav** — H-mark from icons or text wordmark? (Recommend H-mark.)
6. **Forward-compat for M5 nav** — the topnav will eventually link to `/`, `/browse/`, `/category/<slug>/`, `/about/`. M5 creates `/`, `/browse/`, and `/category/*/`. M7 creates `/about/` and `/attribution/`. M4's topnav links to those URLs even though the pages don't exist yet — M5 and M7 then just fill them in without topnav rework.
7. **Page-data shape mismatches** — surface anything from § 2 verification.

---

## Reporting

When the plan doc is landed, report:

1. **File path** — `docs/dev/0004-m4-implementation-plan-...` (or design-doc mirror).
2. **Doc structure** — section headings only (so I can scan).
3. **Design decisions you made** — TOC strategy, topnav structure, brand mark, theme toggle placement, etc. Justify each.
4. **Design decisions you surfaced for review** — any open questions you couldn't resolve confidently.
5. **Data shape verification result** — for each of breadcrumbs/see-also/provenance/profile, confirm the actual JSON shape matches what the includes will need; flag any mismatches.
6. **Class vocabulary check** — did every CSS class the mockups use have a corresponding entry in `05-components.css`? If not, list the gaps.
7. **Approximate scope** — line counts per include (rough), per layout, total LOC estimate.
8. **Anything surprising** during the planning pass.

Don't commit the plan yet — Duncan will review and either approve as-is or ask for revisions before kicking off execution.

---

If you encounter a substantive design question that you can't resolve confidently from the reference docs and mockups, surface it in the plan with a "for review" note rather than picking silently. We'd rather you flag than guess.
