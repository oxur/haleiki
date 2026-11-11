---
number: 1
title: "Haleiki — Architecture Document"
author: "\"Jane Rustacean\""
component: All
tags: [change-me]
created: 2026-03-17
updated: 2026-03-18
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# Haleiki — Architecture Document

**House of Knowledge · A Read-Only Knowledge Base Framework**

Version 0.3 · March 2026 · Living Document

---

## 1. Project Identity

**Name:** Haleiki (Hawaiian: *hale* "house" + *ʻike* "knowledge")

**What it is:** A static-site knowledge base framework — a beautifully designed, read-only wiki for structured knowledge. No logins, no edits, no comments. Content goes in as source pages and concept cards; a searchable, navigable, interlinked knowledge base comes out.

**What it is not:** A CMS. A wiki engine. A documentation generator. It is a *knowledge publication tool* — closer to an encyclopedia than a wiki in philosophy, closer to a wiki than an encyclopedia in navigation and UX.

**Design lineage:** Wikipedia's information architecture, modernized. Warm/cool dual-theme aesthetic. Design system principles from the Cowboys & Beans design manifesto (geometric scales, OKLCH color, semantic tokens, fluid typography, Every Layout compositional primitives).

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    AUTHORING LAYER                      │
│                                                         │
│  Source Pages (.md with frontmatter)                    │
│       ↓                                                 │
│  Published as-is as first-class wiki pages              │
│       ↓                                                 │
│  Concept Extraction (v3 methodology, AI-driven)         │
│       ↓                                                 │
│  Concept Cards generated, merged, or augmented          │
│  Analysis Records persisted for future merging          │
└───────────────────────┬─────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────┐
│                   PRE-BUILD LAYER                       │
│                  (Rust CLI tool)                        │
│                                                         │
│  1. Parse ALL content (source pages + concept cards)    │
│  2. Build unified relationship graph (petgraph)         │
│  3. Validate: broken links, orphans, circular deps,     │
│     metadata conflicts (user vs. AI classification)     │
│  4. Compute derived data:                               │
│     - "See Also" groups (typed relationships)           │
│     - Breadcrumb paths (category hierarchy)             │
│     - Prerequisite chains (topological sort)            │
│     - Category indices (both page types)                │
│     - Source ↔ Concept provenance links                 │
│  5. Write graph-derived JSON → _data/ directory         │
│  6. Build search index (JSON for client-side search)    │
└───────────────────────┬─────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────┐
│                    BUILD LAYER                          │
│                     (Cobalt)                            │
│                                                         │
│  1. Reads source pages as collection items              │
│  2. Reads concept cards as collection items             │
│  3. Reads graph-derived JSON as data files              │
│  4. Applies Liquid templates:                           │
│     - Source page (full narrative + taxonomy sidebar)   │
│     - Concept page (structured article + taxonomy)      │
│     - Category listing page                             │
│     - Browse/index page                                 │
│     - Home page                                         │
│  5. Copies static assets (CSS, JS, fonts, search index) │
│  6. Outputs static HTML site                            │
└───────────────────────┬─────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────┐
│                   RUNTIME LAYER                         │
│               (Static files only)                       │
│                                                         │
│  - HTML pages (pre-rendered, zero server logic)         │
│  - CSS (design system, day/night themes)                │
│  - JS (search, theme toggle — minimal, progressive)     │
│  - Search index (compressed JSON, loaded on demand)     │
└─────────────────────────────────────────────────────────┘
```

---

## 3. Content Model

Haleiki has two first-class page types. Both are full wiki citizens: classified, searchable, navigable, and rendered with the complete Haleiki chrome (navigation, search, taxonomy sidebar, theming).

### 3.1 Source Pages

Source pages are authored Markdown documents — guides, tutorials, essays, reference material, chapters — published as-is in the wiki. They serve dual roles: they are readable content in their own right, and they are the raw material from which concept cards are extracted.

#### Source Page Frontmatter

```yaml
---
# === CORE IDENTIFICATION ===
title: "Understanding Ownership in Rust"
slug: "understanding-ownership-in-rust"
page_type: "source"

# === CLASSIFICATION (user-provided) ===
category: "memory-model"
subcategory: "ownership-system"
tier: "foundational"
keywords:
  - "ownership"
  - "memory safety"
  - "move semantics"
tags:
  - "tutorial"
  - "beginner-friendly"

# === AUTHORSHIP ===
author: "Jane Rustacean"
date: "2026-03-15"
updated: "2026-03-17"

# === ORIGINAL SOURCE (if adapted from external material) ===
original_source:
  title: "The Rust Programming Language"
  url: "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html"
  license: "MIT/Apache-2.0"

# === EXTRACTION STATUS ===
extraction_status: "complete"         # pending | in-progress | complete | skipped
concepts_generated:                   # Populated by extraction pipeline
  - "ownership"
  - "move-semantics"
  - "scope-based-dropping"

# === METADATA ===
status: "published"                   # draft | review | published
---
```

#### Source Page Body

The body is freeform Markdown — the author's prose, published as written. No standardized section structure is imposed.

### 3.2 Concept Cards (Generated Content)

Concept cards are atomic knowledge units generated from source pages via the v3 extraction methodology. Each captures a single concept in a structured, queryable format optimized for graph navigation and quick reference.

#### Concept Card Frontmatter

```yaml
---
# === CORE IDENTIFICATION ===
concept: "Ownership"
slug: "ownership"
page_type: "concept"

# === CLASSIFICATION ===
category: "memory-model"
subcategory: "ownership-system"
tier: "foundational"
keywords:
  - "move semantics"
  - "RAII"
  - "resource management"

# === PROVENANCE ===
derived_from:                         # Source pages (Haleiki wiki pages)
  - slug: "understanding-ownership-in-rust"
    title: "Understanding Ownership in Rust"
  - slug: "rustonomicon-ownership"
    title: "Rustonomicon: Ownership"
external_references:                  # Non-wiki sources (books, RFCs, docs)
  - title: "The Rust Programming Language"
    chapter: "Understanding Ownership"
    url: "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS (authority control) ===
aliases:
  - "ownership system"
  - "ownership model"
  - "Rust ownership"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "variable-binding"
  - "stack-and-heap"
extends:
  - "raii"
related:
  - "borrowing"
  - "lifetimes"
  - "clone-and-copy"
contrasts_with:
  - "garbage-collection"
  - "manual-memory-management"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is ownership in Rust?"
  - "How does Rust manage memory without a garbage collector?"
  - "What are the three rules of ownership?"

# === METADATA ===
stable_since: "1.0"
status: "published"
last_reviewed: "2026-03-17"
---
```

#### Concept Card Body Sections

| Section | Required | Purpose |
|---------|----------|---------|
| Quick Definition | Yes | 1–2 sentence accessible summary |
| Core Definition | Yes | Authoritative technical definition with sources |
| Prerequisites | Yes | What to know first, with rationale |
| Key Properties | Recommended | Enumerated defining characteristics |
| Construction / Recognition | If applicable | Procedural knowledge |
| Context & Application | Recommended | When, where, why this concept matters |
| Examples | Yes | Concrete examples with source citations |
| Relationships | Recommended | Prose elaboration of typed relationships |
| Common Errors | If applicable | Procedural mistakes |
| Common Confusions | If applicable | Conceptual misunderstandings |

### 3.3 Concept Analysis Records

When the extraction pipeline analyzes a source page and generates concept cards, the analysis itself is persisted. This enables future merging, auditing, and re-extraction.

```
_analysis/
├── understanding-ownership-in-rust/
│   ├── analysis.json           # Full extraction analysis
│   ├── concept-map.json        # Concepts identified, scoped, linked
│   └── merge-log.json          # History of merges with existing concepts
```

#### Analysis Record Schema

```json
{
  "source_slug": "understanding-ownership-in-rust",
  "analyzed_at": "2026-03-17T14:30:00Z",
  "analyzer": "claude-opus-4",
  "concepts_identified": [
    {
      "slug": "ownership",
      "action": "merged",
      "confidence": "high",
      "merge_target": "ownership",
      "merge_rationale": "Existing concept covers same scope. New source adds examples.",
      "sections_augmented": ["examples", "context-application"],
      "sections_unchanged": ["quick-definition", "core-definition"]
    },
    {
      "slug": "scope-based-dropping",
      "action": "created",
      "confidence": "high",
      "rationale": "No existing concept covers deterministic destruction as independent topic."
    }
  ],
  "classification_generated": {
    "category": "memory-model",
    "tier": "foundational"
  },
  "conflicts": []
}
```

### 3.4 Concept Merging Pipeline

When a new source covers an existing concept:

1. **Fuzzy-match** against existing slugs + aliases
2. **No match** → CREATE new concept card
3. **Strong match, same scope** → MERGE automatically (add source, augment body, log)
4. **Overlapping scope** → FLAG for human review (save pending merge)
5. **Same name, different concept** → CREATE with disambiguation

#### Metadata Conflict Resolution

| Priority | Resolution | Config value |
|----------|-----------|--------------|
| Default | User-provided metadata wins; AI disagreements are warnings | `user-first` |
| Alternative | AI classification wins; user overrides are warnings | `ai-first` |
| Strict | Any disagreement blocks the build | `strict` |

### 3.5 Taxonomy

Defined per-deployment in `content/taxonomy.yaml`. Each deployment chooses its own categories and tiers. Categories support optional subcategories for finer-grained classification within a category.

Example (multi-domain knowledge base):

```
Categories:
  tibetan-buddhism, theoretical-physics, mathematics, music,
  programming-languages, artificial-intelligence, philosophy-of-mind,
  linguistics, botany, geology

Tiers:
  foundational, intermediate, advanced
```

Subcategories are optional and free-form — they are not enumerated in the taxonomy file but validated against usage patterns during `haleiki validate`.

### 3.6 Media Convention

Media (images, diagrams, SVGs) lives in a shared directory organized by source slug. Any page — source or concept — can reference any media file.

```
content/
├── media/
│   ├── garbage-collection/
│   │   └── Mark-and-sweep.svg
│   └── smart-pointer/
│       └── Shared-ptr-ref-count.svg
├── sources/
│   └── garbage-collection.md      # References: ../media/garbage-collection/Mark-and-sweep.svg
└── concepts/
    └── mark-and-sweep.md          # Can also reference: ../media/garbage-collection/Mark-and-sweep.svg
```

This keeps media enumerable (easy to find all images across the site), separates content from assets, and allows concept cards to reference media from their source pages without co-location.

### 3.7 Directory Structure

```
haleiki/
├── content/
│   ├── sources/                    # Source pages (authored, published as-is)
│   ├── concepts/                   # Concept cards (generated)
│   ├── media/                      # Shared media organized by source slug
│   └── taxonomy.yaml
├── _analysis/                      # Persisted extraction metadata (framework-level)
├── _data/                          # Graph-derived data (generated by pre-build)
│   ├── graph.json
│   ├── see-also/                   # Per-page relationship data
│   ├── provenance/                 # Source ↔ Concept links
│   ├── categories/
│   ├── breadcrumbs.json
│   └── search-index.json
├── _layouts/                       # Cobalt Liquid templates
│   ├── default.liquid
│   ├── concept.liquid              # Concept card → structured article
│   ├── source.liquid               # Source page → narrative article
│   ├── category.liquid
│   ├── browse.liquid
│   └── home.liquid
├── _includes/
│   ├── topnav.liquid
│   ├── sidebar-taxonomy.liquid     # Info card (both page types)
│   ├── sidebar-toc.liquid
│   ├── see-also.liquid
│   ├── breadcrumb.liquid
│   ├── provenance.liquid           # Source ↔ Concept cross-links
│   └── search.liquid
├── assets/
│   ├── css/
│   ├── js/
│   └── fonts/
├── tools/                          # Pre-build Rust CLI
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── parser.rs               # Both page types
│       ├── graph.rs
│       ├── validator.rs            # Including conflict detection
│       ├── generator.rs
│       ├── merger.rs               # Concept merging logic
│       └── search.rs
└── _cobalt.yml
```

---

## 4. Design System

### 4.1 Principles (from the Manifesto)

1. **Multiply, never add** — geometric type scale (perfect fourth, 1.333)
2. **Perception is the only coordinate system** — OKLCH color, `rem`/`ch` units
3. **Name the role, not the value** — semantic tokens over primitives in components
4. **Constraint liberates** — 3 type families, 9 spacing values, constrained palette
5. **Unity is not uniformity** — day/night themes from same token architecture
6. **Proximity carries meaning** — spacing as syntax (tighter within, looser between)
7. **Design the invariants** — tokens and scales ARE the design; pages are instances

### 4.2 Type Stack

| Role | Typeface | Usage |
|------|----------|-------|
| Display / Headings | Red Hat Display | Article titles, section heads, nav, UI labels |
| Body / Long-form | Newsreader | Article prose, definitions, examples |
| Code / Mono | IBM Plex Mono | Code blocks, inline code, metadata |

**Scale:** Perfect fourth (1.333), fluid via `clamp()` between 360px and 1440px viewports.

### 4.3 Color Architecture

Two complete semantic theme mappings over shared primitive scales. All color in OKLCH.

| Theme | Surface mood | Accent family | Hue anchor |
|-------|-------------|---------------|------------|
| Day | Warm ochre / sandstone | Terracotta | ~45° |
| Night | Deep oxide | Amber | ~70° |

Warm neutrals (low chroma at warm hue), never pure gray.

### 4.4 Layout System

Every Layout primitives + Utopia fluid spacing. Zero media queries for layout.

### 4.5 Page Type Visual Differentiation

| Element | Concept Page | Source Page |
|---------|-------------|-------------|
| Overline | Category label | "Source · {tags}" |
| Body structure | Standardized sections | Freeform authored prose |
| Provenance section | "Derived from" → source pages | "Concepts in this source" → concept pages |
| See Also | Graph-derived typed relationships | Related sources + extracted concepts |

---

## 5. Pre-Build Tool (Rust CLI)

### 5.1 Commands

```bash
haleiki build                    # Full pipeline (parse, graph, validate, generate JSON)
haleiki validate                 # Validation only (CI-friendly)
haleiki stats                    # Graph statistics and health
haleiki search                   # Generate search index (wraps npx pagefind --site _site)
haleiki dev                      # Serve + watch
haleiki new source "Title"       # Scaffold source page
haleiki new concept "Name"       # Scaffold concept card
haleiki extract <source.md>      # Extract concepts from source (AI)
haleiki merges pending           # List pending concept merges
haleiki merges accept <slug>     # Accept a pending merge
```

### 5.2 Graph Construction

Unified graph with both page types as nodes. Source ↔ Concept provenance edges. Source relatedness computed via shared concepts.

### 5.3 Validation

Broken refs, circular deps, orphans, duplicate slugs, unknown categories, metadata conflicts, dangling provenance, stale extractions.

---

## 6. Search

Pagefind (Rust-based, static-site-native). Faceted by category, tier, and page type. `⌘K` activation. Both source pages and concept pages indexed.

### 6.1 Build-Time Integration

Cobalt templates mark indexable content with `data-pagefind-body` on article content areas and `data-pagefind-filter` on taxonomy elements (category, tier, page type) to enable faceted search. After Cobalt renders HTML, `npx pagefind --site _site` generates the search index.

The `haleiki search` CLI command wraps this: it runs `npx pagefind --site _site` to regenerate the search index.

### 6.2 Runtime

A minimal `assets/js/search.js` imports Pagefind's client library, wires it to a search input (activated by `⌘K`), and renders results into a dropdown or modal. Results can be grouped/filtered by page type, category, or tier using the facets tagged at build time. The search UX is custom (matching the Haleiki design system), not Pagefind's default UI.

All client-side, no server, works on GitHub Pages.

---

## 7. Cobalt Configuration

Two collections (`concepts`, `sources`). Flat URLs for concepts (`/{slug}/`), prefixed for sources (`/source/{slug}/`).

---

## 8. Build Pipeline

```bash
# Extraction (one-time / on-demand)
haleiki extract content/sources/my-article.md --merge

# Build
haleiki build && cobalt build && npx pagefind --site _site

# Dev
haleiki dev
```

---

## 9. Open Questions

1. Should `haleiki extract` call the Anthropic API directly, or remain a separate Claude Code workflow?
2. Rust edition/version tracking policy for concept cards?
3. ~~Image/diagram strategy~~ — **Resolved:** Shared `content/media/` directory, organized by source slug. See section 3.6. SVGs are served as-is (resolution-independent). Raster images are stored at a configurable max width. Future: consider post-processing SVGs to replace hardcoded colors with CSS custom properties for theme adaptation.
4. i18n: future concern? Account for it now in URL structure?
5. When source pages are updated post-extraction, is re-extraction automatic or manual?
6. Should generated concept cards be hand-editable? How to protect manual edits from re-extraction?

---

## 10. Phase Plan

### Phase 0: Demo Site (in progress, parallel with Phase 1)

- [ ] Demo fetch pipeline: Wikimedia + MediaWiki API content acquisition
- [ ] HTML cleaning, link rewriting, Markdown conversion
- [ ] Demo build integration (`haleiki demo build` → content/ → `haleiki build` → Cobalt → Pagefind)
- [ ] 90 source pages across 10 categories (fetched from Wikipedia, Wikibooks, Rigpa Wiki)
- [ ] CI/CD: GitHub Actions → GitHub Pages deployment
- [ ] See design doc 0002 (demo site) and 0003 (project plan) for details

### Phase 1: Foundation

- [ ] Pre-build CLI (Rust): parsing, validation, graph, JSON generation
- [ ] Design system CSS (tokens, themes, layout primitives, components)
- [ ] Cobalt templates (concept, source, category, browse, home)
- [ ] 3–5 seed source pages + 5–10 seed concept cards

### Phase 2: Search & Polish

- [ ] Pagefind integration + search UX
- [ ] Responsive refinement + accessibility audit
- [ ] Performance optimization

### Phase 3: Extraction Pipeline

- [ ] AI-driven concept extraction (v3 methodology)
- [ ] Merging pipeline + conflict resolution
- [ ] Analysis record persistence

### Phase 4: Content Scale

- [ ] 50+ concepts, 20+ source pages
- [ ] Learning paths from prerequisite graph

### Phase 5: Framework Extraction

- [ ] Reusable framework (not Rust-specific)
- [ ] Open source publication
