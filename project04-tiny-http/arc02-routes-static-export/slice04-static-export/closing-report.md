# Slice 04 — static export writer · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 8 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run. Not yet committed. This is
> Arc 02's fourth and final slice. 2026-07-28.

## Row-by-row walk

**F-1 — Complete `_site/` tree.** `done`. `cargo test
export::tests::test_export_site_writes_complete_tree_with_correct_counts`
passes. Every singleton route (home, browse, about, attribution, search)
gets its `index.html`; every one of the real corpus's 10 categories, 186
concepts, and 91 sources gets its own, counted against the corpus directly
rather than hardcoded. `404.html` confirmed at the flat path and confirmed
absent at `404/index.html`. Before writing any test, I ran the real CLI
against the real demo corpus by hand (`cargo run -- export --content-dir
demo ...`) and inspected the tree — 186/10/91 matched expectations on the
first invocation, which is what let the subsequent tests assert real counts
with confidence rather than guessed ones.

**F-2 — Asset allowlist.** `done`. `cargo test
export::tests::test_export_site_copies_asset_allowlist_only` passes. All six
allowlisted subdirectories (`css`, `fonts`, `icons`, `images`, `js`,
`vendor`) present; `ai` and `mockups` absent. Spot-checks real file content
(`haleiki.css`, all 8 font files) rather than just directory existence — an
empty allowlisted directory would pass a directory-only check but fail this
one.

**F-3 — Clean-before-write.** `done`. `cargo test
export::tests::test_export_site_cleans_before_write` passes. Plants a stale
file in an existing output directory, re-exports, confirms it's gone.

**F-4 — Internal link resolution, with an escaping surprise caught and
fixed.** `done`. `cargo test
export::tests::test_export_site_internal_links_resolve_to_real_files`
passes, but not on the first attempt. Confirms a real chain: home →
`/category/mathematics/` → an article card → a `/source/` or a bare concept
link → the real written file; and `abelian-group`'s breadcrumb and see-also
targets similarly resolve. **What went wrong first:** `category.jinja`'s
article-card `href="{{ a.url }}"` is entirely template-interpolated, so
minijinja's attribute auto-escaping turns every `/` in it into `&#x2f;` — the
first version of this test searched for the literal `href="/source/` prefix
and found nothing on genuinely correct output. This is the third time this
exact class of surprise has appeared this session (Arc 01 slice 01's footer
links, Arc 02 slice 03's license links, now here); fixed with a small helper
checking both the literal and escaped forms, applied uniformly rather than
patched per occurrence.

**F-5 — Written pages match the renderers.** `done`. `cargo test
export::tests::test_export_site_written_pages_match_renderers` passes.
Reads the actual files written to disk (not the renderer's in-memory return
value) for home, a concept, and a source, confirming the write step didn't
corrupt or diverge from what the renderers produced.

**F-6 — CLI subcommand drives the export.** `done`. `cargo test --test
export_cli` passes. `Command::Export` added to `cli.rs`
(`--content-dir`/`--templates-dir`/`--assets-dir`/`--out-dir`, defaults
`content`/`templates`/`assets`/`_site`), dispatched via a new `run_export` in
`main.rs` — fully additive alongside `build`, which is untouched, per
decision #1. New `tools/tests/export_cli.rs` invokes the real compiled
binary (matching the existing `demo_fetch.rs`/`demo_status.rs` convention)
and asserts both the process outcome and the written tree.

**F-7 — Deterministic, no Liquid.** `done`. Both
`test_export_site_is_deterministic_across_repeated_exports` and
`test_export_path_references_no_liquid_templates` pass. Determinism: two
full exports of the demo corpus to separate temp directories, compared as
sorted relative-path lists *and* byte-for-byte per file (287+ files) — a
real regression guard against any future change that stopped sorting the
route-iteration lists, not a "ran twice without crashing" check. No-Liquid:
needle built by concatenation from the start.

**F-8 — New code clean under `--all-targets`.** `done`. `cargo fmt --all --
--check` clean. `cargo clippy --all-targets --all-features --workspace --
-D warnings` surfaced four real findings in new code, all fixed: a
missing-backticks doc comment (`KaTeX`), two `filter_map(|e| e.ok())` →
`filter_map(Result::ok)` (clippy's own suggested rewrite), and an
`items_after_statements` in a test (two nested helper functions hoisted to
module scope). Remaining findings are the same two pre-existing
`generator.rs` lines carried since Arc 01's close.

## Silent-drop check

Ledger opened with 8 rows (F-1…F-8); this report addresses 8. No missing rows.

## Bubble-up to arc-plan.md — this is the arc's final slice

With slice 04 delivering the export walker, Arc 02's composition rows are
now reproducible against a real artifact rather than against individual
slices' attestations:

- **B-5** (complete `_site/` from the demo corpus): reproducible via
  `test_export_site_writes_complete_tree_with_correct_counts` and the CLI
  smoke test — both actually run the full export and inspect the tree.
- **B-6** (assets copied, non-site dirs excluded): reproducible via
  `test_export_site_copies_asset_allowlist_only`.
- **B-7** (grouping resolved in Rust, not the template): delivered in slice
  02 (browse/category tier and subcategory grouping); this slice's export
  writes real browse/category pages to disk, so whoever closes the arc can
  inspect the written HTML directly rather than trusting slice 02's
  in-memory render tests alone.
- **B-8** (internal links resolve): reproducible via
  `test_export_site_internal_links_resolve_to_real_files` — home → category
  → article, and a concept's breadcrumb/see-also targets, all confirmed
  against real files on disk.

Per `LEDGER-DISCIPLINE.md` Section B, these composition rows still require
**independent reproduction at arc scale** by whoever closes the arc — this
slice's tests are the evidence for that reproduction, not a substitute for
it. When the arc closes, `arc02-routes-static-export/closing-report.md`
should walk B-1…B-8 and bubble up to `project-plan.md`'s P-2.

One finding worth carrying forward, not a defect: the escaped-href surprise
(F-4) has now recurred three times across this session in three different
templates. It might be worth a short, explicit note in `templates/README.md`
or a project-memory entry — "any `href`/`src` built entirely from a single
`{{ variable }}` will have its `/` characters escaped; tests must account for
both forms" — so the next slice (in whichever arc) doesn't rediscover it a
fourth time. Not proposing a template change (out of every slice's boundary
so far); just naming the pattern.

## What was NOT done (explicitly, per the "no silent drops" rule)

- No tiny_http server — Arc 03. This slice is the static sink only.
- No search JSON index, no `search.js` behavior — Arc 04. The exported
  `search/index.html` is still the empty pre-hydration shell from slice 03.
- No news — Arc 05.
- The legacy `_data` JSON writer (`generator::generate`) and the `build`
  command are untouched — this slice is additive, per decision #1. Removing
  them (and the Cobalt/Liquid files) is Arc 06 cleanup.
- No template edits (the escaping surprise in F-4 was fixed in the test).
- The pre-existing clippy debt named in F-8 was not fixed (out of boundary).
- Nothing has been committed to git. All changes are in the working tree.

## Independent verification status

Not yet performed. Per `LEDGER-DISCIPLINE.md`, this closing report is CC's
`attested` claim; a CDC-equivalent pass (fresh context or operator, re-running
each row's Verify command against the actual artifacts) is required before any
row's evidence strengthens to `reproduced`, before this slice can be
considered closed rather than proposed-done, and before Arc 02's own
composition rows can be reproduced and the arc closed. Per the cc-prompt's
explicit instruction, this report does **not** include `cdc-verification.md`
or the arc-level `closing-report.md` — both are the independent reviewer's
to write.
