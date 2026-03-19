# Haleiki

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A beautiful house of knowledge*

---

Haleiki (*hale* "house" + *ʻike* "knowledge," Hawaiian) is a static-site wiki-like knowledge-sharing tool — beautifully designed, searchable, and interlinked. There are no logins, no in-page edits, no comments. Content goes in as source pages and concept cards; a navigable knowledge base comes out.

Closer to an encyclopedia than a wiki in philosophy. Closer to a wiki than an encyclopedia in navigation.

## How It Works

Haleiki has a four-layer pipeline:

1. **Authoring** — Write source pages in Markdown. Concept cards are extracted from them (AI-driven, using the NeON-inspired Oxur Fabryk (v3) ontological methodology), merged, and linked.
2. **Pre-build** (Rust CLI) — Parses all content, builds a unified relationship graph, validates references, computes derived data (see-also groups, breadcrumbs, prerequisite chains, category indices), and writes graph-derived JSON.
3. **Build** (Cobalt) — Reads source pages, concept cards, and graph data. Applies Liquid templates. Outputs a static HTML site.
4. **Runtime** — Static files only. HTML, CSS, JS, and a search index. Zero server logic.

## Content Model

Two first-class page types, both full wiki citizens:

- **Source Pages** — Authored Markdown (guides, tutorials, essays, reference). Published as-is. Also serve as raw material for concept extraction.
- **Concept Cards** — Atomic knowledge units with structured frontmatter: typed relationships, provenance, competency questions, aliases, classification.

## Quick Start

```bash
# Scaffold new content
haleiki new source "Understanding Ownership in Rust"
haleiki new concept "Ownership"

# Build
haleiki build && cobalt build && npx pagefind --site _site

# Dev (serve + watch)
haleiki dev
```

## Design

Warm/cool dual-theme aesthetic built on a semantic design token architecture. DM Sans for headings, Source Serif 4 for body prose, IBM Plex Mono for code. OKLCH color throughout. Fluid typography and Every Layout compositional primitives — zero media queries for layout.

## Project Status

Early development. See the [architecture document](./haleiki-architecture.md) for the full design and phase plan.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT), at your option.

[//]: ---Named-Links---

[logo]: assets/images/logo/v1-x250.png
[logo-large]: assets/images/logo/v1.png
[build]: https://github.com/oxur/haleiki/actions/workflows/cicd.yml
[build-badge]: https://github.com/oxur/haleiki/actions/workflows/cicd.yml/badge.svg
[crate]: https://crates.io/crates/haleiki
[crate-badge]: https://img.shields.io/crates/v/haleiki.svg
[docs]: https://docs.rs/haleiki/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/haleiki.svg
[tag]: https://github.com/oxur/haleiki/tags
