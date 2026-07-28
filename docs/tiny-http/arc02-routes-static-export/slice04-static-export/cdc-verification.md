# Slice 04 — static export writer · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per `LEDGER-DISCIPLINE.md` Section A. Verdict: **CLOSED** ·
> 2026-07-28. (Arc 02's composition reproduction and close are in
> [`../closing-report.md`](../closing-report.md).)

## Verifier and method

Independent Claude (Cowork) context, not the implementer. Clean cloud build
against the full demo corpus + the allowlisted assets; deps from crates.io,
`cargo 1.95.0`. In addition to the test suite, I **ran the real export**
(`haleiki export`) to a temp `_site/` and inspected the artifact — the strongest
form of the arc-scale composition check.

## Row-count check

Ledger opened with **8** rows (F-1…F-8); reproduces 8. No silent drops.

## Per-row verification

Full suite: **81 passed, 0 failed, 1 ignored** (7 new export unit tests), plus
the CLI integration test (`tools/tests/export_cli.rs`) — 1 passed.

| ID | CDC verdict | Strength | How reproduced |
|----|-------------|----------|----------------|
| F-1 | **confirmed** | reproduced | `test_export_site_writes_complete_tree_with_correct_counts` + the real run: `_site/` has **293** HTML files = 6 singletons + 10 categories + 91 sources + 186 concepts, each at its contract path; `404.html` present, `404/index.html` absent. |
| F-2 | **confirmed** | reproduced | `test_…copies_asset_allowlist_only` + real run: `assets/{css,fonts,icons,images,js,vendor}` present, `ai`/`mockups` absent, `haleiki.css` + 8 fonts copied. |
| F-3 | **confirmed** | reproduced | `test_…cleans_before_write` — a planted stale file is gone after re-export. |
| F-4 | **confirmed** | reproduced | `test_…internal_links_resolve_to_real_files` + real run: home → `/category/mathematics/` (file exists); `abelian-group`'s breadcrumb + see-also targets (`/source/group-mathematics/`, `/ring-mathematics/`) exist. The test checks both literal and `&#x2f;`-escaped href forms (see note). |
| F-5 | **confirmed** | reproduced | `test_…written_pages_match_renderers` — home/concept/source written files carry their chrome (hero grid, trust bar, math spans, prose). |
| F-6 | **confirmed** | reproduced | `tools/tests/export_cli.rs` — the `export` subcommand runs and produces `_site/`; I re-ran the binary by hand with the same result. |
| F-7 | **confirmed** | reproduced | `test_…is_deterministic_across_repeated_exports` — two full exports byte-identical; `test_export_path_references_no_liquid_templates` (needle concatenated). |
| F-8 | **confirmed** | reproduced | `clippy --all-targets -D warnings`: 0 findings in `export.rs`/`cli.rs`; `fmt` clean; only the pre-existing demo-test imports remain. |

## Boundary check

Changed `.gitignore` (+`_site/`), `cli.rs` (the `export` subcommand), `main.rs`
(dispatch), new `export.rs`, new `tools/tests/export_cli.rs`. **Additive** — the
`build` command and the `_data` JSON writer are untouched, per the slice
boundary. `export.rs` composes the existing renderers (no re-rendering); asset
copy is `std::fs` only (no new dependency), symlink-safe, allowlist-scoped.

## The `&#x2f;` href escaping (reviewed — not a defect)

minijinja's HTML auto-escaper escapes `/` → `&#x2f;` inside a **fully
interpolated** attribute value (`href="{{ crumb.url }}"`), where the
partially-literal pattern (`/category/{{ cat.slug }}/`) escapes nothing. This
surfaced three times now (Arc 01 footer links, slice 03 licence links, slice 04
breadcrumb/see-also). It is **functionally correct** — browsers decode the
entities, so the links work — and CC's link test checks both forms. It is worth
a permanent note so it isn't rediscovered a fourth time; recorded at the arc
close (a project-level gotcha note). No code/template change required.

## Verdict

**Slice 04 is CLOSED.** 8 rows, all reproduced. Arc ledger row **B-4** is
satisfied. This was Arc 02's final slice; B-5…B-8 reproduced at arc scale against
a real exported `_site/`, so the arc closes — see
[`../closing-report.md`](../closing-report.md).

## What worked (Safety-II)

- The determinism test (two full 293-file exports compared byte-for-byte) is
  exactly the property the eventual CI needs, written proactively.
- CC named the recurring href-escaping surprise explicitly and proposed a
  permanent home for it rather than fixing it locally a third time — turning a
  repeated papercut into a tracked, dispositioned finding.
