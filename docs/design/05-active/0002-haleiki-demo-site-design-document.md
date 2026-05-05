---
number: 2
title: "Haleiki Demo Site — Design Document"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-03-17
updated: 2026-05-04
state: Active
supersedes: null
superseded-by: null
version: 1.2
---


# Haleiki Demo Site — Design Document

**Wikipedia/Wikimedia-sourced demo content for GitHub Pages**

Version 0.3 · March 2026

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

## 4. CLI Commands

The demo pipeline is part of the Haleiki CLI, gated behind the `demo` cargo feature flag. This keeps the core CLI lean — users who fork the repo for their own knowledge base don't compile `reqwest`, `tokio`, or any of the fetching machinery.

```bash
# Build with demo support
cargo build --features demo

# Install with demo support
cargo install haleiki --features demo
```

Everything lives under `haleiki demo`:

```bash
# ─── Fetch & Convert ─────────────────────────────────
haleiki demo fetch                   # Fetch all articles from manifest
haleiki demo fetch --article <slug>  # Fetch/refresh a single article
haleiki demo fetch --dry-run         # Show what would be fetched, don't write

# ─── Build ────────────────────────────────────────────
haleiki demo build                   # Wire demo/ into content/, run full build pipeline
haleiki demo serve                   # Build + local dev server with watch

# ─── Inspection ───────────────────────────────────────
haleiki demo status                  # Show manifest vs. on-disk state:
                                     #   which articles are fetched, stale, or missing
haleiki demo validate                # Run haleiki validate against demo content
haleiki demo attribution             # Generate/preview the attribution page

# ─── Maintenance ──────────────────────────────────────
haleiki demo clean                   # Remove all generated demo content (sources/, media/)
haleiki demo refresh                 # Clean + fetch all — full regeneration
```

### How `demo build` Works

`haleiki demo build` is a convenience wrapper that does what CI does:

1. Symlink (or copy) `demo/sources/` → `content/sources/`
2. Symlink (or copy) `demo/concepts/` → `content/concepts/`
3. Symlink (or copy) `demo/media/` → `content/media/`
4. Copy `demo/taxonomy.yaml` → `content/taxonomy.yaml`
5. Run the normal `haleiki build` pipeline (graph, validation, derived data)
6. Shell out to `cobalt build` and `pagefind` (or print instructions if not installed)

`haleiki demo serve` does the same but watches for changes and serves locally.

### Updated Architecture Doc CLI Section

These commands extend the existing CLI surface from the architecture document:

```bash
# ─── Existing commands (from architecture doc) ────────
haleiki build                    # Full pipeline
haleiki validate                 # Validation only (CI-friendly)
haleiki stats                    # Graph statistics and health
haleiki search                   # Search index only
haleiki dev                      # Serve + watch
haleiki new source "Title"       # Scaffold source page
haleiki new concept "Name"       # Scaffold concept card
haleiki extract <source.md>      # Extract concepts from source (AI)
haleiki merges --pending         # Review pending merges
haleiki merges --accept <slug>   # Accept a pending merge

# ─── New: demo subcommands ────────────────────────────
haleiki demo fetch [--article <slug>] [--dry-run]
haleiki demo build
haleiki demo serve
haleiki demo status
haleiki demo validate
haleiki demo attribution
haleiki demo clean
haleiki demo refresh
```

---

## 5. Fetch & Convert Pipeline

The `haleiki demo fetch` command reads the manifest and produces Haleiki-ready source pages with local media. All stages run in-process in Rust.

### Pipeline Stages

```
manifest.yaml
    │
    ▼
┌──────────────────┐
│  1. FETCH HTML   │  Wikimedia REST API → raw HTML per article
│                  │  (reqwest, async, respects rate limits)
└────────┬─────────┘
         │
    ▼
┌──────────────────┐
│  2. EXTRACT      │  Walk DOM: identify images, captions, infoboxes,
│     MEDIA        │  see-also sections, categories. Download images
│     METADATA     │  from Wikimedia CDN at configured resolution.
│                  │  (scraper crate for HTML parsing)
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
│  4. CONVERT TO   │  HTML → Markdown conversion.
│     MARKDOWN     │  Default: htmd crate (pure Rust, no external deps)
│                  │  Optional: --pandoc flag shells out to pandoc
│                  │  Post-process: fix image syntax, normalize headings.
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

### Rust Crate Dependencies (demo feature only)

All demo-specific dependencies are gated behind the `demo` cargo feature flag. The core CLI (`haleiki build`, `haleiki validate`, etc.) compiles without any of these.

| Crate | Purpose |
|-------|---------|
| `reqwest` (async + rustls) | HTTP client for Wikimedia API + CDN downloads |
| `tokio` | Async runtime for concurrent fetching |
| `scraper` | HTML parsing and DOM manipulation (CSS selector–based) |
| `htmd` | HTML → Markdown (pure Rust, avoids Pandoc dependency) |
| `globset` | Image skip pattern matching |
| `indicatif` | Progress bars for fetch operations |

Crates already in the core dependency tree (`serde`, `serde_yaml`, `clap`) are shared; no duplication.

The advantage of pure-Rust HTML→Markdown conversion is that `cargo install haleiki --features demo` gives you a fully working fetch pipeline with no external tools required. We start with `htmd` and support `--pandoc` as a quality fallback.

#### `Cargo.toml` Feature Configuration

```toml
[features]
default = []
demo = ["dep:reqwest", "dep:tokio", "dep:scraper", "dep:htmd", "dep:globset", "dep:indicatif"]

[dependencies]
# ─── Core (always compiled) ───
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1"
petgraph = "0.6"
# ...

# ─── Demo feature (optional) ───
reqwest = { version = "0.12", features = ["rustls-tls", "json"], optional = true }
tokio = { version = "1", features = ["full"], optional = true }
scraper = { version = "0.20", optional = true }
htmd = { version = "0.1", optional = true }
globset = { version = "0.4", optional = true }
indicatif = { version = "0.17", optional = true }
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
6. **Rewrite in HTML** (before Markdown conversion): Replace the `src` with a relative path `../media/{slug}/{filename}`

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

## 6. Directory Structure

```
haleiki/
├── content/                        # Empty in repo — user's content goes here
│   ├── sources/                    #   .gitkeep
│   ├── concepts/                   #   .gitkeep
│   └── taxonomy.yaml               #   Starter template (not demo-specific)
│
├── demo/                           # Self-contained demo content
│   ├── manifest.yaml               # What to fetch (checked in, drives the pipeline)
│   ├── sources/                    # Converted Wikimedia → Haleiki source pages
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
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                 # CLI entry point (clap)
│       ├── parser.rs               # Content parsing (both page types)
│       ├── graph.rs                # Graph construction + derived data
│       ├── validator.rs            # Validation including conflict detection
│       ├── generator.rs            # JSON generation for _data/
│       ├── merger.rs               # Concept merging logic
│       ├── search.rs               # Search index generation
│       └── demo/                   # Demo subcommand module
│           ├── mod.rs              # Subcommand dispatch
│           ├── manifest.rs         # Manifest parsing + validation
│           ├── fetch.rs            # Wikimedia API client + HTML fetching
│           ├── clean.rs            # HTML cleaning / Wikipedia chrome stripping
│           ├── media.rs            # Image download, metadata, skip logic
│           ├── rewrite.rs          # Link rewriting (internal ↔ external)
│           ├── convert.rs          # HTML → Markdown conversion
│           ├── frontmatter.rs      # YAML frontmatter injection
│           └── attribution.rs      # Attribution page generation
│
├── .github/
│   └── workflows/
│       └── demo-site.yml           # CI: build demo → deploy to Pages
│
└── _cobalt.yml
```

### How CI Wires Demo Content Into the Build

`haleiki demo build` handles this automatically. Under the hood it symlinks the demo directories into `content/`:

```bash
# What `haleiki demo build` does internally:
#   1. Symlink demo/sources/  → content/sources/
#   2. Symlink demo/concepts/ → content/concepts/
#   3. Symlink demo/media/    → content/media/
#   4. Copy    demo/taxonomy.yaml → content/taxonomy.yaml
#   5. Run haleiki build (graph, validation, derived data)
#   6. Shell out to cobalt build + pagefind (or print instructions)
```

This keeps the repo structure clean: `content/` stays empty (ready for users who fork), and `demo/` is self-contained.

---

## 7. Generated Frontmatter

A fetched-and-converted Wikimedia article becomes a Haleiki source page. Here's what the generated frontmatter looks like:

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

## 8. Licensing & Attribution

All Wikimedia content is CC BY-SA (3.0 or 4.0). The demo site must attribute properly.

### Automated Attribution Page

`haleiki demo attribution` generates an `attribution.html` page from `demo/media/manifest.json` and the source page frontmatter. It lists:

- Every source article with its original Wikimedia URL and license
- Every image with its Commons URL, author, and license
- A general CC BY-SA notice for the demo content

This page is also generated automatically as part of `haleiki demo build`.

### Per-Page Attribution

Each source page includes a "Source" line in the taxonomy sidebar linking to the original Wikimedia article, populated from `original_source` in the frontmatter.

---

## 9. CI/CD Pipeline

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

      # ── Rust toolchain ──
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      # ── Build Haleiki CLI (with demo feature) ──
      - name: Build haleiki
        run: cargo build --release --features demo --manifest-path tools/Cargo.toml

      # ── Build demo site (wires content + runs full pipeline) ──
      - name: Build demo
        run: ./tools/target/release/haleiki demo build

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
          ./tools/target/release/haleiki demo validate
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

The `haleiki demo validate` step catches regressions in the demo content:

- No broken internal links (all `slug` references resolve)
- No orphan pages (every page reachable from at least one other)
- No dangling provenance (concept → source links valid)
- No unknown categories or tiers
- Search index has expected entry count
- Media files referenced in Markdown actually exist

---

## 10. Refresh Strategy

Demo content is **committed to the repo**, not fetched during CI. This means:

- CI has no dependency on Wikimedia API availability
- Builds are reproducible
- No risk of Wikipedia edits breaking the demo unexpectedly

To refresh the demo content:

```bash
# Refresh all articles
haleiki demo refresh

# Refresh a single article
haleiki demo fetch --article garbage-collection

# See what's changed / stale
haleiki demo status
```

This overwrites `demo/sources/` and `demo/media/`. The diff is reviewed and committed like any other content change. The `revision_id` in each page's frontmatter makes it clear exactly which Wikipedia revision was fetched.

A scheduled GitHub Action could run `haleiki demo refresh` weekly/monthly and open a PR with the diff, but that's a Phase 2 nicety.

---

## 11. Framework Media Convention (Design Decision)

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

## 12. Rust Module Design Notes

### `tools/src/demo/mod.rs` — Subcommand Dispatch

The entire `demo` module is conditionally compiled:

```rust
// In main.rs
#[cfg(feature = "demo")]
mod demo;

#[derive(Subcommand)]
pub enum Command {
    Build,
    Validate,
    Stats,
    // ... other core commands ...

    #[cfg(feature = "demo")]
    /// Demo site management (fetch from Wikimedia, build, serve)
    Demo {
        #[command(subcommand)]
        command: demo::DemoCommand,
    },
}
```

```rust
// In demo/mod.rs
use clap::Subcommand;

#[derive(Subcommand)]
pub enum DemoCommand {
    /// Fetch articles from Wikimedia and convert to Haleiki source pages
    Fetch {
        /// Fetch only this article (by slug)
        #[arg(long)]
        article: Option<String>,
        /// Show what would be fetched without writing files
        #[arg(long)]
        dry_run: bool,
        /// Use pandoc for HTML→Markdown instead of built-in converter
        #[arg(long)]
        pandoc: bool,
    },
    /// Wire demo content into content/ and run full build
    Build,
    /// Build + serve locally with file watching
    Serve,
    /// Show manifest vs. on-disk state
    Status,
    /// Validate demo content
    Validate,
    /// Generate the attribution page
    Attribution,
    /// Remove all generated demo content
    Clean,
    /// Clean + fetch all (full regeneration)
    Refresh,
}
```

### Key Design Decisions

**HTML parsing:** `scraper` (CSS selector–based, similar to BeautifulSoup) is the natural choice for the DOM walking we need — identifying images, stripping navboxes, rewriting links. `lol_html` is faster (streaming) but less ergonomic for the kind of multi-pass inspection and transformation this pipeline requires. Start with `scraper`.

**HTML → Markdown:** Default to `htmd` (pure Rust, reasonable quality). If conversion quality is insufficient for certain Wikipedia structures (complex tables, nested lists, math markup), the `--pandoc` flag shells out to Pandoc when available. Goal: zero external dependencies by default, with Pandoc as an optional quality upgrade.

**Async fetching:** Use `reqwest` with `tokio` for concurrent downloads. Respect Wikimedia's rate limits (200 req/s for API, be conservative with media downloads). Use `indicatif` for progress feedback — fetching 12 articles with images should show per-article progress.

**Manifest validation:** `haleiki demo status` cross-references `manifest.yaml` against what's on disk in `demo/sources/` and `demo/media/`, reporting missing articles, stale fetches (revision ID changed upstream — requires an API call), and orphaned files not referenced by any manifest entry.

---

## 13. Open Questions

1. **HTML → Markdown quality** — how well do pure-Rust crates (`htmd`, `html2md`) handle Wikipedia's HTML? Particularly: complex tables, citation footnotes, math (`<math>` tags), nested definition lists. Worth a spike early to evaluate before committing to a crate.
2. **SVG theming** — should the build pipeline post-process SVGs to replace hardcoded colors with CSS custom properties? This would let diagrams adapt to day/night theme. Worth prototyping with one or two SVGs.
3. **Wikipedia infoboxes** — strip entirely, or convert to a structured sidebar? Stripping is simpler; converting exercises the sidebar component.
4. **Image captions in Markdown** — `![caption](path)` needs to render as `<figure><img><figcaption>` in the final HTML. Verify Cobalt handles this, or add a template helper.
5. **Demo concept cards** — hand-author 5–8 concept cards for the demo, or run AI extraction? Hand-authoring is more predictable for Phase 1; AI extraction can wait for Phase 3.