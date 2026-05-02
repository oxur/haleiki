# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Haleiki ("house of knowledge" in Hawaiian) is a static-site knowledge base framework — a read-only wiki for structured knowledge. It has a four-layer pipeline: Authoring (Markdown) → Pre-build (Rust CLI) → Build (Cobalt static site generator) → Runtime (static HTML/CSS/JS).

First deployment target: a Rust programming language knowledge base.

**Status:** Early development — architecture and design docs exist but no Rust code has been written yet.

## Build Commands

```bash
make build              # Build all binaries (debug)
make build-release      # Build optimized release binaries
make test               # Run all tests
make lint               # Clippy + format check
make format             # Format code with rustfmt
make coverage           # Test coverage (haleiki crate, ≥95% target)
make coverage-html      # HTML coverage report
make check              # build + lint + test
make check-all          # build + lint + coverage
make deps               # Update dependencies
make docs               # Generate rustdoc
```

Run a single test: `cargo test <test_name>`

## Architecture

Two first-class content types:

- **Source Pages** — authored Markdown (guides, tutorials, essays). Published as-is and also serve as input for concept extraction.
- **Concept Cards** — atomic knowledge units with structured frontmatter: typed relationships, provenance, competency questions, aliases, classification. Generated via AI-driven extraction from source pages.

The Rust CLI (pre-build layer) parses all content, builds a relationship graph (petgraph), validates references, computes derived data (see-also groups, breadcrumbs, prerequisite chains, category indices), and writes graph-derived JSON for the build layer.

## Design Documents

Design docs are managed with ODM (`odm.toml` config) in `docs/design/`, organized by state folders (01-draft, 02-under-review, ..., 06-final). View them with `./bin/odm show <number>`.

Key docs:

- **0001** — Architecture document (draft)
- **0002** — Demo site design document (under review)

Working documents (not committed): `workbench/`

## AI Rust Skill

For Rust code quality, load the skill and guides:

1. **`assets/ai/rust/SKILL.md`** — Advanced Rust programming skill
2. **`assets/ai/rust/guides/*`** — Comprehensive Rust guidelines (especially `11-anti-patterns.md`)
3. **`assets/ai/CLAUDE-CODE-COVERAGE.md`** — Test coverage guide (95%+ target)

**Note:** `assets/ai/ai-rust` may be a symlink. If it doesn't exist, check `~/lab/oxur/ai-rust` or `~/lab/oxur/ai-rust-skill`, or ask permission to clone:

```bash
git clone https://github.com/oxur/ai-rust assets/ai/ai-rust
```

## Project Conventions

- Test naming: `test_<fn>_<scenario>_<expectation>`
- Coverage target: ≥95%
- Always load `11-anti-patterns.md` before writing/reviewing Rust code
- Check against anti-patterns AP-01 through AP-20
- Reference design doc numbers in commits when making architectural changes
- Dual license: Apache 2.0 / MIT
