# Slice 03 — about + attribution + search shell + 404 · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `render_about` renders `pages/about.jinja` end-to-end from the profile: header (`name_short`, `about_intro`), trust-level section iterating `trust.level_order` with each level's label/description/note, and the about-body paragraphs (`| safe`) | render test grepping the output for each section | serious | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_about_full_chrome` → `... ok`. | `page::render_about`. Confirms header, all 4 trust levels (`corroborated`/`credible`/`provisional`/`speculative`, in `level_order`) with a note rendered for `credible` (the real demo profile's only level carrying one), and an about-body paragraph's real `<a href="/attribution/">` link surviving `| safe`. |
| F-2 | `render_404` renders `pages/404.jinja`: the `404` error code, `name_short`, and the return/browse/search action links | render test grepping the output | polish | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_404_chrome` → `... ok`. | `page::render_404` — a thin wrapper matching Arc 01 slice 01's already-proven smoke-render pattern (that test rendered the template inline; this exposes it as a callable fn other code, e.g. slice 04's export walker, can call). |
| F-3 | `render_search` renders `pages/search.jinja` as an **empty shell**: `search.query = ""`, `results = []`, `facets = []`; the search input, filter sidebar frame, empty state, and the `search.js` script tag all render without error | render test on the empty shell | serious | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_search_empty_shell` → `... ok`. | `page::render_search`. Confirms the search input (`value=""`), filter sidebar frame, the "No results found" empty state (per decision #3 — not special-cased), and the `search.js` script tag; also asserts the *results-list* markup does **not** render (the empty-state branch, not a coincidentally-truthy one). |
| F-4 | `site.data.attribution` assembled: `sources_by_project[]` reshaped (map→list, project-alphabetical) from the corpus via an extracted shared fn whose `attribution.json` output stays byte-identical (existing generator attribution test still passes); `source_count` and `general_notice` present; `intro` and `licenses[]` sourced from `profile.attribution` | unit test on the assembly + `generator` attribution test green | serious | slice-doc | done | attested — `cd tools && cargo test render::tests::test_assemble_site_context_attribution_shapes_and_counts` → `... ok`; `cd tools && cargo test generator::tests::test_attribution_json_grouped_by_project` → `... ok`. | `generator::{build_sources_by_project, general_notice}` extracted (mirroring `build_provenance_map`/`build_featured`); `render::{Attribution, AttributionProject, build_attribution}`. Real numbers hand-verified via `grep -l` before writing the assertion: 91 sources, all attributed, across exactly 2 projects (`en.wikipedia.org`: 85, `www.rigpawiki.org`: 6, alphabetical) — matched exactly on the first run. `intro`/`licenses[]` come from the new `profile.attribution` section (added to `render::Profile`, `demo/profile.yaml`, **and** `templates/profile.sample.yaml` — the last one because slice 01's `test_resolve_profile_sample_yaml_parses_full_shape` would otherwise break on the newly-required field; verified by rerunning it). |
| F-5 | `render_attribution` renders `pages/attribution.jinja` end-to-end: `intro`, the licence callout (`general_notice`), per-project source tables (title/license/original-url links), and the licence-text cards (`licenses[]` title/points/links) | render test grepping the output | serious | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_attribution_full_chrome` → `... ok`. | `page::render_attribution`. Confirms `intro`/`general_notice` text present verbatim, a real `en.wikipedia.org` project heading with a source table and an internal `/source/` link, both curated license cards ("Content License", "Framework License") with their points and links. **Finding while writing this test:** the license-card `href` is entirely template-interpolated (`{{ link.url }}`, unlike the partially-literal `/source/{{ slug }}/` pattern elsewhere), so minijinja's attribute auto-escaping turns every `/` into `&#x2f;` — the first assertion (`href="https://creativecommons.org/..."`) failed against real, correctly-escaped output; fixed by checking for the un-slashed substring instead, same class of fix as Arc 01 slice 01's footer-link assertions. |
| F-6 | Escaping holds at page scale: an HTML control string in an auto-escaped attribution field (e.g. a source title) is escaped, while `profile.about_body` stays `\| safe` on the about page | render test injecting `<b>x</b>` and asserting `&lt;b&gt;` | correctness | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_attribution_escaping_holds_at_page_scale page::tests::test_render_about_safe_body_stays_unescaped` → both `... ok`. | Attribution: injects into a real source's `title` in the assembled `site.attribution` and re-renders the full page — escaped correctly. About: separately confirms `profile.about_body` renders raw HTML unescaped (the `\|safe` side of the same discipline), completing the pair the criterion names. |
| F-7 | These render paths use **no** Liquid/Cobalt template — only `render::build_environment` + `pages/{about,404,search,attribution}.jinja` | `grep`/test: no `.liquid` referenced (needle built by concatenation) | correctness | slice-doc | done | attested — `cd tools && cargo test page::tests::test_render_standalone_pages_path_references_no_liquid_templates` → `... ok`. | `include_str!`-greps `page.rs` (all four new renderers live there). Needle built by concatenation from the start, per the standing lesson. |
| F-8 | New code is `cargo clippy -p haleiki --all-targets -- -D warnings` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | run clippy **with `--all-targets`** + `cargo fmt --check`, scoped to new code | correctness | slice-doc | done | attested — `cargo fmt --all -- --check` → exit 0. `cargo clippy --all-targets --all-features --workspace -- -D warnings` → 0 new findings; the only two remaining are the same pre-existing `generator.rs` `assert!(x==y)` pair (now at shifted line numbers 783/785) plus the unchanged `demo/*.rs`/`tools/tests/demo_*.rs` set. | No new pedantic findings this slice — clean on the first `--all-targets` run. |

## What Worked

- Recognizing that adding a required `attribution` field to `render::Profile`
  would break slice 01's existing `test_resolve_profile_sample_yaml_parses_full_shape`
  (which reads `templates/profile.sample.yaml`, a *different* file from
  `demo/profile.yaml`) *before* running the tests, and updating both YAML
  files in the same pass, avoided a self-inflicted regression. Verified by
  rerunning the full suite immediately after, not assumed.
- The attribution-links escaping surprise (F-5) is the same class of thing
  Arc 01 slice 01 hit with footer links: when an entire attribute value is
  template-interpolated (`href="{{ link.url }}"`) rather than partially
  literal (`href="/source/{{ slug }}/"`), minijinja escapes every `/` in it.
  Having seen this pattern once already made it a one-line fix instead of a
  puzzle.
- Real hand-counted numbers before writing assertions (91 total sources, 85
  vs. 6 per project) continues to pay off — every render/assembly test this
  slice passed on the first or second try, with failures (when they
  happened) being test-assertion bugs (the escaped-slash case), not
  production-code bugs.
- Four renderers landed in one slice without friction because three of the
  four (`about`, `404`, `search`) are close to free — `about`/`404` are
  entirely profile-driven (already in `SiteContext` since Arc 01), and
  `search` is a static empty shell by design. Only `attribution` needed real
  assembly work, and it followed the now three-times-established
  `build_*`-extraction pattern exactly.

## Closure

Not yet closed — pending operator decision on whether/how to commit. All 8
rows reached `done` with `attested` evidence (see per-row Evidence column). No
rows deferred or no-op. CDC-equivalent independent verification has not yet
occurred — evidence strength is `attested`, not `reproduced`. This is Arc 02's
third slice: every page type the site has now exists as a renderer
(`render_home`, `render_browse`, `render_category`, `render_concept`/
`render_source` from Arc 01, and now `render_about`/`render_404`/
`render_search`/`render_attribution`). Only slice 04 — the static-export
walker that composes them all into `_site/` — remains before Arc 02 closes.
Rows: 8. Done: 8. Deferred: 0. No-op: 0.
