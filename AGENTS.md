# AGENTS.md

This file provides guidance to LLMs when working with code in this repository.

## Project Overview

Haleiki ("house of knowledge" in Hawaiian) is a static-site knowledge base framework — a read-only wiki for structured knowledge. It has a four-layer pipeline: Authoring (Markdown) → Pre-build (Rust CLI) → Build (static site generation) OR HTML server (dynamic template use)

First deployment target: a Rust programming language knowledge base.


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

## Workflow

- **Commit footer convention (operator override, 2026-08-07):** every future
  assistant-authored commit message includes these trailers:
  `Co-authored-by: Codex <noreply@openai.com>` and
  `Co-authored-by: Billo AI <ai-engineering@billo.systems>`.

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

### tiny_http pivot planning (branch `framework-base/tiny-http`)

Planning artefacts for the tiny_http/minijinja pivot live under `docs/tiny-http/`
using the collaboration-framework **arc/slice tree**, without a version-numbered
directory (operator override of the canonical `docs/design-vX.Y.Z/` default, per
`PROJECT-MANAGEMENT.md` Part VI):

```
docs/tiny-http/
  design-delta.md            architecture of the pivot
  project-plan.md            roadmap (6 arcs) + project ledger
  arcNN-<slug>/
    arc-plan.md
    sliceNN-<slug>/{slice-doc,ledger,cc-prompt,closing-report,cdc-verification}.md
```

Confirmed decisions: deployment = **both** (static export default + hardened
tiny_http self-host); **news/blog** content type in scope; **Pagefind** search
is intended after the 2026-07-29 operator reversal (do not reintroduce the
superseded custom JSON search-index plan); **minijinja replaces Cobalt/Liquid**
using Claude Design's refactor templates in `workbench/Hale'iki Wiki Design
Refactor/` (these are authoritative; the `main` design integration is
abandoned).

**Known gotcha — `&#x2f;` in fully-interpolated hrefs.** minijinja's HTML
auto-escaper escapes `/` to `&#x2f;` inside an attribute value that is a *single*
interpolated variable (`href="{{ crumb.url }}"`), but not in a partially-literal
one (`/category/{{ cat.slug }}/`). It is **functionally correct** — browsers
decode the entities, so links work — but tests that grep for `href="/..."` must
also accept the `&#x2f;` form (or assert on the partially-literal templates).
Seen in Arc 01 footer links and Arc 02 slices 03–04. Do not "fix" it in the
template; match both forms in tests.

## AI Rust Skill

For Rust code quality, load the skill and guides:

1. **`assets/ai/rust/SKILL.md`** — Advanced Rust programming skill
2. **`assets/ai/rust/guides/*`** — Comprehensive Rust guidelines (especially `11-anti-patterns.md`)
3. **`assets/ai/CLAUDE-CODE-COVERAGE.md`** — Test coverage guide (95%+ target)

**Note:** `assets/ai/ai-rust` may be a symlink. If it doesn't exist, check `~/lab/oxur/ai-rust` or `~/lab/oxur/ai-rust-skill`, or /rust-guidelines

## Project Conventions

- Test naming: `test_<fn>_<scenario>_<expectation>`
- Coverage target: ≥95%
- Always load `11-anti-patterns.md` before writing/reviewing Rust code
- Check against anti-patterns AP-01 through AP-20
- Reference design doc numbers in commits when making architectural changes
- Dual license: Apache 2.0 / MIT
- In Haleiki, **News is blog**: dated wiki-maintainer updates and related news
  items, not a CMS/editorial workflow.
- Shipped browser JavaScript under `assets/js/*.js` is classic browser-script
  code: no browser build step, no ES-module conversion, no CommonJS/Node/npm
  workflow. Do not apply Deno's default lint/fmt rules directly to these assets;
  run `make check-browser-js` for the canonical browser-asset gate. The
  Deno/Playwright harness under `tests/e2e/` remains governed separately by
  `make lint-e2e` and `make test-e2e`.
