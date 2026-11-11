# Slice 02 — markdown render + TOC · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per the collaboration-framework's `LEDGER-DISCIPLINE.md`
> Section A (CDC protocol) and the slice bubble-up check in
> `PROJECT-MANAGEMENT.md` Part IV. Verdict: **CLOSED** (after one returned
> finding was fixed and re-verified) · 2026-07-28.

## Verifier and method

Verified by an independent Claude context (Cowork), not the implementing
context. Reproduction was done in a clean cloud build: the slice's changed
sources (`tools/src/markdown.rs`, the `mod markdown;` wiring, the `comrak`
manifest line) and the two demo fixtures the tests read
(`demo/concepts/abelian-group.md`, `demo/sources/vimalamitra.md`) were staged
into a fresh container, `comrak 0.54` was resolved from crates.io, and the
suite was run with `cargo 1.95.0`. Grep/inspection and boundary checks were run
against the working tree on the device.

## Row-count check

Ledger opened with **9** rows (F-1…F-9); the closing report walks **9**; this
verification addresses **9**. No silent drops.

## Per-row verification

| ID | Closer status | CDC verdict | Strength | How reproduced |
|----|---------------|-------------|----------|----------------|
| F-1 | done | **confirmed** | reproduced | `cargo test --bin haleiki markdown` → `test_render_markdown_commonmark_basics ... ok`. |
| F-2 | done | **confirmed** | reproduced | `…heading_ids_are_deterministic_and_deduped ... ok`. `quick-definition`, `context-application`, and `background`/`background-2`/`background-3` dedupe verified. |
| F-3 | done | **confirmed** | reproduced | `…toc_matches_document_order_and_anchors ... ok`. H2-only, document order, each `toc[i].id` present as an `id="…"` in the HTML. |
| F-4 | done | **confirmed** | reproduced | `…math_preserved_and_code_dollar_untouched ... ok`. `$a_1 * b_2$` and `$$\sum…$$` survive byte-intact as `data-math-style` spans; `` `echo $HOME` `` stays literal in `<code>`. |
| F-5 | done | **confirmed** | reproduced | `…fenced_and_inline_code_plain_and_escaped ... ok`. `<pre><code class="language-rust">` escaped, not md-processed. |
| F-6 | done | **confirmed** | reproduced | `…gfm_table ... ok`. |
| F-7 | done (count note) | **confirmed** | reproduced | `…real_demo_concept_body` + `…real_demo_source_body` both `ok`. Section count is **9** (not the ledger's stale "8" — see below); CC's test computes it from the file rather than hardcoding, which is the correct handling. |
| F-8 | done | **confirmed** | reproduced | `…html_passes_through_safe_context_intact ... ok`. Rendered HTML flows through `render::build_environment` as `{{ page.content \| safe }}` unescaped and intact. |
| F-9 | done (claimed clippy-clean) | **RETURNED, fixed, re-confirmed** | reproduced | See below — this row did **not** reproduce as claimed. |

Full suite reproduced: **9 markdown tests pass** (`test result: ok. 9 passed`).
`rustfmt --check` on `markdown.rs` is clean.

## F-9 — the returned finding

CC's closing report claimed `markdown.rs` is clippy-clean. It was **not**.
Independent `cargo clippy -p haleiki --all-targets -- -D warnings` surfaced one
finding in the new code:

- `clippy::uninlined_format_args` at `markdown.rs:378` (the F-7 test's TOC-anchor
  assertion): `"toc entry {:?} …", entry` should inline to `"toc entry
  {entry:?} …"`. `uninlined_format_args` is **pedantic** (which this crate sets
  to `warn`), so it only appears with `--all-targets` (test modules are not
  linted otherwise) and becomes a hard error under `make lint`'s `-D warnings`.
  This is almost certainly why CC's run missed it — a clippy invocation without
  `--all-targets` does not lint the test module the finding lives in.

The same idiom appeared in a second assert (F-3 test). **Fix applied** (both
asserts inlined) by the CDC at the operator's explicit direction to apply
quick-fixes directly. **Independence caveat (disclosed):** because the verifier
applied this fix, its re-verification is not fully arm's-length; mitigations are
that the fix is clippy-dictated and mechanical, and it was re-checked in a clean
build: `cargo clippy -p haleiki --all-targets -- -D warnings` now reports **0**
`markdown.rs` findings, `rustfmt --check` is clean, and all 9 markdown tests
still pass.

## Boundary check

`git status` (working tree): the slice changed only `tools/src/markdown.rs`
(new), `tools/src/main.rs` (+`mod markdown;`), `tools/Cargo.toml` (+comrak),
`assets/js/math.js`, and the slice docs. **`parser.rs`, `graph.rs`,
`validator.rs` untouched.** `comrak` is pulled with `default-features = false`
(no `syntect`/CLI/`bon`) — matches the "lightweight" intent and the no-server-
side-highlighting decision.

## Disclosed items reviewed

- **Section count 8 → 9.** `demo/concepts/abelian-group.md` has 9 `##` sections
  (a "Verification Notes" section was added since the slice-doc was written). The
  slice-doc/ledger's "8" was stale; CC's test computes the count from the file,
  which is correct. Ledger-quality note, not a defect.
- **Math path A.** Confirmed correctly implemented: `comrak`'s `math_dollars`
  emits `data-math-style` spans (code-context-correct), and `math.js` was
  rewired to `katex.render(el.textContent, …)`. `katex.render` is provided by
  the still-loaded `katex.min.js`, so math renders.

## Bubble-up findings — resolved now (operator directed)

Both arc-level findings this slice surfaced were fixed immediately (rather than
deferred), at the operator's direction, and verified:

1. **`source.jinja` did not load KaTeX** → source-page math would render raw.
   **Fixed:** added a `head_extra` block loading `katex.min.css`, `katex.min.js`,
   and `math.js`. Verified: `pages/source.jinja` parses via
   `build_environment().get_template(...)`. *(Loads unconditionally on all source
   pages; a conditional `page.data.has_math` load is a possible slice-03
   optimization — noted, not required.)*
2. **`concept.jinja` still loaded the now-dead `auto-render.min.js`** → wasted
   fetch. **Fixed:** removed the include (`math.js` no longer uses
   `renderMathInElement`). Verified: `pages/concept.jinja` parses.

Both are template edits, outside slice 02's original code boundary; they are
recorded as the disposition of the arc-plan v1.3 findings, not as slice-02
ledger rows.

## Verdict

**Slice 02 is CLOSED.** 9 rows: 9 `done` (F-1…F-8 reproduced on first pass; F-9
returned for a pedantic clippy finding in new test code, fixed, and
re-confirmed clippy-clean + fmt-clean + tests-green). Arc ledger row **A-2** is
satisfied (evidence: this file). The two bubble-up findings are resolved. Arc 01
now has one slice left — slice 03 (wiki-article context) — before the arc's
composition rows (A-4…A-7) can close.

## What worked (Safety-II)

- The clippy finding was caught only because CDC re-ran the lint with
  `--all-targets` in a clean environment. It confirms the slice-01 lesson: an
  attestation of "clippy-clean" needs an independent re-run with the same flags
  CI uses, not trust.
- CC's decision to compute the H2 count from the file (F-7) rather than hardcode
  the ledger's number turned a stale spec value into a self-correcting test —
  the right instinct.
