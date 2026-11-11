# M5 Implementation Plan: Home, Category, and Browse Pages

## Context

M4 added full page chrome for source and concept pages. M5 adds the navigation layer: a real home page, per-category listing pages, and a full-corpus browse page. After M5, users can navigate the entire wiki from the home page through categories and back.

## Design Decisions (Resolved)

### Featured-pages selection: Option C (hybrid)
Pre-compute `_data/featured.json` in `generator.rs` with top-6 concept cards by in-degree (most-referenced-by-other-concepts). Override via optional `featured` list in `profile.yaml` if specified. Liquid can't do graph queries, so this must be pre-computed.

### Category page generation: Option A (stub markdown)
Extend `tools/src/demo/build.rs` to write `demo/_build/categories/<category>.md` for each category in taxonomy, with frontmatter pointing to `category.liquid` layout. Clean, fits existing pattern.

### Category page structure: Grouped by tier, then subcategory
Per mockup 11: entries grouped under Foundational → Intermediate → Advanced tier headings, with subcategory groups within each tier.

### Browse page structure: Grouped by category, then subcategory
Per mockup 12: left-nav with category jump links, main content grouped by category sections with subcategory groups. Compact list entries (title + tier badge), not full cards.

### Topnav update: Add Categories link
Link to `/browse/` (which functions as the categories index). No dropdown needed — the browse page IS the categories index.

### Breadcrumbs for nav pages
Breadcrumbs.json only contains source/concept slugs. For category and browse pages, the breadcrumb include will compute the trail directly from page frontmatter rather than data lookup. Home gets no breadcrumb; Browse gets "Home › Browse"; Category gets "Home › Browse › {Category Name}".

## Data Shape Verification

### categories.json ✅
Shape confirmed: `{ "slug": { name, description, sources[], concepts[], subcategories: { "subcat": { sources[], concepts[] } } } }`. All arrays sorted. Descriptions from taxonomy.yaml.

### graph.json ✅ (needs pre-computation for featured)
Shape: `{ nodes: [{slug, type, title}], edges: [{source, target, relationship}] }`. In-degree computable by counting edges where each node appears as `target`. Pre-compute in generator.rs → `featured.json`.

### profile.json ✅
Has `name: "Haleiki Demo Wiki"` for hero. Has `taxonomy.categories` and `taxonomy.tiers` for counts.

### breadcrumbs.json — GAP
Only contains source/concept slugs. Category and browse pages need breadcrumbs computed in-template. Not a data-shape bug — these are generated pages, not parsed content.

## CSS Class Gaps — FLAGGED

**Home page classes: ALL present ✅** (hero, category-card, featured-card, tier-card, quick-links, section-overline, section-heading)

**Category page classes: MANY MISSING ❌**
Missing from `05-components.css`:
- `.category-header`, `.category-overline`, `.category-title`, `.category-description`, `.category-stats`
- `.tier-section`, `.tier-header`, `.tier-name`, `.tier-count`
- `.subcategory-group`, `.subcategory-name`
- `.article-list`, `.article-card`, `.article-card-top`, `.article-card-title`, `.article-card-badges`
- `.tier-badge`, `.source-label`, `.article-card-desc`, `.article-card-tags`

**Browse page classes: MANY MISSING ❌**
Missing from `05-components.css`:
- `.page-header`, `.page-overline`, `.page-title`, `.page-stats`, `.page-description`
- `.article-item`, `.article-title-link`, `.article-meta-inline`, `.source-indicator`

**Resolution:** Extract these classes from mockups 11 and 12 into `05-components.css` as part of M5. This is an M3-style CSS extraction for the two new page types — a natural extension, not a scope change.

## Files to Create

| File | Purpose | Approx lines |
|------|---------|------|
| `demo/_layouts/home.liquid` | Home page with hero, categories, featured, tiers, quick links | ~120 |
| `demo/_layouts/category.liquid` | Per-category listing grouped by tier and subcategory | ~100 |
| `demo/_layouts/browse.liquid` | Full corpus index with category sections and left-nav | ~120 |

## Files to Modify

| File | Changes |
|------|---------|
| `assets/css/05-components.css` | Add category-page and browse-page component classes (~200 lines) |
| `tools/src/demo/build.rs` | Replace `generate_index()` with `generate_home_page()`, add `generate_category_pages()`, add `generate_browse_page()` |
| `tools/src/generator.rs` | Add `write_featured_json()` for pre-computed featured pages |
| `demo/_includes/topnav.liquid` | Update nav links |
| `demo/_includes/breadcrumb.liquid` | Add fallback for category/browse pages without data-lookup |

## Implementation Details

### Home page (`home.liquid`)

Reads `site.data.categories`, `site.data.featured`, `site.data.profile`. Renders:
1. **Hero**: brand mark, title from profile name, subtitle, stats (computed from category/concept counts), search placeholder
2. **Category section**: 10 cards from `site.data.categories`, each showing name, count (sources.size + concepts.size), description, top 3 articles
3. **Featured section**: 3-6 cards from `site.data.featured` with title, category tag, tier badge, teaser (first sentence of concept's Quick Definition — or just title if no teaser available)
4. **Tiers section**: 3 tier cards from `site.data.profile.taxonomy.tiers` with descriptions and counts
5. **Quick links**: Random Article (link to a specific page), Browse All, About, Attribution, GitHub

### Category pages (generated stubs)

`generate_category_pages()` in build.rs writes one `.md` per category:
```yaml
---
layout: category.liquid
title: "Tibetan Buddhism"
permalink: "/category/tibetan-buddhism/"
data:
  category_slug: "tibetan-buddhism"
---
```

The `category.liquid` layout reads `site.data.categories[page.data.category_slug]` to populate the listing.

### Browse page (generated stub)

`generate_browse_page()` writes `demo/_build/browse.md`:
```yaml
---
layout: browse.liquid
title: "Browse All Articles"
permalink: "/browse/"
---
```

The `browse.liquid` layout reads `site.data.categories` (all of them) to build the full index.

### Featured pages pre-computation

Add to `generator.rs`:
```rust
fn write_featured_json(corpus: &Corpus, graph: &HaleikiGraph, output_dir: &Path) -> Result<()>
```

Computes in-degree for each concept card (count edges where node is target, typed relationships only — not provenance). Takes top 6. Writes `featured.json`:
```json
[
  { "slug": "nyingma", "title": "Nyingma", "category": "tibetan-buddhism", "tier": "foundational", "url": "/nyingma/" },
  ...
]
```

### Breadcrumb update

The breadcrumb include currently does `site.data.breadcrumbs[page.slug]`. For category and browse pages (which aren't in the data), add a fallback:

```liquid
{% if site.data.breadcrumbs contains page.slug %}
  {# existing data-driven breadcrumb #}
{% elsif page.data.category_slug %}
  {# category page: Home › Browse › Category Name #}
{% elsif page.permalink == "/browse/" %}
  {# browse page: Home › Browse #}
{% endif %}
```

## Implementation Order

1. Extract category/browse CSS from mockups into `05-components.css`
2. Add `write_featured_json()` to `generator.rs`
3. Update `build.rs`: replace `generate_index()`, add `generate_category_pages()`, `generate_browse_page()`
4. Write `demo/_layouts/home.liquid`
5. Write `demo/_layouts/category.liquid`
6. Write `demo/_layouts/browse.liquid`
7. Update `demo/_includes/breadcrumb.liquid` (category/browse fallback)
8. Update `demo/_includes/topnav.liquid` (nav links)
9. Build and verify: `make check && haleiki demo build`
10. Visual verify: home → category → source → see-also → browse navigation flow

## Verification

- `make check` passes
- `haleiki demo build` succeeds
- `/` renders home page with hero, 10 category cards, featured articles, tier cards, quick links
- `/category/tibetan-buddhism/` renders category listing grouped by tier, then subcategory
- `/browse/` renders full corpus index with category sections and left-nav
- Navigation flow: home → click category → click article → breadcrumb back → browse → click different category
- Breadcrumbs correct on all page types

## Open Questions (For Review)

1. **Featured page teaser text**: The mockup shows a 2-3 line teaser per featured card. Where does this text come from? Options: (a) first sentence of concept body (requires parsing markdown at generation time), (b) the concept's Quick Definition section, (c) just the concept name + category + tier (no teaser). Recommend (c) for M5 simplicity — teaser text can be a polish item.

2. **Category page sidebar**: Mockup 11 has a right sidebar with category metadata (article count, subcategories, related categories, source breakdown). This reuses the `sidebar-taxonomy` include pattern. Should M5 include this, or defer to polish? Recommend include — the data is available and the include pattern exists.

## Approximate Scope

- **CSS**: ~200 lines added to `05-components.css`
- **Layouts**: ~340 lines across 3 new layouts
- **Rust**: ~80 lines (build.rs page generation + generator.rs featured computation)
- **Include updates**: ~20 lines (breadcrumb fallback + topnav links)
- **Total**: ~640 lines
