# CC Prompt — Arc 04 / Slice 01: index build + bundle in both sinks

You are implementing the first slice of Arc 04 of the Haleiki tiny_http/minijinja
pivot, on branch `framework-base/tiny-http`. Work to the collaboration-framework's
ledger discipline: this slice is defined by its ledger, and every ledger row must
reach a final, evidence-backed status before you claim the slice done.

## Direction note (important)

The pivot's founding decision #3 was a *custom* JSON index (no Pagefind). The
operator **reversed that decision** on 2026-07-29 — the branch uses **Pagefind**
after all (see `../../design-delta.md` Addendum A and `../../project-plan.md`
v1.4). Search is therefore **client-only** and behaves identically in both sinks;
the index is Pagefind's **own `/pagefind/` bundle**, not a route-map JSON. This
slice makes that bundle exist and makes both sinks host it. **No client UI in
this slice** — no `search.js` change (that is slice 02).

## Before you write code

1. Load the **collaboration-framework** skill and the **rust-guidelines** skill
   (anti-patterns first; **CLI-tools + error-handling** guides — a new subcommand
   and a new dependency). Crate lints: `clippy::all` denied, `pedantic` warned,
   `unsafe_code` forbidden, edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, context, the **four decisions to
     confirm at slice start**, scope, verification approach.
   - [`ledger.md`](./ledger.md) — the eight rows G-1…G-8. The contract.
   - [`../arc-plan.md`](../arc-plan.md) — the arc capability + composition rows
     D-3…D-6 this arc reproduces (slice 02 exercises them in a browser).
   - `tools/src/parser.rs` — the `Corpus` / `SourcePage` / `ConceptCard` model
     you build records from (note: title is `concept` on cards, `title` on
     sources; `trust_level` is concept-only; `body` is the parsed Markdown body).
   - `tools/src/export.rs` — `export_site` (you add the index-build step) and its
     route walk / asset copy.
   - `tools/src/server.rs` — `resolve_asset`, `ASSET_ALLOWLIST`,
     `content_type_for`, `resolve` (the pattern `/pagefind/*` serving extends;
     keep the traversal guard).
   - `tools/src/cli.rs` + `main.rs` — the `Search` subcommand stub you make real,
     and the `export`/`serve` wiring pattern.
   - `tools/src/search.rs` — the doc-comment stub you flesh out.

## Confirm the four decisions first (slice-doc §"Decisions to confirm")

Then, as the **first coding task, spike the `pagefind` crate**: pin an exact
version and pin down its record API — can you add records with explicit
**content + filters + meta** through the service `api` (the proposed path), or
must you index annotated HTML? Report what you found before building on it. Name
the version in the closing report.

## What to build

- Add `pagefind` to `[dependencies]`. Build the index **in-process** (no `npx`,
  no `Command::new("pagefind")`).
- In `search.rs`: a record builder from `Corpus` → one record per concept +
  source, each with content (title + keywords + aliases + plain-text body),
  filters (`category`/`tier`/`type`/`trust`), meta (`title`/`url`/`excerpt`), and
  url (`/{slug}/` concept, `/source/{slug}/` source). Sources get the agreed
  default `trust` bucket (decision #3).
- Emit the `/pagefind/` bundle: `export_site` builds it into `_site/pagefind/` as
  a final step; a real `haleiki search` subcommand builds it into an output dir.
- Serve `/pagefind/*` from disk in the tiny_http server, reusing the traversal
  guard, with the content-types in G-6.

## Boundaries (do not cross in this slice)

- **No `search.js` / client change; no `render_search` / `search.jinja` change.**
  Query, result rendering, facet interaction, `?q=` seeding are **slice 02**.
- Keep the Arc 03 shared state immutable + lock-free; `/pagefind/*` is a disk read
  like assets — no new `Mutex`/`RwLock`.
- Do not change `resolve`'s existing routing semantics or the export's HTML
  output. Do not remove Cobalt/Liquid (Arc 06). Do not fix unrelated lint debt.

## Definition of done

All of G-1…G-8 reach a final status with reproducible evidence — **Rust-only, no
browser** (slice 02 owns the headless-browser end-to-end). Keep verification in
unit/integration tests: record count/filters/meta/urls, the exported bundle
files, and the served bundle content-types + traversal 404. Run clippy **with
`--all-targets`**. In the closing report, walk the ledger row by row — status +
evidence, no silent drops, no "deviations: none" prose; record which of the four
decisions were taken and the pinned `pagefind` version; note the bubble-up for
slice 02 (the bundle is ready to wire to the JS API). Name any uncertainty (the
crate's record API especially). Do **not** write `cdc-verification.md`. Iteration
budget: five.
