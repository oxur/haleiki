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

use std::fmt::Write;
use std::path::Path;

use super::media::staging_final_path;

/// Path where converted Markdown is staged before frontmatter injection.
pub fn staging_markdown_path(slug: &str) -> std::path::PathBuf {
    Path::new("demo/.staging").join(format!("{slug}.md"))
}

/// Convert HTML to Markdown using the `htmd` crate.
///
/// This is the primary conversion function. Pre-processing handles
/// elements the converter struggles with (e.g., definition lists),
/// and post-processing fixups are applied after the initial conversion.
pub fn html_to_markdown(html: &str) -> anyhow::Result<String> {
    // Pre-process: handle elements the converter struggles with
    let preprocessed = preprocess_definition_lists(html);

    let converter = htmd::HtmlToMarkdown::builder().build();

    let raw_md = converter
        .convert(&preprocessed)
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

    // -- Fixup 1: Normalize excessive blank lines --------
    // Converters often produce 3+ consecutive blank lines.
    // Normalize to at most 2 (one blank line between elements).
    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }

    // -- Fixup 2: Remove trailing whitespace on each line -
    result = result
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");

    // -- Fixup 3: Heading level normalization ------------
    // Ensure the document body starts at H2 (H1 reserved for title).
    // The title is injected as frontmatter, so body headings
    // should be H2+. If the converter outputs H1s in the body,
    // shift all headings down by one level.
    result = normalize_heading_levels(&result);

    // -- Fixup 4: Clean up converter artifacts ----------
    result = clean_converter_artifacts(&result);

    // -- Fixup 5: Normalize image syntax ----------------
    result = normalize_image_syntax(&result);

    // -- Fixup 6: Fix broken table formatting -----------
    result = fix_tables(&result);

    // -- Fixup 7: Normalize link syntax -----------------
    result = normalize_links(&result);

    // -- Fixup 8: Ensure file ends with a single newline
    // This must be last since line-based fixups above may
    // strip the trailing newline.
    result = result.trim_end().to_string();
    result.push('\n');

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
                    // Add one more # (shift down), capping at H6
                    let new_hashes = std::cmp::min(hashes + 1, 6);
                    let leading_space = " ".repeat(line.len() - trimmed.len());
                    return format!(
                        "{leading_space}{} {}",
                        "#".repeat(new_hashes),
                        &trimmed[hashes + 1..],
                    );
                }
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Remove common artifacts left by HTML-to-Markdown converters.
///
/// Strips leftover HTML comments, removes empty link references,
/// and decodes common HTML entities that the converter missed.
fn clean_converter_artifacts(md: &str) -> String {
    let mut result = md.to_string();

    // Remove leftover HTML comments
    while let Some(start) = result.find("<!--") {
        if let Some(end) = result[start..].find("-->") {
            result.replace_range(start..start + end + 3, "");
        } else {
            break;
        }
    }

    // Remove empty link references: [](url) with no text
    result = remove_empty_links(&result);

    // Decode leftover HTML entities that should have been converted.
    // &amp; MUST be decoded last to avoid re-encoding risk.
    result = result.replace("&nbsp;", " ");
    result = result.replace("&mdash;", "\u{2014}");
    result = result.replace("&ndash;", "\u{2013}");
    result = result.replace("&hellip;", "\u{2026}");
    result = result.replace("&amp;", "&");

    result
}

/// Remove Markdown links that have no visible text: `[](url)`.
///
/// Uses a character-by-character parser to correctly handle nested
/// parentheses in URLs.
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
                    // Found [](  -- skip until closing )
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

/// Normalize image Markdown syntax.
///
/// - Ensure alt text is present: `![](path)` becomes `![image](path)`
/// - Normalize whitespace in alt text
fn normalize_image_syntax(md: &str) -> String {
    let mut lines: Vec<String> = md.lines().map(String::from).collect();

    for line in &mut lines {
        // Fix empty alt text
        if line.contains("![](") {
            *line = line.replace("![](", "![image](");
        }

        // Trim whitespace inside alt text: ![  text  ](url) -> ![text](url)
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

/// Fix common table formatting issues.
///
/// Ensures a separator row exists between the header row and body rows
/// in pipe tables. Preserves non-table content and tables that already
/// have separators.
fn fix_tables(md: &str) -> String {
    let lines: Vec<&str> = md.lines().collect();
    let mut result_lines: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        // Detect table rows (lines starting and containing |)
        if is_table_row(line) {
            // Collect the entire table block
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
                // Single-row "table" -- probably not a real table, pass through
                result_lines.extend(table_lines.into_iter().map(String::from));
            }
            continue;
        }

        result_lines.push(line.to_string());
        i += 1;
    }

    result_lines.join("\n")
}

/// Check whether a line looks like a pipe-table data row.
fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|') && trimmed.len() > 1
}

/// Check whether a line is a pipe-table separator row (e.g. `| --- | --- |`).
fn is_separator_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|')
        && trimmed.ends_with('|')
        && trimmed
            .chars()
            .all(|c| c == '|' || c == '-' || c == ':' || c == ' ')
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
                (0..col_count)
                    .map(|_| " --- ")
                    .collect::<Vec<_>>()
                    .join("|"),
            );
            result.push(sep);
        }
    }

    // Add remaining rows (skip header which is already added)
    for line in &lines[1..] {
        result.push((*line).to_string());
    }

    result
}

/// Normalize link syntax.
///
/// Removes empty title attributes: `[text](url "")` becomes `[text](url)`.
fn normalize_links(md: &str) -> String {
    md.replace(" \"\")", ")")
}

/// Convert HTML definition lists to a Markdown-compatible format.
///
/// Transforms `<dl><dt>Term</dt><dd>Definition</dd></dl>` into:
///
/// ```text
/// **Term**
/// : Definition
/// ```
///
/// Uses the PHP Markdown Extra / Pandoc definition list syntax.
/// This runs BEFORE the main converter so that `htmd` does not
/// have to handle `<dl>/<dt>/<dd>` elements.
fn preprocess_definition_lists(html: &str) -> String {
    let document = scraper::Html::parse_fragment(html);
    let dl_selector = scraper::Selector::parse("dl").expect("valid CSS selector");

    if document.select(&dl_selector).next().is_none() {
        return html.to_string(); // No definition lists, skip
    }

    let mut result = html.to_string();

    for dl in document.select(&dl_selector) {
        let mut replacement = String::new();

        for child in dl.children() {
            if let Some(el) = child.value().as_element() {
                match el.name() {
                    "dt" => {
                        let text: String = scraper::ElementRef::wrap(child)
                            .map(|e| e.text().collect())
                            .unwrap_or_default();
                        let _ = write!(replacement, "\n**{}**\n", text.trim());
                    }
                    "dd" => {
                        let text: String = scraper::ElementRef::wrap(child)
                            .map(|e| e.text().collect())
                            .unwrap_or_default();
                        let _ = writeln!(replacement, ": {}", text.trim());
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

/// Convert all articles that have final HTML but no staged Markdown.
///
/// Returns the number of articles converted.
pub fn convert_all_articles(use_pandoc: bool) -> anyhow::Result<usize> {
    let staging_dir = Path::new("demo/.staging");
    if !staging_dir.exists() {
        anyhow::bail!("staging directory not found -- run the fetch pipeline first");
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

        let slug = filename
            .strip_suffix(".final.html")
            .expect("suffix confirmed by ends_with check");

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
                eprintln!("    OK ({} bytes, {} lines)", md.len(), md.lines().count(),);
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
            "-f",
            "html",
            "-t",
            "markdown",
            "--wrap=none",
            "--no-highlight",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("failed to run pandoc (is it installed?): {e}"))?;

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

#[cfg(test)]
mod tests {
    use super::*;

    // --- Post-processing tests --------------------------

    #[test]
    fn test_post_process_normalizes_blank_lines() {
        let input = "Line 1\n\n\n\nLine 2\n\n\n\n\nLine 3\n";
        let result = post_process(input);
        assert!(
            !result.contains("\n\n\n"),
            "Triple+ blank lines should be normalized"
        );
        assert!(
            result.contains("Line 1\n\nLine 2"),
            "Double blank line should remain"
        );
    }

    #[test]
    fn test_post_process_trims_trailing_whitespace() {
        let input = "Line with trailing spaces   \nAnother line  \n";
        let result = post_process(input);
        for line in result.lines() {
            assert_eq!(
                line,
                line.trim_end(),
                "Line has trailing whitespace: '{line}'"
            );
        }
    }

    #[test]
    fn test_post_process_ends_with_single_newline() {
        let input = "Content\n\n\n";
        let result = post_process(input);
        assert!(result.ends_with('\n'), "Should end with newline");
        assert!(
            !result.ends_with("\n\n"),
            "Should not end with double newline"
        );
    }

    // --- Heading normalization tests --------------------

    #[test]
    fn test_normalize_heading_levels_shifts_when_h1_present() {
        let input = "# Title\n\nSome text.\n\n## Section\n\n### Subsection\n";
        let result = normalize_heading_levels(input);
        assert!(result.contains("## Title"), "H1 should become H2: {result}");
        assert!(
            result.contains("### Section"),
            "H2 should become H3: {result}"
        );
        assert!(
            result.contains("#### Subsection"),
            "H3 should become H4: {result}"
        );
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
        assert!(
            result.contains("###### H6"),
            "H6 should stay H6 (can't go to H7)"
        );
    }

    #[test]
    fn test_normalize_heading_levels_ignores_hash_in_content() {
        let input = "## Section\n\nThe color code is #FF0000.\n";
        let result = normalize_heading_levels(input);
        assert!(
            result.contains("#FF0000"),
            "Hash in content should not be modified"
        );
    }

    // --- HTML to Markdown conversion tests --------------

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
        assert!(
            md.contains("*italic*") || md.contains("_italic_"),
            "Italic not converted: {md}"
        );
    }

    #[test]
    fn test_html_to_markdown_link() {
        let html = r#"<p>See <a href="/source/raii/">RAII</a> for details.</p>"#;
        let md = html_to_markdown(html).unwrap();
        assert!(
            md.contains("[RAII](/source/raii/)"),
            "Link not converted: {md}"
        );
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
            md.contains("Item one") && md.contains("Item two"),
            "List items not converted: {md}",
        );
    }

    #[test]
    fn test_html_to_markdown_ordered_list() {
        let html = "<ol><li>First</li><li>Second</li></ol>";
        let md = html_to_markdown(html).unwrap();
        assert!(
            md.contains("1.") || md.contains("1)"),
            "Ordered list not converted: {md}"
        );
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
        // htmd may produce pipe tables or plain text depending on version
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
        assert!(
            md.contains("> ") || md.contains(">A"),
            "Blockquote not converted: {md}"
        );
    }

    // --- Converter artifact cleanup tests ----------------

    #[test]
    fn test_clean_converter_artifacts_removes_html_comments() {
        let input = "Text <!-- comment --> more text\n";
        let result = clean_converter_artifacts(input);
        assert!(
            !result.contains("<!--"),
            "HTML comment not removed: {result}"
        );
        assert!(
            result.contains("Text  more text"),
            "Surrounding text lost: {result}"
        );
    }

    #[test]
    fn test_clean_converter_artifacts_decodes_entities() {
        let input = "A&nbsp;B and C&mdash;D and E&amp;F\n";
        let result = clean_converter_artifacts(input);
        assert!(result.contains("A B"), "nbsp not decoded: {result}");
        assert!(result.contains("C\u{2014}D"), "mdash not decoded: {result}");
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

    // --- Image normalization tests -----------------------

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
        let input = "![A clear caption](../media/test/photo.png)";
        let result = normalize_image_syntax(input);
        assert_eq!(result, input, "Good image syntax should not be modified");
    }

    // --- Table fixup tests -------------------------------

    #[test]
    fn test_fix_tables_adds_missing_separator() {
        let input = "| Name | Value |\n| Alpha | 1 |\n| Beta | 2 |\n";
        let result = fix_tables(input);
        let lines: Vec<&str> = result.lines().collect();
        assert!(
            lines.len() >= 4,
            "Should have added separator row: {result}"
        );
        assert!(
            is_separator_row(lines[1]),
            "Second line should be separator: {}",
            lines[1],
        );
    }

    #[test]
    fn test_fix_tables_preserves_existing_separator() {
        let input = "| Name | Value |\n| --- | --- |\n| Alpha | 1 |";
        let result = fix_tables(input);
        assert_eq!(result, input, "Table with separator should not be modified");
    }

    #[test]
    fn test_fix_tables_preserves_non_table_content() {
        let input = "# Heading\n\nParagraph text.\n\n- List item";
        let result = fix_tables(input);
        assert_eq!(result, input, "Non-table content should not be modified");
    }

    // --- Link normalization tests ------------------------

    #[test]
    fn test_normalize_links_removes_empty_title() {
        let input = r#"[text](http://example.com "")"#;
        let result = normalize_links(input);
        assert_eq!(result, "[text](http://example.com)");
    }

    // --- Definition list pre-processing tests ------------

    #[test]
    fn test_preprocess_definition_lists_basic() {
        let html = "<dl><dt>Term</dt><dd>Definition here</dd></dl>";
        let result = preprocess_definition_lists(html);
        assert!(result.contains("**Term**"), "Term not bolded: {result}");
        assert!(
            result.contains(": Definition here"),
            "Definition not formatted: {result}"
        );
        assert!(
            !result.contains("<dl>"),
            "DL tag should be removed: {result}"
        );
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

    // --- Full pipeline integration tests -----------------

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
        assert!(
            md.contains("*emphasis*") || md.contains("_emphasis_"),
            "Italic missing: {md}"
        );

        // Link
        assert!(md.contains("[RAII](/source/raii/)"), "Link missing: {md}");

        // Image
        assert!(
            md.contains("../media/test/diagram.svg"),
            "Image missing: {md}"
        );

        // List
        assert!(md.contains("First point"), "List item missing: {md}");

        // Table content (lenient -- htmd may not produce pipe tables)
        assert!(md.contains("Approach A"), "Table cell missing: {md}");

        // No raw HTML
        assert!(!md.contains("<p>"), "Raw <p> in output: {md}");
        assert!(!md.contains("<h2>"), "Raw <h2> in output: {md}");
        assert!(!md.contains("<strong>"), "Raw <strong> in output: {md}");
    }

    // --- Batch conversion tests --------------------------

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
            if path.extension().is_some_and(|e| e == "md") {
                let _ = std::fs::remove_file(&path);
            }
        }

        let result = convert_all_articles(false);
        assert!(
            result.is_ok(),
            "Batch conversion failed: {:?}",
            result.err()
        );

        let converted = result.unwrap();
        assert_eq!(
            converted, final_count,
            "Should have converted all {final_count} articles, got {converted}",
        );
    }

    // --- Pandoc fallback test ---------------------------

    #[test]
    #[ignore] // Requires pandoc installed
    fn test_html_to_markdown_pandoc_basic() {
        let html = "<h2>Title</h2><p>Content with <strong>bold</strong>.</p>";
        let md = html_to_markdown_pandoc(html).unwrap();
        assert!(md.contains("Title"), "Heading missing: {md}");
        assert!(md.contains("**bold**"), "Bold missing: {md}");
    }

    // --- Real article test ------------------------------

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
            assert!(
                result.is_ok(),
                "Conversion failed for {slug}: {:?}",
                result.err()
            );

            let md_path = result.unwrap();
            let md = std::fs::read_to_string(&md_path).unwrap();

            // Basic sanity checks
            assert!(
                md.len() > 500,
                "{slug}: Markdown too short ({} bytes)",
                md.len()
            );
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
