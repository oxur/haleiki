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
    let converter = htmd::HtmlToMarkdown::builder().build();

    let raw_md = converter
        .convert(html)
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

    // -- Fixup 3: Ensure file ends with a single newline --
    result = result.trim_end().to_string();
    result.push('\n');

    // -- Fixup 4: Normalize image syntax -----------------
    // Some converters produce `![](path)` without alt text
    // when the alt was empty. This is technically valid but
    // we prefer `![image](path)` for accessibility.
    // (Evaluate whether this is actually needed with htmd)

    // -- Fixup 5: Heading level normalization ------------
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
