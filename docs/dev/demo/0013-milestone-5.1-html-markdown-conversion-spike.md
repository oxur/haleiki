# Milestone 5.1 — HTML → Markdown Conversion Spike

**Version:** 1.0
**Depends on:** Milestone 3.1 (cleaned HTML), Milestone 4.3 (fully transformed HTML with local image paths)
**Produces:** Written evaluation of `htmd` (and alternatives) + decision documented; initial `tools/src/demo/convert.rs`

---

## Overview

This milestone is **investigative**. The goal is to evaluate pure-Rust HTML→Markdown crates against real fetched-and-cleaned Wikipedia articles, document conversion quality, identify post-processing fixups needed, and make a crate selection decision.

The output is:
1. A working prototype in `tools/src/demo/convert.rs` that converts at least 2–3 real articles
2. A quality evaluation documented as comments in the code (or a brief ADR)
3. A list of post-processing fixups needed (carried into milestone 5.2)

---

## Candidate Crates

### `htmd` (primary candidate)

- **Crate**: [htmd](https://crates.io/crates/htmd)
- **Approach**: Pure Rust, DOM-based conversion
- **Already a dependency**: Yes — declared in `Cargo.toml` under the `demo` feature
- **API**: `htmd::convert(html)` → `String`; supports custom element handlers

```rust
use htmd::HtmlToMarkdown;

let converter = HtmlToMarkdown::builder()
    .build();
let md = converter.convert(html)?;
```

### `html2md` (fallback candidate)

- **Crate**: [html2md](https://crates.io/crates/html2md)
- **Approach**: Pure Rust, regex + DOM hybrid
- **Not currently a dependency**: Would need adding if chosen
- **API**: `html2md::parse_html(html)` → `String`

```rust
let md = html2md::parse_html(html);
```

### Pandoc (optional quality fallback)

- **Not a Rust crate**: External binary, shelled out via `std::process::Command`
- **API**: `pandoc -f html -t markdown --wrap=none`
- **Already planned**: The `--pandoc` CLI flag from milestone 1.2's `DemoCommand::Fetch`
- **Pros**: Best conversion quality, handles complex tables and math
- **Cons**: External dependency, not available by default

---

## Evaluation Criteria

Test each candidate against real `.final.html` files from the pipeline. Score on:

| Criterion | Weight | What to check |
|-----------|--------|---------------|
| **Headings** | High | H1–H6 preserved with correct level, no extra whitespace |
| **Paragraphs** | High | Clean paragraph breaks, no collapsed or doubled spacing |
| **Inline formatting** | High | Bold, italic, links, inline code preserved |
| **Images** | High | `![caption](path)` syntax, alt text preserved |
| **Tables** | Medium | GFM table syntax, alignment, multiline cells |
| **Lists** | High | Ordered, unordered, nested lists with correct indentation |
| **Definition lists** | Low | HTML `<dl>/<dt>/<dd>` — Markdown has no native equivalent |
| **Code blocks** | Medium | Fenced code blocks with language hints |
| **Blockquotes** | Medium | `>` prefix, nesting |
| **Links** | High | Inline `[text](url)` with fragment anchors |
| **Math** | Low | `<math>` / MathJax — probably dropped or raw HTML |
| **Whitespace** | Medium | No excessive blank lines, no trailing whitespace |
| **Edge cases** | Medium | Empty cells, nested formatting, special characters |

---

## Step 1: Create `tools/src/demo/convert.rs` with evaluation harness

### File: `tools/src/demo/convert.rs`

```rust
//! HTML → Markdown conversion.
//!
//! ## Crate Evaluation (Milestone 5.1)
//!
//! This module evaluates pure-Rust HTML→Markdown crates against real
//! Wikipedia articles cleaned by the demo pipeline.
//!
//! ### Evaluation results
//!
//! (To be filled in after running against real articles)
//!
//! #### htmd
//! - Headings:
//! - Paragraphs:
//! - Inline formatting:
//! - Images:
//! - Tables:
//! - Lists:
//! - Links:
//! - Overall:
//!
//! #### Post-processing fixups needed
//! - (To be identified during evaluation)

use std::path::Path;

use super::media::staging_final_path;

/// Path where converted Markdown is staged before frontmatter injection.
pub fn staging_markdown_path(slug: &str) -> std::path::PathBuf {
    Path::new("demo/.staging").join(format!("{slug}.md"))
}

/// Convert HTML to Markdown using the `htmd` crate.
///
/// This is the primary conversion function. Post-processing fixups
/// are applied after the initial conversion.
pub fn html_to_markdown(html: &str) -> anyhow::Result<String> {
    let converter = htmd::HtmlToMarkdown::builder()
        .build();

    let raw_md = converter.convert(html)
        .map_err(|e| anyhow::anyhow!("htmd conversion failed: {e}"))?;

    let cleaned = post_process(&raw_md);

    Ok(cleaned)
}

/// Post-processing fixups applied after initial conversion.
///
/// These address known issues with the converter output identified
/// during the evaluation spike.
fn post_process(md: &str) -> String {
    let mut result = md.to_string();

    // ── Fixup 1: Normalize excessive blank lines ────────
    // Converters often produce 3+ consecutive blank lines.
    // Normalize to at most 2 (one blank line between elements).
    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }

    // ── Fixup 2: Remove trailing whitespace on each line ─
    result = result
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");

    // ── Fixup 3: Ensure file ends with a single newline ──
    result = result.trim_end().to_string();
    result.push('\n');

    // ── Fixup 4: Normalize image syntax ─────────────────
    // Some converters produce `![](path)` without alt text
    // when the alt was empty. This is technically valid but
    // we prefer `![image](path)` for accessibility.
    // (Evaluate whether this is actually needed with htmd)

    // ── Fixup 5: Heading level normalization ────────────
    // Ensure the document body starts at H2 (H1 reserved for title).
    // The title is injected as frontmatter, so body headings
    // should be H2+. If the converter outputs H1s in the body,
    // shift all headings down by one level.
    result = normalize_heading_levels(&result);

    result
}

/// Ensure no H1 headings appear in the body.
///
/// If any `# Heading` (H1) is found, shift all headings down one level:
/// `#` → `##`, `##` → `###`, etc.
///
/// This is needed because the article title becomes the H1 via frontmatter,
/// and Wikipedia articles often have an H1 that duplicates the title.
fn normalize_heading_levels(md: &str) -> String {
    // Check if any H1 exists
    let has_h1 = md.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("# ") && !trimmed.starts_with("## ")
    });

    if !has_h1 {
        return md.to_string();
    }

    // Shift all headings down one level
    md.lines()
        .map(|line| {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                // Count leading #s
                let hashes = trimmed.chars().take_while(|&c| c == '#').count();
                if hashes <= 6 && trimmed.len() > hashes && trimmed.as_bytes()[hashes] == b' ' {
                    // Add one more # (shift down)
                    let new_hashes = std::cmp::min(hashes + 1, 6);
                    return format!(
                        "{}{} {}",
                        " ".repeat(line.len() - trimmed.len()), // preserve leading whitespace
                        "#".repeat(new_hashes),
                        &trimmed[hashes + 1..], // text after "# "
                    );
                }
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Convert a single article's final HTML to Markdown.
///
/// Reads `demo/.staging/{slug}.final.html`, converts to Markdown,
/// writes `demo/.staging/{slug}.md`.
///
/// Note: This produces Markdown WITHOUT frontmatter. Frontmatter
/// injection happens in milestone 5.3.
pub fn convert_article(slug: &str) -> anyhow::Result<std::path::PathBuf> {
    let input_path = staging_final_path(slug);
    if !input_path.exists() {
        anyhow::bail!(
            "final HTML not found at {}. Run the full HTML pipeline first.",
            input_path.display(),
        );
    }

    let html = std::fs::read_to_string(&input_path)?;
    let markdown = html_to_markdown(&html)?;

    let output_path = staging_markdown_path(slug);
    std::fs::write(&output_path, &markdown)?;

    Ok(output_path)
}

/// Convert using Pandoc as an external process (optional quality fallback).
///
/// Requires `pandoc` to be installed and on PATH.
pub fn html_to_markdown_pandoc(html: &str) -> anyhow::Result<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("pandoc")
        .args([
            "-f", "html",
            "-t", "markdown",
            "--wrap=none",
            "--no-highlight",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!(
            "failed to run pandoc (is it installed?): {e}"
        ))?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(html.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("pandoc failed: {stderr}");
    }

    let raw_md = String::from_utf8(output.stdout)?;
    let cleaned = post_process(&raw_md);

    Ok(cleaned)
}
```

---

## Step 2: Wire into `demo/mod.rs`

### Update module declarations

```rust
pub mod clean;
pub mod convert;
pub mod fetch;
pub mod manifest;
pub mod media;
pub mod rewrite;
pub mod status;
```

### Optional: add a hidden dev command for testing

```rust
/// [Dev] Convert a single article's HTML to Markdown
#[command(hide = true)]
Convert {
    /// Article slug
    slug: String,

    /// Use pandoc instead of built-in converter
    #[arg(long)]
    pandoc: bool,
},
```

---

## Step 3: Evaluation procedure

This is the investigative part. Run the conversion against 2–3 real articles spanning different content types:

### Recommended test articles

1. **`quantum-mechanics`** — Physics article with math, tables, diagrams, complex structure
2. **`johann-sebastian-bach`** — Biography with infobox remnants, images, timeline-style content
3. **`dzogchen`** — Tibetan Buddhist topic with specialized terminology, potentially simpler structure

### Evaluation steps

```bash
# 1. Ensure articles are fetched and fully processed through Phase 4
cargo run --features demo -- demo fetch --article quantum-mechanics
# Then run clean, rewrite, media pipeline (via dev commands or test harness)

# 2. Convert with htmd
cargo run --features demo -- convert quantum-mechanics

# 3. Inspect the output
cat demo/.staging/quantum-mechanics.md | head -100

# 4. Compare against pandoc (if available)
cargo run --features demo -- convert quantum-mechanics --pandoc
diff demo/.staging/quantum-mechanics.md demo/.staging/quantum-mechanics.pandoc.md

# 5. Open in a Markdown previewer to visually assess quality
```

### What to look for

For each test article, check:

```
[ ] Headings: correct level? clean text? no stray markup?
[ ] Paragraphs: proper spacing? no collapsed lines?
[ ] Bold/italic: preserved? no broken nesting?
[ ] Links: [text](url) format? fragments work?
[ ] Images: ![alt](../media/slug/file.ext) format?
[ ] Tables: GFM pipe tables? readable? alignment?
[ ] Lists: correct nesting? proper markers (- vs 1.)?
[ ] Code blocks: fenced? language hints present?
[ ] Special chars: &amp; decoded? unicode preserved?
[ ] Whitespace: no trailing spaces? reasonable blank lines?
```

### Record findings

Update the doc comment at the top of `convert.rs` with the evaluation results. Example:

```rust
//! #### htmd evaluation results
//!
//! Tested against: quantum-mechanics, johann-sebastian-bach, dzogchen
//!
//! - Headings: GOOD — correct levels, clean text
//! - Paragraphs: GOOD — proper spacing
//! - Inline formatting: GOOD — bold/italic preserved
//! - Images: NEEDS FIXUP — alt text sometimes missing
//! - Tables: FAIR — simple tables ok, complex tables break
//! - Lists: GOOD — nesting correct
//! - Links: GOOD — [text](url) format correct
//! - Math: DROPS — <math> elements become empty strings
//! - Overall: ACCEPTABLE with post-processing
//!
//! #### Post-processing fixups needed
//! 1. Excessive blank lines (3+ → 2)
//! 2. Trailing whitespace on some lines
//! 3. H1 in body needs shifting to H2+
//! 4. Empty image alt text: ![](path) → ![image](path)
//! 5. (add more as discovered)
//!
//! #### Decision: USE htmd
//! htmd produces acceptable output for the demo's content types.
//! Complex tables may need manual review post-conversion.
//! Pandoc fallback available via --pandoc flag for quality-critical content.
```

---

## Step 4: Write tests

### Unit tests in `convert.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ─── Post-processing tests ──────────────────────────

    #[test]
    fn test_post_process_normalizes_blank_lines() {
        let input = "Line 1\n\n\n\nLine 2\n\n\n\n\nLine 3\n";
        let result = post_process(input);
        assert!(!result.contains("\n\n\n"), "Triple+ blank lines should be normalized");
        assert!(result.contains("Line 1\n\nLine 2"), "Double blank line should remain");
    }

    #[test]
    fn test_post_process_trims_trailing_whitespace() {
        let input = "Line with trailing spaces   \nAnother line  \n";
        let result = post_process(input);
        for line in result.lines() {
            assert_eq!(line, line.trim_end(), "Line has trailing whitespace: '{line}'");
        }
    }

    #[test]
    fn test_post_process_ends_with_single_newline() {
        let input = "Content\n\n\n";
        let result = post_process(input);
        assert!(result.ends_with('\n'), "Should end with newline");
        assert!(!result.ends_with("\n\n"), "Should not end with double newline");
    }

    // ─── Heading normalization tests ────────────────────

    #[test]
    fn test_normalize_heading_levels_shifts_when_h1_present() {
        let input = "# Title\n\nSome text.\n\n## Section\n\n### Subsection\n";
        let result = normalize_heading_levels(input);
        assert!(result.contains("## Title"), "H1 should become H2: {result}");
        assert!(result.contains("### Section"), "H2 should become H3: {result}");
        assert!(result.contains("#### Subsection"), "H3 should become H4: {result}");
    }

    #[test]
    fn test_normalize_heading_levels_no_shift_when_no_h1() {
        let input = "## Section\n\n### Subsection\n";
        let result = normalize_heading_levels(input);
        assert_eq!(result, input, "Should not modify when no H1 present");
    }

    #[test]
    fn test_normalize_heading_levels_caps_at_h6() {
        let input = "# H1\n\n###### H6\n";
        let result = normalize_heading_levels(input);
        assert!(result.contains("## H1"), "H1 should become H2");
        assert!(result.contains("###### H6"), "H6 should stay H6 (can't go to H7)");
    }

    #[test]
    fn test_normalize_heading_levels_ignores_hash_in_content() {
        let input = "## Section\n\nThe color code is #FF0000.\n";
        let result = normalize_heading_levels(input);
        assert!(result.contains("#FF0000"), "Hash in content should not be modified");
    }

    // ─── HTML to Markdown conversion tests ──────────────

    #[test]
    fn test_html_to_markdown_basic_paragraph() {
        let html = "<p>Hello, world!</p>";
        let md = html_to_markdown(html).unwrap();
        assert!(md.contains("Hello, world!"), "Paragraph text missing: {md}");
    }

    #[test]
    fn test_html_to_markdown_heading() {
        let html = "<h2>Section Title</h2><p>Content.</p>";
        let md = html_to_markdown(html).unwrap();
        assert!(md.contains("## Section Title"), "H2 not converted: {md}");
    }

    #[test]
    fn test_html_to_markdown_bold_and_italic() {
        let html = "<p>This is <strong>bold</strong> and <em>italic</em>.</p>";
        let md = html_to_markdown(html).unwrap();
        assert!(md.contains("**bold**"), "Bold not converted: {md}");
        assert!(md.contains("*italic*"), "Italic not converted: {md}");
    }

    #[test]
    fn test_html_to_markdown_link() {
        let html = r#"<p>See <a href="/source/raii/">RAII</a> for details.</p>"#;
        let md = html_to_markdown(html).unwrap();
        assert!(md.contains("[RAII](/source/raii/)"), "Link not converted: {md}");
    }

    #[test]
    fn test_html_to_markdown_image() {
        let html = r#"<img src="../media/test/photo.png" alt="A photo" />"#;
        let md = html_to_markdown(html).unwrap();
        assert!(
            md.contains("![") && md.contains("](../media/test/photo.png)"),
            "Image not converted: {md}",
        );
    }

    #[test]
    fn test_html_to_markdown_unordered_list() {
        let html = "<ul><li>Item one</li><li>Item two</li></ul>";
        let md = html_to_markdown(html).unwrap();
        // htmd may use - or * for list markers
        assert!(
            (md.contains("- Item one") || md.contains("* Item one")),
            "List not converted: {md}",
        );
    }

    #[test]
    fn test_html_to_markdown_ordered_list() {
        let html = "<ol><li>First</li><li>Second</li></ol>";
        let md = html_to_markdown(html).unwrap();
        assert!(md.contains("1.") || md.contains("1)"), "Ordered list not converted: {md}");
        assert!(md.contains("First"), "List item text missing: {md}");
    }

    #[test]
    fn test_html_to_markdown_table() {
        let html = r#"
            <table>
                <tr><th>Name</th><th>Value</th></tr>
                <tr><td>Alpha</td><td>1</td></tr>
                <tr><td>Beta</td><td>2</td></tr>
            </table>
        "#;
        let md = html_to_markdown(html).unwrap();
        // GFM tables use pipe syntax
        assert!(md.contains('|'), "Table not converted to pipe syntax: {md}");
        assert!(md.contains("Alpha"), "Table cell missing: {md}");
        assert!(md.contains("Beta"), "Table cell missing: {md}");
    }

    #[test]
    fn test_html_to_markdown_code_block() {
        let html = "<pre><code>fn main() {}\n</code></pre>";
        let md = html_to_markdown(html).unwrap();
        assert!(
            md.contains("```") || md.contains("    fn main()"),
            "Code block not converted: {md}",
        );
    }

    #[test]
    fn test_html_to_markdown_blockquote() {
        let html = "<blockquote><p>A wise saying.</p></blockquote>";
        let md = html_to_markdown(html).unwrap();
        assert!(md.contains("> ") || md.contains(">A"), "Blockquote not converted: {md}");
    }

    // ─── Pandoc fallback test ───────────────────────────

    #[test]
    #[ignore] // Requires pandoc installed
    fn test_html_to_markdown_pandoc_basic() {
        let html = "<h2>Title</h2><p>Content with <strong>bold</strong>.</p>";
        let md = html_to_markdown_pandoc(html).unwrap();
        assert!(md.contains("Title"), "Heading missing: {md}");
        assert!(md.contains("**bold**"), "Bold missing: {md}");
    }

    // ─── Real article test ──────────────────────────────

    #[test]
    #[ignore] // Requires demo/.staging/{slug}.final.html
    fn test_convert_article_real() {
        // Try converting a real article
        for slug in &["quantum-mechanics", "johann-sebastian-bach", "dzogchen"] {
            let final_path = staging_final_path(slug);
            if !final_path.exists() {
                eprintln!("Skipping {slug}: no .final.html");
                continue;
            }

            let result = convert_article(slug);
            assert!(result.is_ok(), "Conversion failed for {slug}: {:?}", result.err());

            let md_path = result.unwrap();
            let md = std::fs::read_to_string(&md_path).unwrap();

            // Basic sanity checks
            assert!(md.len() > 500, "{slug}: Markdown too short ({} bytes)", md.len());
            assert!(md.contains("##"), "{slug}: No headings found");
            assert!(!md.contains("<p>"), "{slug}: Raw <p> tags in output");
            assert!(!md.contains("<div>"), "{slug}: Raw <div> tags in output");

            eprintln!(
                "{slug}: converted OK ({} bytes, {} lines)",
                md.len(),
                md.lines().count(),
            );
        }
    }
}
```

---

## Step 5: Run the evaluation

This is a manual step. After implementing the code above:

1. Ensure at least 2–3 articles have been fully processed through Phase 4
2. Run the conversion against each
3. Visually inspect the Markdown output
4. Document findings in the `convert.rs` doc comment
5. Identify post-processing fixups (add to the `post_process()` function)
6. Make the crate decision

### Expected outcome

Based on `htmd`'s capabilities and typical Wikipedia HTML structure:

- **Likely acceptable**: headings, paragraphs, bold/italic, links, basic lists, images, code blocks
- **Likely needs fixups**: excessive whitespace, heading levels, empty alt text
- **Likely problematic**: complex nested tables, math markup, definition lists
- **Decision**: Probably `htmd` with post-processing, plus `--pandoc` flag as escape hatch

---

## Verification

### 6.1: Conversion produces valid Markdown

```bash
cargo run --features demo -- convert dzogchen
cat demo/.staging/dzogchen.md | head -50
```

### 6.2: No raw HTML in output

```bash
grep -c '<p>\|<div>\|<span>\|<table>' demo/.staging/dzogchen.md
# Should be 0 (or very low — some inline HTML may be intentional)
```

### 6.3: Images converted correctly

```bash
grep '!\[' demo/.staging/dzogchen.md
# Should show ![alt](../media/dzogchen/filename.ext) patterns
```

### 6.4: Tests pass

```bash
cargo test --features demo
make lint
```

---

## Acceptance Criteria

- [ ] `tools/src/demo/convert.rs` exists with `html_to_markdown()` function
- [ ] `htmd` crate successfully converts cleaned Wikipedia HTML to Markdown
- [ ] `post_process()` applies identified fixups (blank lines, trailing whitespace, heading levels)
- [ ] `normalize_heading_levels()` shifts H1→H2 when H1 is present in body
- [ ] `convert_article()` reads `.final.html` and writes `.staging/{slug}.md`
- [ ] `html_to_markdown_pandoc()` works as a fallback (when pandoc is installed)
- [ ] Evaluation run against 2–3 real articles
- [ ] Quality findings documented in `convert.rs` doc comment
- [ ] Post-processing fixup list documented
- [ ] Crate selection decision documented
- [ ] All unit tests pass (15+ tests)
- [ ] `make lint` passes

---

## Gotchas

1. **`htmd` API surface**: The `htmd` crate may have a different API than shown. Check the actual docs with `cargo doc --open` after adding the dependency. The builder pattern shown above is common but may differ. Adjust the code to match the actual API.

2. **`htmd` version**: The design doc specifies `htmd = "0.1"`. This may be outdated. Run `cargo search htmd --limit 1` to find the latest version. If `htmd` doesn't exist or is unmaintained, fall back to `html2md`.

3. **HTML fragment vs. document**: The `.final.html` files may be full HTML documents (with `<html>`, `<body>`) or fragments. Most converters handle both, but test to be sure. If the converter chokes on a full document, extract just the `<body>` content before converting.

4. **Unicode preservation**: Wikipedia articles contain Unicode characters (Tibetan script, diacritics like ö, ä, é). The converter must preserve these — no mojibake, no HTML entity escaping.

5. **Image alt text**: If `htmd` produces `![](path)` (empty alt), the post-processor should fix it to `![image](path)`. Check whether this actually happens before adding the fixup.

6. **Table quality**: GFM tables require consistent column counts and separator rows. Wikipedia tables are often complex (colspan, rowspan, nested content). If `htmd` can't handle these, document it as a known limitation and note that `--pandoc` handles them better.

7. **Definition lists**: HTML `<dl>/<dt>/<dd>` has no standard Markdown equivalent. Most converters either drop them or produce something nonstandard. Document the behavior — it may need a custom handler.

8. **Math markup**: Wikipedia `<math>` elements contain LaTeX-like content. These will likely be dropped or mangled by `htmd`. For the demo, this is acceptable (math-heavy articles are few). Document the limitation.

9. **Staging file naming**: The staged Markdown is `demo/.staging/{slug}.md`, NOT `demo/sources/{slug}.md`. The final location is written by milestone 5.3 (frontmatter injection). Don't write directly to `demo/sources/` in this milestone.

10. **This milestone is investigative**: The primary output is knowledge (what works, what doesn't), not production-ready code. It's OK if the code needs refactoring in 5.2. The goal is to validate the approach and identify all the fixups needed.
