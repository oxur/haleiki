# Slice 02 — markdown render + TOC · Closing Report

> Per-row walk against [`ledger.md`](./ledger.md), per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A (CC protocol, step 4). Status:
> **proposed-done** — all 9 rows `done` at `attested` strength; independent
> (CDC-equivalent) verification has not yet run. Not yet committed. 2026-07-28.

## Row-by-row walk

**F-1 — CommonMark basics.** `done`.
`cargo test markdown::tests::test_render_markdown_commonmark_basics` passes.
Bold/emphasis (`<strong>`/`<em>`), unordered/ordered lists, and
`[text](/url/)` links all produce the expected tags — comrak's default
CommonMark rendering with no extensions beyond `table`/`math_dollars`/
`r#unsafe`.

**F-2 — Deterministic, deduped heading-id slugs.** `done`.
`cargo test markdown::tests::test_render_markdown_heading_ids_are_deterministic_and_deduped`
passes. This row is where the slice's one real design decision lives:
comrak ships a built-in `Anchorizer`, but it dedupes `foo`/`foo-1`/`foo-2`
(starts its suffix at `-1`) and *drops* punctuation runs rather than
collapsing them — `"Context & Application"` would come out `context--application`
(a double dash from the space-ampersand-space), not `context-application`.
Neither matches this slice's contract. A custom `slugify()` (Unicode-aware,
collapses any run of non-alphanumerics to one `-`) plus a small dedupe counter
in `TocCollector` (first occurrence unsuffixed, then `-2`, `-3`, …) implement
the actual spec instead — documented as a design decision in
`tools/src/markdown.rs`'s module doc comment, not left implicit. Verified
against three literal duplicate `## Background` headings.

**F-3 — TOC extraction, order, and id/anchor correspondence.** `done`.
`cargo test markdown::tests::test_render_markdown_toc_matches_document_order_and_anchors`
passes. A fixture with an `h1`, two `h2`s, and an `h3` nested under the first
`h2` confirms: `toc.len() == 2` (only `h2` collected), document order
preserved, each `toc[i].id`/`label` correct, and — separately — that the `h3`
still received its own anchor (ids are assigned to every heading level; only
`h2` feeds the TOC, per the slice-doc).

**F-4 — Math preservation + code-context correctness.** `done`. Design
option **(A)** from the slice-doc, confirmed correct by reading comrak's own
`tests/math.rs` before writing any code:
`cargo test markdown::tests::test_render_markdown_math_preserved_and_code_dollar_untouched`
passes. `extension.math_dollars` tokenizes `$…$`/`$$…$$` as a distinct AST
node *before* CommonMark's inline parser runs, so `a_1 * b_2` never becomes
emphasis-mangled and `\sum_{i=0}^{n}` doesn't lose its backslash; it's emitted
as `<span data-math-style="inline|display">…</span>` with the TeX
HTML-escaped. `` `echo $HOME` `` confirmed to stay literal inside `<code>` —
comrak's own code-context handling, not a hand-rolled scan that could miss an
edge case. `assets/js/math.js` was updated in the same commit (not a separate,
silent change) to read these spans via `el.textContent` + `katex.render(...)`
instead of scanning the rendered page for `$` delimiters — the slice-doc
names this asset as in-scope explicitly, and the update is one function body,
not a redesign.

**F-5 — Plain fenced/inline code, no server-side highlighting.** `done`.
`cargo test markdown::tests::test_render_markdown_fenced_and_inline_code_plain_and_escaped`
passes. `comrak` is added with `default-features = false` — its defaults
otherwise pull in `syntect` (server-side highlighting), a CLI, and a builder
crate, none of which this primitive uses; confirmed via a clean `cargo check`
build log with no `syntect`/`onig` compiled. A rust fenced block containing
`<` and `&&` renders as
`<pre><code class="language-rust">let x = a &lt; b &amp;&amp; *ptr;\n</code></pre>`
(escaped, not markdown-reprocessed — the `*` stays literal, no emphasis).
Inline `` `*not emphasis*` `` → `<code>*not emphasis*</code>`.

**F-6 — GFM pipe tables.** `done`.
`cargo test markdown::tests::test_render_markdown_gfm_table` passes.
`options.extension.table = true`; a two-column pipe table renders `<table>`,
`<th>`, `<td>`, `</table>`.

**F-7 — Real demo concept body, with a disclosed correction to the ledger's
own criterion.** `done`.
`cargo test markdown::tests::test_render_markdown_real_demo_concept_body`
and `…test_render_markdown_real_demo_source_body` both pass.

- *Correction, not a deviation:* the ledger criterion states
  `demo/concepts/abelian-group.md` has "8" `##` sections. It has **9**
  (`grep -c '^## ' demo/concepts/abelian-group.md` → `9`: Quick Definition,
  Core Definition, Prerequisites, Key Properties, Context & Application,
  Examples, Relationships, Source Reference, Verification Notes). The test
  computes the expected count from the file itself (`body.lines().filter(...)`)
  rather than hardcoding either `8` or `9`, so it's correct regardless of
  which number is "right," and asserts every `toc[].id` has a matching HTML
  anchor, that at least one inline math span survives (math-heavy body,
  per the slice-doc), and that a cross-reference link survives. Flagging
  the "8" vs "9" discrepancy here rather than quietly matching whichever
  number made the test pass — per ledger discipline, criteria that turn out
  wrong get raised, not silently worked around.
- *Beyond the letter of the row:* also renders a second real body,
  `demo/sources/vimalamitra.md` (non-math: an image, bold text, a titled
  link, one `h2`, Tibetan Unicode prose) — the ledger only names the concept
  fixture by name, but the slice-doc's Scope section asks for "one
  `demo/sources/*.md`" too, and this is real coverage of the non-math,
  Wikipedia-derived path the math-heavy concept fixture doesn't exercise.

**F-8 — `RenderedContent.html` flows through `render::build_environment`
unescaped and intact.** `done`.
`cargo test markdown::tests::test_render_markdown_html_passes_through_safe_context_intact`
passes. Reuses `render::build_environment` (per arc-plan v1.1's finding —
auto-escape is per-`Environment`; no second `Environment` was constructed
ad hoc). Renders a body with a heading, inline math, and a raw `<sub>` tag as
`{{ page.content | safe }}`; asserts the template's output is byte-identical
to the raw HTML (wrapped in the test template's `<div>`), confirming both
`render.r#unsafe = true` (raw HTML passthrough) and the `| safe` filter
compose correctly — nothing gets double-escaped or silently stripped.

**F-9 — New code is clippy-clean and fmt-clean.** `done`, scoped to new
code per the criterion's own wording (same scoping as slice 01's F-8).
`cargo fmt --all -- --check` is clean workspace-wide (exit 0).
`cargo clippy --all-features --workspace -- -D warnings` reports the same
**6 pre-existing errors** as slice 01 (`generator.rs:541`, `main.rs:57`, and
4 in `demo/{convert,frontmatter,media,rewrite}.rs`) — zero of which touch
`tools/src/markdown.rs` (confirmed by filtering clippy's output for
`markdown.rs`: 0 hits). One real finding surfaced during development —
`clippy::doc_markdown` wanting `` `CommonMark` `` backticked in a module doc
comment — and was fixed, not suppressed with an `#[allow]`.

## Silent-drop check

Ledger opened with 9 rows (F-1…F-9); this report addresses 9. No missing rows.

## Bubble-up to arc-plan.md

Nothing new to add beyond what arc-plan.md v1.1/v1.2 already carry forward
(auto-escape is per-`Environment`; pre-existing lint debt; `demo/profile.yaml`
migration). This slice's own finding — the "8" vs "9" section-count
discrepancy in its own ledger — is local to this slice's ledger.md and
doesn't propagate upward; it doesn't affect A-4/A-5 (arc-scale rendering of a
real concept/source page), since those rows don't assert a section count.

## What was NOT done (explicitly, per the "no silent drops" rule)

- Not wired to `parser.rs` or `Corpus`: `SourcePage`/`ConceptCard` still carry
  no `body` field (the parser discards it). Retaining the body and calling
  `render_markdown` during page-context assembly is slice 03's job, named
  explicitly here so it isn't assumed done.
- No `page.data` assembly (facts, aliases, category/tier/type, disambiguator,
  see-also, provenance) and no end-to-end `concept.jinja`/`source.jinja`
  render — slice 03.
- No server-side syntax highlighting and no server-side math rendering
  (client-side KaTeX, now driven by `data-math-style` spans instead of a
  `$`-delimiter scan, stays the design).
- No server or static export (Arcs 02–03).
- The pre-existing clippy debt named in F-9 (and originally in slice 01's
  F-8/arc-plan v1.2) was not fixed — still out of scope for this slice.
- `templates/pages/concept.jinja`'s and `source.jinja`'s `<script>` tags for
  `katex.min.js`/`auto-render.min.js`/`math.js` were left untouched even
  though `auto-render.min.js` (the delimiter-scanning KaTeX extension) is no
  longer used by the new `math.js`. Removing that now-redundant script tag is
  a template change; this slice's boundary is "touches the small, first-party
  `math.js` asset," not template edits. Flagging as a small, low-risk cleanup
  for whichever slice next touches these templates (likely slice 03).
- Nothing has been committed to git. All changes are in the working tree.

## Independent verification status

Not yet performed. Per `LEDGER-DISCIPLINE.md`, this closing report is CC's
`attested` claim; a CDC-equivalent pass (fresh context or operator, re-running
each row's Verify command against the actual artifacts) is required before any
row's evidence strengthens to `reproduced` and before this slice can be
considered closed rather than proposed-done. Per the cc-prompt's explicit
instruction, this report does **not** include `cdc-verification.md` — that
file is the independent reviewer's to write.
