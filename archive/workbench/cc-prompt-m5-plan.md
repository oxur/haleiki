# M5 Implementation Plan: Home, Category, and Browse Pages

Pre-implementation planning pass for milestone M5 of Phase 6. Same pattern as M1–M4 plans: draft the implementation plan as a design doc, Duncan and I will review, then you execute.

**This pass is plan-only — do not implement the templates yet.** Land the plan doc, report what's in it, and surface the design decisions you've made or want to surface for review. We'll iterate on the plan if needed before kicking off execution.

---

## Reference docs

- `docs/design/06-final/0006-...site-build...project-plan.md` § M5 — milestone scope and task breakdown
- `docs/design/01-draft/0001-haleiki-architecture-document.md` § 4 (design system principles), § 4.5 (page-type visual differentiation)
- `docs/design/06-final/0007-m2-implementation-plan-haleiki-build-core.md` and the actual generated files at `demo/_build/_data/*.json` — confirm the data shapes M5 will read (`categories.json`, `graph.json`, `profile.json`, `breadcrumbs.json`)
- `assets/mockups/` — visual contract:
  - **`10-home-page.html`** — primary reference for home-page chrome (hero, category cards, featured pages, tier sections, quick links)
  - **`11-category-page.html`** — primary reference for per-category listing pages
  - **`12-browse-page.html`** — primary reference for the full-corpus browse page
  - `08-footer.html`, `13-concept-page.html`, `17-news-post.html` — chrome that should be reused via M4's includes
- `assets/css/05-components.css` — **the existing class vocabulary M3 produced**. The M5 templates **MUST** use these existing class names (`.hero`, `.category-section`, `.category-card`, `.featured-section`, `.featured-card`, `.tiers-section`, `.quick-links`, `.browse-grid`, etc.). If a class is in the mockups but missing from `05-components.css`, flag the gap rather than silently inventing.
- M4's `_includes/topnav.liquid`, `_includes/footer.liquid` — reuse, don't reinvent. M5 may need to update topnav (Categories dropdown population) but otherwise composes existing includes.

---

## Scope (from project plan § M5)

Goal: navigation pages work. Home page is the entry; category pages list per-category; browse lists everything. After M5, a user can land on the home page, click into a category, click into a source/concept page, click see-also to another, click breadcrumbs back, and land on browse to scan the entire corpus.

Tasks per the project plan (M5.1–M5.6):

- **M5.1** — Author `_layouts/home.liquid` per mockup 10: hero, category-section (10 cards), featured-section, tiers-section, quick-links
- **M5.2** — Author `_layouts/category.liquid` per mockup 11: per-category page listing all source pages and concept cards in that category
- **M5.3** — Author `_layouts/browse.liquid` per mockup 12: full corpus index with category/tier groupings
- **M5.4** — Cobalt generators (or build-time stubs) for category pages — one per declared category in `taxonomy.yaml`
- **M5.5** — Add home, browse, and per-category pages to topnav (update `_includes/topnav.liquid` from M4)
- **M5.6** — Verify navigation: home → category → source/concept → see-also → another category, with breadcrumbs and back-navigation throughout

---

## Plan document requirements

Write the implementation plan to `docs/dev/0005-m5-implementation-plan-home-category-browse-pages.md`. If the project's plan-doc convention has settled on `docs/design/05-active/` under ODM lifecycle (per M3 and M4 patterns), mirror there as well — but pick one as canonical and note in both that they are mirrors.

The plan should cover:

### 1. Files to create / modify

A clear table of all new and modified files, like the M1–M4 plans. Approximate line counts where reasonable. Should include:

- The three new layouts (`home.liquid`, `category.liquid`, `browse.liquid`)
- Updates to `_includes/topnav.liquid` (categories dropdown / nav links)
- Possibly updates to `tools/src/demo/build.rs` to generate per-category stub pages (see § 4)
- The home-page index (currently a stub `generate_index()` — replace with the real home page wired to `home.liquid`)
- Possibly profile.yaml additions if featured-pages selection uses a hand-curated slot

### 2. Data shape verification

Before authoring layouts, walk through each `site.data.*` source the templates will read and verify the actual JSON shape (look at `demo/_build/_data/*.json`) matches what the layout needs:

- `categories.json` — confirm the `{name, description, sources, concepts, subcategories}` shape per category. Templates need to render: category name, description, total page count (sources + concepts), maybe a representative thumbnail/icon. Surface any shape mismatch.
- `graph.json` — for featured-page in-degree calculation, the template (or a build-time pre-computation) needs `{nodes, edges}`. If computing in-degree at template time is awkward in Liquid, propose a generator pre-computation step that writes `_data/featured.json` with the top-N pages by in-degree.
- `profile.json` — the home page's hero copy may reference profile name and description. Confirm the shape.
- `breadcrumbs.json` — category and browse pages need their own breadcrumb entries (e.g., Home / Categories / Music). The current `breadcrumbs.json` is keyed on slug — does it include entries for category pages or just source/concept slugs? If only source/concept, the breadcrumb include needs a fallback path computation for category and browse pages.

If any data shape diverges from what the layouts need, **surface it as a design call**, not as a silent template hack. We can extend `generator.rs` if the data shape needs to grow.

### 3. Featured-pages selection

The home-page mockup has a "Featured" section. Decide the selection logic:

- **Option A — Graph-derived (in-degree)**: pick top N concept cards by number of incoming typed-relationship edges (most-prerequisite-of, most-related-to). Data-driven, automatic, reflects actual graph centrality. Recommended default.
- **Option B — Hand-curated**: a list in `profile.yaml`. More control, requires manual curation.
- **Option C — Hybrid**: data-driven by default, override via profile.yaml's optional `featured: [...]` field.

**Recommend**: Option C. Hand-curated wins if specified; otherwise graph in-degree, top 6 (or whatever N matches the mockup's grid).

For implementation: if data-driven, this likely needs a generator pre-computation step in `generator.rs` that writes `_data/featured.json` with the top N — Liquid is awkward for graph queries. Plan should specify whether to add this or compute in template.

### 4. Category page generation

Each of the 10 categories needs its own page at `/category/<category>/`. Three options:

- **Option A — Stub markdown files**: extend `tools/src/demo/build.rs` to write `demo/_build/categories/<category>.md` for each category in `taxonomy.yaml`, with frontmatter `layout: category.liquid`, `permalink: /category/<category>/`, and a `category: <slug>` field. The `category.liquid` layout reads `site.data.categories[page.category]` to populate the listing. Clean, fits existing build-orchestrator pattern. Recommended.
- **Option B — Cobalt generator (if cobalt-bin supports it)**: some static-site generators have native data-driven page generation. Worth checking cobalt-bin's docs, but option A is the pragmatic path.
- **Option C — One markdown file per category, hand-authored**: tedious for 10, but might be needed if we want per-category prose at the top of each listing. Probably not worth it.

**Recommend**: Option A. Plan should sketch the build-orchestrator addition (~20 lines of Rust to enumerate categories from `taxonomy.yaml` and emit stub markdown).

If a per-category description or other prose is desired (currently `categories.json` has a `description` field from `taxonomy.yaml`), the layout reads it from `site.data.categories[page.category].description`.

### 5. Subcategory handling on category pages

Mockup 11 may or may not group entries by subcategory within a category page. Decide:

- **Option A — Flat list within category**: just show all sources + concepts in alphabetical order or by tier
- **Option B — Grouped by subcategory**: show sections per subcategory (e.g., "Teachers", "Schools", "Practice") with the entries grouped under each
- **Option C — Tabbed/filtered**: subcategory as filter UI (probably overkill for static demo)

The data is there in `categories.json` (`subcategories: { teachers: { sources, concepts }, ... }`). Pick based on what mockup 11 shows.

### 6. Browse page structure

Mockup 12 sets the visual contract. Decide what shape the browse page takes:

- All-in-one alphabetical list?
- Grouped by category, then tier?
- Grouped by tier, then category?
- Two columns (sources / concepts)?

Match the mockup, but flag in plan if the mockup's groupings don't match the data we have available.

### 7. URL structure

- Home: `/`
- Browse: `/browse/`
- Per-category: `/category/<category>/` (the slug, e.g., `/category/tibetan-buddhism/`)
- Per-subcategory: not needed for M5 unless the mockup has subcategory pages — flag if so

These URLs are referenced by M4's breadcrumb include and sidebar info card (which links category→`/category/<category>/`). M5 makes the URLs resolve.

### 8. Topnav update

The topnav from M4 has placeholder links for `/`, `/browse/`, `/about/`. Decide M5's update:

- Confirm `/` and `/browse/` are wired (M5 makes them exist)
- Add a Categories dropdown? Or just a link to `/browse/`? The mockup will guide.
- If a dropdown: read `site.data.profile.taxonomy.categories` (or `site.data.categories` keys) to populate.

### 9. Forward-compat for M6 (trust) and M7 (about/attribution)

- Trust glyphs on listings: when M6 lands trust styling, listings will want trust glyphs next to entries. M5 doesn't render trust, but the template structure should leave room (e.g., a `<span class="trust-glyph"></span>` placeholder slot per entry, or the M6 work just adds it without restructuring listings).
- Footer links to `/about/` and `/attribution/`: M4 already has these. M7 fills in the targets.

### 10. Out of scope (what M5 explicitly doesn't do)

- **Trust bar / canon badge / trust glyphs** — that's M6
- **Auto-generated definitions / about / attribution pages** — that's M7
- **Math/code rendering** — that's M8
- **Search / search bar functionality** — separate phase entirely (the topnav search slot stays a placeholder)
- **Per-subcategory pages** — the data supports them, but M5 stops at category-level. If subcategory pages are useful later, add as a small follow-up.
- **Pagination** — the corpus is 91 source pages + 186 concept cards, well within "render all on one page" territory. No pagination needed for the demo. Flag if any listing exceeds a reasonable scroll length.

---

## Open questions to surface in the plan

1. **Mockup vs CSS class alignment** — confirm every CSS class used by M5 is in `05-components.css`. If gaps exist, flag for either CSS-layer follow-up or template substitution.
2. **Featured-pages selection** — confirm Option C (hybrid) and the in-degree top-N logic, or surface alternative.
3. **Subcategory grouping on category pages** — confirm mockup 11 grouping pattern.
4. **Browse page grouping** — confirm mockup 12 structure.
5. **Generator pre-computation for featured pages** — propose adding `_data/featured.json` if in-template graph queries are awkward.
6. **Categories dropdown in topnav** — yes/no; if yes, what data source.
7. **Breadcrumbs for category and browse pages** — does `breadcrumbs.json` include them, or does the breadcrumb include compute them on-the-fly for non-content pages?
8. **Page-data shape mismatches** — surface anything from § 2 verification.

---

## Reporting

When the plan doc is landed, report:

1. **File path** — `docs/dev/0005-m5-implementation-plan-...` (or design-doc mirror).
2. **Doc structure** — section headings only.
3. **Design decisions you made** — featured-page selection, subcategory grouping, browse structure, category-page generation approach, topnav update.
4. **Design decisions you surfaced for review** — any open questions you couldn't resolve confidently.
5. **Data shape verification result** — for each of `categories.json`, `graph.json`, `profile.json`, `breadcrumbs.json`: confirm shape matches what the layouts need; flag mismatches.
6. **Class vocabulary check** — did every CSS class the mockups use have a corresponding entry in `05-components.css`? If not, list the gaps.
7. **Approximate scope** — line counts per layout (rough), build-orchestrator additions, total LOC estimate.
8. **Anything surprising** during the planning pass.

Don't commit the plan yet — Duncan will review and either approve as-is or ask for revisions before kicking off execution.

---

If you encounter a substantive design question that you can't resolve confidently from the reference docs and mockups, surface it in the plan with a "for review" note rather than picking silently. We'd rather you flag than guess.
