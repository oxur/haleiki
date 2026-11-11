# Slice 01 — index build + bundle in both sinks

> Plan-of-record for the first slice of Arc 04 (Search / Pagefind).
> Open set: this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** (planned, not implemented) · 2026-07-29.

---

## Goal

Make the Pagefind index *exist* and make **both sinks host it** — the substrate
every later search behavior stands on. Build the index in-process from the
rendered corpus, emit the `/pagefind/` bundle, write it into the static export,
and serve `/pagefind/*` from the tiny_http server with correct content-types.
This slice ships **no client UI** — no `search.js` change, no facet interaction —
so that all of it is verifiable in Rust without a browser. Slice 02 adds the
browser experience.

## Context (what exists, what's a stub)

- Arc 02 left `page::render_search` rendering an **empty pre-hydration shell**
  (`query=""`, `results=[]`, `facets=[]`) — untouched by this slice.
- `search.js` is a **17-line stub** (focus + Cmd-K only; a documented "integration
  point") — untouched by this slice.
- `tools/src/search.rs` is a 4-line doc-comment stub; `main.rs` dispatches the
  `Search` subcommand to `println!("search: not yet implemented")`, and `cli.rs`
  describes it as "wraps Pagefind". This slice makes it real.
- The corpus model (`parser.rs`): `SourcePage{title,slug,category,subcategory,
  tier,keywords,tags,body,…}` and `ConceptCard{concept,slug,category,tier,
  keywords,aliases,trust_level,body,…}`. **Trust lives only on concepts**
  (`trust_level`); sources have none.
- Arc 03's asset server (`server.rs::resolve_asset` + `ASSET_ALLOWLIST` +
  `content_type_for`) is the pattern `/pagefind/*` serving extends.

## What to build

- **Add the `pagefind` crate** (`api`/`options`/`runner` — the in-process
  indexer). No `npx`/Node.
- **A record builder** (`search.rs`): from the `Corpus`, produce one Pagefind
  record per concept + source carrying:
  - **content** to index: title + keywords + aliases + plain-text body;
  - **filters** (facets): `category`, `tier`, `type` (= `page_type`), `trust`;
  - **meta** for result rendering: `title`, `url`, `excerpt` (a leading
    plain-text snippet), and enough for the template's badges (`type`,
    `category`, `tier`).
  - **url**: concept → `/{slug}/`, source → `/source/{slug}/`.
- **Emit the bundle**: build to a `/pagefind/` directory.
  - `export::export_site` runs the index build as a final step → `_site/pagefind/`.
  - `haleiki search` (replacing the stub) builds the index for a given site/output.
- **Serve `/pagefind/*`** in the tiny_http server from disk, reusing the Arc 03
  traversal guard, with content-types: `.wasm`→`application/wasm`,
  `.js`→`text/javascript`, `.json`→`application/json`, the binary index/fragment
  files (`.pf_*`)→`application/octet-stream`.

## Decisions to confirm at slice start

1. **Indexing mechanism.** Proposed: the **embedded `pagefind` crate**, feeding
   **structured records** through its service `api` (explicit content + filters +
   meta) — Rust-native, and it avoids round-tripping through annotated HTML.
   Fallbacks if the crate's record API can't express what we need: (b) annotate
   the rendered pages with `data-pagefind-body` / `data-pagefind-filter[...]` /
   `data-pagefind-meta` and index the HTML directory; (c) shell out to the
   external `pagefind` binary. Confirm the crate-with-records path, or pick a
   fallback. (A short spike to pin the crate's exact record API is the first CC
   task — name the version.)
2. **How `serve` gets the bundle.** Proposed: **build once at startup into a temp
   dir**, then serve `/pagefind/*` from disk (mirrors Arc 03's "assets from
   disk"). Alternative: hold the bundle bytes in memory in the `Arc<SiteRoutes>`
   sibling. Confirm disk-temp vs. in-memory.
3. **Trust facet for sources.** Sources have no `trust_level`. Proposed: give
   them a single well-formed default bucket (the profile's default trust, or
   `"unrated"`) so the `trust` facet is complete and never empty-valued.
   Alternative: omit sources from the `trust` filter entirely. Confirm.
4. **Bundle locations / defaults.** Proposed: export writes `_site/pagefind/`;
   `haleiki search --content-dir <c> --site <dir>` (or `-o`) writes `<dir>/pagefind/`,
   defaulting to `_site`. Confirm the flag shape.

## Scope

### In

- `pagefind` dependency; the in-process record builder; the `/pagefind/` bundle
  emitted by `export` and by a real `search` subcommand.
- Serving `/pagefind/*` from the tiny_http server (content-types + traversal
  guard).
- Rust unit/integration tests: record count == corpus size, filters + meta +
  urls correct, export emits the bundle files, server serves each bundle file
  kind `200` with the right content-type, `/pagefind/` traversal → `404`.

### Out

- **Any `search.js` / client change** — Pagefind JS API wiring, live query,
  result rendering, facet interaction, `?q=` seeding: all **slice 02**.
- **Any change to `render_search`'s shell** or `search.jinja` (beyond nothing).
- Browser / headless-browser tests (slice 02 owns the end-to-end query).
- News (Arc 05); Cobalt/Liquid removal, CI/deploy (Arc 06). The "no Pagefind"
  boundary is withdrawn (operator reversal) — Pagefind is intended.

## Verification approach

Rust-only, no browser. Unit-test the record builder: exactly `|concepts| +
|sources|` records against the demo corpus; a sampled concept record has
`type=concept`, its `category`/`tier`/`trust` filters, `url=/{slug}/`, and a
non-empty excerpt; a sampled source record has `type=source`,
`url=/source/{slug}/`, and the default trust bucket. Integration-test `export`:
after `export_site`, `_site/pagefind/` exists and contains the JS entry, a
`.wasm`, and index/fragment files, plus `pagefind-entry.json`. Server: extend the
`resolve`/live tests — `/pagefind/pagefind.js`→`200 text/javascript`, the
`.wasm`→`200 application/wasm`, a fragment→`200 application/octet-stream`, and a
`/pagefind/../../etc/passwd` traversal→`404`. `cargo clippy -p haleiki
--all-targets -- -D warnings` + `cargo fmt --check` clean on new code.

## Exit criteria

Ledger rows G-1…G-8 reach a final status. In short: the `pagefind` crate builds
an in-process index of the full corpus with the four filters + result meta and
correct URLs; `export` writes `_site/pagefind/`; `haleiki search` builds it
standalone; the server serves `/pagefind/*` with correct content-types and the
traversal guard intact; new code is clippy(`--all-targets`)/fmt-clean.

## Relationship to the arc

This is the substrate for arc rows **D-3** (both sinks host the bundle) and,
transitively, **D-4…D-6** (which slice 02 exercises in a browser). Slice 02 wires
`search.js` to the Pagefind JS API and reproduces the end-to-end query + facets +
sink-parity against a live server and a built export, then the arc closes.

## Notes

- Load the **rust-guidelines** (anti-patterns; CLI-tools + error-handling — a new
  subcommand and a new dependency). Crate lints: `clippy::all` denied, `pedantic`
  warned, `unsafe_code` forbidden, edition 2024.
- Keep the Arc 03 shared state immutable + lock-free; serving `/pagefind/*` is a
  read of on-disk files, same as assets — no new synchronization.
- Pagefind requires the bundle be **fetched over HTTP** (both sinks satisfy this:
  a static host, or tiny_http). Confirm the browser bundle path is `/pagefind/`
  (Pagefind's default) so slice 02's `search.js` can `import('/pagefind/pagefind.js')`.
- Run clippy **with `--all-targets`** (the standing lesson); concatenate any
  needle. Name the exact `pagefind` crate version pinned.
