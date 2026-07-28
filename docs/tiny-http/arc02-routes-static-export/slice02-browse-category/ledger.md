# Slice 02 — browse + category · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `site.data.browse_categories` built from `category_index` in taxonomy order, grouped **by subcategory**: each `{slug, name, count, description, groups[]}`; each group `{name, articles[]{title, url, type}}` with `"_uncategorized"` → `name = ""`; `type` correct (`concept`/`source`); articles concepts-then-sources, alphabetical | unit test asserting order, a known category's groups, and article `type`/url shapes | serious | slice-doc | done, **with a real bug found and fixed before this row could pass** | attested — `cd tools && cargo test render::tests::test_assemble_site_context_browse_categories_grouped_by_subcategory` → `... ok`. | `render::{BrowseCategory, BrowseGroup, ArticleRef, build_browse_categories}`. **Finding:** `graph::category_index` deliberately does *not* put a `"_uncategorized"` key in `entry.subcategories` (its own code comment: "Don't include `_uncategorized` as a named subcategory") — those articles only appear folded into the flat `entry.concepts`/`entry.sources`. My first implementation iterated `entry.subcategories` directly expecting to find an `"_uncategorized"` key there; it doesn't exist, so those articles were silently dropped from `groups[]` entirely (present in `count`, absent from every group). Caught by the first test run against the real `philosophy-of-mind` category (8 real uncategorized items, hand-verified via `grep`/`awk` against the frontmatter before writing the assertion) expecting 4 groups and getting 3. Fixed: the unlabeled group is now computed as the set difference (every slug in the category's full article list not present in any named subcategory), placed first. |
| F-2 | `build_category_page_data` for a known category groups **by tier** in taxonomy tier order (non-empty tiers only): each group `{name, modifier, articles[]{title, url, type}}`; `subcategories[]` excludes `"_uncategorized"`; `count`/`source_count`/`concept_count` correct | unit test on a known category (e.g. `mathematics`) asserting tier groups, counts, subcategories | serious | slice-doc | done | attested — `cd tools && cargo test render::tests::test_build_category_page_data_mathematics_grouped_by_tier` → `... ok`. | `render::{CategoryPageData, TierGroup, build_category_page_data}`. Real `mathematics` category: foundational=7, intermediate=20, advanced=6 (hand-counted via `awk` against real frontmatter before writing the assertion — matched exactly), all three tiers present and in taxonomy order (not count order, so a `sort_by_count` bug would be caught). `subcategories` = `["algebra","foundations","topology"]`, `"_uncategorized"` correctly excluded (mathematics happens to have none, but the exclusion filter is exercised regardless). Tier `name` is capitalized in Rust (`"foundational"` → `"Foundational"`) since `category.jinja` prints `group.name` with no template-side filter, unlike `home.jinja`'s `tier.name \| capitalize` — passed on the first run because this was checked against the template source before writing the code, not discovered by a failing test. |
| F-3 | `render_browse` renders `pages/browse.jinja` end-to-end from the demo corpus: left-nav category list with counts, per-category sections with subcategory groups, article links + `type` badges, header stats | render test grepping the output for each marker | serious | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_browse_full_chrome` → `... ok`. | `page::render_browse`. Confirms the left-nav list with counts, header stats, a real `mathematics` section with subcategory-group markup, article links, both badge types (`concept`/`source`), and — specifically — that `philosophy-of-mind`'s unlabeled `_uncategorized` group renders (the F-1 fix's actual template-level effect, not just the Rust-side shape). |
| F-4 | `render_category` renders `pages/category.jinja` end-to-end for a real category: header (name, description, count, group count), tier sections with `tier-header--{modifier}`, article cards + badges, sidebar info card (`count`/`source_count`/`concept_count`/`subcategories`) | render test grepping the output | serious | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_category_mathematics_full_chrome` → `... ok`. | `page::render_category(env, site, corpus, graph, category)` — takes the raw category name (e.g. `"mathematics"`), not a pre-built `CategoryPageData`, mirroring `article::render_concept`'s "look up + build + render" shape rather than requiring the caller to pre-assemble. Confirms header (name/description/count "33 articles"/"3 groups"), all three `tier-header--{modifier}` classes, article cards with both badge types, and the sidebar info card's `count`/`source_count`/`concept_count` (33/9/24) and a real subcategory tag (`algebra`). |
| F-5 | Escaping holds at page scale: an HTML control string in an auto-escaped browse/category field (e.g. a group name or category description) is escaped in the rendered page | render test injecting `<b>x</b>` and asserting `&lt;b&gt;` | correctness | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_browse_escaping_holds_at_page_scale page::tests::test_render_category_escaping_holds_at_page_scale` → both `... ok`. | Browse: injects into a real `mathematics` browse-category's `description` *and* its first group's `name` (chosen because mathematics has no `_uncategorized` group, so `groups[0].name` is reliably a template-printed value, not a value the `{% if group.name %}` guard would skip) — both escaped. Category: injects into a cloned `CategoryIndexEntry.description` before calling `build_category_page_data`, confirming the escaping discipline holds from the `graph`-derived input through the reshape and into the render, not just at the render step alone. |
| F-6 | The browse/category render path uses **no** Liquid/Cobalt template — only `render::build_environment` + `pages/{browse,category}.jinja` | `grep`/test: no `.liquid` referenced by the render path (needle built by concatenation) | correctness | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_browse_category_path_references_no_liquid_templates` → `... ok`. | `include_str!`-greps `render.rs` (the module that gained the new browse/category reshaping code this slice; `page.rs` was already covered by Arc 02 slice 01's equivalent test). Needle built by concatenation from the start, per the slice-02 (Arc 01) lesson — no self-reference bug this time. |
| F-7 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | done | attested — `cargo fmt --all -- --check` → exit 0. `cargo clippy --all-targets --all-features --workspace -- -D warnings` → one real finding in new code (`clippy::bool_to_int_with_if` in the concepts-then-sources `rank` helper — `if x {0} else {1}` → `u8::from(!x)`), fixed per clippy's own suggestion; all other findings confirmed pre-existing (the same `generator.rs` `assert!(x==y)` pair plus the unchanged `demo/*.rs`/`tools/tests/demo_*.rs` set). | |

## What Worked

- Hand-verifying real corpus data via `grep`/`awk` against the actual
  frontmatter *before* writing each assertion (philosophy-of-mind's 8
  uncategorized items across 4 groups; mathematics's 7/20/6 tier split; 24
  concepts + 9 sources) turned every test into a real check instead of a
  tautology restating whatever the code happened to produce. This is what
  caught F-1's real bug — a hardcoded `4` the code didn't meet, not a
  self-fulfilling assertion.
- The F-1 bug (`category_index` doesn't key `"_uncategorized"` into
  `subcategories` at all) is exactly the kind of thing reading the data
  source's comments carefully would have caught in advance — the comment
  ("Don't include `_uncategorized` as a named subcategory") was read during
  planning but its implication for *this* slice's construction wasn't
  propagated into the first implementation. Worth naming: reading a
  dependency's code is necessary but not sufficient — the specific
  consequence for the new code still has to be traced through, and a test
  against real data is what actually closes that gap.
- Checking `category.jinja`'s template source for whether `group.name`
  passes through a `| capitalize` filter (it doesn't, unlike `home.jinja`'s
  `tier.name`) *before* writing `build_category_page_data`, rather than
  after a test failed on an uncapitalized tier label, avoided an iteration.
- Refactoring `build_home_categories` to use the new shared
  `resolve_article_ref` (rather than leaving its original inline duplicate
  logic untouched) and rerunning slice 01's existing tests immediately
  confirmed it stayed behavior-preserving — a low-risk moment to do the
  consolidation the cc-prompt asked for, not a separate risky pass later.

## Closure

**Independently verified (CDC): CLOSED** 2026-07-28. All 7 rows reproduced in a
clean full-corpus build — `cargo test` = 66 passed / 0 failed. The disclosed
`_uncategorized` bug fix (F-1) was independently confirmed via its exact-count
test (`philosophy-of-mind`). `clippy --all-targets -D warnings` = 0 findings in
slice-02 code; `fmt` clean; the `bool_to_int_with_if` fix confirmed. The
`build_home_categories` refactor to the shared `resolve_article_ref` is
behavior-preserving (slice-01 test intact + green). One latent tier-drop
robustness note (does not fire on the demo; validator-guarded) recorded in
[`cdc-verification.md`](./cdc-verification.md). Evidence `attested → reproduced`.
Verified by: independent Claude (Cowork) context. Rows: 7. Done: 7 (reproduced). Deferred: 0. No-op: 0.
