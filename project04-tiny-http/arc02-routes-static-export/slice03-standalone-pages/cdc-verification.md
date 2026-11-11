# Slice 03 — about + attribution + search shell + 404 · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per the collaboration-framework's `LEDGER-DISCIPLINE.md`
> Section A. Verdict: **CLOSED** · 2026-07-28.

## Verifier and method

Verified by an independent Claude context (Cowork), not the implementer.
Reproduced in a clean cloud build against the full demo corpus, deps from
crates.io, `cargo 1.95.0`.

## Row-count check

Ledger opened with **8** rows (F-1…F-8); reproduces 8. No silent drops.

## Per-row verification

Full suite: **74 passed, 0 failed, 1 ignored**. Each row maps to a passing test:

| ID | CDC verdict | Strength | How reproduced |
|----|-------------|----------|----------------|
| F-1 | **confirmed** | reproduced | `page::…test_render_about_full_chrome` — header, trust-level cards from `level_order`, about-body paragraphs. |
| F-2 | **confirmed** | reproduced | `page::…test_render_404_chrome` — error code, `name_short`, action links. |
| F-3 | **confirmed** | reproduced | `page::…test_render_search_empty_shell` — empty `search` context renders input, filter frame, empty state, `search.js` tag. |
| F-4 | **confirmed** | reproduced | `render::…test_assemble_site_context_attribution_shapes_and_counts` + `generator::…test_attribution_json_grouped_by_project` — reshape correct; `source_count`/`general_notice`; `intro`/`licenses` from profile; **byte-identical** `attribution.json` (the extraction guard passes). |
| F-5 | **confirmed** | reproduced | `page::…test_render_attribution_full_chrome` — intro, licence callout, per-project source tables, licence-text cards. |
| F-6 | **confirmed** | reproduced | `page::…test_render_attribution_escaping_holds_at_page_scale` + `…test_render_about_safe_body_stays_unescaped` — injected `<b>x</b>` escaped; `about_body` stays `\| safe`. |
| F-7 | **confirmed** | reproduced | no-Liquid test in `page.rs` (needle concatenated) — green in suite. |
| F-8 | **confirmed** | reproduced | `clippy --all-targets -D warnings`: 0 findings in `page.rs`/`render.rs`/`generator.rs`; `fmt` clean. Only the same pre-existing demo-test imports remain. |

## Boundary + the required-field ripple

Changed exactly `generator.rs`, `render.rs`, `page.rs`, `demo/profile.yaml`, and
`templates/profile.sample.yaml` — all expected. `build_sources_by_project` is the
**fourth** shared-derivation extraction (byte-identical, guarded). The new
`Profile.attribution` is a **required** field, so every resolvable profile must
carry it. CC recognized this would break slice 01's sample-profile test *before*
running, and updated **both** YAMLs in the same pass — verified: the slice-01
`test_resolve_profile_sample_yaml_parses_full_shape` and the demo migration test
both pass in my build. `parse_corpus` is unaffected (the older parser `Profile`
ignores the new section). No scope creep; no regression left behind.

## Verdict

**Slice 03 is CLOSED.** 8 rows, all reproduced. Arc ledger row **B-3** is
satisfied (evidence: this file). Every page type the site has now exists as a
renderer — only **slice 04** (the static-export walker) remains, and closing it
closes Arc 02.

## What worked (Safety-II)

- CC handled the required-field ripple as *foresight*, not a discovered
  regression — recognizing that a new mandatory `Profile` field breaks any test
  resolving a profile, and fixing both YAMLs in one pass. This is the pattern the
  session has been building toward: corrections become anticipations.
- The `intro`/`licenses` → profile (not Rust) decision keeps deployment content
  in config, consistent with `about_body`/`content_notice`/`footer` — the right
  framework instinct, and it composed cleanly with the corpus-derived half.
