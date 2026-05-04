---
number: 6
title: "Cobalt Templates and Static Rendering Project Plan"
author: "category and"
component: All
tags: [change-me]
created: 2026-05-03
updated: 2026-05-03
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# Haleiki Site Build — Cobalt Templates and Static Rendering Project Plan

**Phase 6 · Static Demo Rendering**

Version 0.1 · May 2026 · Living Document

---

## 1. Purpose

The demo content corpus is on disk: 91 source pages with full provenance, 186 concept cards forming a real graph, bidirectional source↔concept linkage. The next phase produces a *running, navigable static site* that renders the corpus end-to-end, served locally via Cobalt and matching the design language captured in the mockups (`assets/mockups/`).

This document captures the scope, the mockup-based visual contract, the gap analysis, and a milestone plan. Per-milestone development docs will land in `docs/dev/site-build/` as work proceeds.

---

## 2. End-Goal (Definition of Done)

When this phase is complete, all of these work locally:

1. `cd demo && cobalt serve` starts a local dev server.
2. Visiting the URL renders the Haleiki home page with category cards, featured pages, and tier-grouped navigation.
3. Clicking a category card navigates to the category listing page; that page lists all source pages and concept cards in the category.
4. Clicking a source page renders the article with full chrome: topnav, taxonomy sidebar, breadcrumb, see-also section, provenance back to concept cards, footer with attribution.
5. Clicking a concept card renders the structured article: overline, definition sections, prerequisites, examples, relationships sidebar, see-also.
6. The trust bar (per the encyclopedia profile) renders at the top of pages, tinted by `trust_level`.
7. The browse page lists every page in the wiki, filterable by category and tier (no search yet — that's a separate phase).
8. The auto-generated attribution page lists every source's upstream URL/license and every image's Commons attribution.
9. The auto-generated `/about/` definitions page explains trust levels and the deployment profile.
10. Internal cross-references resolve correctly. Multilingual scripts render. Math notation renders via MathJax/KaTeX. Code blocks render with syntax highlighting.

**Out of scope for this phase:**

- **Search** — Pagefind integration is a separate phase per design doc 0001 §6.
- **Writer profile features** — canon badges, brief/sketch page types, worldbuilding-dialect trust descriptions. The demo deploys as `encyclopedia` profile.
- **News section** — mockups 16-20 sketch a news/blog area for genre=conversation/essay/blog content. This is forward design; the demo corpus is encyclopedia-only.
- **GitHub Actions deployment** — local dev server first; CI/CD per design doc 0002 §9 happens after the local demo works.

---

## 3. Mockup Visual Contract

The mockups in `assets/mockups/` are self-contained HTML files that inline the full design system. They are the visual contract for what the rendered demo should look like.

### Page-type mockups

| Mockup | File | Purpose |
|--------|------|---------|
| Home | `10-home-page.html` | Front page: hero, 10 category cards, featured pages, tier sections, quick links |
| Category | `11-category-page.html` | Per-category listing of source pages and concept cards |
| Browse | `12-browse-page.html` | Full corpus index with category/tier filters |
| Concept | `13-concept-page.html` | Concept card with overline, structured sections, taxonomy sidebar, see-also |
| Source / News post | `17-news-post.html`, `03-07-newsreader-*` | Source page with freeform body, taxonomy sidebar, post nav |
| Attribution | `14-attribution-page.html` | Auto-generated attribution listing |
| Trust bar | `21-trust-bar-prototype.html` | Trust-level visualization component |
| Footer | `08-footer.html` | Site footer with brand, links |
| Brand guidelines | `09-brand-guidelines.html` | Design system reference |

### Component vocabulary (extracted from mockups)

- `<nav class="topnav">` — site-wide top navigation
- `<main class="post-layout">` / `<main class="article">` — page body wrapper
- `<header class="post-header">` / `<header class="article-header">` — page-level header (overline + title)
- `<aside class="left-nav">` — left side navigation (concept page TOC)
- `<aside class="sidebar">` — right-side taxonomy info card
- `<nav class="post-nav">` — post-internal nav (TOC for source pages)
- `<section class="hero">` — home page hero
- `<section class="category-section">` containing `<article class="category-card">` — home page category grid
- `<section class="featured-section">` — home page featured pages
- `<section class="tiers-section">` — home page tier-grouped browse
- `<nav class="quick-links">` — home page quick links
- `<footer class="site-footer">` — site-wide footer

### Design system layers (from mockup `<style>` blocks)

The mockups use a CSS `@layer` cascade:

```css
@layer tokens, reset, base, layout, components, utilities;
```

- **tokens** — type scale (perfect fourth, fluid via `clamp()`), spacing scale, OKLCH color palette, theme tokens (day/night), font stack
- **reset** — normalization
- **base** — element-level styles (body, headings, links, etc.)
- **layout** — Every-Layout primitives (Stack, Cluster, Sidebar, etc.)
- **components** — page-type-specific components
- **utilities** — single-purpose modifiers

### Trust-bar mockup discrepancy (flag)

Mockup `21-trust-bar-prototype.html` uses `data-trust="established"` on `<html>`. Per `0005-deployment-profiles-epistemic-architecture.md` §3.2, trust-level values are `corroborated | credible | provisional | speculative`; `established` belongs to the *canon-status* axis (§4.2). The mockup appears to predate the trust/canon axis split. **Action item**: confirm in M6 whether the mockup needs updating or whether the addendum should consolidate the value space.

---

## 4. State of the World

### What exists

- **Demo corpus** (`demo/sources/`, `demo/concepts/`, `demo/media/`): 91 source pages, 186 concept cards, 949 images, full bidirectional provenance, manifest, taxonomy, _analysis records.
- **Mockups** (`assets/mockups/`): 18 HTML files including 11 page-type mockups, 5 typography exploration mockups, 1 trust-bar prototype.
- **Demo pipeline CLI** (`tools/src/demo/`): fully implemented fetch → clean → rewrite → media → convert → frontmatter pipeline, gated behind `--features demo`.
- **Top-level CLI skeleton** (`tools/src/cli.rs`): `Build`, `Validate`, `Stats`, `Search`, `Dev`, `New`, `Extract`, `Merges` enum variants are declared; `main.rs` has stubs (`println!("…: not yet implemented")`).
- **Concept-card schema and process docs** (`docs/dev/concept-cards/`): v1.2 howto + parallel guide, formalized through three iterations of pilot/corpus extraction.
- **Architecture and design** (`docs/design/`): arch (0001, draft), demo design (0002, under-review), demo project plan (0003, active), brand guidelines (0004, draft), deployment profiles (0005, final).

### What's missing

- **`_layouts/`** — no Liquid templates. The mockups exist but aren't wired into Cobalt.
- **`_includes/`** — no shared Liquid partials.
- **`_cobalt.yml`** — no Cobalt configuration.
- **`_data/`** — no graph data. Templates can't render see-also, breadcrumbs, prerequisite chains, category indices, profile.json.
- **`assets/css/`, `assets/js/`, `assets/fonts/`** — no published design system assets. CSS lives inline in mockups; needs extraction.
- **`content/profile.yaml`** — no deployment profile config. The encyclopedia profile defaults need to be made explicit.
- **`haleiki build`** — implementation. The Rust modules `parser.rs`, `graph.rs`, `validator.rs`, `generator.rs`, `merger.rs`, `search.rs` are 4-5 line stubs; need real implementations.
- **`haleiki demo build`** — implementation. The wiring command that symlinks `demo/*` into `content/*` and invokes the build chain.
- **`haleiki demo serve`** — implementation. The dev-server command that wraps `cobalt serve` with file watching.

---

## 5. Milestones

Milestones are organized as **thin user-visible slices**: each one produces a runnable demo with progressively more polish. Earlier slices are uglier; later slices add fidelity.

### M1 — Cobalt configuration + minimal source/concept rendering

**Goal**: `haleiki demo build` produces a working `_site/` directory; `cobalt serve` runs; you can navigate to a source page and a concept page and see the markdown rendered with no styling. No graph data yet, no design system. Just bones.

**Tasks**:

- M1.1 — Author `_cobalt.yml` declaring two collections (`sources`, `concepts`), permalink patterns (`/source/{slug}/` and `/{slug}/`), output settings.
- M1.2 — Author `_layouts/default.liquid` (skeleton HTML5 wrapper, no CSS).
- M1.3 — Author `_layouts/source.liquid` and `_layouts/concept.liquid` (render frontmatter title + body markdown; minimal chrome).
- M1.4 — Implement `haleiki demo build` to symlink `demo/sources/`, `demo/concepts/`, `demo/media/` into `content/`, copy `demo/taxonomy.yaml` into `content/`, then shell out to `cobalt build`.
- M1.5 — Verify: `haleiki demo build && cobalt serve`; open browser; navigate `/source/dzogchen/` and `/ownership-rust/`; both render.

**Produces**: a runnable but bare-bones demo. Useful as the foundation; not yet visually meaningful.

### M2 — `haleiki build` core

**Goal**: `_data/` is populated with the relationship graph, derived data (see-also, breadcrumbs, category indices, provenance), and profile config. Templates can read `site.data.*` to render graph-derived content.

**Tasks**:

- M2.1 — Implement `parser.rs` to parse source pages and concept cards into structured Rust types (using `serde_yaml` for frontmatter, raw markdown for body).
- M2.2 — Implement `graph.rs` to construct a unified `petgraph` with both page types as nodes; edges per typed relationships and `derived_from` provenance.
- M2.3 — Implement `validator.rs` for: broken refs (cross-reference targets must exist), orphans (warn), duplicate slugs (error), unknown categories/tiers (error vs taxonomy.yaml), dangling provenance, slug-filename mismatch.
- M2.4 — Implement `generator.rs` to write `_data/graph.json`, `_data/categories/{category}.json`, `_data/breadcrumbs.json`, `_data/see-also/{slug}.json`, `_data/provenance/{slug}.json`, `_data/profile.json` (from `content/profile.yaml`).
- M2.5 — Wire `Command::Build` in `main.rs` to invoke the chain: parse → validate → graph → generate.
- M2.6 — Implement `Command::Stats` (graph stats: node counts, edge counts, orphan counts, etc.) and `Command::Validate` (validate-only, exits non-zero on errors).
- M2.7 — Verify against the demo corpus: `haleiki demo build` runs through the full pipeline, produces `_data/`, no validation errors.

**Produces**: graph data ready for templates. Pages still bare; data is in place.

### M3 — Design system CSS extraction

**Goal**: design system lives in `assets/css/` and `assets/fonts/`; Cobalt copies them to `_site/`; pages styled per the mockups' aesthetic.

**Tasks**:

- M3.1 — Extract token layer (type scale, spacing scale, OKLCH color, theme tokens) from mockups into `assets/css/01-tokens.css`.
- M3.2 — Extract reset and base layers into `assets/css/02-reset.css` and `assets/css/03-base.css`.
- M3.3 — Extract layout primitives (Every Layout style: Stack, Cluster, Sidebar, etc.) into `assets/css/04-layout.css`.
- M3.4 — Extract components (topnav, footer, sidebar, article, category-card, etc.) into `assets/css/05-components.css`.
- M3.5 — Extract utilities into `assets/css/06-utilities.css`.
- M3.6 — Author `assets/css/main.css` that imports all layer files in @layer order.
- M3.7 — Set up font loading: either local `assets/fonts/` files (DM Sans, Source Serif 4, IBM Plex Mono — or whichever final fonts are chosen per brand guidelines) or CDN with preconnect.
- M3.8 — Implement day/night theme toggle JS in `assets/js/theme.js` (sets `data-theme` attribute on root, persists to localStorage).
- M3.9 — Cobalt config copies `assets/css/`, `assets/js/`, `assets/fonts/` (and `assets/images/logo/`) to `_site/assets/`.
- M3.10 — Update `_layouts/default.liquid` to reference the published CSS and JS.

**Produces**: visually styled pages. Even the bare templates from M1 now look like the mockups.

### M4 — Source and concept page templates with full chrome

**Goal**: source pages and concept pages render with full chrome matching mockups 13 (concept) and 17 (news-post / source).

**Tasks**:

- M4.1 — Author `_includes/topnav.liquid` (site logo, primary nav, theme toggle).
- M4.2 — Author `_includes/footer.liquid` (per mockup 8: brand, links, attribution notice).
- M4.3 — Author `_includes/sidebar-taxonomy.liquid` (info card: category, subcategory, tier, subject_headings, aliases, derived_from / concepts_generated).
- M4.4 — Author `_includes/breadcrumb.liquid` (reads `site.data.breadcrumbs`).
- M4.5 — Author `_includes/see-also.liquid` (reads `site.data.see-also[page.slug]`, renders typed-relationship groups).
- M4.6 — Author `_includes/provenance.liquid` (renders source ↔ concept cross-links from `site.data.provenance[page.slug]`).
- M4.7 — Author `_layouts/source.liquid` (full source-page layout: topnav, breadcrumb, header with overline, body, post-nav, sidebar, footer).
- M4.8 — Author `_layouts/concept.liquid` (full concept-page layout: topnav, breadcrumb, header, body sections, sidebar, footer; left-nav TOC if mockup 13 specifies).
- M4.9 — Verify: dzogchen source page and ownership-rust concept page render with full mockup-equivalent chrome.

**Produces**: source and concept pages that look like the mockups.

### M5 — Home, category, and browse pages

**Goal**: navigation pages work. Home page is the entry; category pages list per-category; browse lists everything.

**Tasks**:

- M5.1 — Author `_layouts/home.liquid` per mockup 10: hero, category-section (10 cards), featured-section, tiers-section, quick-links. Featured-pages selection logic (e.g., highest in-degree concept cards from graph data, or hand-curated list).
- M5.2 — Author `_layouts/category.liquid` per mockup 11: per-category page listing all source pages and concept cards in that category. Generate one page per category from `_data/categories/{category}.json`.
- M5.3 — Author `_layouts/browse.liquid` per mockup 12: full corpus index with category/tier groupings.
- M5.4 — Cobalt generators (or build-time stubs) for category pages — one per declared category in taxonomy.yaml.
- M5.5 — Add home, browse, and per-category pages to topnav.
- M5.6 — Verify navigation: home → category → source/concept → see-also → another category, with breadcrumbs and back-navigation throughout.

**Produces**: complete navigable demo. Most users can now reach any page from the home.

### M6 — Profile system: trust bar and info card tinting

**Goal**: the encyclopedia profile is enabled. Trust bar renders at the top of pages tinted by `trust_level`. Info card header tints by trust. Article overline includes trust glyph and label.

**Tasks**:

- M6.1 — Author `content/profile.yaml` declaring `personality: encyclopedia`, trust enabled with default `credible`, canon disabled, page_types `[source, concept]`, relationship vocabulary.
- M6.2 — Update `generator.rs` to read `profile.yaml` and write `_data/profile.json` per addendum 0005 §8.1.
- M6.3 — Author `_includes/trust-bar.liquid` per mockup 21, conditional on `site.data.profile.trust.enabled`. Tinted by `page.trust_level`. Resolve the mockup's `data-trust="established"` discrepancy with the addendum's value space.
- M6.4 — Author `_includes/trust-glyph.liquid` for the four trust glyphs (solid filled circle, half-filled, outlined, dashed-outlined per addendum §3.5).
- M6.5 — Update `_includes/sidebar-taxonomy.liquid` to tint info card header by trust accent token.
- M6.6 — Update `_layouts/source.liquid` and `_layouts/concept.liquid` to set `data-trust` on `<html>` so per-trust CSS tokens apply.
- M6.7 — Update article overline in source/concept layouts to include trust glyph + label when trust is enabled.
- M6.8 — Author trust-token CSS layer (per addendum §5.1: 8 theme × trust combinations) in `assets/css/`.
- M6.9 — Verify: a `corroborated` page (e.g., `quantum-mechanics`) and a `provisional` page render with distinct trust accents; theme toggle preserves trust tinting.

**Produces**: profile-aware rendering. Demo deploys as encyclopedia, all pages communicate epistemic posture visually.

### M7 — Attribution and definitions pages

**Goal**: legally and editorially complete. Attribution page lists upstream sources and image credits. Definitions page explains trust levels.

**Tasks**:

- M7.1 — Implement `attribution.rs` in the Rust pre-build (or as part of `generator.rs`): read source-page frontmatter and `demo/media/manifest.json`, write `content/attribution.md` (or directly into `_data/` for a Liquid layout to consume).
- M7.2 — Author `_layouts/attribution.liquid` per mockup 14: list every source page with title/upstream URL/license/revision_id; list every image with Commons URL/author/caption/license; CC BY-SA general notice.
- M7.3 — Implement definitions page generation: from `profile.yaml`, write `content/about.md` (or `content/about/index.md`) per addendum §7.4. Conditional sections per axis enabled.
- M7.4 — Author `_layouts/about.liquid` for the definitions page rendering.
- M7.5 — Add Attribution and About to topnav.
- M7.6 — Verify: attribution page lists all 91 source pages and ~600 images correctly; about page explains trust levels per the encyclopedia dialect.

**Produces**: legal compliance + reader orientation. Demo is now ready for public eyes.

### M8 — Polish, dev experience, deployment readiness

**Goal**: shippable demo. Responsive, accessible, fast.

**Tasks**:

- M8.1 — Responsive audit. Mockups are designed for fluid layout; verify the rendered demo holds together at 360px–1440px viewports.
- M8.2 — Accessibility audit: semantic HTML, ARIA where needed, color contrast (especially trust-tinted headers), keyboard navigation, screen reader tested.
- M8.3 — Math notation rendering: pick MathJax or KaTeX, integrate (likely as a static asset bundled in `assets/js/`); verify physics articles (`quantum-mechanics`, `general-relativity`) render LaTeX correctly.
- M8.4 — Code block syntax highlighting: pick Prism or highlight.js, integrate; verify Rust articles (`rust-language`, `lfe-language`) render code correctly.
- M8.5 — Performance: minify CSS, defer fonts, lazy-load images below fold, optimize SVG.
- M8.6 — Implement `haleiki demo serve`: `haleiki demo build` followed by `cobalt serve` with file watching on `demo/sources/`, `demo/concepts/`, `_layouts/`, `_includes/`, `assets/css/`. Live-reload on change.
- M8.7 — Implement `haleiki dev` (the user-facing equivalent for non-demo deployments per arch doc §5.1).
- M8.8 — Update `README.md` with `cd demo && haleiki demo serve` quickstart; link to live demo URL when deployed.
- M8.9 — Author CI/CD GitHub Action per design doc 0002 §9 (build + deploy demo to GitHub Pages on push to main). This may be deferred to a follow-up phase if local-demo readiness is the milestone.

**Produces**: shippable static demo site. From here, the production deployment phase can begin.

---

## 6. Dependency Graph

```
M1 (cobalt setup)
  └── M2 (build core) ──── M4 (page templates) ──── M5 (nav pages)
  └── M3 (CSS extraction) ─┘                                  │
                                                              │
                                M6 (profile system) ──────────┤
                                                              │
                                M7 (attribution + about) ─────┤
                                                              │
                                M8 (polish + dev experience) ─┘
```

- M1 gates everything (no Cobalt = no rendering).
- M2 and M3 can run in parallel after M1.
- M4 needs M1 + M2 + M3.
- M5 needs M2 + M4.
- M6 needs M2 + M4 (profile data + per-trust styling hooks).
- M7 needs M2 + M5 (auto-generated pages plug into nav).
- M8 is final polish, can begin once M5 lands and continue alongside M6/M7.

---

## 7. Open Questions

1. **Trust value space** — the trust-bar mockup uses `data-trust="established"` (a canon value per addendum 0005). Either the mockup predates the trust/canon axis split, or the addendum's value space needs reconciling with the mockup vocabulary. Resolve in M6.

2. **Featured-pages selection** — the home page mockup has a "featured" section. Logic options: highest in-degree concept cards from the graph (data-driven), hand-curated list in `profile.yaml`, or both with manual override. M5 decision.

3. **Math/code rendering** — MathJax vs KaTeX (M8.3); Prism vs highlight.js (M8.4). Both choices have build-time-render variants that avoid runtime JS. Worth evaluating during M8.

4. **Font hosting** — local `assets/fonts/` (offline-friendly, slower-CSS but reproducible) vs CDN (faster TTFB, requires network at build time and at view). M3 decision.

5. **News section** — mockups 16-20 sketch a news/blog section. Out of scope for this phase but should be considered when the genre system (encyclopedia/conversation/worldbuilding/essay) lands. Forward-compatible decisions in M4-M5 templates avoid painting into a corner.

6. **Content vs demo** — the architecture has `content/` empty for users-who-fork; `demo/` is self-contained; `haleiki demo build` symlinks demo into content. Some milestones touch `content/` (e.g., profile.yaml in M6, attribution.md in M7). Decision: profile.yaml lives at `content/profile.yaml` (it's per-deployment); attribution and about are *generated* into `content/` (or directly into `_data/` with no markdown intermediary). Confirm in M6/M7.

7. **Per-page TOC** — concept-page mockup 13 has a `<aside class="left-nav">` for table of contents. Source-page mockups don't. Should TOCs be generated from heading structure? Manual? Per-page-type? M4 decision.

---

## 8. Phase Plan Summary

| Milestone | Focus | Dependency | User-visible outcome |
|-----------|-------|------------|----------------------|
| M1 | Cobalt setup | — | bare pages render |
| M2 | Build core | M1 | graph data populated |
| M3 | Design system | M1 | pages styled |
| M4 | Source & concept templates | M1+M2+M3 | full-chrome pages |
| M5 | Home, category, browse | M2+M4 | navigation works |
| M6 | Profile system | M2+M4 | trust-aware rendering |
| M7 | Attribution & about | M2+M5 | editorial completeness |
| M8 | Polish & dev experience | M4-M7 | shippable demo |

After M8: the local demo is complete. CI/CD deployment to GitHub Pages can be its own follow-up; search (Pagefind) can be its own; news section / writer profile are forward-design tracks.

---

## 9. References

- `docs/design/01-draft/0001-haleiki-architecture-document.md` — architecture, content model, build pipeline, layout system
- `docs/design/02-under-review/0002-haleiki-demo-site-design-document.md` — demo design, taxonomy, attribution
- `docs/design/05-active/0003-haleiki-demo-site-project-plan.md` — original demo project plan (now mostly executed via the demo pipeline phase)
- `docs/design/01-draft/0004-haleiki-brand-guidelines.md` — brand guidelines
- `docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md` — profile system, trust/canon axes, definitions page
- `docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md` (v1.2) — concept card schema
- `docs/dev/concept-cards/0002-a-guide-for-parallel-concept-card-extraction.md` (v1.2) — corpus extraction process
- `assets/mockups/` — visual contract for all page types
