# Slice 01 — site-index + home · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `site.data.stats` = `{sources, concepts, categories}` with the correct demo-corpus counts (`sources.len()`, `concepts.len()`, `taxonomy.categories.len()`) | `cargo test` reading `demo/`, asserting the three counts | serious | slice-doc | done | attested — `cd tools && cargo test render::tests::test_assemble_site_context_stats_match_demo_corpus_counts` → `... ok`. | `render::{Stats, build_stats}`; asserts `site.stats` equals `corpus.sources.len()`/`corpus.concepts.len()`/`corpus.taxonomy.categories.len()` computed independently in the test, plus a real-corpus sanity check (`categories == 10`, matching the migrated `demo/profile.yaml`'s 10 taxonomy categories from Arc 01 slice 03). |
| F-2 | `site.data.featured` reuses the existing top-N-by-in-degree selection via an extracted `pub(crate) build_featured`; the shared fn produces output byte-identical to the current `featured.json` (the existing `generator` featured test still passes) | `cargo test`: featured list matches; `generator` featured/JSON test green | serious | slice-doc | done | attested — `cd tools && cargo test generator::tests::test_build_featured_real_demo_corpus_ranked_by_typed_indegree` → `... ok`; `cd tools && cargo test generator::tests::test_generate_creates_all_files` → `... ok`. | `generator::{FeaturedEntry (now pub(crate) + Deserialize), build_featured}`; `write_featured_json` now calls it. Added a new content-check test (none existed before) against the **real demo corpus**: independently re-derives each featured entry's typed in-degree by walking the graph directly (not reusing `build_featured`'s own internal loop) and asserts the returned order is non-increasing by that count — a real ranking check, not just an existence check. Also asserts `write_featured_json`'s JSON output and `build_featured`'s in-memory output agree field-by-field, in the same test run. |
| F-3 | `site.data.home_categories` built from `category_index` in `profile.taxonomy.categories` order: each `{slug, name, count, description, articles[]}`; `count` = sources+concepts in the category; `articles[]` are `≤3` `{title, url}` with `url` = `/{slug}/` (concept) or `/source/{slug}/` (source) and titles resolved from the corpus | unit test asserting order, a known category's count, and its article title/url shapes | serious | slice-doc | done | attested — `cd tools && cargo test render::tests::test_assemble_site_context_home_categories_order_and_shape` → `... ok`. | `render::{HomeCategory, ArticleLink, build_home_categories}`. Verified taxonomy order is preserved (and explicitly *not* equal to alphabetical order, since `category_index` itself is a `BTreeMap` and would silently pass an order check by coincidence otherwise). Real category: `mathematics` → 24 concepts + 9 sources = 33 (`grep -l` count against `demo/{concepts,sources}/*.md`, matched exactly on the first test run), all 3 article slots are concept links (decision #1: concepts before sources, alphabetical within each — math has >3 concepts alone). |
| F-4 | `render_home` renders `pages/home.jinja` end-to-end from the demo corpus with full chrome: hero (name, subtitle, `stats` line), category grid (cards with title, count, description, ≤3 article links), featured grid, tiers row, quick-links | render test grepping the output for each section marker | serious | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_home_full_chrome` → `... ok`. | `page::render_home`, new module `tools/src/page.rs` (see Notes). Greps confirm every named section: hero title/subtitle/stats line (source-page and concept-card counts), category grid with a real `/category/mathematics/` link + count + description, featured grid with a "Read article" link, tiers row with "Foundational", quick-links including `/attribution/`. |
| F-5 | Escaping holds at page scale: an HTML control string placed in an auto-escaped home field (e.g. a `home_categories` name/description) is escaped in the rendered page | render test injecting `<b>x</b>` and asserting `&lt;b&gt;` | correctness | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_home_escaping_holds_at_page_scale` → `... ok`. | Injects `<b>x</b>` into a real `SiteContext`'s first `home_categories[].description` and re-renders through the full `render_home` pipeline; raw `<b>x</b>` absent, `&lt;b&gt;x&lt;` present. |
| F-6 | The home render path uses **no** Liquid/Cobalt template — only `render::build_environment` + `pages/home.jinja` | `grep`/test: no `.liquid` referenced by the home render path | correctness | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_home_path_references_no_liquid_templates` → `... ok`. | `include_str!`-greps `page.rs` (the module unique to the home path; `render.rs` was already covered by Arc 01 slice 03's equivalent test). Needle built by string concatenation, not a literal — the Arc 01 slice 02 self-referential-grep lesson applied proactively this time rather than rediscovered. |
| F-7 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | done | attested — `cargo fmt --all -- --check` → exit 0. `cargo clippy --all-targets --all-features --workspace -- -D warnings` → 30 findings total, **all pre-existing** (confirmed via line-content inspection, not just file-name matching): 28 in `demo/*.rs`/`tools/tests/demo_fetch.rs` (unchanged from Arc 01's close) and 2 in `generator.rs`'s pre-existing `test_provenance_json_bidirectional` (the same `assert!(x == y)` pair CDC found at Arc 01 close, at shifted line numbers 759/761 due to this slice's unrelated edits above them in the file) — zero in any code this slice added or touched. | Ran clippy with `--all-targets --all-features` from the start (the now-standing lesson from Arc 01 slices 02/03), not discovered after the fact. |

## What Worked

- Grepping the real demo corpus (`grep -l category: "mathematics" demo/{concepts,sources}/*.md | wc -l`)
  before writing F-3's test gave a hand-computed expected count (33) that the
  code either matched or didn't — a real check, not a tautology. It matched
  on the first run.
- Carrying forward Arc 01's accumulated lessons *proactively* instead of
  rediscovering them: `--all-targets` from the first clippy run (not added
  after a miss), and the self-referential `include_str!`-grep needle built by
  concatenation from the start (not fixed after the test failed against
  itself, as happened in slice 02). Both would have cost an iteration if
  reintroduced.
- Writing a real content-check test for `build_featured` (F-2) instead of
  relying solely on the pre-existing existence-only test — the extraction
  pattern from slice 03 (`build_provenance_map`) had a strong pre-existing
  guard to lean on; `build_featured` didn't, so this slice added one rather
  than accepting a weaker regression guard for its own refactor.
- Explicitly asserting taxonomy order is *not* alphabetical (rather than
  merely asserting it matches profile order) caught the exact failure mode
  a weaker test would miss: `category_index` returns a `BTreeMap`, so an
  order bug that silently fell back to alphabetical iteration would still
  pass a same-order-as-taxonomy check if the taxonomy happened to already be
  alphabetical — it isn't, so the check is real, but the extra assertion
  documents *why* the ordering test is meaningful.

## Closure

**Independently verified (CDC): CLOSED** 2026-07-28. All 7 rows reproduced in a
clean cloud build against the full demo corpus — `cargo test` = 59 passed / 0
failed (6 new slice-01 tests), each row mapped to a passing test (see
[`cdc-verification.md`](./cdc-verification.md)). `clippy --all-targets -D warnings`
shows 0 findings in slice-01 code; `fmt` clean; zero new lint debt. `build_featured`
extraction is byte-identical (guarded, plus a new real ranking test). Evidence
strengthened `attested → reproduced`. Verified by: independent Claude (Cowork)
context. Rows: 7. Done: 7 (reproduced). Deferred: 0. No-op: 0.
