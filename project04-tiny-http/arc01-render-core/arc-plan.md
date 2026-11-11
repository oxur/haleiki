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


See the sibling [ledger.md](./ledger.md). All original row IDs, criteria,
statuses and evidence are retained there.

## 5. Version history


### Migration — 2026-11-11

Moved from `docs/tiny-http/` to orphan branch `planning`, worktree
`.worktrees/planning/project-04-tiny-http/`. The embedded ledger now lives in sibling
`ledger.md`; row text and acceptance states are unchanged. See the
[byte-preserved August plan](../../archive/docs/tiny-http/arc01-render-core/arc-plan.md). Existing issued prompts, ledgers, close reports,
verification records and `slice-doc.md` documents retain their bytes.
Canonical `slice-plan.md` entrypoints link the original slice documents.
Historical repository-relative paths resolve via the root migration manifest;
source paths such as `tools/`, `templates/` and `demo/` refer to the implementation
checkout, not this planning-only worktree. No feature verification was rerun.

### v1.5 — 2026-07-28 (bubble-up from slice 03)
Slice 03 (wiki-article context) closed (proposed-done; see
`slice03-wiki-article-context/closing-report.md`) — Arc 01's third and final
slice. It delivered the article render pipeline (`article::render_concept` /
`render_source`) and, in doing so, produced the evidence this arc's
composition rows (A-4…A-7) will reproduce at arc scale: a real demo concept
and source each render end-to-end with full chrome, escaping holds at page
scale, and the path uses no Liquid. Per `LEDGER-DISCIPLINE.md` Section B,
A-4…A-7 still require **independent reproduction at arc scale** by whoever
closes the arc — slice 03's tests are evidence for that, not a substitute.

Two further items surfaced:

- **A new pre-existing-debt category**: running `cargo clippy --all-targets`
  (per the slice-02 lesson, and per slice 03's own cc-prompt instruction)
  found 28 findings invisible to every prior lint pass, all of it — including
  the v1.4 remediation — having run clippy *without* `--all-targets`.
  Entirely in `demo/{convert,media,clean,frontmatter,rewrite}.rs` and
  `tools/tests/demo_fetch.rs`, plus two `assert!(x == y)` in `generator.rs`'s
  own pre-existing test. **Disposition:** route to the same cleanup pass as
  the v1.2/v1.4 debt; whatever CI gate eventually gets wired in should include
  `--all-targets`, not just `--all-features`.
- **`demo/profile.yaml` migration (carried from v1.2/v1.3) is done.** Slice 03
  rewrote it to the render `Profile` shape, with `taxonomy` corrected to the
  real `demo/taxonomy.yaml` (10 categories, 3 tiers) and the `about_body`
  source-project list corrected to what the corpus actually cites (Wikipedia +
  Rigpa Wiki only — no Wikibooks content exists here, despite
  `templates/profile.sample.yaml`'s copy).

### v1.4 — 2026-07-28 (operator-directed remediation)
The v1.2 "pre-existing lint debt blocks `make lint`/CI" finding is **resolved**.
At the operator's direct instruction (outside the slice-02 ledger's scope), all
six pre-existing `cargo clippy --all-features --workspace -- -D warnings` errors
were fixed rather than left for a dedicated cleanup slice: `generator.rs:541`
(`sort_by` → `sort_by_key`), `main.rs:57` (`map(...).unwrap_or(false)` →
`is_ok_and(...)`), and four in `demo/{convert,frontmatter,media,rewrite}.rs`
(three unnecessary-trailing-comma removals, one `loop` → `while let`). The
project's (gitignored) `Cargo.lock` was also refreshed via `cargo update` — no
`Cargo.toml` version-requirement changes were needed; `make check-deps` was
already reporting "up to date" (nothing exceeded its SemVer requirement) both
before and after. Verified: `make check` (build + lint + test, `--all-features
--workspace`) passes clean end-to-end — 327 lib tests + 14 + 3 integration
tests, 0 clippy/fmt findings. `make check`/`make lint` is now a green merge
gate on this branch, superseding the "not a green gate until fixed" note in
v1.2.

### v1.3 — 2026-07-28 (bubble-up from slice 02 CDC close)
Slice 02 (markdown render + TOC) closed via independent verification
(`slice02-markdown-toc/cdc-verification.md`): 9/9 rows reproduced. F-9 was
returned once — independent `clippy --all-targets -D warnings` found a pedantic
`uninlined_format_args` in the new test asserts that CC's non-`--all-targets`
run had missed — then fixed and re-confirmed clean. Two findings this slice
surfaced were **resolved immediately** (operator-directed), not deferred:

- **`source.jinja` did not load KaTeX** → source-page math would render raw.
  Fixed: added a `head_extra` block loading `katex.min.css` + `katex.min.js` +
  `math.js`. Loads unconditionally; a conditional `page.data.has_math` load is a
  possible slice-03 optimization.
- **`concept.jinja` loaded the now-dead `auto-render.min.js`.** Fixed: removed
  (`math.js` renders comrak's `data-math-style` spans via `katex.render`).

Both template edits parse-verified via `build_environment().get_template(...)`.
Carried into slice 03 (from v1.2, still open): retain the markdown body in the
parser, and migrate `demo/profile.yaml` to the new profile shape.

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
