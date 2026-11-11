# CC Prompt — Arc 01 / Slice 02: markdown render + TOC

You are implementing the second slice of the Haleiki tiny_http/minijinja pivot,
on branch `framework-base/tiny-http`. Work to the collaboration-framework's
ledger discipline: this slice is defined by its ledger, and every ledger row
must reach a final, evidence-backed status before you claim the slice done.

## Before you write code

1. Load the **collaboration-framework** skill (ledger discipline, peer frame,
   anti-degradation) and the **rust-guidelines** skill (`assets/ai/rust/SKILL.md`
   + guides; anti-patterns first). This crate lints hard: `clippy::all` denied,
   `pedantic` warned, `unsafe_code` forbidden, edition 2024.
2. Read, in order:
   - [`slice-doc.md`](./slice-doc.md) — goal, scope in/out, the **Math** design
     decision, exit criteria.
   - [`ledger.md`](./ledger.md) — the nine rows F-1…F-9 you are closing. If a row
     is wrong or impossible, raise an amendment request; do not quietly ignore it.
   - [`../arc-plan.md`](../arc-plan.md) — the arc, incl. the v1.1 auto-escape
     finding and the v1.2 findings (lint debt; `demo/profile.yaml` migration).
   - [`../slice01-minijinja-shell/cdc-verification.md`](../slice01-minijinja-shell/cdc-verification.md)
     — what slice 01 delivered (`render::build_environment`, the `Profile`).
   - The consumers you are feeding: `templates/pages/concept.jinja`,
     `templates/pages/source.jinja`, `templates/partials/left-nav-toc.jinja`,
     `assets/js/toc-scrollspy.js`, `assets/js/math.js`, and the `.prose` rules in
     `assets/css/haleiki.css`.
   - A real target body: `demo/concepts/abelian-group.md` (math-heavy).

## What to build

A pure Markdown → HTML rendering primitive in a new `tools/src/markdown.rs`:

- `render_markdown(body: &str) -> RenderedContent` where
  `RenderedContent { html: String, toc: Vec<TocEntry> }` and
  `TocEntry { id, label }` (serde field names `id`, `label`). `mod markdown;` in
  `main.rs` — module only, called by no command yet.
- Deterministic heading-id slugify with collision dedupe; emit `id="…"` on
  headings; extract the `## H2` TOC in document order with ids equal to the
  emitted heading ids.
- Preserve math for the client-side KaTeX in `math.js` (see the slice-doc's
  **Math** section — recommended path (A): comrak `math_dollars` + a one-line
  `math.js` update; state which path you took). Math interiors must not be
  mangled, and `$` inside code must be left alone.
- Fenced code → plain `<pre><code class="language-…">` (no `syntect`, no
  highlighting); inline code → `<code>`. Enable GFM tables. State your raw-HTML
  passthrough choice.
- Recommended crate: `comrak` (AST access makes heading-id + TOC + code-correct
  math clean). `pulldown-cmark` acceptable. Confirm the crate + math path with
  the operator at slice start if unsettled.

## Boundaries (do not cross in this slice)

- Do **not** wire into `parser.rs` / the `Corpus`. The parser discarding the body
  is real, but retaining `body` on `SourcePage`/`ConceptCard` and calling this
  primitive during page assembly is **slice 03**. Read demo files directly in
  tests instead.
- Do **not** assemble `page.data` or render `concept.jinja`/`source.jinja`
  end-to-end (slice 03); do **not** build the server or export (Arcs 02–03).
- Do **not** fix the pre-existing lint debt (arc v1.2) — out of scope; keep your
  new module clippy-clean and leave the rest.
- If you construct a minijinja `Environment` (F-8), **reuse
  `render::build_environment`** — auto-escape is per-`Environment` (arc v1.1).

## Definition of done

All of F-1…F-9 reach a final status with reproducible evidence (each `Verify` is
a command a reviewer can re-run). In the closing report, walk the ledger row by
row — status + evidence for each — no silent drops, no prose "deviations: none".
Name any uncertainty (math edge cases especially). Do **not** write
`cdc-verification.md` — that is the independent reviewer's job. Iteration budget:
five; if you hit it without convergence, stop and flag for re-scoping.
