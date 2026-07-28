# Arc 01 — Render Core

> The arc plan-of-record: the minijinja rendering spine that replaces the
> Cobalt/Liquid renderer for the wiki-article path. Part of the tiny_http
> pivot ([`../project-plan.md`](../project-plan.md)); architecture in
> [`../design-delta.md`](../design-delta.md). Status: **active** · 2026-07-28.

---

## 1. Capability

Render a **fully-chromed wiki article — both the concept and the source page
type — to correct HTML from the real demo corpus**, using minijinja over Claude
Design's refactor template set and the unified `haleiki.css`, replacing the
Cobalt/Liquid renderer for the article path.

"Fully-chromed" means: the shared shell (topnav, footer, no-flash theme boot,
skip link) from `base.jinja`; the trust bar and trust glyph; the taxonomy
sidebar; the see-also and provenance blocks; the breadcrumb; and the body
rendered from Markdown with section anchors and a table of contents — all
populated from the resolved `site.data.profile` and per-page context exactly as
the refactor's `templates/README.md` contract specifies.

This is the load-bearing foundation: nothing in later arcs (home/browse/category
routes, the server, search, news) paints until the render spine and the
article-context assembly exist. It deliberately excludes every non-article route
(Arc 02), the server (Arc 03), search (Arc 04), and news (Arc 05).

## 2. Slice breakdown

Three slices, each sized to one context with iteration headroom, each
load-bearing for the next.

| Slice | Scope | Load-bearing for |
|-------|-------|------------------|
| **01 — minijinja shell** | Add minijinja; `Environment` + `path_loader` over the refactor templates; resolve `profile.yaml` → the `site.data.profile` context; swap the asset set to the unified `haleiki.css` + refactor JS; render `base.jinja` + topnav + footer for a smoke route (`404.jinja`). Establishes the `| safe`/auto-escape discipline and the `active_nav` block-capture. | 02, 03 |
| **02 — markdown render + TOC** | Render body Markdown → HTML in Rust (comrak) with heading anchor ids; extract `page.data.toc[]`; produce the `page.content` (`\| safe`) contract. | 03 |
| **03 — wiki-article context** | Assemble the concept + source `page.data` (facts, subject_headings, aliases, category/tier/type, disambiguator) and `site.data.see_also[slug]` / `site.data.provenance[slug]` — porting the existing `generator.rs` derivations to the minijinja shape — and render `concept.jinja` + `source.jinja` end-to-end from the demo corpus. | Arc 02+ |

Slices 02 and 03 are planned deep only when they become the next work; slice 01's
open set is written (`slice01-minijinja-shell/`).

## 3. Dependencies

**Consumes** (reused, not modified in this arc): `parser::parse_corpus →
Corpus`; `graph::HaleikiGraph`; `validator::validate`; and the graph-derived
computations currently in `generator.rs` (see-also, provenance, breadcrumbs,
category indices) — reshaped, not re-derived. The fonts already on the branch;
client-side KaTeX assets.

**Leaves for later arcs**: all non-article routes and the static-export writer
(Arc 02); the tiny_http server (Arc 03); the search index and `search.jinja`
(Arc 04); the entire news content type (Arc 05).

**Introduces** (new, owned by this arc): the `render` module (minijinja
`Environment`, loader, filters, context types); the `profile.yaml` → context
resolver; Markdown→HTML rendering; the repo-root `templates/` home and
consolidated `assets/`.

## 4. Arc ledger

Opens here; closes (per-row walk) in `closing-report.md` at arc close. Class (a)
= slices closed (attested by pointer); class (b) = composition, **reproduced at
arc scale** (actually render real demo pages and inspect); class (c) = bubble-up
findings dispositioned. Rows accrue as slices close.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence |
|----|-----------|--------|--------------|--------|--------|----------|
| A-1 | Slice 01 (minijinja shell) closed | ptr: `slice01-minijinja-shell/cdc-verification.md` | correctness | arc-plan | done | reproduced — slice 01 CDC-closed 2026-07-28, 8/8 rows reproduced |
| A-2 | Slice 02 (markdown + TOC) closed | ptr: slice02 cdc-verification | correctness | arc-plan | open | |
| A-3 | Slice 03 (wiki-article context) closed | ptr: slice03 cdc-verification | correctness | arc-plan | open | |
| A-4 | A real demo **concept** page renders end-to-end with full chrome (trust bar, taxonomy sidebar, see-also, provenance, breadcrumb, TOC) via the render path | reproduce at arc scale: render a named demo concept, inspect HTML against the mockup | serious | arc-plan | open | |
| A-5 | A real demo **source** page renders end-to-end with full chrome | reproduce at arc scale: render a named demo source, inspect | serious | arc-plan | open | |
| A-6 | Escaping is correct: only `page.content`/`profile.about_body` (and other contract fields) are `\| safe`; all other fields auto-escape | reproduce: inject an HTML control string into an escaped field, confirm it is escaped | correctness | arc-plan | open | |
| A-7 | The article path uses **no** Liquid/Cobalt template for rendering | `grep` the render path; Liquid article render is gone | correctness | arc-plan | open | |

_(Class-(c) bubble-up rows are added as slices surface findings.)_

## 5. Version history

### v1.2 — 2026-07-28 (bubble-up from slice 01 CDC close)
Slice 01's independent verification (`slice01-minijinja-shell/cdc-verification.md`)
closed the slice — all 8 rows reproduced — and surfaced two tracked findings that
belong to this arc but sit outside slice 01's boundary:

- **Pre-existing lint debt blocks `make lint` / CI.** `cargo clippy -D warnings`
  fails on debt that predates this arc: `generator.rs:541` (`unnecessary_sort_by`),
  `main.rs:57` (`map_unwrap_or`, in the pre-existing `Dev` command), and four in
  `demo/{convert,frontmatter,media,rewrite}.rs` (under `--all-features`). None are
  in slice-01 code (`render.rs` is clippy-clean). **Disposition:** route to a
  dedicated small cleanup slice — fold into Arc 06 ("remove Cobalt/Liquid remnants
  + lint") or a standalone near-term slice. Until then, `make check`/`make lint`
  is not a green merge gate on this branch.
- **`demo/profile.yaml` not yet migrated to the new `site.data.profile` shape.**
  The typed `Profile`'s required fields mean the Cobalt-era `demo/profile.yaml`
  will not deserialize; slice 01 correctly tested against
  `templates/profile.sample.yaml`. **Disposition:** name the migration explicitly
  in the scope of the first slice that renders real demo content (slice 03, or
  Arc 02), so it isn't assumed done.

### v1.1 — 2026-07-28 (bubble-up from slice 01)
Slice 01 (minijinja shell) closed (proposed-done; see
`slice01-minijinja-shell/closing-report.md`) and surfaced one finding load-bearing
for this arc's composition rows: minijinja's default auto-escape detection treats
`.jinja` as an ignored suffix and falls back to `AutoEscape::None`, so **every**
`Environment` this arc (or later slices) constructs must explicitly set
`set_auto_escape_callback(|_| AutoEscape::Html)` — it is a per-`Environment`
setting, not a property of the loader or the template files. A-6 ("escaping is
correct … reproduced at arc scale") should verify this callback is set on
whatever `Environment` slice 02/03 end up using, not just that one page's output
happens to be escaped. `render::build_environment()` (`tools/src/render.rs`,
slice 01) sets it correctly; reuse that constructor rather than building a second
`Environment` ad hoc.

### v1.0 — 2026-07-28
Initial arc plan. Capability (render a fully-chromed concept + source article via
minijinja from the demo corpus), three-slice breakdown (minijinja shell →
markdown+TOC → wiki-article context), dependencies against the reused content
pipeline, and the arc ledger opened with the composition rows. Slice 01 planned
deep; slices 02–03 roadmap-level.
