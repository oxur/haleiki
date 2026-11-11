# Slice 04 — static export writer

> Plan-of-record for the fourth and final slice of Arc 02 (Routes & Static
> Export). Open set: this doc + [`ledger.md`](./ledger.md) +
> [`cc-prompt.md`](./cc-prompt.md). Status: **open** (planned, not implemented)
> · 2026-07-28.

---

## Goal

Walk **every route** through the page renderers built in slices 01–03 (and Arc
01), write a complete `_site/` of static HTML, copy the site assets, and wire a
CLI command — the pivot's **default deliverable**. This is the arc's capstone:
its tests reproduce the arc composition rows (B-5…B-8), and closing it closes
Arc 02.

This slice is meatier than its siblings: it does real filesystem output and
touches the CLI for the first time in the pivot.

## The route → path map

| Route | File written |
|-------|--------------|
| `/` (home) | `_site/index.html` |
| `/browse/` | `_site/browse/index.html` |
| `/category/{slug}/` (per category) | `_site/category/{slug}/index.html` |
| `/{slug}/` (per concept) | `_site/{slug}/index.html` |
| `/source/{slug}/` (per source) | `_site/source/{slug}/index.html` |
| `/about/` | `_site/about/index.html` |
| `/attribution/` | `_site/attribution/index.html` |
| `/search/` (shell) | `_site/search/index.html` |
| `/404.html` | `_site/404.html` (not `404/index.html` — static hosts serve this) |
| `/assets/…` | `_site/assets/…` (copied tree, allowlist below) |

## Scope

### In

- A new `export` module (`tools/src/export.rs`): an `export_site(content_dir,
  templates_dir, assets_dir, out_dir)` that parses → validates → builds graph →
  `assemble_site_context` → renders each route via the existing
  `page::render_*` / `article::render_*` and writes it, then copies assets.
- **Route walk**: the singletons (home, browse, about, attribution, search,
  404) once; `render_category` per category (from the taxonomy/`category_index`);
  `render_concept` per concept; `render_source` per source. A `write_page(out,
  route, html)` helper that creates parent dirs and writes `index.html` (or
  `404.html` for the 404 route).
- **Asset copy**: a hand-rolled recursive `std::fs` copy (no new dependency) of
  an **allowlist** of site-asset subdirs — `css`, `fonts`, `icons`, `images`,
  `js`, `vendor` — into `_site/assets/`. **Excludes** `assets/ai/` (the Rust
  dev skill) and `assets/mockups/` (design sources), which are not site assets.
- **Clean-before-write**: remove any existing `_site/` before rendering, so a
  stale file from a prior build cannot linger.
- **CLI wiring**: a subcommand that runs `export_site` (see decision #1).
- Tests: render the demo to a temp `_site/` and assert the file tree, the asset
  allowlist, clean-before-write, link resolution, and spot page contents.

### Out

- The tiny_http **server** sink (Arc 03) — this slice is the static sink only.
- The search JSON index / `search.js` behavior / facets (Arc 04).
- Removing the legacy `_data` JSON writer or the Cobalt/Liquid files (Arc 06
  cleanup) — this slice leaves `generator::generate` and the `build` command as
  they are (see decision #1).
- News (Arc 05).

## Decisions to confirm at slice start

- **CLI command surface.** Proposed: add a **new subcommand** (e.g. `haleiki
  export --content-dir demo --out _site`) that renders `_site/`, leaving the
  existing `build` (→ `_data` JSON) untouched — fully additive, no disruption to
  the legacy path mid-arc. The design-delta's eventual "`haleiki build` renders
  `_site/`" is an **Arc 06** end-state (when the JSON writer is dropped and the
  commands reconcile). Confirm the name (`export` / `build-site` / `site`) and
  the additive-vs-extend-`build` choice.
- **Asset allowlist.** Proposed: copy `css`, `fonts`, `icons`, `images`, `js`,
  `vendor`; exclude `ai`, `mockups`. Confirm the set.
- **Clean-before-write.** Proposed: `remove_dir_all(_site)` (if present) before
  rendering. Confirm (vs. write-over-in-place).
- **Templates/assets location.** Proposed: default to repo-root `templates/` and
  `assets/` (overridable args), consistent with how the render tests resolve
  them.

## Verification approach

Integration-style tests writing to a temp dir from the `demo/` corpus: assert the
full `_site/` file tree exists (home, browse, every category, every concept,
every source, about, attribution, search, `404.html`); assert
`_site/assets/{css,fonts,icons,images,js,vendor}` present and `ai`/`mockups`
absent; assert clean-before-write (a planted stale file is gone after re-export);
assert link resolution (home → a `category/{slug}/index.html`; a category →
an article `index.html`; a concept's breadcrumb/see-also targets exist); and
spot-check a written concept page + home page for expected chrome. Plus a CLI
smoke test (the subcommand runs and produces `_site/`). `cargo clippy -p haleiki
--all-targets -- -D warnings` + `cargo fmt --check` clean on new code.

## Exit criteria

Ledger rows F-1…F-8 reach a final status. In short: `export_site` writes a
complete, correctly-pathed `_site/` from the demo corpus; the asset allowlist is
copied and non-site dirs excluded; clean-before-write works; internal links
resolve as files; written pages match the renderers; the CLI subcommand drives
it; and new code is clippy(`--all-targets`)/fmt-clean.

## Relationship to the arc close

This is Arc 02's last slice. Its `_site/` export is the demonstration the arc
composition rows reproduce: **B-5** (complete `_site/`), **B-6** (assets copied),
**B-7** (grouping-in-Rust, already delivered in slice 02, confirmed in the
written browse/category pages), **B-8** (links resolve). After this slice
CDC-closes, the arc closes: reproduce B-5…B-8 at arc scale, write
`arc02-routes-static-export/closing-report.md`, and bubble up to `project-plan.md`
(P-2).

## Notes

- Context/renderer authority: the `page::render_*` and `article::render_*`
  functions from slices 01–03 and Arc 01 — the walker composes them, it does not
  re-render.
- Reuse `render::build_environment` (one `Environment`, auto-escape per it).
  Run clippy **with `--all-targets`** (F-8). Deterministic output (stable
  iteration order) so re-exports are byte-stable — helpful for the eventual CI.
