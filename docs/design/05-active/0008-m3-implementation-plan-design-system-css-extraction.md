---
number: 8
title: "Design System CSS Extraction"
author: "inline JS"
component: All
tags: [change-me]
created: 2026-05-05
updated: 2026-05-05
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# M3 Implementation Plan: Design System CSS Extraction

Pre-implementation planning pass for milestone M3 of Phase 6 (Cobalt site build). Same pattern as the M1 and M2 plans: draft the implementation plan as a design doc, Duncan and I will review, then you execute.

**This pass is plan-only — do not implement the CSS yet.** Land the plan doc, report what's in it, and surface the design decisions you've made or want to surface for review. We'll iterate on the plan if needed before kicking off execution.

---

## Reference docs

- `docs/design/01-draft/0006-haleiki-site-build-project-plan.md` § M3 — milestone scope and task breakdown
- `docs/design/01-draft/0001-haleiki-architecture-document.md` § 4 — design system principles, type stack, color architecture, layout system, page-type visual differentiation
- `docs/design/01-draft/0004-haleiki-brand-guidelines.md` — brand reference (typography, colors, voice — *this should be the authoritative source for typography choice*)
- `docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md` § 5 — trust and canon design tokens (M6 will need these; M3 should set up the token architecture so M6 doesn't have to refactor)
- `assets/mockups/` — visual contract. The mockups inline the full design system as `<style>` blocks. Each one is self-contained, so they're slightly inconsistent across files. Look at:
  - `09-brand-guidelines.html` — the brand-guidelines page itself; canonical reference for tokens
  - `10-home-page.html` — home page; comprehensive design-system inline
  - `13-concept-page.html` — concept-page chrome; sidebar tokens, article tokens
  - `17-news-post.html` — source-page chrome (post-layout)
  - `21-trust-bar-prototype.html` — trust-bar component (M6 forward-compat — note the trust/canon value-space discrepancy already flagged in 0006 § Open Question 1)
  - `08-footer.html` — footer component
  - The typography variants (`02-literata.html`, `03-newsreader.html`, `04-newsreader-instrument-sans.html`, `05-newsreader-plus-jakarta-sans.html`, `06-newsreader-source-sans-3.html`, `07-newsreader-red-hat-display.html`) — typography exploration; pick one as the type-stack source-of-truth (or reconcile with arch doc 0001 § 4.2's DM Sans / Source Serif 4 / IBM Plex Mono spec, whichever is canonical per brand guidelines)

---

## Scope (from project plan 0006 § M3)

Goal: design system lives in `assets/css/` and `assets/fonts/`; Cobalt copies them to `_site/`; pages styled per the mockups' aesthetic.

Tasks per the project plan (M3.1–M3.10):

- **M3.1** — Extract token layer (type scale, spacing scale, OKLCH color, theme tokens) into `assets/css/01-tokens.css`
- **M3.2** — Extract reset and base layers into `assets/css/02-reset.css` and `assets/css/03-base.css`
- **M3.3** — Extract layout primitives (Every Layout: Stack, Cluster, Sidebar, etc.) into `assets/css/04-layout.css`
- **M3.4** — Extract components (topnav, footer, sidebar, article, category-card, etc.) into `assets/css/05-components.css`
- **M3.5** — Extract utilities into `assets/css/06-utilities.css`
- **M3.6** — Author `assets/css/main.css` that imports the layer files in `@layer` order
- **M3.7** — Set up font loading
- **M3.8** — Implement day/night theme toggle in `assets/js/theme.js` (sets `data-theme` on root, persists to localStorage)
- **M3.9** — Cobalt config copies `assets/css/`, `assets/js/`, `assets/fonts/` (and `assets/images/logo/`) to `_site/assets/`
- **M3.10** — Update `_layouts/default.liquid` to reference the published CSS and JS

---

## Plan document requirements

Write the implementation plan to `docs/dev/0003-m3-implementation-plan-design-system-css-extraction.md` (continuing the dev-plan numbering after the M1 plan at `0002-m1-...`; this normalizes placement to `docs/dev/` per the convention, even though the M2 plan accidentally landed in `docs/design/05-active/`).

The plan should cover:

### 1. Files to create / modify

A clear table of all new and modified files, like the M1 and M2 plans. Approximate line counts where reasonable. Should include the seven CSS files, the JS file, font files (or note CDN), the cobalt config update, and the default layout update.

### 2. Token architecture decisions

The mockups' inlined CSS uses an `@layer` cascade (tokens, reset, base, layout, components, utilities). M3 needs to:

- Decide the **canonical token set** — the mockups have minor variations across files. Pick a source-of-truth (likely `09-brand-guidelines.html` if it's authoritative, or `10-home-page.html` as the most comprehensive). Reconcile differences.
- Decide on **token naming conventions** — semantic (`--color-text`, `--color-surface`) vs primitive (`--neutral-100`). The mockups use semantic; preserve that.
- Set up **theme variants** — `[data-theme="day"]` and `[data-theme="night"]` token blocks. Verify the mockups have both; if night-mode tokens are sparse or inconsistent, flag for follow-up.
- **Forward-compat for trust tokens (M6)** — addendum 0005 § 5.1 specifies trust tokens use compound selectors `[data-theme][data-trust]`. M3 should structure `01-tokens.css` so M6 just *adds* compound-selector blocks rather than restructuring. Don't write the trust tokens themselves in M3 (that's M6) — but leave room.

### 3. Typography decision

This is a real design call to address in the plan. Inputs:

- Architecture doc 0001 § 4.2 specifies: DM Sans (display/headings), Source Serif 4 (body), IBM Plex Mono (code).
- Brand guidelines 0004 — read this carefully; it should be authoritative.
- Home-page mockup 10 uses Newsreader + Red Hat Display + IBM Plex Mono.
- Typography exploration mockups (02–07) show variants with Literata, Source Sans 3, Plus Jakarta Sans, Newsreader, Red Hat Display.

If the brand guidelines have a final answer, follow that. If they conflict with arch 0001 § 4.2 or the mockups, surface the conflict and recommend a resolution. Don't pick silently.

### 4. Font hosting decision

Local fonts (`assets/fonts/*.woff2`) vs CDN (Google Fonts with preconnect). Trade-offs from project plan 0006 § Open Question 4:

- Local: offline-friendly, reproducible builds, controlled. Slightly slower CSS load (font files).
- CDN: faster TTFB, requires network at view time (and at build time if CSS uses `@import`).

Default recommendation: **local**. It's better long-term (no third-party dependency), better for offline development, and `woff2` is small enough that the load impact is minimal.

If you recommend differently, justify in the plan.

### 5. Day/night theme toggle

`assets/js/theme.js` should:

- On page load: read `localStorage.getItem('haleiki-theme')`. If unset, fall back to system preference via `prefers-color-scheme`.
- Apply `data-theme="day"` or `data-theme="night"` to `<html>`.
- Provide a toggle function that updates both the DOM attribute and localStorage.
- Be exposed as a global function or via a button in the topnav (M4 will wire the button; M3 just provides the toggle infrastructure).

Tiny script, no dependencies. ~30 lines of JS.

### 6. Cobalt asset pipeline

`_cobalt.yml` (currently at `demo/_cobalt.yml` per the M1 architectural divergence) needs to copy the framework's `assets/` directory to `_site/assets/`. Specifically:

- `assets/css/` → `_site/assets/css/`
- `assets/js/` → `_site/assets/js/`
- `assets/fonts/` → `_site/assets/fonts/` (if local fonts)
- `assets/images/logo/` → `_site/assets/images/logo/`

Also: the M1 build orchestrator (`tools/src/demo/build.rs`) currently sets up `demo/_build/` ephemerally. The framework's `assets/` are copied (not symlinked) into `demo/_build/assets/` because cobalt-bin doesn't reliably traverse symlinks for asset directories. Update `setup_layouts()` style logic to add `setup_assets()` using `copy_dir_for_cobalt`.

### 7. Default layout updates

`demo/_layouts/default.liquid` (and the source/concept layouts) need to reference the published CSS and JS. Also need:

- `<link rel="preconnect">` for fonts (if CDN)
- `<link rel="stylesheet" href="/assets/css/main.css">` (or whatever the final filename)
- `<script src="/assets/js/theme.js" defer></script>`
- `data-theme` attribute on `<html>` set by inline JS at the very top of `<head>` (to avoid flash-of-wrong-theme)

The concept and source layouts (`concept.liquid`, `source.liquid`) inherit from default and don't need to repeat the asset references — verify the M1 layouts use a proper composition pattern.

### 8. Out of scope (what M3 explicitly doesn't do)

- **Page-type-specific chrome** — topnav structure, sidebar markup, breadcrumb HTML, see-also markup. That's M4. The CSS classes will exist (extracted from mockups) but the templates that use them stay as M1's bare HTML for now. Pages will *look* styled (typography, color, spacing) but not have the full chrome until M4.
- **Trust bar / canon badge** — M6.
- **Home / category / browse pages** — M5.
- **Math/code rendering** — M8.

After M3, dzogchen and ownership-rust pages should look polished (good fonts, good color, good spacing) — but with M1's bare HTML structure (no sidebar, no breadcrumb, no see-also, no topnav navigation). The chrome lands in M4.

---

## Open questions to surface in the plan

1. **Trust/canon value-space discrepancy** — the trust-bar mockup uses `data-trust="established"` (a canon value per addendum 0005 §3.2/4.2). Should be flagged in the plan as a non-blocking M6 issue but worth noting in M3's token architecture so we don't paint into a corner.

2. **Typography reconciliation** — arch 0001 § 4.2 vs brand guidelines 0004 vs mockup variants. Resolve in the plan with a recommendation.

3. **Font hosting** — local vs CDN. Default local; justify if otherwise.

4. **Day/night theme default** — system preference fallback or always day on first load. Default: system preference if available, day otherwise.

5. **Where the framework's `_layouts/`, `_cobalt.yml`, and `assets/` live long-term** — currently at `demo/_*` per the M1 divergence noted in 0006. M3 doesn't need to fix this, but the plan should flag whether `assets/css/` should land at repo root (matching arch 0001 § 3.6) or `demo/assets/css/` (matching the M1 demo-self-contained pattern). Probably repo root since assets are framework-level (not deployment-specific) — but worth confirming.

---

## Reporting

When the plan doc is landed, report:

1. **File path** — `docs/dev/0003-m3-implementation-plan-design-system-css-extraction.md`.
2. **Doc structure** — section headings only (so I can scan).
3. **Design decisions you made** — typography pick, font hosting, theme default, asset location. Justify each.
4. **Design decisions you surfaced for review** — any open questions you couldn't resolve confidently.
5. **Approximate scope** — how many lines of CSS you expect to land across the layered files (rough estimate from looking at the mockups), how many lines of JS for the toggle.
6. **Anything surprising** during the planning pass.

Don't commit the plan yet — Duncan will review and either approve as-is or ask for revisions before kicking off execution.

---

If you encounter a substantive design question that you can't resolve confidently from the reference docs, surface it in the plan with a "for review" note rather than picking silently. We'd rather you flag than guess.
