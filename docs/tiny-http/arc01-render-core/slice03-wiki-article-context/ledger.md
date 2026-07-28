# Slice 03 — wiki-article context · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | The parser retains the Markdown body: `SourcePage.body` and `ConceptCard.body` are populated (non-empty) after `parse_corpus`, for a real demo source and concept | `cargo test` reading `demo/`, asserting both bodies contain expected substrings | serious | slice-doc | open | | |
| F-2 | `demo/profile.yaml` migrated to the render `Profile` shape: `resolve_profile("demo/profile.yaml")` yields a valid `Profile` (name, `trust.levels`×4, `level_order`, `footer.columns`, `taxonomy`) **and** `parse_corpus("demo")` still succeeds with no validation errors | `cargo test`: both calls succeed; assert a few resolved fields | serious | slice-doc | open | | |
| F-3 | `site.data` assembly builds `see_also`, `provenance`, `breadcrumbs` maps in the template-contract shapes; for a known concept slug, `see_also[slug]` entries expose `{slug,title,relationship,url}`, `provenance[slug].links` `{title,url}`, `breadcrumbs[slug]` `{label,url}` | unit test on the assembly fn for a fixed demo slug | serious | slice-doc | open | | |
| F-4 | Concept `page.data` assembled correctly for a real demo concept: `concept`, `disambiguator`, `category`, `category_slug`, `tier`, `page_type:"concept"`, `trust_level`, `subject_headings[]`, `aliases[]`, `facts[]{label,value}` (defined subset), `toc[]{id,label}` — matching the card's frontmatter and body | unit test asserting each field against a known demo concept | serious | slice-doc | open | | |
| F-5 | A real demo **concept** renders end-to-end via `render::build_environment` + `pages/concept.jinja` with full chrome: trust bar (label), left-nav TOC entries, breadcrumb, article title + overline, prose containing `data-math-style` spans, see-also cards grouped by relationship, sidebar info card (category / tier / subject headings / aliases), and provenance "Derived From" links | render test grepping the output for each chrome marker | serious | slice-doc (A-4) | open | | |
| F-6 | A real demo **source** renders end-to-end via `pages/source.jinja`: breadcrumb, trust bar (default level), article title + overline, prose, and see-also | render test grepping the output | serious | slice-doc (A-5) | open | | |
| F-7 | Escaping holds at full-page scale: an HTML control string injected into an auto-escaped page field (e.g. a concept alias or the title) is escaped in the rendered page, while `page.content` remains `\| safe` and intact | render test injecting `<b>x</b>` and asserting `&lt;b&gt;` in the escaped field | correctness | slice-doc (A-6) | open | | |
| F-8 | The concept/source render path uses **no** Liquid/Cobalt template — it resolves `pages/*.jinja` via `render::build_environment` only | `grep` the render module + test: no `.liquid` referenced by the render path | correctness | slice-doc (A-7) | open | | |
| F-9 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** (per the slice-02 lesson) + `cargo fmt --check`, scoped to new code | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 9. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
