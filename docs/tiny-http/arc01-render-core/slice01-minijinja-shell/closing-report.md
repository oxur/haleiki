# Slice 01 — minijinja shell · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 8 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run. Not yet committed. 2026-07-28.

## Row-by-row walk

**F-1 — Environment + path_loader resolves `pages/404.jinja` and its includes.**
`done`. `cargo test render::tests::test_environment_loads_and_renders_404_template`
passes: `env.get_template("pages/404.jinja")` resolves the template plus its
`{% extends "base.jinja" %}` and `base.jinja`'s two `{% include %}`s
(`partials/topnav.jinja`, `partials/footer.jinja`) with no load/parse error, and
the render produces non-empty HTML.

**F-2 — `profile.yaml` parses into the `site.data.profile` shape.** `done`.
`cargo test render::tests::test_resolve_profile_sample_yaml_parses_full_shape`
passes. The typed `Profile` struct in `tools/src/render.rs` mirrors
`templates/README.md`'s contract field-for-field (`name`, `name_short`, `tagline`,
`hero_subtitle`, `about_intro`, `about_body[]`, `content_notice`, `trust{…}`,
`canon{enabled}`, `taxonomy{…}`, `footer{…}`); `resolve_profile()` parses
`templates/profile.sample.yaml` with `serde_yaml` and the test asserts
`trust.levels["credible"]` has `label`/`description`/`note`, `trust.level_order`
lists all four levels, and `taxonomy`/`footer` are populated.

**F-3 — `base.jinja` renders full chrome from the resolved profile.** `done`.
`cargo test render::tests::test_base_jinja_renders_chrome_from_profile` passes.
Renders `base.jinja` undecorated (no child template), which exercises the
*default* `{% block title %}` — `{{ page.title }} — {{ site.data.profile.name }}`
— rather than a child override. Asserts the literal `<title>` string, the
`haleiki.css` stylesheet `<link>`, the no-flash theme bootstrap `<script>`, and
the included topnav (`class="topnav"`) and footer (`class="site-footer"`).

**F-4 — Escaping discipline (`| safe` vs auto-escape).** `done`, **with a
disclosed deviation from the literal criterion and one load-bearing finding.**
`cargo test render::tests::test_escaping_safe_vs_autoescaped_fields` passes.

- *Deviation:* the criterion's example field is `profile.name`; the test instead
  injects the control string into `profile.about_intro` and renders
  `pages/about.jinja`, the actual README-documented consumer of both
  `about_intro` (auto-escaped) and `about_body` (`| safe`) together. Same
  discipline demonstrated, on the fields the contract actually pairs.
- *Finding, not a deviation — a defect this row exists to catch:* minijinja's
  default auto-escape callback keys off `.html`/`.htm`/`.xml` and treats
  `.jinja`/`.jinja2`/`.j2` as an *ignored* suffix stripped before the extension
  check — so every template in this set (named `*.jinja`, not `*.html.jinja`)
  would silently resolve to `AutoEscape::None`. Had F-4 not been written and run,
  the entire `| safe` discipline would have shipped as a no-op: nothing would be
  escaped, not just the fields meant to bypass escaping. `build_environment()`
  now sets `env.set_auto_escape_callback(|_| AutoEscape::Html)` explicitly. This
  is the single most consequential thing this slice found.

**F-5 — Asset swap (CSS + JS).** `done`. `grep` across `templates/` for the old
modular CSS filenames (`01-tokens.css` … `07-trust-tokens.css`, `main.css`)
returns no matches; `haleiki.css` is referenced once, in `base.jinja:30`.
`search.js`, `toc-scrollspy.js`, `browse-scrollspy.js` are present in
`assets/js/` (copied from the refactor) alongside the retained `theme.js` and
`math.js`. Note: the old modular CSS files still exist at repo-root
`assets/css/` — untouched, because they remain in active use by the
Cobalt/Liquid path (`demo/_layouts/*.liquid`), which this slice's boundary
explicitly does not touch ("this slice only stops *using* Liquid for the smoke
route"). F-5 is about the tiny-http template path, which is clean.

**F-6 — Fonts present and referenced.** `done`. `assets/fonts/` has exactly 8
`.woff2` files (Newsreader ×2, RedHatDisplay ×2, IBMPlexMono ×4);
`assets/css/haleiki.css` has 8 `@font-face` rules and 8 matching
`url('../fonts/…woff2')` references, one per file. These fonts were already
present at repo-root `assets/fonts/` before this slice (confirmed byte-identical
to the refactor's copies via `cmp`) — design-delta §3.1 called this out
("fonts already on this branch … already match the refactor"); only
`haleiki.css` itself was new.

**F-7 — `active_nav` block-capture reaches the included topnav.** `done`.
`cargo test render::tests::test_active_nav_block_capture_marks_topnav_item`
passes. Registers a minimal in-memory template via `env.add_template` (which
coexists with the `path_loader` — explicit templates are checked before the
loader falls back to disk) that extends `base.jinja` and sets
`{% block active_nav %}browse{% endblock %}`; asserts the rendered topnav marks
Browse `class="active" aria-current="page"` and does *not* mark Categories
active. No template file was added under `templates/pages/` — the boundary
against adding a non-404 route is intact; this is a unit test fixture, not a
route.

**F-8 — New code is clippy-clean (deny `all`, warn `pedantic`) and
fmt-clean.** `done`, **scoped to new code, per the criterion's own wording** —
with a disclosed pre-existing gap named rather than worked around.
`cargo fmt --all -- --check` is clean workspace-wide (exit 0). For clippy:
`cargo clippy --all-features --workspace -- -D warnings` currently reports 6
errors, **zero of which touch `tools/src/render.rs`** or the one-line
`mod render;` addition to `tools/src/main.rs` (confirmed by filtering clippy's
output for `src/render.rs`: 0 hits). The 6 pre-existing errors —
`generator.rs:541` (`unnecessary_sort_by`), `main.rs:57` (`map_unwrap_or`, in
the pre-existing `Dev` command — unrelated to render), and 4 more in
`demo/{convert,frontmatter,media,rewrite}.rs` (only surfaced under
`--all-features`) — predate this slice. Verified via `git diff HEAD --
tools/src/generator.rs` (empty) and comparing `git show HEAD:tools/src/generator.rs`
line 541 against the working tree (identical). These files are outside this
slice's boundary (not `parser.rs`/`graph.rs`/`validator.rs`, but also not code
this slice added or was asked to touch); fixing them is a separate, small,
low-risk cleanup this report flags rather than folds in silently.
**Consequence:** `make lint` will not go green on this branch until that
pre-existing debt is fixed — a fact worth knowing before treating `make check`
as a merge gate.

## Silent-drop check

Ledger opened with 8 rows (F-1…F-8); this report addresses 8. No missing rows.

## Bubble-up to arc-plan.md

One finding worth carrying into Arc 01's composition rows (A-4/A-5/A-6, which
require escaping correctness "reproduced at arc scale"): **the auto-escape
callback must be set explicitly on every `Environment` this arc or later slices
construct** — it is not a property of the loader or the template files, it's a
per-`Environment` setting, and the default is wrong for this template set. If
slice 02 or 03 constructs a second `Environment` (e.g. for a different render
entry point) rather than reusing `render::build_environment`, this defect
reappears. Recommend arc-plan.md's A-6 verification step explicitly re-check the
callback is set, not just that one page's output happens to be escaped.

## What was NOT done (explicitly, per the "no silent drops" rule)

- No CLI/route wiring: `render::build_environment` and `resolve_profile` are not
  called from `main.rs`/`cli.rs`. Out of scope — routing is Arc 02, the server is
  Arc 03.
- No markdown rendering, no `page.content`/TOC — slice 02.
- No concept/source `page.data` assembly, no `see_also`/`provenance` — slice 03.
- `demo/profile.yaml` (the old, minimal Cobalt-era shape) was not migrated to the
  new `site.data.profile` shape — F-2's test fixture is
  `templates/profile.sample.yaml` per the slice-doc's own verification approach,
  not `demo/profile.yaml`. Migrating the demo profile is implied but not
  scheduled by name in the arc-plan; flagging so it isn't assumed done.
- The pre-existing clippy debt named in F-8 was not fixed (see above).
- Nothing was committed to git. All changes are in the working tree.

## Independent verification status

Not yet performed. Per `LEDGER-DISCIPLINE.md`, this closing report is CC's
`attested` claim; a CDC-equivalent pass (fresh context or operator, re-running
each row's Verify command against the actual artifacts) is required before any
row's evidence strengthens to `reproduced` and before this slice can be
considered closed rather than proposed-done.
