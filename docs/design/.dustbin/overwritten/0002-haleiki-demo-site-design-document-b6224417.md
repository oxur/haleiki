---
number: 2
title: "Haleiki Demo Site — Design Document"
author: "\"Wikipedia contributors\""
component: All
tags: [change-me]
created: 2026-03-17
updated: 2026-03-17
state: Overwritten
supersedes: null
superseded-by: null
version: 1.0
---

# Haleiki Demo Site — Design Document

**Wikipedia/Wikimedia-sourced demo content for GitHub Pages**

Version 0.1 · March 2026

---

## 1. Purpose

Ship a living demo site as part of the Haleiki repo, built from Wikipedia and Wikimedia content. This serves three goals:

1. **Showcase** — anyone who finds the repo sees a real, navigable knowledge base immediately
2. **Integration test** — the demo exercises templates, design system, graph logic, and search against real-world content with images, interlinks, and varied structure
3. **Development fixture** — a stable content corpus to develop against without hand-authoring dozens of pages

The demo is built and deployed to GitHub Pages via CI on every push to `main`.

---

## 2. Content Sources

Not all content comes from English Wikipedia. The Wikimedia ecosystem includes several projects, and the best article for a given topic may live on a different site. The manifest supports any Wikimedia REST API–compatible source.

### Supported Wikimedia Projects

| Project | API base | Typical content |
|---------|----------|-----------------|
| English Wikipedia | `en.wikipedia.org` | Encyclopedia articles |
| Wikibooks | `en.wikibooks.org` | Textbook-style chapters |
| Wikiversity | `en.wikiversity.org` | Lessons, learning resources |
| Wikimedia Commons | `commons.wikimedia.org` | Media files, gallery pages |
| Simple English Wikipedia | `simple.wikipedia.org` | Simplified explanations |

All use the same REST API pattern: `https://{project}/api/rest_v1/page/html/{title}`

### Topic Cluster

A tightly interlinked CS/systems topic cluster, chosen to exercise the graph features naturally:

**Core cluster (~10–12 articles):**

- Memory management, Garbage collection, Reference counting
- Stack-based memory allocation, Region-based memory management
- RAII (Resource acquisition is initialization)
- Smart pointer, Ownership type (may be thin — supplement from Wikibooks/Wikiversity)
- Pointer (computer programming), Dangling pointer
- Memory safety, Type safety

This gives natural prerequisite chains, contrasts-with relationships, and shared-concept overlap between source pages.

---

## 3. Manifest Format

A single YAML file drives the entire fetch pipeline. One entry per source article.

### `demo/manifest.yaml`

```yaml
# Haleiki Demo Site — Content Manifest
# All content sourced under CC BY-SA 4.0 (or compatible) from Wikimedia projects.

# Default settings (can be overridden per-article)
defaults:
  project: "en.wikipedia.org"
  license: "CC BY-SA 4.0"
  media:
    max_width: 1024            # Download thumbnail variant at this max pixel width
    formats:                   # Preferred formats in priority order
      - "svg"
      - "png"
      - "jpg"
    skip_patterns:             # Glob patterns for images to skip
      - "Flag_of_*"            # Country flags in infoboxes
      - "Wiki-*.svg"           # Wikipedia UI icons
      - "Commons-logo*"
      - "Ambox_*"              # Maintenance message box icons

# Classification defaults for the demo taxonomy
taxonomy:
  categories:
    - "memory-management"
    - "type-systems"
    - "programming-concepts"
    - "data-structures"
  tiers:
    - "foundational"
    - "intermediate"
    - "advanced"

# ─── Articles ───────────────────────────────────────────

articles:

  - title: "Memory management"
    slug: "memory-management"
    category: "memory-management"
    tier: "foundational"
    tags: ["overview", "foundational"]
    keywords: ["allocation", "deallocation", "memory model"]

  - title: "Garbage collection (computer science)"
    slug: "garbage-collection"
    category: "memory-management"
    tier: "intermediate"
    tags: ["runtime", "automatic-memory"]
    keywords: ["GC", "mark-and-sweep", "tracing"]

  - title: "Reference counting"
    slug: "reference-counting"
    category: "memory-management"
    tier: "intermediate"
    tags: ["runtime", "automatic-memory"]
    keywords: ["ref count", "weak reference", "cycle detection"]

  - title: "Stack-based memory allocation"
    slug: "stack-memory"
    category: "memory-management"
    tier: "foundational"
    tags: ["allocation"]
    keywords: ["stack frame", "LIFO", "automatic storage"]

  - title: "Region-based memory management"
    slug: "region-based-memory"
    category: "memory-management"
    tier: "advanced"
    tags: ["allocation", "research"]
    keywords: ["arena", "region inference", "Cyclone"]

  - title: "Resource acquisition is initialization"
    slug: "raii"
    category: "memory-management"
    tier: "intermediate"
    tags: ["pattern", "C++", "Rust"]
    keywords: ["RAII", "destructor", "scope-based cleanup"]

  - title: "Smart pointer"
    slug: "smart-pointer"
    category: "data-structures"
    tier: "intermediate"
    tags: ["pointer", "automatic-memory"]
    keywords: ["unique_ptr", "shared_ptr", "Box", "Rc"]

  - title: "Pointer (computer programming)"
    slug: "pointer"
    category: "programming-concepts"
    tier: "foundational"
    tags: ["fundamentals"]
    keywords: ["address", "dereference", "null pointer"]

  - title: "Dangling pointer"
    slug: "dangling-pointer"
    category: "memory-management"
    tier: "intermediate"
    tags: ["bug", "safety"]
    keywords: ["use-after-free", "wild pointer", "stale reference"]

  - title: "Memory safety"
    slug: "memory-safety"
    category: "type-systems"
    tier: "foundational"
    tags: ["safety", "verification"]
    keywords: ["buffer overflow", "bounds checking", "spatial safety"]

  - title: "Type safety"
    slug: "type-safety"
    category: "type-systems"
    tier: "foundational"
    tags: ["safety", "type-theory"]
    keywords: ["type checking", "strong typing", "soundness"]

  # ─── Cross-project sources ───────────────────────────

  - title: "Introduction to Computer Science/Memory Management"
    project: "en.wikibooks.org"          # Override default project
    slug: "wikibooks-memory-management"
    category: "memory-management"
    tier: "foundational"
    tags: ["tutorial", "beginner-friendly"]
    keywords: ["introduction", "textbook"]
    license: "CC BY-SA 3.0"              # Wikibooks may differ

  # Example: Wikiversity learning resource
  # - title: "Computer Science/Memory Management"
  #   project: "en.wikiversity.org"
  #   slug: "wikiversity-memory-management"
  #   category: "memory-management"
  #   tier: "foundational"
```

### Per-Article Schema

```yaml
# Required
title: string              # Exact Wikimedia page title (including parenthetical disambiguation)
slug: string               # Haleiki slug — becomes the URL and filename

# Classification (required for demo, maps to Haleiki taxonomy)
category: string
tier: string               # foundational | intermediate | advanced

# Optional
project: string            # Wikimedia project domain (overrides default)
license: string            # License (overrides default)
tags: [string]
keywords: [string]
media:
  skip_patterns: [string]  # Additional skip patterns for this article
  include: [string]        # Force-include specific filenames even if they match a skip pattern
  exclude: [string]        # Force-exclude specific filenames
```

---

## 4. Fetch & Convert Pipeline

A Python script (practical choice — good HTTP, HTML parsing, and Pandoc integration) that reads the manifest and produces Haleiki-ready source pages with local media.

### Pipeline Stages

```
manifest.yaml
    │
    ▼
┌──────────────────┐
│  1. FETCH HTML   │  Wikimedia REST API → raw HTML per article
└────────┬─────────┘
         │
    ▼
┌──────────────────┐
│  2. EXTRACT      │  Walk DOM: identify images, captions, infoboxes,
│     MEDIA        │  see-also sections, categories. Download images
│     METADATA     │  from Wikimedia CDN at configured resolution.
└────────┬─────────┘
         │
    ▼
┌──────────────────┐
│  3. CLEAN HTML   │  Strip Wikipedia chrome: navboxes, edit links,
│                  │  citation superscripts, amboxes, hatnotes,
│                  │  external link icons, [edit] spans.
│                  │  Rewrite image srcs to local relative paths.
│                  │  Rewrite internal wiki links:
│                  │    - Link targets in manifest → relative Haleiki links
│                  │    - Link targets not in manifest → full Wikipedia URLs
└────────┬─────────┘
         │
    ▼
┌──────────────────┐
│  4. CONVERT TO   │  Pandoc: cleaned HTML → Markdown
│     MARKDOWN     │  Post-process: fix image syntax, normalize headings,
│                  │  clean up Pandoc artifacts.
└────────┬─────────┘
         │
    ▼
┌──────────────────┐
│  5. INJECT       │  Prepend YAML frontmatter from manifest entry.
│     FRONTMATTER  │  Set page_type: "source", status: "published",
│                  │  populate original_source with Wikimedia URL.
│                  │  Set extraction_status: "pending".
└────────┬─────────┘
         │
    ▼
┌──────────────────┐
│  6. WRITE        │  Write .md to demo/sources/
│     OUTPUT       │  Write media to demo/media/{slug}/
│                  │  Write/update demo/media/manifest.json
└──────────────────┘
```

### Link Rewriting Strategy

This is critical for making the demo feel like a real interlinked knowledge base:

1. Parse every `<a>` in the fetched HTML
2. If `href` points to a Wikipedia/Wikimedia article whose title matches a manifest entry → rewrite to Haleiki source page URL (`/source/{slug}/`)
3. If `href` points to an article NOT in the manifest → rewrite to full absolute Wikimedia URL (opens externally)
4. Section anchors (`#fragment`) are preserved in both cases

This means internal navigation works within the demo cluster, and links outside the cluster gracefully fall through to Wikipedia.

### Image Handling

For each `<img>` / `<figure>` in the fetched HTML:

1. **Identify the source**: Parse the `src` — typically `//upload.wikimedia.org/wikipedia/commons/thumb/...`
2. **Check skip patterns**: Match filename against `skip_patterns` (global + per-article). Skip if matched (unless force-included).
3. **Select resolution**: Request the thumbnail at `max_width` pixels. For SVGs, download the original (they're resolution-independent).
4. **Download**: Fetch to `demo/media/{article-slug}/{filename}`
5. **Record metadata**: For each image, capture:
   - Original Wikimedia Commons URL
   - License (fetched from Commons API if available, or inherited from article)
   - Author/attribution (from Commons file page)
   - Caption text (from the `<figcaption>` or `alt` attribute)
   - Local path
6. **Rewrite in HTML** (before Pandoc conversion): Replace the `src` with a relative path `../media/{slug}/{filename}`

#### Media Manifest (`demo/media/manifest.json`)

```json
{
  "generated_at": "2026-03-17T14:30:00Z",
  "images": [
    {
      "local_path": "garbage-collection/Mark-and-sweep.svg",
      "original_url": "https://commons.wikimedia.org/wiki/File:Mark-and-sweep.svg",
      "commons_filename": "Mark-and-sweep.svg",
      "license": "CC BY-SA 3.0",
      "author": "Wikimedia Commons contributor",
      "caption": "Illustration of mark-and-sweep garbage collection",
      "source_article": "garbage-collection",
      "format": "svg",
      "size_bytes": 14320
    }
  ]
}
```

This manifest powers the automated attribution page in the built demo site.

---

## 5. Directory Structure

```
haleiki/
├── content/                        # Empty in repo — user's content goes here
│   ├── sources/                    #   .gitkeep
│   ├── concepts/                   #   .gitkeep
│   └── taxonomy.yaml               #   Starter template (not demo-specific)
│
├── demo/                           # Self-contained demo content
│   ├── manifest.yaml               # ⬆ What to fetch (checked in)
│   ├── sources/                    # Converted Wikipedia → Haleiki source pages
│   │   ├── memory-management.md
│   │   ├── garbage-collection.md
│   │   ├── reference-counting.md
│   │   └── ...
│   ├── concepts/                   # Hand-seeded or AI-extracted concept cards
│   │   ├── mark-and-sweep.md
│   │   ├── use-after-free.md
│   │   └── ...
│   ├── media/                      # Downloaded images, organized by source article
│   │   ├── manifest.json           # Image metadata for attribution
│   │   ├── garbage-collection/
│   │   │   ├── Mark-and-sweep.svg
│   │   │   └── Tracing-gc-phases.png
│   │   ├── smart-pointer/
│   │   │   └── Shared-ptr-ref-count.svg
│   │   └── ...
│   ├── taxonomy.yaml               # Demo-specific taxonomy
│   └── _analysis/                  # Demo extraction records (if AI extraction is run)
│
├── tools/
│   ├── demo-fetch/                 # The fetch + convert pipeline
│   │   ├── fetch.py                # Main script: reads manifest, fetches, converts
│   │   ├── requirements.txt        # requests, beautifulsoup4, pyyaml, etc.
│   │   ├── clean.py                # HTML cleaning / Wikipedia chrome stripping
│   │   ├── media.py                # Image download, metadata extraction, skip logic
│   │   ├── rewrite.py              # Link rewriting (internal ↔ external)
│   │   └── frontmatter.py          # YAML frontmatter injection
│   ├── src/                        # Haleiki CLI (Rust)
│   └── Cargo.toml
│
├── .github/
│   └── workflows/
│       └── demo-site.yml           # CI: build demo → deploy to Pages
│
└── _cobalt.yml
```

### How CI Wires Demo Content Into the Build

The demo content lives in `demo/` but Haleiki expects content in `content/`. At build time, CI symlinks (or copies) the demo directories into place:

```bash
# In CI (or a Makefile target)
ln -s ../demo/sources content/sources
ln -s ../demo/concepts content/concepts
ln -s ../demo/media content/media          # Or wherever the media convention lands
cp demo/taxonomy.yaml content/taxonomy.yaml
```

This keeps the repo structure clean: `content/` stays empty (ready for users who fork), and `demo/` is self-contained.

---

## 6. Generated Frontmatter

A fetched-and-converted Wikipedia article becomes a Haleiki source page. Here's what the generated frontmatter looks like:

```yaml
---
# === CORE IDENTIFICATION ===
title: "Garbage Collection (Computer Science)"
slug: "garbage-collection"
page_type: "source"

# === CLASSIFICATION ===
category: "memory-management"
tier: "intermediate"
keywords:
  - "GC"
  - "mark-and-sweep"
  - "tracing"
tags:
  - "runtime"
  - "automatic-memory"

# === AUTHORSHIP ===
author: "Wikipedia contributors"
date: "2026-03-17"                      # Date of fetch

# === ORIGINAL SOURCE ===
original_source:
  title: "Garbage collection (computer science)"
  project: "en.wikipedia.org"
  url: "https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)"
  license: "CC BY-SA 4.0"
  fetched_at: "2026-03-17T14:30:00Z"
  revision_id: 1234567890               # Wikipedia revision ID for reproducibility

# === EXTRACTION STATUS ===
extraction_status: "pending"
concepts_generated: []

# === METADATA ===
status: "published"
---
```

---

## 7. Licensing & Attribution

All Wikimedia content is CC BY-SA (3.0 or 4.0). The demo site must attribute properly.

### Automated Attribution Page

The build pipeline generates an `attribution.html` page from `demo/media/manifest.json` and the source page frontmatter. It lists:

- Every source article with its original Wikimedia URL and license
- Every image with its Commons URL, author, and license
- A general CC BY-SA notice for the demo content

### Per-Page Attribution

Each source page includes a "Source" line in the taxonomy sidebar linking to the original Wikimedia article, populated from `original_source` in the frontmatter.

---

## 8. CI/CD Pipeline

### `.github/workflows/demo-site.yml`

```yaml
name: Build & Deploy Demo Site

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]    # Build but don't deploy on PRs

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # ── Rust toolchain (for Haleiki CLI) ──
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      # ── Build Haleiki CLI ──
      - name: Build haleiki
        run: cargo build --release --manifest-path tools/Cargo.toml

      # ── Wire demo content into content/ ──
      - name: Prepare content
        run: |
          rm -rf content/sources content/concepts
          ln -s "$PWD/demo/sources" content/sources
          ln -s "$PWD/demo/concepts" content/concepts
          ln -s "$PWD/demo/media" content/media
          cp demo/taxonomy.yaml content/taxonomy.yaml

      # ── Pre-build: graph, validation, derived data ──
      - name: Haleiki build
        run: ./tools/target/release/haleiki build

      # ── Static site generation ──
      - name: Install Cobalt
        run: cargo install cobalt-bin --locked
      - name: Cobalt build
        run: cobalt build

      # ── Search index ──
      - name: Install Pagefind
        run: npx pagefind --site _site

      # ── Integrity checks ──
      - name: Validate demo
        run: |
          ./tools/target/release/haleiki validate
          echo "Graph stats:"
          ./tools/target/release/haleiki stats

      # ── Deploy (only on main) ──
      - name: Deploy to GitHub Pages
        if: github.ref == 'refs/heads/main'
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./_site
```

### What CI Validates

The `haleiki validate` step in CI catches regressions in the demo content:

- No broken internal links (all `slug` references resolve)
- No orphan pages (every page reachable from at least one other)
- No dangling provenance (concept → source links valid)
- No unknown categories or tiers
- Search index has expected entry count
- Media files referenced in Markdown actually exist

---

## 9. Refresh Strategy

Demo content is **committed to the repo**, not fetched during CI. This means:

- CI has no dependency on Wikimedia API availability
- Builds are reproducible
- No risk of Wikipedia edits breaking the demo unexpectedly

To refresh the demo content (manually, or on a schedule):

```bash
cd tools/demo-fetch
pip install -r requirements.txt
python fetch.py ../../demo/manifest.yaml --output ../../demo/
```

This overwrites `demo/sources/` and `demo/media/`. The diff is reviewed and committed like any other content change. The `revision_id` in each page's frontmatter makes it clear exactly which Wikipedia revision was fetched.

A scheduled GitHub Action could run this weekly/monthly and open a PR with the diff, but that's a Phase 2 nicety.

---

## 10. Framework Media Convention (Design Decision)

The demo forces us to establish the framework's image/media convention. Proposed:

### Option A: Shared media directory (recommended)

```
content/
├── sources/
│   └── garbage-collection.md      # References: ../media/garbage-collection/Mark-and-sweep.svg
├── concepts/
├── media/
│   ├── garbage-collection/
│   │   └── Mark-and-sweep.svg
│   └── smart-pointer/
│       └── Shared-ptr-ref-count.svg
└── taxonomy.yaml
```

**Pros:** Clean separation, easy to enumerate all media, simple asset pipeline (Cobalt copies `content/media/` to `_site/media/`). Media directory can be gitignored separately if needed.

**Cons:** Paths are relative and a bit long. Moving an article means updating media paths.

### Option B: Co-located (page bundles)

```
content/
├── sources/
│   └── garbage-collection/
│       ├── index.md
│       └── Mark-and-sweep.svg
```

**Pros:** Article + its media travel together. Simple relative paths (`./Mark-and-sweep.svg`).

**Cons:** Cobalt may not support page bundles natively. Mixes content and assets in the same directory. Harder to enumerate all media across the site.

**Recommendation:** Option A. It's more compatible with Cobalt, makes the media manifest straightforward, and the demo already uses this structure. Concept pages (which are generated) can reference the same shared media directory without needing to co-locate.

---

## 11. Open Questions

1. **Pandoc availability in CI** — install via apt, or bundle a binary? (`pandoc` is used by the fetch script, but fetch is offline — only the *results* are committed. So Pandoc is a dev dependency, not a CI dependency.)
2. **SVG theming** — should the build pipeline post-process SVGs to replace hardcoded colors with CSS custom properties? This would let diagrams adapt to day/night theme. Worth prototyping with one or two SVGs.
3. **Wikipedia infoboxes** — strip entirely, or convert to a structured sidebar? Stripping is simpler; converting exercises the sidebar component.
4. **Image captions in Markdown** — Pandoc produces `![caption](path)` syntax. Cobalt needs to render this as `<figure><img><figcaption>`. Verify this works or add a Cobalt plugin/template helper.
5. **Demo concept cards** — hand-author 5–8 concept cards for the demo, or run AI extraction? Hand-authoring is more predictable for Phase 1; AI extraction can wait for Phase 3.