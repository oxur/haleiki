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

## Planning and Historical Design

Planning lives on the orphan `planning` branch, checked out at
`.worktrees/planning/` (Operator-requested migration, 2026-11-11). Start with
that worktree's `README.md` and `AGENTS.md`. Load the current
`collaboration-framework` skill and its project-management `guides/README.md`
before planning or closing work; the migration used installed version 3.0.1.

- `project01-demo-site/`: historical source acquisition and demo pipeline.
- `project02-concept-cards/`: historical pilot, corpus extraction and backlinks.
- `project03-cobalt-site-build/`: historical M1–M8 Cobalt site build.
- `project04-tiny-http/`: active tiny_http/minijinja pivot.

Use `project-plan.md`, `arc-plan.md`, `slice-plan.md` and a sibling `ledger.md`
at each scale. Durable supporting outputs default to the owning slice's
`artifacts/` directory. Original `slice-doc.md`, issued prompts and verification
records remain preserved; new follow-ups use `cc-prompt-iterationNN.md`.

The planning worktree's `archive/` preserves all former `docs/design/`,
`docs/dev/`, and ignored `workbench/` content, including ODM originals and assets.
`migration-manifest.json` maps old paths to byte-preserved payloads. ODM state
folders are historical evidence, not the active planning layout. Older project
reconstructions are finding aids, with no inferred independent acceptance.

### tiny_http pivot (source branch `framework-base/tiny-http`)

Read `.worktrees/planning/project04-tiny-http/project-plan.md` and its ledger.
Arcs 01–04 retain their recorded closure; Arc 05 Slice 01 is still held at the
Operator discussion gate for authoring location, URLs, frontmatter and fixtures.
Arc 06 remains roadmap-level. Migration does not authorize feature implementation.

Confirmed decisions: deployment = **both** (static export default + hardened
tiny_http self-host); **news/blog** content type in scope; **Pagefind** search
is intended after the 2026-07-29 operator reversal (do not reintroduce the
superseded custom JSON search-index plan); **minijinja replaces Cobalt/Liquid**
using Claude Design's refactor inputs preserved in the planning worktree at
`archive/workbench/Hale'iki Wiki Design Refactor/` and the adjacent ZIP (which
also retains the original `templates/` tree). Current shipped templates live
in source `templates/`; the `main` design integration is abandoned.

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
