# Slice 03 — about + attribution + search shell + 404 · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `render_about` renders `pages/about.jinja` end-to-end from the profile: header (`name_short`, `about_intro`), trust-level section iterating `trust.level_order` with each level's label/description/note, and the about-body paragraphs (`| safe`) | render test grepping the output for each section | serious | slice-doc | open | | |
| F-2 | `render_404` renders `pages/404.jinja`: the `404` error code, `name_short`, and the return/browse/search action links | render test grepping the output | polish | slice-doc | open | | |
| F-3 | `render_search` renders `pages/search.jinja` as an **empty shell**: `search.query = ""`, `results = []`, `facets = []`; the search input, filter sidebar frame, empty state, and the `search.js` script tag all render without error | render test on the empty shell | serious | slice-doc | open | | |
| F-4 | `site.data.attribution` assembled: `sources_by_project[]` reshaped (map→list, project-alphabetical) from the corpus via an extracted shared fn whose `attribution.json` output stays byte-identical (existing generator attribution test still passes); `source_count` and `general_notice` present; `intro` and `licenses[]` sourced from `profile.attribution` | unit test on the assembly + `generator` attribution test green | serious | slice-doc | open | | |
| F-5 | `render_attribution` renders `pages/attribution.jinja` end-to-end: `intro`, the licence callout (`general_notice`), per-project source tables (title/license/original-url links), and the licence-text cards (`licenses[]` title/points/links) | render test grepping the output | serious | slice-doc | open | | |
| F-6 | Escaping holds at page scale: an HTML control string in an auto-escaped attribution field (e.g. a source title) is escaped, while `profile.about_body` stays `\| safe` on the about page | render test injecting `<b>x</b>` and asserting `&lt;b&gt;` | correctness | slice-doc | open | | |
| F-7 | These render paths use **no** Liquid/Cobalt template — only `render::build_environment` + `pages/{about,404,search,attribution}.jinja` | `grep`/test: no `.liquid` referenced (needle built by concatenation) | correctness | slice-doc | open | | |
| F-8 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 8. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
