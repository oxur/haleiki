---
number: 3
title: "Haleiki Demo Site — Project Plan"
author: "any article"
component: All
tags: [change-me]
created: 2026-03-18
updated: 2026-03-18
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# Haleiki Demo Site — Project Plan

**Milestone-driven plan for Claude Code implementation**

Reference: `demo-site-design.md` v0.3

---

## How to Read This Plan

Each milestone is scoped to be completable in a single Claude Code session — a coherent unit of work that produces a testable, committable result. Milestones within a phase are ordered; phases are roughly sequential but some can overlap where noted.

**Notation:**
- **Depends on:** milestones that must be complete before starting
- **Produces:** the concrete, testable output
- **Key files:** where the code lands

---

## Phase 1: Project Scaffolding & Manifest

*Set up the demo directory structure, feature-flagged CLI skeleton, and the manifest that drives everything.*

### 1.1 — Demo directory structure and manifest file

Create the `demo/` directory tree with `.gitkeep` files, the `content/` directory with `.gitkeep` files, and write the full `demo/manifest.yaml` with all 12 article entries (Wikipedia + Wikibooks). Create `demo/taxonomy.yaml` with the demo-specific categories and tiers.

- **Depends on:** nothing (can start immediately)
- **Produces:** `demo/manifest.yaml`, `demo/taxonomy.yaml`, directory skeleton
- **Key files:** `demo/`, `content/`

### 1.2 — Feature-flagged `demo` module and CLI subcommand skeleton

Add the `demo` cargo feature to `Cargo.toml` with all optional dependencies declared. Create `tools/src/demo/mod.rs` with the `DemoCommand` enum (all variants stubbed). Wire it into `main.rs` behind `#[cfg(feature = "demo")]`. Verify `cargo build` compiles without the feature (no demo code included), and `cargo build --features demo` compiles with it. All subcommands print "not yet implemented" and exit.

- **Depends on:** existing CLI skeleton (clap setup in `main.rs`)
- **Produces:** compiling CLI with `haleiki demo fetch`, `haleiki demo build`, etc. all recognized
- **Key files:** `tools/Cargo.toml`, `tools/src/main.rs`, `tools/src/demo/mod.rs`

### 1.3 — Manifest parsing and validation (`manifest.rs`)

Implement `serde` deserialization for the manifest schema: defaults, taxonomy, article entries with per-article overrides (project, license, media skip patterns). Write a `validate_manifest()` function that checks for duplicate slugs, unknown categories/tiers (vs. taxonomy), and missing required fields. Wire into `haleiki demo status` as a first real subcommand — prints a table of articles with their fetch status (all "missing" at this point).

- **Depends on:** 1.1, 1.2
- **Produces:** `haleiki demo status` reads manifest and prints article table
- **Key files:** `tools/src/demo/manifest.rs`

---

## Phase 2: Fetch Pipeline — HTML Acquisition

*Get articles from Wikimedia and land them as raw HTML on disk. No conversion yet.*

### 2.1 — Wikimedia REST API client (`fetch.rs`)

Implement the HTTP client using `reqwest` + `tokio`. Fetch a single article's HTML from the Wikimedia REST API (`/api/rest_v1/page/html/{title}`). Handle the `project` field for cross-project fetches (Wikipedia vs. Wikibooks, etc.). Capture the revision ID from the response's `ETag` header. Respect rate limits with a configurable delay. Write the raw HTML to a staging area (`demo/.staging/{slug}.html`). Wire into `haleiki demo fetch --article <slug>`.

- **Depends on:** 1.3
- **Produces:** `haleiki demo fetch --article memory-management` downloads raw HTML
- **Key files:** `tools/src/demo/fetch.rs`

### 2.2 — Batch fetching with progress

Extend `haleiki demo fetch` (no `--article` flag) to fetch all manifest articles concurrently with bounded parallelism. Add `indicatif` progress bars showing per-article status. Implement `--dry-run` mode that resolves API URLs and prints them without fetching. Add basic caching: skip articles whose staging HTML already exists unless `--force` is passed.

- **Depends on:** 2.1
- **Produces:** `haleiki demo fetch` downloads all 12 articles with progress bar
- **Key files:** `tools/src/demo/fetch.rs`

---

## Phase 3: HTML Cleaning & Link Rewriting

*Transform the raw Wikipedia HTML into something ready for Markdown conversion.*

### 3.1 — Wikipedia chrome stripping (`clean.rs`)

Using the `scraper` crate, implement DOM cleaning that removes: navboxes, hatnotes, edit section links, citation superscripts, ambox maintenance banners, external link icons, `[edit]` spans, footer sections (references, external links, categories), table of contents div. Preserve the article's substantive content: prose, headings, figures, tables, lists, code blocks. Test against the raw HTML from at least 2 fetched articles.

- **Depends on:** 2.1 (needs raw HTML to work with)
- **Produces:** cleaned HTML files in staging, visually verifiable
- **Key files:** `tools/src/demo/clean.rs`

### 3.2 — Link rewriting (`rewrite.rs`)

Implement the link rewriting pass over cleaned HTML. For each `<a href>`: resolve relative wiki links to their canonical article title, look up against the manifest's title→slug index, rewrite matches to `/source/{slug}/` (preserving `#fragment`), rewrite non-matches to absolute Wikimedia URLs. Handle edge cases: red links (dead wiki links), interwiki links, anchor-only links, external links (leave untouched).

- **Depends on:** 3.1, 1.3 (needs manifest for the title→slug index)
- **Produces:** cleaned HTML with correct internal/external links
- **Key files:** `tools/src/demo/rewrite.rs`

---

## Phase 4: Media Pipeline

*Download images, record metadata, rewrite image references.*

### 4.1 — Image extraction and download (`media.rs`)

Walk the cleaned HTML DOM to find all `<img>` and `<figure>` elements. For each image: parse the Wikimedia CDN URL, extract the filename, apply skip patterns (global + per-article) via `globset`, select the appropriate resolution variant (thumbnail at `max_width` for raster, original for SVG), download to `demo/media/{slug}/{filename}`. Handle download failures gracefully (warn and continue).

- **Depends on:** 3.1 (needs cleaned HTML with images still in DOM)
- **Produces:** `demo/media/{slug}/` directories with downloaded images
- **Key files:** `tools/src/demo/media.rs`

### 4.2 — Media metadata and manifest generation

For each downloaded image, capture metadata: original Commons URL, filename, license (from Commons API or inherited from article), author/attribution, caption text (from `<figcaption>` or `alt`), local path, format, file size. Write `demo/media/manifest.json`. Extend `haleiki demo status` to show media statistics (total images, by-article breakdown, total size).

- **Depends on:** 4.1
- **Produces:** `demo/media/manifest.json` with full attribution data
- **Key files:** `tools/src/demo/media.rs`

### 4.3 — Image source rewriting in HTML

After images are downloaded, rewrite all `<img src>` attributes in the cleaned HTML to point to local relative paths (`../media/{slug}/{filename}`). Remove images that were skipped (matched skip patterns) from the DOM entirely rather than leaving broken references. This is the final HTML transformation before Markdown conversion.

- **Depends on:** 4.1, 4.2 (needs to know which images were downloaded vs. skipped)
- **Produces:** cleaned HTML with all image references pointing to local files
- **Key files:** `tools/src/demo/media.rs` or `tools/src/demo/clean.rs`

---

## Phase 5: Markdown Conversion & Frontmatter

*Turn the cleaned, rewritten HTML into Haleiki source pages.*

### 5.1 — HTML → Markdown conversion spike

Evaluate `htmd` (and/or `html2md`) against 2–3 real fetched-and-cleaned Wikipedia articles. Test against: headings, paragraphs, inline formatting, images with captions, tables, ordered/unordered lists, nested lists, definition lists, code blocks. Document quality findings and any post-processing needed. This milestone is investigative — the output is a decision on which crate to use and what fixups are needed.

- **Depends on:** 3.1, 4.3 (needs fully cleaned HTML)
- **Produces:** written evaluation + decision documented in code comments or a brief ADR
- **Key files:** `tools/src/demo/convert.rs` (initial version)

### 5.2 — Markdown conversion implementation (`convert.rs`)

Implement the chosen HTML→Markdown converter with post-processing fixes identified in 5.1. Handle: image syntax normalization (`![caption](path)`), heading level normalization (ensure H1 is title, body starts at H2), cleanup of Pandoc/htmd artifacts (excess blank lines, trailing whitespace). Optionally implement `--pandoc` flag that shells out to Pandoc as an alternative backend.

- **Depends on:** 5.1
- **Produces:** clean Markdown files from Wikipedia HTML
- **Key files:** `tools/src/demo/convert.rs`

### 5.3 — Frontmatter injection (`frontmatter.rs`)

Generate YAML frontmatter from the manifest entry + fetch metadata. Populate: `title`, `slug`, `page_type: "source"`, `category`, `tier`, `keywords`, `tags`, `author: "Wikipedia contributors"`, `date` (fetch date), `original_source` block (title, project, URL, license, `fetched_at`, `revision_id`), `extraction_status: "pending"`, `status: "published"`. Prepend to the Markdown body. Write the final `.md` file to `demo/sources/{slug}.md`.

- **Depends on:** 5.2, 1.3 (manifest data)
- **Produces:** complete Haleiki source page `.md` files in `demo/sources/`
- **Key files:** `tools/src/demo/frontmatter.rs`

---

## Phase 6: Build Integration & Demo Commands

*Wire the demo content into the Haleiki build pipeline.*

### 6.1 — `haleiki demo build` implementation

Implement the content wiring logic: symlink `demo/sources/` → `content/sources/`, `demo/concepts/` → `content/concepts/`, `demo/media/` → `content/media/`, copy `demo/taxonomy.yaml` → `content/taxonomy.yaml`. Then invoke the existing `haleiki build` pipeline. Handle cleanup: remove symlinks after build (or on error). Detect and warn if `content/` already has real content that would be overwritten.

- **Depends on:** 5.3, existing `haleiki build` command
- **Produces:** `haleiki demo build` produces `_data/` with graph and derived data from demo content
- **Key files:** `tools/src/demo/mod.rs`

### 6.2 — `haleiki demo validate` implementation

Run `haleiki validate` against the demo-wired content, plus demo-specific checks: every manifest article has a corresponding `.md` in `demo/sources/`, every image referenced in Markdown exists in `demo/media/`, media manifest JSON is present and parseable, no orphaned media files (images on disk but not referenced by any article).

- **Depends on:** 6.1
- **Produces:** `haleiki demo validate` exits 0 on healthy demo, non-zero with diagnostics on problems
- **Key files:** `tools/src/demo/mod.rs` or a dedicated `validate_demo.rs`

### 6.3 — `haleiki demo clean` and `haleiki demo refresh`

Implement `clean`: remove `demo/sources/*.md`, `demo/media/*/`, `demo/media/manifest.json`, and staging files. Preserve `demo/manifest.yaml`, `demo/taxonomy.yaml`, and `demo/concepts/`. Implement `refresh`: run `clean` then `fetch` (all articles).

- **Depends on:** 2.2, 6.1
- **Produces:** working `haleiki demo clean` and `haleiki demo refresh`
- **Key files:** `tools/src/demo/mod.rs`

### 6.4 — `haleiki demo status` full implementation

Extend the stub from 1.3 to show: per-article fetch state (missing / fetched / stale), local revision ID vs. manifest, media count per article, total media size, last fetch timestamp. Stale detection compares the `revision_id` in frontmatter against a quick API HEAD request (optional, behind `--check-upstream` flag to avoid network dependency by default).

- **Depends on:** 5.3, 4.2
- **Produces:** rich status output showing demo health at a glance
- **Key files:** `tools/src/demo/mod.rs`, `tools/src/demo/manifest.rs`

---

## Phase 7: Attribution & Licensing

*Ensure proper CC BY-SA compliance.*

### 7.1 — Attribution page generation (`attribution.rs`)

Read `demo/media/manifest.json` and all source page frontmatter. Generate an `attribution.html` (or `.md`) page listing: every source article (title, original URL, project, license), every image (filename, Commons URL, author, license, caption), a general CC BY-SA notice. Wire into `haleiki demo attribution` as a standalone preview command and into `haleiki demo build` as an automatic step.

- **Depends on:** 4.2, 5.3
- **Produces:** `demo/attribution.md` (or `.html`) with complete licensing info
- **Key files:** `tools/src/demo/attribution.rs`

---

## Phase 8: CI/CD Pipeline

*Automate the demo site build and deployment.*

### 8.1 — GitHub Actions workflow

Write `.github/workflows/demo-site.yml`: checkout, Rust toolchain setup, `cargo build --release --features demo`, `haleiki demo build`, install + run Cobalt, install + run Pagefind, `haleiki demo validate`, deploy to GitHub Pages on `main` (build-only on PRs). Use `rust-cache` for fast CI builds.

- **Depends on:** 6.1, 6.2 (demo build and validate must work)
- **Produces:** working CI that builds and deploys the demo site
- **Key files:** `.github/workflows/demo-site.yml`

### 8.2 — First successful deployment

Run the full pipeline end-to-end: fetch all articles, build, validate, deploy. Fix any issues that surface in CI that didn't appear locally (path differences, missing tools, permission issues). Verify the deployed GitHub Pages site is accessible and navigable.

- **Depends on:** 8.1, all prior phases
- **Produces:** live demo site at `https://{org}.github.io/haleiki/`
- **Key files:** (debugging, no new files)

---

## Phase 9: Demo Concept Cards

*Add concept cards so the demo exercises the full content model, not just source pages.*

### 9.1 — Hand-author seed concept cards

Write 5–8 concept cards by hand for key concepts in the demo cluster: mark-and-sweep, reference counting (as algorithm), use-after-free, RAII pattern, stack frame, memory leak, null pointer. Use the full concept card frontmatter schema from the architecture doc. Include `derived_from` links pointing to the demo source pages. Place in `demo/concepts/`.

- **Depends on:** 5.3 (source pages must exist so `derived_from` links are valid)
- **Produces:** `demo/concepts/*.md` with real concept cards
- **Key files:** `demo/concepts/`

### 9.2 — Validate concept ↔ source graph

Run `haleiki demo validate` and `haleiki stats` against the demo with both source pages and concept cards. Verify: provenance links (concept → source) resolve, prerequisite chains produce valid topological sort, see-also relationships are populated, category indices include both page types. Fix any graph issues.

- **Depends on:** 9.1, 6.2
- **Produces:** clean validation pass with both page types
- **Key files:** (debugging, fixes to concept card frontmatter)

---

## Phase 10: Polish & Hardening

*Make the demo pipeline robust and pleasant to use.*

### 10.1 — Error handling and user-facing messages

Audit the entire `demo/` module for error handling. Replace `unwrap()`/`expect()` with proper error types and user-friendly messages. Handle: network failures (per-article, don't abort batch), malformed HTML (warn and skip), missing images (warn, don't break article), manifest parse errors (clear line/column reporting). Use `anyhow` or a custom error type with context.

- **Depends on:** all prior phases
- **Produces:** robust CLI that fails gracefully with actionable messages
- **Key files:** all `tools/src/demo/*.rs`

### 10.2 — `haleiki demo serve` implementation

Implement the dev server command: run `demo build`, then watch `demo/` for changes, rebuild on change, and serve the `_site/` directory locally. This can shell out to Cobalt's built-in server or use a simple Rust HTTP server (e.g., `warp` or `axum` — evaluate whether the dep is worth it vs. shelling out). Live reload is nice-to-have, not required.

- **Depends on:** 6.1
- **Produces:** `haleiki demo serve` opens a browser-ready local dev server
- **Key files:** `tools/src/demo/mod.rs`

### 10.3 — Documentation

Write a `demo/README.md` explaining: what the demo is, how to fetch content (`cargo build --features demo && haleiki demo fetch`), how to build and serve locally, how to add articles to the manifest, how to refresh content, licensing obligations. Also add a section to the top-level project README about the demo site with a link to the live Pages deployment.

- **Depends on:** all prior phases
- **Produces:** `demo/README.md`, updated project `README.md`
- **Key files:** `demo/README.md`, `README.md`

---

## Dependency Graph (Phases)

```
Phase 1: Scaffolding & Manifest
    │
    ▼
Phase 2: Fetch (HTML acquisition)
    │
    ├──────────────────┐
    ▼                  ▼
Phase 3: Clean &    Phase 4: Media
  Link Rewrite        Pipeline
    │                  │
    └──────┬───────────┘
           ▼
    Phase 5: Markdown Conversion
           │
           ▼
    Phase 6: Build Integration
           │
    ┌──────┼──────────┐
    ▼      ▼          ▼
Phase 7  Phase 8    Phase 9
Attrib.  CI/CD      Concepts
    │      │          │
    └──────┴──────────┘
           │
           ▼
    Phase 10: Polish
```

Note: Phases 3 and 4 can overlap — media extraction happens during HTML cleaning. They're separated as milestones because they're independently testable.
