# Slice 01 — minijinja shell

> Plan-of-record for the first slice of Arc 01 (Render Core). Open set:
> this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** (planned, not implemented) · 2026-07-28.

---

## Goal

Stand up the minijinja **rendering spine** inside the `haleiki` crate: a
configured `Environment` that loads Claude Design's refactor templates, a
`profile.yaml` → `site.data.profile` resolver matching the refactor's context
contract, the asset set swapped to the unified `haleiki.css` + refactor JS, and
`base.jinja` + topnav + footer rendering correctly for a smoke route
(`pages/404.jinja`). This is the skeleton every later slice and arc hangs on.

## Scope

### In

- Add the `minijinja` dependency; introduce a `render` module owning the
  `Environment`, `path_loader`, and context types.
- Relocate the refactor's `templates/` to a repo-root `templates/` (the
  `path_loader` base) and consolidate its `assets/` into the repo `assets/`.
  Confirm this location with the operator before moving (Part VI micro-confirm);
  proposed default is repo-root `templates/`.
- Parse `profile.yaml` into a typed `ProfileContext` that serializes to the
  exact `site.data.profile` shape in `templates/README.md` /
  `templates/profile.sample.yaml`: `name`, `name_short`, `tagline`,
  `hero_subtitle`, `about_intro`, `about_body[]`, `content_notice`,
  `trust{enabled,default,level_order,levels{label,description,note}}`,
  `canon{enabled}`, `taxonomy{categories[],tiers[]}`, and
  `footer{columns[],copyright,meta[]}`.
- Wire the asset set: unified `haleiki.css` in, old modular CSS
  (`01-tokens.css` … `main.css`) out of the tiny-http asset set; add
  `search.js`, `toc-scrollspy.js`, `browse-scrollspy.js`; retain `theme.js`
  (and `math.js`/KaTeX). Confirm the 8 refactor woff2 are present and referenced
  by `haleiki.css`.
- Establish the two rendering disciplines the shell depends on: the `| safe`
  filter on pre-rendered HTML fields (here, `profile.about_body`) with
  auto-escaping everywhere else, and the `active_nav` block-capture mechanism in
  `base.jinja` reaching the included topnav.
- Render `pages/404.jinja` (self-contained, minimal context) as the smoke target
  and assert the produced HTML.

### Out

- Markdown → HTML rendering and TOC extraction (slice 02).
- Concept/source `page.data` assembly, see-also, provenance (slice 03).
- Any non-article route: home, browse, category, about, attribution, search
  (Arc 02).
- The tiny_http server and static export writer (Arcs 02–03).
- Search index, news, deleting the Cobalt/Liquid files repo-wide (later arcs;
  this slice only stops *using* Liquid for the smoke route).

## Verification approach

Unit + render tests in the `haleiki` crate: profile-resolution tests asserting
the parsed shape; a render smoke test asserting the rendered `404` HTML contains
the expected chrome (stylesheet link, no-flash theme script, topnav, footer) and
that escaping behaves (an injected control string in an auto-escaped field is
escaped; a `| safe` field is not). `cargo clippy` (deny all / warn pedantic) and
`cargo fmt` clean on new code. Manual: render the 404 to a file and eyeball
against the mockup.

## Exit criteria

The [ledger](./ledger.md) rows F-1 … F-8 all reach a final status. In short: the
`Environment` loads the refactor templates; `profile.yaml` resolves to the
contract shape; `base.jinja` renders with the resolved profile and correct
assets; the escape/`| safe` and `active_nav` disciplines are demonstrated; the
asset swap is complete for the tiny-http path; and new code is clippy/fmt clean.

## Notes

- **Context contract authority:** `workbench/Hale'iki Wiki Design Refactor/
  templates/README.md`. Do not invent context shapes; satisfy that file.
- The refactor's `templates/README.md` "Rendering (Rust sketch)" is the intended
  wiring (`path_loader("templates")`, `context!{ site, page }`).
- Reuse boundary: do **not** modify `parser.rs`, `graph.rs`, or `validator.rs`
  in this slice.
