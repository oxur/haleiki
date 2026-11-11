# Milestone 5.2 — Markdown Conversion Implementation (`convert.rs`)

**Version:** 1.0
**Depends on:** Milestone 5.1 (evaluation complete, crate selected, fixups identified)
**Produces:** Clean Markdown files from Wikipedia HTML for all 90 articles

---

## Overview

Implement the production-quality HTML→Markdown converter based on the findings from the 5.1 spike. This milestone hardens the prototype into robust, tested code that handles all 90 articles in the demo manifest.

Key activities:
1. Refine `post_process()` with all fixups identified during the 5.1 evaluation
2. Add custom element handlers for problematic HTML patterns (tables, definition lists, etc.)
3. Implement the `--pandoc` flag as an alternative backend
4. Wire conversion into the pipeline so all articles can be batch-converted
5. Add comprehensive tests covering edge cases discovered during evaluation

---

## Step 1: Refine post-processing based on 5.1 findings

### Update `post_process()` in `tools/src/demo/convert.rs`

The 5.1 spike identified the initial fixups. This milestone adds the full set. The exact fixups depend on what 5.1 discovers, but based on typical converter output, expect these:

```rust
fn post_process(md: &str) -> String {
    let mut result = md.to_string();

    // ── Fixup 1: Normalize excessive blank lines ────────
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

    // ── Fixup 4: Heading level normalization ────────────
    result = normalize_heading_levels(&result);

    // ── Fixup 5: Clean up converter artifacts ───────────
    result = clean_converter_artifacts(&result);

    // ── Fixup 6: Normalize image syntax ─────────────────
    result = normalize_image_syntax(&result);

    // ── Fixup 7: Fix broken table formatting ────────────
    result = fix_tables(&result);

    // ── Fixup 8: Normalize link syntax ──────────────────
    result = normalize_links(&result);

    result
}
```

---

## Step 2: Implement individual fixup functions

### `clean_converter_artifacts()`

```rust
/// Remove common artifacts left by HTML→Markdown converters.
fn clean_converter_artifacts(md: &str) -> String {
    let mut result = md.to_string();

    // Remove leftover HTML comments
    // (converters sometimes pass these through)
    while let Some(start) = result.find("<!--") {
        if let Some(end) = result[start..].find("-->") {
            result.replace_range(start..start + end + 3, "");
        } else {
            break;
        }
    }

    // Remove empty link references: [](url) with no text
    // These are usually artifacts from stripped elements
    result = remove_empty_links(&result);

    // Remove leftover HTML entities that should have been decoded
    result = result.replace("&nbsp;", " ");
    result = result.replace("&mdash;", "—");
    result = result.replace("&ndash;", "–");
    result = result.replace("&hellip;", "…");
    result = result.replace("&amp;", "&"); // Must be last (re-encoding risk)

    result
}

/// Remove Markdown links that have no visible text: `[](url)`.
fn remove_empty_links(md: &str) -> String {
    let mut result = String::with_capacity(md.len());
    let mut chars = md.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '[' {
            // Check for []( pattern
            if chars.peek() == Some(&']') {
                let mut lookahead = chars.clone();
                lookahead.next(); // consume ]
                if lookahead.peek() == Some(&'(') {
                    // Found [](  — skip until closing )
                    chars.next(); // consume ]
                    chars.next(); // consume (
                    let mut depth = 1;
                    for ch in chars.by_ref() {
                        if ch == '(' {
                            depth += 1;
                        } else if ch == ')' {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                    }
                    continue;
                }
            }
        }
        result.push(c);
    }

    result
}
```

### `normalize_image_syntax()`

```rust
/// Normalize image Markdown syntax.
///
/// - Ensure alt text is present: `![](path)` → `![image](path)`
/// - Normalize whitespace in alt text
/// - Handle figure-style images (image on its own line with caption below)
fn normalize_image_syntax(md: &str) -> String {
    let mut lines: Vec<String> = md.lines().map(String::from).collect();

    for line in &mut lines {
        // Fix empty alt text
        if line.contains("![](") {
            *line = line.replace("![](", "![image](");
        }

        // Trim whitespace inside alt text: ![  text  ](url) → ![text](url)
        if let Some(start) = line.find("![") {
            if let Some(mid) = line[start..].find("](") {
                let alt_start = start + 2;
                let alt_end = start + mid;
                if alt_start < alt_end {
                    let alt = line[alt_start..alt_end].trim().to_string();
                    let before = &line[..alt_start];
                    let after = &line[alt_end..];
                    *line = format!("{before}{alt}{after}");
                }
            }
        }
    }

    lines.join("\n")
}
```

### `fix_tables()`

```rust
/// Fix common table formatting issues.
///
/// - Ensure separator row exists between header and body
/// - Normalize column count (pad short rows)
/// - Remove empty tables
fn fix_tables(md: &str) -> String {
    let lines: Vec<&str> = md.lines().collect();
    let mut result_lines: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        // Detect table rows (lines starting and containing |)
        if is_table_row(line) {
            // Collect the entire table
            let table_start = i;
            let mut table_lines: Vec<&str> = Vec::new();

            while i < lines.len() && (is_table_row(lines[i]) || is_separator_row(lines[i])) {
                table_lines.push(lines[i]);
                i += 1;
            }

            // Process the table
            if table_lines.len() >= 2 {
                let fixed = fix_table_block(&table_lines);
                result_lines.extend(fixed);
            } else {
                // Single-row "table" — probably not a real table, pass through
                result_lines.extend(table_lines.into_iter().map(String::from));
            }
            continue;
        }

        result_lines.push(line.to_string());
        i += 1;
    }

    result_lines.join("\n")
}

fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|') && trimmed.len() > 1
}

fn is_separator_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|')
        && trimmed.ends_with('|')
        && trimmed.chars().all(|c| c == '|' || c == '-' || c == ':' || c == ' ')
}

/// Fix a block of table lines.
///
/// Ensures a separator row exists after the first row (header).
fn fix_table_block(lines: &[&str]) -> Vec<String> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    result.push(lines[0].to_string());

    // Check if second line is already a separator
    let has_separator = lines.len() > 1 && is_separator_row(lines[1]);

    if !has_separator {
        // Generate a separator row matching the column count of the first row
        let col_count = lines[0].matches('|').count().saturating_sub(1);
        if col_count > 0 {
            let sep = format!(
                "|{}|",
                (0..col_count).map(|_| " --- ").collect::<Vec<_>>().join("|"),
            );
            result.push(sep);
        }
    }

    // Add remaining rows
    let start = if has_separator { 1 } else { 1 };
    for line in &lines[start..] {
        result.push(line.to_string());
    }

    result
}
```

### `normalize_links()`

```rust
/// Normalize link syntax.
///
/// - Decode remaining percent-encoded characters in display text
/// - Remove empty title attributes: [text](url "")  → [text](url)
fn normalize_links(md: &str) -> String {
    // Remove empty title attributes in links
    md.replace(" \"\")", ")")
}
```

---

## Step 3: Add custom element handlers for `htmd`

If `htmd` supports custom element handlers (check during 5.1), register handlers for problematic patterns:

```rust
/// Build the HTML→Markdown converter with custom element handlers.
fn build_converter() -> htmd::HtmlToMarkdown {
    htmd::HtmlToMarkdown::builder()
        // Handle <dl>/<dt>/<dd> (definition lists → bold term + indented definition)
        // Handle <sup> (superscripts → keep as HTML or drop)
        // Handle <sub> (subscripts → keep as HTML or drop)
        // Handle <math> (drop or preserve as raw HTML)
        .build()
}
```

**Note**: The exact API for custom handlers depends on the `htmd` crate version. Adapt based on what 5.1 discovers. If `htmd` doesn't support custom handlers, implement them as a pre-processing step (transform the HTML before passing to `htmd`) or a post-processing step (regex/string fixes on the Markdown output).

### Definition list pre-processor

If the converter can't handle `<dl>/<dt>/<dd>`, transform them before conversion:

```rust
/// Convert HTML definition lists to a Markdown-compatible format.
///
/// `<dl><dt>Term</dt><dd>Definition</dd></dl>`
/// → `**Term**\n: Definition\n`
///
/// Uses the PHP Markdown Extra / Pandoc definition list syntax.
/// If the target Markdown renderer doesn't support this, use a simpler
/// format: `**Term** — Definition`
fn preprocess_definition_lists(html: &str) -> String {
    // Simple regex-free approach: use scraper to find <dl> elements
    // and replace them with formatted text.
    //
    // This runs BEFORE the main converter.
    let document = scraper::Html::parse_fragment(html);
    let dl_selector = scraper::Selector::parse("dl").unwrap();

    if document.select(&dl_selector).next().is_none() {
        return html.to_string(); // No definition lists, skip
    }

    let mut result = html.to_string();

    let dt_selector = scraper::Selector::parse("dt").unwrap();
    let dd_selector = scraper::Selector::parse("dd").unwrap();

    for dl in document.select(&dl_selector) {
        let mut replacement = String::new();

        for child in dl.children() {
            if let Some(el) = child.value().as_element() {
                match el.name() {
                    "dt" => {
                        let text: String = scraper::ElementRef::wrap(child)
                            .map(|e| e.text().collect())
                            .unwrap_or_default();
                        replacement.push_str(&format!("\n**{}**\n", text.trim()));
                    }
                    "dd" => {
                        let text: String = scraper::ElementRef::wrap(child)
                            .map(|e| e.text().collect())
                            .unwrap_or_default();
                        replacement.push_str(&format!(": {}\n", text.trim()));
                    }
                    _ => {}
                }
            }
        }

        // Replace the <dl>...</dl> in the original HTML
        let dl_html = dl.html();
        result = result.replace(&dl_html, &replacement);
    }

    result
}
```

---

## Step 4: Wire batch conversion

### Add batch conversion function to `convert.rs`

```rust
/// Convert all articles that have final HTML but no staged Markdown.
///
/// Returns the number of articles converted.
pub fn convert_all_articles(use_pandoc: bool) -> anyhow::Result<usize> {
    let staging_dir = Path::new("demo/.staging");
    if !staging_dir.exists() {
        anyhow::bail!("staging directory not found — run the fetch pipeline first");
    }

    let mut converted = 0;
    let mut failed = 0;

    // Find all .final.html files
    for entry in std::fs::read_dir(staging_dir)? {
        let entry = entry?;
        let path = entry.path();

        let Some(filename) = path.file_name().and_then(|f| f.to_str()) else {
            continue;
        };

        if !filename.ends_with(".final.html") {
            continue;
        }

        let slug = filename.strip_suffix(".final.html").unwrap();

        // Skip if already converted
        let md_path = staging_markdown_path(slug);
        if md_path.exists() {
            eprintln!("  {slug}: already converted, skipping");
            continue;
        }

        eprintln!("  Converting: {slug}");

        let html = std::fs::read_to_string(&path)?;

        let md_result = if use_pandoc {
            html_to_markdown_pandoc(&html)
        } else {
            html_to_markdown(&html)
        };

        match md_result {
            Ok(md) => {
                std::fs::write(&md_path, &md)?;
                converted += 1;
                eprintln!(
                    "    OK ({} bytes, {} lines)",
                    md.len(),
                    md.lines().count(),
                );
            }
            Err(e) => {
                eprintln!("    FAILED: {e}");
                failed += 1;
            }
        }
    }

    eprintln!();
    eprintln!("Conversion complete: {converted} converted, {failed} failed");

    if failed > 0 {
        anyhow::bail!("{failed} article(s) failed conversion");
    }

    Ok(converted)
}

/// Force-reconvert a single article (overwrites existing staged Markdown).
pub fn reconvert_article(slug: &str, use_pandoc: bool) -> anyhow::Result<std::path::PathBuf> {
    let input_path = staging_final_path(slug);
    if !input_path.exists() {
        anyhow::bail!(
            "final HTML not found at {}. Run the full HTML pipeline first.",
            input_path.display(),
        );
    }

    let html = std::fs::read_to_string(&input_path)?;

    let markdown = if use_pandoc {
        html_to_markdown_pandoc(&html)?
    } else {
        html_to_markdown(&html)?
    };

    let output_path = staging_markdown_path(slug);
    std::fs::write(&output_path, &markdown)?;

    Ok(output_path)
}
```

### Update `html_to_markdown()` to include pre-processing

```rust
pub fn html_to_markdown(html: &str) -> anyhow::Result<String> {
    // Pre-process: handle elements the converter struggles with
    let preprocessed = preprocess_definition_lists(html);

    let converter = build_converter();

    let raw_md = converter.convert(&preprocessed)
        .map_err(|e| anyhow::anyhow!("htmd conversion failed: {e}"))?;

    let cleaned = post_process(&raw_md);

    Ok(cleaned)
}
```

---

## Step 5: Wire the `--pandoc` flag through the CLI

### Update `demo/mod.rs`

The `DemoCommand::Fetch` already has a `pandoc` flag. When the pipeline reaches the conversion step, pass it through:

```rust
DemoCommand::Fetch { article, dry_run, force, pandoc } => {
    // ... existing fetch logic ...
    // When conversion is integrated into the full pipeline,
    // the pandoc flag controls which backend to use.
}
```

For now, the hidden `Convert` dev command from 5.1 is the primary way to invoke conversion:

```rust
DemoCommand::Convert { slug, pandoc } => {
    if let Some(slug) = slug {
        convert::reconvert_article(&slug, pandoc)?;
    } else {
        convert::convert_all_articles(pandoc)?;
    }
}
```

Update the `Convert` command to support optional slug (convert one vs. all):

```rust
/// [Dev] Convert final HTML to Markdown
#[command(hide = true)]
Convert {
    /// Article slug (omit to convert all)
    slug: Option<String>,

    /// Use pandoc instead of built-in converter
    #[arg(long)]
    pandoc: bool,
},
```

---

## Step 6: Write tests

### Additional unit tests in `convert.rs`

```rust
// ─── Converter artifact cleanup tests ───────────────

#[test]
fn test_clean_converter_artifacts_removes_html_comments() {
    let input = "Text <!-- comment --> more text\n";
    let result = clean_converter_artifacts(input);
    assert!(!result.contains("<!--"), "HTML comment not removed: {result}");
    assert!(result.contains("Text  more text"), "Surrounding text lost: {result}");
}

#[test]
fn test_clean_converter_artifacts_decodes_entities() {
    let input = "A&nbsp;B and C&mdash;D and E&amp;F\n";
    let result = clean_converter_artifacts(input);
    assert!(result.contains("A B"), "nbsp not decoded: {result}");
    assert!(result.contains("C—D"), "mdash not decoded: {result}");
    assert!(result.contains("E&F"), "amp not decoded: {result}");
}

#[test]
fn test_remove_empty_links() {
    let input = "Before [](http://example.com) after\n";
    let result = remove_empty_links(input);
    assert!(!result.contains("[]("), "Empty link not removed: {result}");
    assert!(result.contains("Before"), "Surrounding text lost");
    assert!(result.contains("after"), "Surrounding text lost");
}

#[test]
fn test_remove_empty_links_preserves_non_empty() {
    let input = "See [RAII](/source/raii/) for details.\n";
    let result = remove_empty_links(input);
    assert_eq!(result, input, "Non-empty link should be preserved");
}

// ─── Image normalization tests ──────────────────────

#[test]
fn test_normalize_image_syntax_empty_alt() {
    let input = "![](../media/test/photo.png)\n";
    let result = normalize_image_syntax(input);
    assert!(
        result.contains("![image](../media/test/photo.png)"),
        "Empty alt not fixed: {result}",
    );
}

#[test]
fn test_normalize_image_syntax_whitespace_alt() {
    let input = "![  some text  ](../media/test/photo.png)\n";
    let result = normalize_image_syntax(input);
    assert!(
        result.contains("![some text]("),
        "Alt text whitespace not trimmed: {result}",
    );
}

#[test]
fn test_normalize_image_syntax_preserves_good_images() {
    let input = "![A clear caption](../media/test/photo.png)\n";
    let result = normalize_image_syntax(input);
    assert_eq!(result, input, "Good image syntax should not be modified");
}

// ─── Table fixup tests ─────────────────────────────

#[test]
fn test_fix_tables_adds_missing_separator() {
    let input = "| Name | Value |\n| Alpha | 1 |\n| Beta | 2 |\n";
    let result = fix_tables(input);
    let lines: Vec<&str> = result.lines().collect();
    assert!(lines.len() >= 4, "Should have added separator row: {result}");
    assert!(
        is_separator_row(lines[1]),
        "Second line should be separator: {}",
        lines[1],
    );
}

#[test]
fn test_fix_tables_preserves_existing_separator() {
    let input = "| Name | Value |\n| --- | --- |\n| Alpha | 1 |\n";
    let result = fix_tables(input);
    assert_eq!(result, input, "Table with separator should not be modified");
}

#[test]
fn test_fix_tables_preserves_non_table_content() {
    let input = "# Heading\n\nParagraph text.\n\n- List item\n";
    let result = fix_tables(input);
    assert_eq!(result, input, "Non-table content should not be modified");
}

// ─── Link normalization tests ───────────────────────

#[test]
fn test_normalize_links_removes_empty_title() {
    let input = r#"[text](http://example.com "")"#;
    let result = normalize_links(input);
    assert_eq!(result, "[text](http://example.com)");
}

// ─── Batch conversion tests ────────────────────────

#[test]
#[ignore] // Requires .final.html files in staging
fn test_convert_all_articles_processes_finals() {
    let staging_dir = Path::new("demo/.staging");
    if !staging_dir.exists() {
        return;
    }

    // Count .final.html files
    let final_count = std::fs::read_dir(staging_dir)
        .unwrap()
        .filter(|e| {
            e.as_ref()
                .ok()
                .and_then(|e| e.file_name().to_str().map(|s| s.ends_with(".final.html")))
                .unwrap_or(false)
        })
        .count();

    if final_count == 0 {
        eprintln!("No .final.html files found, skipping batch test");
        return;
    }

    // Clean existing .md files to force reconversion
    for entry in std::fs::read_dir(staging_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "md") {
            let _ = std::fs::remove_file(&path);
        }
    }

    let result = convert_all_articles(false);
    assert!(result.is_ok(), "Batch conversion failed: {:?}", result.err());

    let converted = result.unwrap();
    assert_eq!(
        converted, final_count,
        "Should have converted all {final_count} articles, got {converted}",
    );
}

// ─── Definition list pre-processing tests ───────────

#[test]
fn test_preprocess_definition_lists_basic() {
    let html = "<dl><dt>Term</dt><dd>Definition here</dd></dl>";
    let result = preprocess_definition_lists(html);
    assert!(result.contains("**Term**"), "Term not bolded: {result}");
    assert!(result.contains(": Definition here"), "Definition not formatted: {result}");
    assert!(!result.contains("<dl>"), "DL tag should be removed: {result}");
}

#[test]
fn test_preprocess_definition_lists_multiple_terms() {
    let html = "<dl><dt>First</dt><dd>Def 1</dd><dt>Second</dt><dd>Def 2</dd></dl>";
    let result = preprocess_definition_lists(html);
    assert!(result.contains("**First**"));
    assert!(result.contains("**Second**"));
    assert!(result.contains(": Def 1"));
    assert!(result.contains(": Def 2"));
}

#[test]
fn test_preprocess_definition_lists_no_dl_passes_through() {
    let html = "<p>No definition lists here.</p>";
    let result = preprocess_definition_lists(html);
    assert_eq!(result, html);
}

// ─── Full pipeline integration tests ────────────────

#[test]
fn test_html_to_markdown_full_article_fragment() {
    let html = r#"
        <h2>Overview</h2>
        <p>This is an <strong>important</strong> article about
        <a href="/source/raii/">RAII</a>.</p>
        <figure>
            <img src="../media/test/diagram.svg" alt="Architecture diagram" />
            <figcaption>System architecture</figcaption>
        </figure>
        <h3>Details</h3>
        <ul>
            <li>First point</li>
            <li>Second point with <em>emphasis</em></li>
        </ul>
        <table>
            <tr><th>Method</th><th>Speed</th></tr>
            <tr><td>Approach A</td><td>Fast</td></tr>
        </table>
    "#;

    let md = html_to_markdown(html).unwrap();

    // Headings
    assert!(md.contains("## Overview"), "H2 missing: {md}");
    assert!(md.contains("### Details"), "H3 missing: {md}");

    // Inline formatting
    assert!(md.contains("**important**"), "Bold missing: {md}");
    assert!(md.contains("*emphasis*"), "Italic missing: {md}");

    // Link
    assert!(md.contains("[RAII](/source/raii/)"), "Link missing: {md}");

    // Image
    assert!(md.contains("../media/test/diagram.svg"), "Image missing: {md}");

    // List
    assert!(md.contains("First point"), "List item missing: {md}");

    // Table
    assert!(md.contains("Approach A"), "Table cell missing: {md}");
    assert!(md.contains('|'), "Table pipe syntax missing: {md}");

    // No raw HTML
    assert!(!md.contains("<p>"), "Raw <p> in output: {md}");
    assert!(!md.contains("<h2>"), "Raw <h2> in output: {md}");
    assert!(!md.contains("<strong>"), "Raw <strong> in output: {md}");
}
```

---

## Verification

### 7.1: Convert all articles in batch

```bash
cd /Users/oubiwann/lab/oxur/haleiki

# Convert all articles that have .final.html
cargo run --features demo -- convert
```

Expected: all articles with `.final.html` produce `.staging/{slug}.md`.

### 7.2: Spot-check quality across categories

```bash
# Check a Tibetan Buddhism article (may have Tibetan script)
head -30 demo/.staging/dzogchen.md

# Check a physics article (may have math, complex tables)
head -30 demo/.staging/quantum-mechanics.md

# Check a music article (standard biography structure)
head -30 demo/.staging/johann-sebastian-bach.md

# Check a geology article (may have data tables)
head -30 demo/.staging/earths-crust.md
```

### 7.3: No raw HTML in Markdown output

```bash
# Check for common HTML tags that should have been converted
for f in demo/.staging/*.md; do
    count=$(grep -cE '<(p|div|span|table|tr|td|th|ul|ol|li|h[1-6]|strong|em|a |img)[ >]' "$f" 2>/dev/null || echo 0)
    if [ "$count" -gt 0 ]; then
        echo "$(basename $f): $count raw HTML tags found"
    fi
done
```

### 7.4: Images have correct syntax

```bash
# All image references should use ![...](../media/...)
grep -n '!\[' demo/.staging/quantum-mechanics.md
# Should show ![alt text](../media/quantum-mechanics/filename.ext)
```

### 7.5: Pandoc comparison (if installed)

```bash
cargo run --features demo -- convert --pandoc
# Compare a few articles
diff demo/.staging/dzogchen.md demo/.staging/dzogchen.pandoc.md | head -30
```

### 7.6: Tests pass

```bash
cargo test --features demo
make lint
```

---

## Acceptance Criteria

- [ ] `html_to_markdown()` produces clean Markdown from real Wikipedia HTML
- [ ] `post_process()` applies all identified fixups from 5.1 evaluation
- [ ] `normalize_heading_levels()` shifts H1→H2+ when body contains H1
- [ ] `clean_converter_artifacts()` removes HTML comments, empty links, decodes entities
- [ ] `normalize_image_syntax()` fixes empty alt text, trims whitespace
- [ ] `fix_tables()` adds missing separator rows, preserves existing separators
- [ ] `normalize_links()` removes empty title attributes
- [ ] `preprocess_definition_lists()` converts `<dl>/<dt>/<dd>` before conversion
- [ ] `convert_all_articles()` batch-converts all `.final.html` files
- [ ] `reconvert_article()` force-reconverts a single article
- [ ] `--pandoc` flag shells out to pandoc as alternative backend
- [ ] No raw HTML tags in Markdown output (checked across all 90 articles)
- [ ] Unicode preserved (Tibetan script, diacritics, special characters)
- [ ] Tables render as valid GFM pipe tables
- [ ] All unit tests pass (20+ tests)
- [ ] `make lint` passes

---

## Gotchas

1. **Fixup list is provisional**: The exact fixups depend on what 5.1 discovers. The functions above cover the most likely issues. Add or remove fixups based on real evaluation data. Don't implement fixups for problems that don't actually occur.

2. **Pre-processing vs. post-processing**: Some problems are easier to fix in HTML (before conversion) and others in Markdown (after). Definition lists are best handled in HTML (pre-processing). Whitespace issues are best handled in Markdown (post-processing). Evaluate each fixup individually.

3. **`htmd` custom handlers**: If `htmd` supports custom element handlers, use them instead of pre/post-processing. They run during conversion with full DOM context, which is more reliable than string manipulation. Check the actual API during 5.1.

4. **Table edge cases**: Wikipedia tables can have colspan, rowspan, nested tables, and embedded HTML. GFM tables can't represent these. Accept that some complex tables will be lossy. The `--pandoc` flag handles these better.

5. **Overwriting staged Markdown**: `convert_all_articles()` skips articles that already have a `.staging/{slug}.md`. Use `reconvert_article()` to force-reconvert a specific article. For a full re-conversion, delete the `.md` files from staging first.

6. **Staging vs. final location**: The staged Markdown (`demo/.staging/{slug}.md`) does NOT have frontmatter. It is NOT the final source page. Milestone 5.3 adds frontmatter and writes to `demo/sources/{slug}.md`.

7. **HTML entities**: Some converters double-decode or fail to decode HTML entities. The `clean_converter_artifacts` function handles common entities. Be careful with `&amp;` — decode it last to avoid re-encoding `&` that appears in other entity decodings.

8. **Line length**: Wikipedia articles can have very long paragraphs that produce very long Markdown lines. Don't wrap them — `--wrap=none` is intentional. Cobalt/Markdown renderers handle wrapping at display time.

9. **Performance**: Converting 90 articles is fast (<1 second total for `htmd`). Pandoc is slower (~0.5s per article due to process spawning). Batch conversion with pandoc may take 30-60 seconds.

10. **Empty articles**: Some Wikipedia articles (especially stubs or disambiguation pages) may produce very short Markdown. This is expected — they'll still be valid source pages.
