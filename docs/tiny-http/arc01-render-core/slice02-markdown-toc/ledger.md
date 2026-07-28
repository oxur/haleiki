# Slice 02 — markdown render + TOC · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `render_markdown(&str)` renders CommonMark to HTML: paragraphs, unordered/ordered lists, bold/emphasis, and `[text](/url/)` links all produce the expected tags | `cargo test` unit test on a mixed-markdown fixture | serious | slice-doc | open | | |
| F-2 | Heading ids are deterministic slugs: lowercase, non-alphanumerics→`-`, collapsed/trimmed; `"Quick Definition"`→`quick-definition`, `"Context & Application"`→`context-application`; duplicate headings dedupe with a numeric suffix (`-2`, `-3`) | unit test asserting emitted `id="…"` for a fixture incl. a duplicate heading | serious | slice-doc | open | | |
| F-3 | TOC is extracted as `[{id,label}]` for `## H2` headings in document order; each `toc[i].id` equals the emitted `id` of the corresponding heading in the HTML, and `label` equals the heading text | unit test cross-checking `toc` ids against `id="…"` occurrences in `html` | serious | slice-doc | open | | |
| F-4 | Math preserved for KaTeX: real `$…$` and `$$…$$` survive with interior TeX byte-intact (no `_`/`*`→emphasis, no backslash loss) **and** `$` inside inline/fenced code is left untouched (not math) | unit test: `$a_1 * b_2$` + a `$$\sum_{i=0}^{n} x_i$$` block render intact; `` `echo $HOME` `` renders `$HOME` literally in a `<code>` | correctness | slice-doc | open | | |
| F-5 | Fenced code → `<pre><code class="language-…">…</code></pre>` with contents HTML-escaped and not markdown-processed; inline `` `code` `` → `<code>` | unit test on a rust fenced block containing `*` and `<` | serious | slice-doc | open | | |
| F-6 | GFM pipe tables render to `<table>…</table>` | unit test on a pipe-table fixture | polish | slice-doc | open | | |
| F-7 | A real demo concept body (`demo/concepts/abelian-group.md`, math-heavy) renders without error; its 8 `##` sections appear in `toc` and each `toc[].id` has a matching `id="…"` anchor in the HTML | unit test reading the file, splitting frontmatter, rendering the body | serious | slice-doc | open | | |
| F-8 | The rendered `RenderedContent.html`, passed as `page.content` into a minimal template rendered via `render::build_environment` as `{{ page.content \| safe }}`, appears unescaped and structurally intact (headings, anchors, math, links preserved) | render test reusing `build_environment` | correctness | slice-doc | open | | |
| F-9 | New code is `cargo clippy` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | `cargo clippy -p haleiki` on the new module + `cargo fmt --check` (scoped to new code; the pre-existing debt from arc v1.2 is out of scope) | correctness | slice-doc | open | | |

## What Worked

_(At slice close. Patterns that made the slice close cleanly.)_

## Closure

_(At slice close.)_ Closed at commit `<SHA>` on `<date>`. Verified by: `<CDC/session>`.
Rows: 9. Done: `<n>`. Deferred: `<n>`. No-op: `<n>`.
