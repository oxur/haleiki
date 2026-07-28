# Slice 02 — markdown render + TOC · Ledger

> Grep-verifiable acceptance criteria (the steps). Per the collaboration-
> framework's `LEDGER-DISCIPLINE.md` Section A. Every row reaches a final status
> (`done` / `deferred` / `no-op`) before the slice advances; `done` requires
> reproduced evidence. CC fills Evidence at the commit each row is met; CDC
> verifies each against the artifacts. Status: **open** · 2026-07-28.

## Ledger

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
|----|-----------|--------|--------------|--------|--------|----------|-------|
| F-1 | `render_markdown(&str)` renders CommonMark to HTML: paragraphs, unordered/ordered lists, bold/emphasis, and `[text](/url/)` links all produce the expected tags | `cargo test` unit test on a mixed-markdown fixture | serious | slice-doc | done | attested — `cd tools && cargo test markdown::tests::test_render_markdown_commonmark_basics` → `... ok`. Test at `tools/src/markdown.rs::tests::test_render_markdown_commonmark_basics`. | Asserts `<strong>`/`<em>`, `<ul>`/`<li>`, `<ol>`/`<li>`, and `<a href="/docs/">` all present — comrak's default CommonMark rendering, no options beyond `table`/`math_dollars`/`unsafe`. |
| F-2 | Heading ids are deterministic slugs: lowercase, non-alphanumerics→`-`, collapsed/trimmed; `"Quick Definition"`→`quick-definition`, `"Context & Application"`→`context-application`; duplicate headings dedupe with a numeric suffix (`-2`, `-3`) | unit test asserting emitted `id="…"` for a fixture incl. a duplicate heading | serious | slice-doc | done | attested — `cd tools && cargo test markdown::tests::test_render_markdown_heading_ids_are_deterministic_and_deduped` → `... ok`. | **Deliberately does not use comrak's built-in `Anchorizer`**: it dedupes `foo`/`foo-1`/`foo-2` (starts at `-1`, not `-2`) and *drops* punctuation runs rather than collapsing them to `-` (`"Context & Application"` would become `context--application` — a double dash — via `Anchorizer`, not `context-application`). A custom `slugify()` + `TocCollector::dedupe()` (in `tools/src/markdown.rs`) implement this contract's exact convention instead; documented as a design decision in the module's doc comment. Verified against three literal duplicate `## Background` headings → `background`, `background-2`, `background-3`. |
| F-3 | TOC is extracted as `[{id,label}]` for `## H2` headings in document order; each `toc[i].id` equals the emitted `id` of the corresponding heading in the HTML, and `label` equals the heading text | unit test cross-checking `toc` ids against `id="…"` occurrences in `html` | serious | slice-doc | done | attested — `cd tools && cargo test markdown::tests::test_render_markdown_toc_matches_document_order_and_anchors` → `... ok`. | Fixture has an `h1`, two `h2`s, and an `h3` nested under the first `h2`; asserts `toc.len() == 2` (h1/h3 excluded), order and `id`/`label` correspondence, and that the h3 still got its own (non-TOC'd) anchor — ids are assigned to every heading level, only `h2` is collected into `toc`, per slice-doc. |
| F-4 | Math preserved for KaTeX: real `$…$` and `$$…$$` survive with interior TeX byte-intact (no `_`/`*`→emphasis, no backslash loss) **and** `$` inside inline/fenced code is left untouched (not math) | unit test: `$a_1 * b_2$` + a `$$\sum_{i=0}^{n} x_i$$` block render intact; `` `echo $HOME` `` renders `$HOME` literally in a `<code>` | correctness | slice-doc | done | attested — `cd tools && cargo test markdown::tests::test_render_markdown_math_preserved_and_code_dollar_untouched` → `... ok`. | Design option **(A)**: comrak's `extension.math_dollars` (native, not feature-gated) tokenizes math as a distinct AST node before CommonMark's inline parser runs, so `a_1 * b_2` never becomes `a<em>1 </em> b2</em>`-style emphasis and `\sum` doesn't lose its backslash; emitted as `<span data-math-style="inline\|display">…</span>`. `` `echo $HOME` `` confirmed to render as literal `<code>echo $HOME</code>` — code-context correctness is comrak's, not hand-rolled. `assets/js/math.js` updated in the same commit (see Notes on F-8) to read these spans via `el.textContent` + `katex.render`, replacing the old `$`-delimiter scan — stated in-scope per the slice-doc's Math section, not silent creep. |
| F-5 | Fenced code → `<pre><code class="language-…">…</code></pre>` with contents HTML-escaped and not markdown-processed; inline `` `code` `` → `<code>` | unit test on a rust fenced block containing `*` and `<` | serious | slice-doc | done | attested — `cd tools && cargo test markdown::tests::test_render_markdown_fenced_and_inline_code_plain_and_escaped` → `... ok`. | `codefence_syntax_highlighter` left unset (no `syntect`; the crate is added with `default-features = false`, so `syntect`/`cli`/`bon` aren't even compiled in — confirmed via `cargo tree`/build output showing no `syntect`/`onig` crates). ``` ```rust\nlet x = a < b && *ptr;\n``` ``` → `<pre><code class="language-rust">let x = a &lt; b &amp;&amp; *ptr;\n</code></pre>` (escaped, `*` not turned into emphasis). Inline `` `*not emphasis*` `` → `<code>*not emphasis*</code>`. |
| F-6 | GFM pipe tables render to `<table>…</table>` | unit test on a pipe-table fixture | polish | slice-doc | done | attested — `cd tools && cargo test markdown::tests::test_render_markdown_gfm_table` → `... ok`. | `options.extension.table = true`; asserts `<table>`, `<th>Name</th>`, `<td>a</td>`, `</table>` all present. |
| F-7 | A real demo concept body (`demo/concepts/abelian-group.md`, math-heavy) renders without error; its 8 `##` sections appear in `toc` and each `toc[].id` has a matching `id="…"` anchor in the HTML | unit test reading the file, splitting frontmatter, rendering the body | serious | slice-doc | done, **with a disclosed factual correction to the criterion** | attested — `cd tools && cargo test markdown::tests::test_render_markdown_real_demo_concept_body markdown::tests::test_render_markdown_real_demo_source_body` → both `... ok`. | **The criterion's "8" is wrong; the file has 9** `## ` sections (`grep -c '^## ' demo/concepts/abelian-group.md` → `9`: Quick Definition, Core Definition, Prerequisites, Key Properties, Context & Application, Examples, Relationships, Source Reference, Verification Notes). The test asserts the *actual* count (computed from the file, not hardcoded) rather than silently matching a stale "8" — flagged here per ledger discipline rather than quietly adjusting the number without comment. Substance of the row (every `toc[].id` has a matching HTML anchor; math and cross-reference links survive) holds regardless of the count. Also renders a second, non-math real body (`demo/sources/vimalamitra.md`: image, bold, link-with-title, one `h2`) as additional real-world coverage beyond the ledger's literal ask. |
| F-8 | The rendered `RenderedContent.html`, passed as `page.content` into a minimal template rendered via `render::build_environment` as `{{ page.content \| safe }}`, appears unescaped and structurally intact (headings, anchors, math, links preserved) | render test reusing `build_environment` | correctness | slice-doc | done | attested — `cd tools && cargo test markdown::tests::test_render_markdown_html_passes_through_safe_context_intact` → `... ok`. | Reuses `render::build_environment` per arc v1.1 (not a second ad hoc `Environment`); renders a body containing a heading, inline math, and a raw `<sub>` tag; asserts the template output equals the raw HTML byte-for-byte wrapped in the test template's `<div>`, and that the heading anchor, the math span, and the raw tag all survive unescaped — confirms `render.r#unsafe = true` (raw HTML passthrough) and the `\|safe` discipline compose correctly. |
| F-9 | New code is `cargo clippy` clean (deny `all`, warn `pedantic`) and `cargo fmt`-clean | `cargo clippy -p haleiki` on the new module + `cargo fmt --check` (scoped to new code; the pre-existing debt from arc v1.2 is out of scope) | correctness | slice-doc | done | attested — `cargo fmt --all -- --check` → exit 0 (workspace-wide clean). `cargo clippy --all-features --workspace -- -D warnings 2>&1 \| grep -c markdown.rs` → `0`. | One real pedantic finding during development (`clippy::doc_markdown` wanting `` `CommonMark` `` backticked in a doc comment) — fixed, not suppressed. Remaining workspace clippy errors are the same 6 pre-existing ones from arc v1.2 (`generator.rs:541`, `main.rs:57`, 4 in `demo/*.rs` under `--all-features`) — none in `markdown.rs`, none newly introduced. **CDC:** an independent `clippy -p haleiki --all-targets -- -D warnings` surfaced one pedantic `uninlined_format_args` in the F-3/F-7 test asserts (invisible to a run without `--all-targets`); inlined and re-confirmed clippy-clean + rustfmt-clean + 9 tests green. |

## What Worked

- Reading comrak's actual source (`options.rs`, `html.rs`, `adapters.rs`,
  `tests/math.rs`) before writing any code — rather than guessing at the API
  from memory or docs.rs prose — meant every byte-exact HTML assertion in the
  test suite (fenced-code shape, math span shape, table shape) passed on the
  *first* `cargo test` run after fixing one unrelated `Send + Sync` compile
  error. No iteration was spent on "why doesn't the output match."
- Checking `default-features` before adding the dependency caught that
  `comrak`'s defaults silently pull in `syntect` (server-side highlighting,
  explicitly out of scope) plus a CLI and builder crate nobody asked for.
  `default-features = false` keeps the dependency tree honest about what's
  actually used — confirmed via a clean `cargo check` with no `syntect`/`onig`
  in the build output.
- Discovering that comrak's built-in `Anchorizer`/`header_id_prefix` path
  *doesn't* match this contract's dedupe convention (`-1` vs `-2`) or its
  collapse-punctuation-runs slugify rule — before writing any tests, by
  reading `anchorizer.rs` — avoided shipping heading ids that would have
  looked right in casual testing but drifted from spec on the second
  duplicate heading or the first `&`/`—`-containing title.
- The `HeadingAdapter` trait's `enter`/`exit` hooks turned out to be exactly
  the right shape for a single-pass id-assignment + TOC-collection primitive
  — no second AST walk needed.

## Closure

**Independently verified (CDC): CLOSED** 2026-07-28. All 9 rows reproduced in a
clean cloud build (`comrak 0.54` resolved fresh, real demo fixtures) — 9 markdown
tests pass, `rustfmt` clean. F-9 was **returned once**: independent clippy with
`--all-targets -D warnings` found a pedantic `uninlined_format_args` in the new
test asserts (a run without `--all-targets` does not lint the test module, which
is why it was missed); the two asserts were inlined (operator-directed quick-fix,
applied by the verifier — independence caveat disclosed in `cdc-verification.md`)
and re-confirmed clippy-clean + fmt-clean + tests-green. F-7's "8"→"9" section
count is a stale-criterion correction, not a behavior deviation. Per-row verdicts
and the two resolved bubble-up findings are in
[`cdc-verification.md`](./cdc-verification.md). Verified by: independent Claude
(Cowork) context. Rows: 9. Done: 9 (reproduced). Deferred: 0. No-op: 0.
