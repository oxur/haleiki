# Slice 02 — browse + category · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 7 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run. Not yet committed. 2026-07-28.

## Row-by-row walk

**F-1 — `site.data.browse_categories`, grouped by subcategory — a real bug
found and fixed.** `done`. `cargo test
render::tests::test_assemble_site_context_browse_categories_grouped_by_subcategory`
passes, but not on the first attempt. `graph::category_index` deliberately
excludes `"_uncategorized"` from `entry.subcategories` (its own comment says
so — "don't include `_uncategorized` as a named subcategory"); those
articles are only present folded into the flat `entry.concepts`/
`entry.sources`. My first implementation iterated `entry.subcategories`
directly, so anything with no subcategory silently vanished from `groups[]`
(while still counted in the category's total `count` — a real, user-visible
defect: articles present in the category's stated count but unreachable from
any browse group). Caught immediately because the test target
(`philosophy-of-mind`, 8 real uncategorized items hand-verified against the
frontmatter via `awk` before the assertion was written) expected 4 groups
and got 3. Fixed with a set-difference: every article slug in the category's
full list not present in any named subcategory becomes the unlabeled group,
placed first.

**F-2 — `build_category_page_data`, grouped by tier.** `done`. `cargo test
render::tests::test_build_category_page_data_mathematics_grouped_by_tier`
passes — on the first run. Real `mathematics` category: 7 foundational + 20
intermediate + 6 advanced (hand-counted from frontmatter before writing the
assertion), all three tiers present in taxonomy order — not sorted by count,
so a wrong-ordering bug would have been caught. `subcategories` correctly
excludes `"_uncategorized"`. Tier labels are capitalized in Rust
(`"foundational"` → `"Foundational"`) because `category.jinja` prints
`group.name` with no template-side filter — confirmed by reading the
template before writing the code, which is why this one didn't need a
failing test to discover.

**F-3 — `render_browse` full-chrome render.** `done`. `cargo test
page::tests::test_render_browse_full_chrome` passes. Confirms the left-nav
category list with counts, header stats, a real `mathematics` section with
subcategory-group markup and both article-type badges, and — specifically —
that `philosophy-of-mind`'s unlabeled group renders correctly, which is the
F-1 fix's actual effect visible in real HTML output, not just in the Rust
struct shape.

**F-4 — `render_category` full-chrome render.** `done`. `cargo test
page::tests::test_render_category_mathematics_full_chrome` passes.
`page::render_category` takes the raw category name and does the
"look up in `category_index` → build `CategoryPageData` → render" sequence
internally, mirroring `article::render_concept`'s shape rather than pushing
that assembly onto the caller. Confirms header (name, description, "33
articles", "3 groups"), all three `tier-header--{modifier}` classes, article
cards with both badges, and the sidebar info card's three counts (33/9/24)
plus a real subcategory tag.

**F-5 — Escaping holds at page scale on both pages.** `done`. Both
`page::tests::test_render_browse_escaping_holds_at_page_scale` and
`…test_render_category_escaping_holds_at_page_scale` pass. Browse: injects
into a real `mathematics` browse-category's description *and* its first
group's name — mathematics has no `_uncategorized` group, so `groups[0].name`
is guaranteed to be a value the template actually prints, not one the
`{% if group.name %}` guard would silently skip (picking a category where
that guard could hide a false pass was deliberate, not incidental). Category:
injects into a cloned `CategoryIndexEntry.description` *before* calling
`build_category_page_data`, so the check covers the escaping discipline
from the `graph`-derived input through the reshape and into the render, not
merely the render step in isolation.

**F-6 — No Liquid/Cobalt in the browse/category render path.** `done`.
`cargo test
page::tests::test_render_browse_category_path_references_no_liquid_templates`
passes. Greps `render.rs` (the module that gained the new reshaping code
this slice); `page.rs` was already covered by Arc 02 slice 01's equivalent
test. Built the needle by concatenation from the start — the Arc 01 slice 02
self-reference lesson applied proactively, not rediscovered.

**F-7 — New code clean under `--all-targets`.** `done`. `cargo fmt --all --
--check` clean. `cargo clippy --all-targets --all-features --workspace --
-D warnings` surfaced one real finding in new code —
`clippy::bool_to_int_with_if` in the concepts-then-sources `rank` helper
(`if t == "concept" { 0 } else { 1 }` → `u8::from(t != "concept")`) — fixed
per clippy's own suggested rewrite. Every other finding confirmed pre-existing
by content, not just filename: the same two `generator.rs`
`test_provenance_json_bidirectional` `assert!(x == y)` lines, and the
unchanged `demo/*.rs`/`tools/tests/demo_*.rs` set from Arc 01's close.

## Silent-drop check

Ledger opened with 7 rows (F-1…F-7); this report addresses 7. No missing rows.

## Refactor disclosure (per the cc-prompt's explicit instruction)

`build_home_categories` (Arc 02 slice 01) previously inlined its own
concept/source title-and-URL resolution. This slice extracted a shared
`resolve_article_ref` (concept-first, matching `graph::resolve_url_by_edge`'s
convention) used by `home`, `browse`, and `category` alike, and refactored
`build_home_categories` to call it instead of keeping a second convention.
Slice 01's existing tests (`test_assemble_site_context_home_categories_order_and_shape`,
the full `page::render_home` chrome test) were rerun immediately after the
refactor and still pass unchanged — the consolidation is behavior-preserving,
verified rather than assumed.

## Bubble-up to arc-plan.md

One finding worth naming for whoever plans slice 04 (the static export
writer) or reviews this arc at close: **`graph::category_index`'s
`"_uncategorized"`-exclusion-from-`subcategories` behavior is a real,
easy-to-miss trap** for any future code that groups by subcategory from this
same data source (this slice hit it once already). Not proposing a
`graph.rs` change (out of this slice's boundary, and `graph.rs` is reused,
not modified, per the arc's dependency list) — just flagging that the
pattern ("check `entry.concepts`/`entry.sources` minus everything in
`entry.subcategories` for the actual uncategorized set") is now precedent in
`render::build_browse_categories`, should the same need arise elsewhere.

## What was NOT done (explicitly, per the "no silent drops" rule)

- No `about`/`attribution`/`search` shell/`404` — slice 03.
- No static export writer, no `_site/` output, no enumeration of every
  category route — slice 04. This slice renders one category on demand; it
  does not walk all ten.
- No CLI, server, or search index.
- No article `description` text on category-page cards (decision #2,
  explicitly deferred per the slice-doc, matching slice 01's featured-teaser
  precedent).
- The pre-existing clippy debt named in F-7 was not fixed (out of boundary).
- Nothing has been committed to git. All changes are in the working tree.

## Independent verification status

Not yet performed. Per `LEDGER-DISCIPLINE.md`, this closing report is CC's
`attested` claim; a CDC-equivalent pass (fresh context or operator, re-running
each row's Verify command against the actual artifacts) is required before any
row's evidence strengthens to `reproduced` and before this slice can be
considered closed rather than proposed-done. Per the cc-prompt's explicit
instruction, this report does **not** include `cdc-verification.md` — that
file is the independent reviewer's to write.
