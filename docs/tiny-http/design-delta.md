# Haleiki — tiny_http / minijinja Pivot · Design Delta

> SDLC steps 1–3 (research · project definition · design), condensed into one
> document. This is the architectural record for the pivot on branch
> `framework-base/tiny-http`. It is the *design* doc the `project-plan.md`
> points to; the plan itself carries the roadmap and the ledger.
>
> Status: **draft** · 2026-07-28 · living document.

---

## 1. Why this pivot

The Cobalt line (`framework-base/cobalt`, milestones M1–M8) delivered a working
static wiki: a Rust pre-build CLI parses the corpus, builds a petgraph
relationship graph, validates it, and writes `_data/*.json`; **Cobalt** then
renders **Liquid** templates against that data, and **Pagefind** builds search.
It works, but it carries three heavy runtime dependencies — Cobalt, the Liquid
template language, and Pagefind — for what is fundamentally a small, read-only
site.

The pivot replaces that whole rendering-and-serving tail with a minimal,
Rust-native stack we own end to end:

- **[minijinja](https://docs.rs/minijinja)** — a Jinja2-compatible template
  engine, embedded directly in the `haleiki` binary. Replaces Cobalt + Liquid.
- **[tiny_http](https://docs.rs/tiny_http)** — a tiny synchronous HTTP server.
  Replaces `cobalt serve`, and (per the deployment decision below) is hardened
  into an optional self-hosted deployment target.
- **A custom JSON search index** built by the Rust tool and queried
  client-side by the design team's `search.js`. Replaces Pagefind.

Claude Design has already produced the replacement template set — a complete
minijinja rendering surface — in
`workbench/Hale'iki Wiki Design Refactor/`. **These templates are authoritative
and must be used as delivered.** The `main`-branch design integration is
explicitly abandoned. The refactor's own `templates/README.md` is the binding
**context contract** between the Rust side and the templates; this pivot's job
is to make the Rust side satisfy that contract.

## 2. The four decisions (confirmed with the operator, 2026-07-28)

1. **Deployment model: both.** Static export is the default deliverable
   (`haleiki build` → `_site/` of static HTML/CSS/JS, deployable to any static
   host), **and** the `tiny_http` server is hardened enough to run as an
   optional self-hosted deployment (`haleiki serve`). Both paths render through
   the same minijinja layer, so there is one renderer and two sinks.
2. **News is in scope.** The refactor ships a full news/blog surface
   (`news-index`, `news-post`, `news-archive`, `news-tags`, `news-categories`).
   The current architecture and content model have no news type, so this pivot
   introduces one: a new content kind, its parser, its routing, and its context
   assembly.
3. **Search: custom lightweight JSON index.** The Rust build emits a small
   search-index JSON; the refactor's `search.js` does client-side query and
   faceting. No Pagefind.
4. **Planning layout: framework arc/slice tree**, rooted at `docs/tiny-http/`
   with no version-numbered directory (operator override of the canonical
   `docs/design-vX.Y.Z/` default).

## 3. What is reused, what changes, what is new

The pivot is a *rendering-and-serving* replacement, not a content-model rewrite.
The content pipeline is the durable half and mostly survives.

### 3.1 Reused essentially as-is

- **Content model** for source pages and concept cards (frontmatter, body
  sections) — unchanged.
- **`parser.rs`** (parse corpus → `Corpus`), **`graph.rs`** (petgraph relationship
  graph), **`validator.rs`** (references, orphans, conflicts). These are the
  load-bearing pre-build and are not touched by the render change.
- **The graph-derived computations** already inside `generator.rs` — see-also
  groups, provenance links, category indices, breadcrumbs, featured selection,
  attribution aggregation. The *computation* is reused; only the *output shape
  and sink* change (see below).
- **The `demo/` pipeline** (`fetch`/`clean`/`convert`/`media`/`manifest`/
  `frontmatter`, feature-gated) that produces the demo corpus — unchanged.
- **The fonts** already on this branch (`Newsreader`, `RedHatDisplay`,
  `IBMPlexMono` woff2) — they already match the refactor. Client-side KaTeX
  (`assets/vendor/katex`, `math.js`) stays as a runtime asset for the
  math/physics content.

### 3.2 Changes

- **`generator.rs` becomes a context-builder + renderer**, not a JSON emitter.
  Today it writes nine `_data/*.json` files (`graph`, `categories`,
  `breadcrumbs`, `see_also`, `provenance`, `profile`, `featured`, `titles`,
  `attribution`) shaped for Liquid. The minijinja contract is different — it
  wants resolved `groups[]`, nested `page.data.*`, and a single `site.data.*`
  tree — and grouping/de-duplication that used to live in Liquid now lives in
  Rust (per the refactor README: *"Grouping is done in Rust, not the
  template"*). The derivation logic ports over; the assembly and the output
  target (rendered HTML, not JSON) are rewritten.
- **`profile.yaml` resolution** must produce the exact `site.data.profile`
  shape the templates consume (see `templates/profile.sample.yaml` and the
  README context contract): `name`, `name_short`, `tagline`, `hero_subtitle`,
  `about_intro`, `about_body[]` (pre-rendered HTML), `content_notice`,
  `trust{enabled,default,level_order,levels{label,description,note}}`, `canon`,
  `taxonomy`, and the unified `footer{columns,copyright,meta}`.
- **Asset set**: the old modular CSS (`01-tokens.css` … `main.css`) is replaced
  by the refactor's single unified `haleiki.css`; the refactor's JS
  (`search.js`, `toc-scrollspy.js`, `browse-scrollspy.js`) joins the existing
  `theme.js`/`math.js`.
- **CLI surface**: `build` now renders HTML (not just JSON); `serve` becomes a
  real `tiny_http` server (was a stub pointing at `cobalt serve`); `search`
  builds the JSON index (was a Pagefind stub).

### 3.3 New

- **A minijinja rendering layer** — an `Environment` with
  `path_loader("templates")`, the `| safe` discipline for pre-rendered HTML
  fields (`page.content`, `post.body`, `result.excerpt`,
  `profile.about_body`), auto-escaping everywhere else, and the `active_nav`
  block-capture mechanism the base template relies on.
- **Markdown → HTML in Rust** — Cobalt used to do this. We now render body
  Markdown ourselves (recommended: [`comrak`](https://docs.rs/comrak),
  CommonMark + GFM), emitting section anchor ids and extracting a `toc[]` for
  the concept/source contract. (`pulldown-cmark` is the lighter fallback if we
  want fewer features.)
- **The `tiny_http` server** — routing (path → template + context), static
  asset serving with correct content types, configurable bind/port, structured
  errors and logging, hardened for optional self-host.
- **Static export** — walk every route, render to files under `_site/`, copy
  assets. The default deliverable.
- **Custom search** — a JSON index builder (title, url, category, tier, type,
  trust, excerpt) plus the `search.jinja` route wired to `search.js` with
  facets for category / tier / page-type / trust.
- **The news content type** — a new content kind with its own frontmatter
  (title, date, category, tags, author, body), parser, routing, and context
  assembly for the five news templates, including prev/next/related and
  date-grouped archives.

## 4. Target architecture

```
Markdown corpus (sources, concepts, news)         ← authoring (unchanged)
        │
   parser.rs → Corpus                              ← reused
        │
   graph.rs → HaleikiGraph                         ← reused
        │
   validator.rs                                    ← reused
        │
   context assembly  (was generator.rs → _data)    ← rewritten: build site.*/page.*/post.*/news.*/search.*
        │                                             per templates/README.md contract
        ├────────────► minijinja Environment ──► HTML   ← new render layer (replaces Cobalt/Liquid)
        │                     │
        │         ┌───────────┴────────────┐
        │         ▼                        ▼
   search index   static export (_site/)   tiny_http serve   ← two sinks, one renderer
   (JSON)         [default deliverable]     [optional self-host]
                         │                        │
                         ▼                        ▼
                  static host              long-running server
```

Runtime for the static sink remains "static files only" — matching the design
team's own framing in `profile.sample.yaml` (*"runtime as static HTML, CSS, and
JavaScript"*). The server sink renders the same output for self-hosting.

## 5. The context contract (source of truth)

`workbench/Hale'iki Wiki Design Refactor/templates/README.md` defines, per
template, exactly what `site.*` / `page.*` / `post.*` / `search.*` / `news.*`
shapes are required. That file — not this one — is the authority the Rust
context assembly is written against. This design delta names *that* the
contract exists and where it lives; the per-arc plans discharge it template by
template. The one rule worth restating here because it is a correctness
boundary: **only `page.content`, `post.body`, `result.excerpt`, and
`profile.about_body` are printed `| safe`; everything else is auto-escaped.**

## 6. Boundaries (explicit non-goals for this pivot)

- No change to AI concept extraction / merging (`extract`, `merges` stay stubs).
- No new authored content; the existing demo corpus is the working set.
- No canon-axis UI (demo profile sets `canon.enabled: false`).
- No change to the concept/source frontmatter model (news is *added*, not a
  reshape of the existing types).
- Still read-only: no logins, edits, or comments. Not a CMS.

## 7. Open design questions (to resolve in-arc, not now)

1. **Template + asset home in the repo.** Proposed: relocate the refactor's
   `templates/` to a repo-root `templates/` (the `path_loader` base) and
   consolidate assets under `assets/`. Confirm in Arc 01 slice 01.
2. **Markdown crate.** `comrak` (GFM, header ids, footnotes) vs `pulldown-cmark`
   (lighter). Leaning `comrak` for the anchor/TOC needs. Decide in Arc 01.
3. **Server render mode.** tiny_http rendering live per request (nicer dev loop)
   vs serving pre-built files vs a warm in-memory cache. Decide in Arc 03; the
   render layer is designed to support either.
4. **Static export URL model.** Templates use absolute `/…` paths (root-hosted).
   Confirm whether any deployment needs a base-path prefix; if so, thread it
   through the context. Default: root-hosted, no prefix.
5. **News graph integration.** Whether news posts become nodes in the existing
   `HaleikiGraph` (for related-post computation) or carry a lighter side-index.
   Decide in Arc 05.

## 8. Version history

### v0.1 — 2026-07-28
Initial design delta for the tiny_http/minijinja pivot. Records the four
operator decisions (both-deployment, news in scope, custom search, arc/slice
layout), the reuse/change/new breakdown against the current codebase, the
target architecture (one renderer, two sinks), and the pointer to the refactor
README as the binding context contract.
