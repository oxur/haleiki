# Slice 03 — about + attribution + search shell + 404 · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 8 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run. Not yet committed. 2026-07-28.

## Row-by-row walk

**F-1 — `render_about` full-chrome render.** `done`. `cargo test
page::tests::test_render_about_full_chrome` passes. Confirms the header
(`name_short`, `about_intro`), all four trust levels in `trust.level_order`
order with a note rendered for `credible` (the real demo profile's only
level carrying one), and an about-body paragraph's real `<a
href="/attribution/">` link surviving the `| safe` filter.

**F-2 — `render_404`.** `done`. `cargo test page::tests::test_render_404_chrome`
passes. A thin wrapper around the template Arc 01 slice 01 already proved
renders correctly (that slice tested it inline); this exposes it as a
callable function other code — slice 04's export walker, eventually — can
call directly rather than re-deriving the render call.

**F-3 — `render_search` empty shell.** `done`. `cargo test
page::tests::test_render_search_empty_shell` passes. Confirms the search
input (empty `value`), the filter sidebar frame, the "No results found"
empty state (accepted as the pre-hydration shell per decision #3 — the
template was not edited to special-case an empty query), and the
`search.js` script tag. Also asserts the *results-list* markup is absent —
confirming the empty-state branch actually fired, not merely that some
markup happened to be present.

**F-4 — `site.data.attribution` assembled.** `done`. `cargo test
render::tests::test_assemble_site_context_attribution_shapes_and_counts` and
the pre-existing `generator::tests::test_attribution_json_grouped_by_project`
both pass. `generator::{build_sources_by_project, general_notice}` extracted
from `write_attribution_json`, the fourth instance of the
one-derivation-two-consumers pattern (`build_provenance_map`,
`build_featured`, now this). Real numbers hand-verified via `grep -l` against
the frontmatter before writing the assertion: 91 sources, every one
attributed, across exactly 2 projects (`en.wikipedia.org`: 85,
`www.rigpawiki.org`: 6, in alphabetical order) — matched exactly on the first
run. `intro`/`licenses[]` come from a new `profile.attribution` section
added to `render::Profile`, `demo/profile.yaml`, **and**
`templates/profile.sample.yaml` — the sample file needed the same update
because slice 01's `test_resolve_profile_sample_yaml_parses_full_shape`
reads it and would otherwise fail deserializing the newly-required field;
caught before running tests, confirmed fixed by rerunning the full suite.

**F-5 — `render_attribution` full-chrome render.** `done`. `cargo test
page::tests::test_render_attribution_full_chrome` passes, but not on the
first attempt. Confirms `intro`/`general_notice` text, a real
`en.wikipedia.org` project heading with a source table and internal
`/source/` link, and both curated license cards with their points and links.
**What went wrong first:** the license-card `href` is entirely
template-interpolated (`{{ link.url }}`), unlike the partially-literal
`/source/{{ slug }}/` pattern used elsewhere in the same template family —
minijinja's attribute auto-escaping turns every `/` in a fully-interpolated
value into `&#x2f;`. The first version of this assertion checked for the
literal URL string and failed against genuinely correct output. Fixed by
checking for the un-slashed substring — the same class of fix Arc 01 slice
01 needed for footer links, recognized quickly this time because the
pattern had been seen before.

**F-6 — Escaping holds (both halves of the criterion).** `done`. Both
`page::tests::test_render_attribution_escaping_holds_at_page_scale` and
`…test_render_about_safe_body_stays_unescaped` pass. Attribution: injects
into a real source's `title` in the fully-assembled `SiteContext` and
re-renders the complete page. About: separately confirms `profile.about_body`
renders unescaped — the criterion explicitly names both the escaped and the
`| safe` side, so both got their own test rather than one implying the
other.

**F-7 — No Liquid/Cobalt in these four render paths.** `done`. `cargo test
page::tests::test_render_standalone_pages_path_references_no_liquid_templates`
passes. Greps `page.rs`, where all four new renderers live. Needle built by
concatenation from the start — no self-reference bug, consistent with every
slice since Arc 01 slice 02 first hit it.

**F-8 — New code clean under `--all-targets`.** `done`. `cargo fmt --all --
--check` clean. `cargo clippy --all-targets --all-features --workspace --
-D warnings` reports the same two pre-existing `generator.rs`
`test_provenance_json_bidirectional` `assert!(x == y)` lines (shifted to
783/785 by this slice's unrelated edits above them) plus the unchanged
`demo/*.rs`/`tools/tests/demo_*.rs` set — zero new findings, clean on the
first `--all-targets` run this time.

## Silent-drop check

Ledger opened with 8 rows (F-1…F-8); this report addresses 8. No missing rows.

## Bubble-up to arc-plan.md

Nothing new. `general_notice`'s eventual move into `profile.attribution`
(decision #2, explicitly deferred as a cleanup item) is already the kind of
small, named, non-blocking follow-up this arc's other slices have
accumulated — not raising it as a separate arc-plan finding since the
slice-doc itself already names it as future cleanup, not a defect.

## What was NOT done (explicitly, per the "no silent drops" rule)

- No static export writer, no `_site/` output — slice 04. This slice renders
  each of the four pages on demand; it does not walk or write any of them.
- No real search: no facets, no JSON index, no `search.js` behavior changes,
  no result ranking. `render_search` renders only the empty pre-hydration
  shell — real search is Arc 04.
- No CLI, server. No template edits (the escaping surprise in F-5 was fixed
  in the *test*, not the template).
- `general_notice` stays a hardcoded string in `generator.rs` (now shared via
  a function rather than duplicated, but not moved into `profile.attribution`
  — explicitly deferred per decision #2).
- The pre-existing clippy debt named in F-8 was not fixed (out of boundary).
- Nothing has been committed to git. All changes are in the working tree.

## Independent verification status

Not yet performed. Per `LEDGER-DISCIPLINE.md`, this closing report is CC's
`attested` claim; a CDC-equivalent pass (fresh context or operator, re-running
each row's Verify command against the actual artifacts) is required before any
row's evidence strengthens to `reproduced` and before this slice can be
considered closed rather than proposed-done. Per the cc-prompt's explicit
instruction, this report does **not** include `cdc-verification.md` — that
file is the independent reviewer's to write.
