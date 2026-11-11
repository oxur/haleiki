# Slice 02 — markdown render + TOC

> Plan-of-record for the second slice of Arc 01 (Render Core). Open set:
> this doc + [`ledger.md`](./ledger.md) + [`cc-prompt.md`](./cc-prompt.md).
> Status: **open** (planned, not implemented) · 2026-07-28.

---

## Goal

Implement the Markdown → HTML **rendering primitive** in the `haleiki` crate: a
function that takes a Markdown body string and returns the rendered HTML the
templates print as `{{ page.content | safe }}`, plus an extracted table of
contents (`page.data.toc = [{id, label}]`) whose ids match stable heading anchor
ids emitted into the HTML. Math delimiters are preserved for the design's
client-side KaTeX, and fenced code renders as plain styled blocks (no server-side
highlighting — the CSS styles plain `.prose pre code`).

This is a **pure primitive over `&str`** — it is not yet wired to the parser,
the `Corpus`, or full page rendering (that is slice 03). It is the unit that
turns a concept-card or source-page body into the two things the article
templates need: sectioned prose and a matching TOC.

## Why these constraints (grounded in the templates + assets)

- `concept.jinja` / `source.jinja` render `{{ page.content | safe }}` inside
  `.prose` and expect the section anchors (`#quick-definition`, …) the TOC links
  to. `left-nav-toc.jinja` iterates `page.data.toc` as `[{id,label}]` and links
  `#{{item.id}}`; `toc-scrollspy.js` matches each `#id` to an element with that
  `id`. So heading `id` (in HTML) and `toc[].id` must be produced together and be
  equal.
- Concept-card sections are all `## H2` (Quick Definition, Core Definition,
  Prerequisites, Key Properties, Context & Application, Examples, Relationships,
  Source Reference) — so the TOC is an H2 list.
- `assets/js/math.js` runs KaTeX `renderMathInElement` over `$`/`$$` delimiters,
  ignoring `pre`/`code`. Demo content is math-heavy and inline
  (`$a \cdot b = b \cdot a$`, `$\mathbb{Z}_n$`), which is the classic
  markdown-mangles-math hazard.
- `haleiki.css` styles `.prose code` / `.prose pre code` with the mono font and
  has **no** `.token`/`.hljs` classes → plain code, no server-side highlighting.

## Scope

### In

- Add the Markdown dependency. **Recommended: `comrak`** (CommonMark + GFM,
  AST access for clean heading-id + TOC extraction, and a code-context-correct
  math tokenizer — see *Math* below). `pulldown-cmark` is an acceptable lighter
  alternative if the event-stream approach is preferred; the ledger rows assert
  behaviour, not the crate. **No `syntect`** (no server-side highlighting).
- A new `markdown` module (`tools/src/markdown.rs`) exposing roughly
  `render_markdown(body: &str) -> RenderedContent`, with
  `RenderedContent { html: String, toc: Vec<TocEntry> }` and
  `TocEntry { id: String, label: String }` (serializing to `{id,label}` for the
  template context). `mod markdown;` wired in `main.rs` (module only — no command
  calls it yet).
- **Heading anchor ids:** a deterministic slugify — lowercase, non-alphanumerics
  → `-`, collapse/trim repeats, dedupe collisions with a numeric suffix
  (`-2`, `-3`). "Quick Definition" → `quick-definition`, "Context & Application"
  → `context-application`. Emitted as `id="…"` on the heading element in the HTML.
- **TOC extraction:** collect `## H2` headings in document order into `toc[]`,
  each `id` equal to that heading's emitted id and `label` equal to its text.
  (H2 is the section level; H3+ are not listed — the partial's `.toc-list` is
  flat.)
- **Math preservation** (the risk item — see *Math* below): real `$…$` / `$$…$$`
  math survives for KaTeX with its interior byte-intact (no emphasis conversion of
  `_`/`*`, no backslash loss), **and** `$` inside inline/fenced code is left
  untouched (not treated as math).
- **Fenced code:** `<pre><code class="language-…">…</code></pre>`, contents
  HTML-escaped and not markdown-processed; inline code → `<code>`.
- **GFM tables** enabled (Wikipedia-derived sources may contain them). Decide
  raw-HTML passthrough: default **allow** first-party inline HTML (the templates
  already treat `page.content` as `| safe`); state the choice.
- Unit tests against representative fixtures **and** at least one real demo body
  read directly from disk (`demo/concepts/abelian-group.md`, math-heavy; and one
  `demo/sources/*.md`). Reading a file inside a test is not "wiring" — it needs
  no parser change.

### Out / hand-off

- **Not wired to the parser or `Corpus`.** Note: `parser.rs` currently
  **discards the body** (`let (page, _body) = parse_frontmatter(...)` stores only
  frontmatter; `SourcePage`/`ConceptCard` carry no `body`). Retaining the body
  on those structs and calling `render_markdown` during page-context assembly is
  **slice 03's job** — name it explicitly in slice 03's scope so it is not
  assumed done. This slice reads demo files directly in tests instead.
- No `page.data` assembly (facts, aliases, see-also, provenance) and no
  end-to-end `concept.jinja` / `source.jinja` render — slice 03.
- No server-side syntax highlighting; no server-side math rendering (client-side
  KaTeX stays). No server or static export (Arcs 02–03).

## Math (named design decision)

`math.js` renders math client-side by scanning for `$`/`$$`. A naive CommonMark
pass mangles math interiors (`$a_1 * b_2$` → emphasis; `\{` → `{`), and a naive
`$`-protecting scan would wrongly grab `$` inside shell/code examples. Pick one,
both verified by the same tests (real math preserved **and** `$` in code
untouched):

- **(A) Recommended — comrak `math_dollars` tokenization + a one-line `math.js`
  update.** comrak identifies math spans correctly (respecting code context) and
  emits them as `<span data-math-style="inline|display">TeX</span>`. Update
  `math.js` to render those spans via `katex.render(el.textContent, el, {
  displayMode: el.dataset.mathStyle === 'display', throwOnError: false })`
  instead of scanning `$`. Robust and code-context-correct; touches the small,
  first-party `math.js` asset (state this — it is in-scope, not silent creep).
- **(B) Alternative — code-aware protect/restore.** Tokenize the body,
  replace math spans (outside code) with placeholders, run markdown, restore the
  original `$…$` text so the existing `math.js` auto-render works unchanged.
  Keeps `math.js` as-is but must itself be code-context-correct.

## Verification approach

Unit tests in the crate: CommonMark basics; slugify determinism + collision;
heading-id emission; TOC level/order/id↔label correspondence; math preservation
+ code-`$` safety; fenced + inline code; GFM table; a real demo body renders and
its H2 sections appear in the TOC with anchors present in the HTML; and the
rendered HTML flows through `render::build_environment` as `{{ page.content |
safe }}` intact. `cargo clippy` (deny `all`, warn `pedantic`) + `cargo fmt`
clean on new code.

## Exit criteria

Ledger rows F-1 … F-9 reach a final status. In short: `render_markdown` produces
correct sectioned HTML with stable, deduped heading ids; a matching H2 TOC; math
preserved for KaTeX with `$`-in-code left alone; plain fenced/inline code; GFM
tables; a real math-heavy demo body renders end to end through the primitive; the
output honours the `| safe` contract; and new code is clippy/fmt clean.

## Notes

- Context contract authority remains
  `workbench/Hale'iki Wiki Design Refactor/templates/README.md` (the concept
  contract: `page.content` with section anchors; `page.data.toc[]{id,label}`).
- Confirm the markdown crate (comrak vs pulldown-cmark) and the math approach
  (A vs B) with the operator at slice start if not already settled — both are
  recorded here as recommendations, not locked.
- Carry forward the arc v1.1 finding: any new `Environment` constructed here must
  reuse `render::build_environment` (auto-escape is per-`Environment`).
