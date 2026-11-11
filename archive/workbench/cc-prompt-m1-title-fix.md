# M1 Follow-Up: Concept-Card Title Extraction Fix

Small follow-up to the M1 implementation. One real bug surfaced during review: concept cards end up with an empty top-level `title:` in the Cobalt-injected frontmatter, because `inject_frontmatter` looks for a `title:` line and concept cards have `concept:` instead.

This currently doesn't break visible output — `concept.liquid` renders `{{ page.data.concept }}` directly — but `page.title` being empty across all 186 concept cards will bite in M4+ wherever we use `page.title` generically (breadcrumbs, see-also tiles, search index, fallbacks). Fixing now is cheap and prevents weirdness later.

---

## The bug

In `tools/src/demo/build.rs`, `inject_frontmatter` extracts the title via:

```rust
let title = fm_content
    .lines()
    .find(|l| l.starts_with("title:"))
    .map(|l| l.trim_start_matches("title:").trim().to_string())
    .unwrap_or_default();
```

Concept cards don't have a `title:` field. They have `concept:`, and optionally `disambiguator:`. Result: every concept card's emitted frontmatter has `title:` (empty) followed by the `data:` block.

You can verify by inspecting `demo/_build/concepts/ownership-rust.md`:

```yaml
---
layout: concept.liquid
permalink: "/ownership-rust/"
title:           ← empty
data:
  concept: "Ownership"
  disambiguator: "Rust"
  ...
```

The rendered `_site/ownership-rust/index.html` has `<title> — Haleiki Demo</title>` because of the same path going through `default.liquid` for the `<head>` (concept.liquid sources its own `<head>`, but `page.title` is still empty when referenced).

---

## The fix

When the layout is `concept.liquid`, fall back to extracting the `concept:` field (with the disambiguator appended in parens if present). Sketch:

```rust
let title = fm_content
    .lines()
    .find(|l| l.starts_with("title:"))
    .map(|l| l.trim_start_matches("title:").trim().trim_matches('"').to_string())
    .unwrap_or_default();

let title = if title.is_empty() && layout == "concept.liquid" {
    let concept = fm_content
        .lines()
        .find(|l| l.starts_with("concept:"))
        .map(|l| l.trim_start_matches("concept:").trim().trim_matches('"').to_string())
        .unwrap_or_default();
    let disambiguator = fm_content
        .lines()
        .find(|l| l.starts_with("disambiguator:"))
        .map(|l| l.trim_start_matches("disambiguator:").trim().trim_matches('"').to_string())
        .filter(|d| !d.is_empty());
    match disambiguator {
        Some(d) => format!("{concept} ({d})"),
        None => concept,
    }
} else {
    title
};
```

Note the added `.trim_matches('"')` calls — concept-card YAML frontmatter quotes string values (`concept: "Ownership"`), and we want to strip the quotes when promoting to a top-level YAML field.

Apply consistently for `process_sources()` and `process_concepts()` paths via the shared `inject_frontmatter` function — only the `layout == "concept.liquid"` branch triggers the fallback.

---

## Verification

After the fix:

1. **Rebuild**:
   ```
   cargo build --release --features demo --manifest-path tools/Cargo.toml
   ./tools/target/release/haleiki demo build
   ```

2. **Spot-check the injected frontmatter** for three concept cards:
   - `demo/_build/concepts/ownership-rust.md` — top-level `title:` should be `Ownership (Rust)` (concept + disambiguator)
   - `demo/_build/concepts/nyingma.md` — top-level `title:` should be `Nyingma` (concept, no disambiguator since the multi-source merge case doesn't use one)
   - `demo/_build/concepts/uncertainty-principle.md` — top-level `title:` should be `Uncertainty Principle` (or whatever the concept field exactly says)

3. **Spot-check the rendered HTML** for the same three:
   - `demo/_site/ownership-rust/index.html` should have `<title>Ownership (Rust) — Haleiki Demo</title>`
   - `demo/_site/nyingma/index.html` should have `<title>Nyingma — Haleiki Demo</title>`
   - `demo/_site/uncertainty-principle/index.html` should have `<title>Uncertainty Principle — Haleiki Demo</title>`

4. **Confirm source pages still work** (regression check):
   - `demo/_build/sources/dzogchen.md` should still have `title: Dzogchen` at the top level
   - `demo/_site/source/dzogchen/index.html` should still have `<title>Dzogchen — Haleiki Demo</title>`

---

## While you're in there

A few small things worth eyeballing during the rebuild — flag any surprises but don't go fixing unless something's actually broken:

- **Multi-source merge rendering**: open `demo/_site/nyingma/index.html` and confirm it renders both source pages' contributions cleanly (the body should reference both `padmasambhava` and `longchenpa` per the source markdown).
- **Code-block rendering**: open `demo/_site/ownership-rust/index.html` and confirm Rust code blocks render as `<pre><code>` with the language hint preserved (no syntax highlighting yet — that's M8 — but the structure should be there).
- **LaTeX math preservation**: open `demo/_site/uncertainty-principle/index.html` and confirm `$\hbar$`, `$\sigma_X$`, etc. appear in the rendered HTML as raw text (the `{% raw %}` wrap should be doing its job; M8 will add MathJax/KaTeX rendering).

---

## Reporting

Just confirm:

1. The fix landed (paste the diff if you'd like).
2. The three concept-card spot-checks pass (frontmatter has correct title, rendered HTML has correct `<title>`).
3. Source pages still work (regression).
4. Anything surprising you noticed in the "while you're in there" spot-checks.

Don't commit yet — Duncan will review the M1 implementation as a whole (this fix included) before deciding to commit-to-main or branch-and-PR.
