//! Demo site build — assembles a Cobalt-ready working directory and runs the build.
//!
//! Creates `demo/_build/` with symlinked layouts, media, and processed content
//! (frontmatter injection + media path rewriting), then shells out to `cobalt build`.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

const BUILD_DIR: &str = "demo/_build";
const LAYOUTS_SRC: &str = "demo/_layouts";
const COBALT_YML: &str = "demo/_cobalt.yml";
const SOURCES_DIR: &str = "demo/sources";
const CONCEPTS_DIR: &str = "demo/concepts";

/// Entry point for `haleiki demo build`.
pub fn run() -> Result<()> {
    eprintln!("Building demo site...");

    clean_build_dir()?;
    setup_cobalt_config()?;
    setup_layouts()?;
    setup_media()?;
    setup_data()?;

    let source_count = process_sources()?;
    let concept_count = process_concepts()?;

    run_haleiki_build()?;

    generate_index(source_count, concept_count)?;
    run_cobalt_build()?;

    setup_site_media()?;

    eprintln!("Done. Site built to demo/_site/");
    eprintln!("To preview: cd demo/_build && cobalt serve");

    Ok(())
}

fn clean_build_dir() -> Result<()> {
    let path = Path::new(BUILD_DIR);
    if path.exists() {
        fs::remove_dir_all(path).context("failed to remove demo/_build/")?;
    }
    fs::create_dir_all(path).context("failed to create demo/_build/")?;
    Ok(())
}

fn setup_cobalt_config() -> Result<()> {
    fs::copy(COBALT_YML, format!("{BUILD_DIR}/_cobalt.yml"))
        .context("failed to copy _cobalt.yml")?;
    Ok(())
}

fn setup_layouts() -> Result<()> {
    let dest_dir = format!("{BUILD_DIR}/_layouts");
    fs::create_dir_all(&dest_dir)?;

    for entry in fs::read_dir(LAYOUTS_SRC).context("failed to read demo/_layouts/")? {
        let entry = entry?;
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();
        if filename_str.ends_with(".liquid") {
            let src = format!("{LAYOUTS_SRC}/{filename_str}");
            let dest = format!("{dest_dir}/{filename_str}");
            fs::copy(&src, &dest)
                .with_context(|| format!("failed to copy layout {filename_str}"))?;
        }
    }
    Ok(())
}

fn setup_media() -> Result<()> {
    let link = format!("{BUILD_DIR}/media");
    std::os::unix::fs::symlink("../media", &link).context("failed to symlink media")?;
    Ok(())
}

fn setup_data() -> Result<()> {
    let data_dir = format!("{BUILD_DIR}/_data");
    fs::create_dir_all(&data_dir)?;
    std::os::unix::fs::symlink("../../taxonomy.yaml", format!("{data_dir}/taxonomy.yaml"))
        .context("failed to symlink taxonomy.yaml")?;
    Ok(())
}

fn process_sources() -> Result<usize> {
    let src_dir = Path::new(SOURCES_DIR);
    let dest_dir = format!("{BUILD_DIR}/sources");
    fs::create_dir_all(&dest_dir)?;

    let mut count = 0;
    for entry in fs::read_dir(src_dir).context("failed to read demo/sources/")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let filename = entry.file_name();
        let slug = path.file_stem().unwrap_or_default().to_string_lossy();
        if slug == ".gitkeep" {
            continue;
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let permalink = format!("/source/{slug}/");
        let processed = inject_frontmatter(&content, "source.liquid", &permalink);
        let processed = rewrite_media_paths(&processed);

        fs::write(
            format!("{dest_dir}/{}", filename.to_string_lossy()),
            &processed,
        )?;
        count += 1;
    }

    eprintln!("  Processed {count} source pages");
    Ok(count)
}

fn process_concepts() -> Result<usize> {
    let src_dir = Path::new(CONCEPTS_DIR);
    let dest_dir = format!("{BUILD_DIR}/concepts");
    fs::create_dir_all(&dest_dir)?;

    let mut count = 0;
    for entry in fs::read_dir(src_dir).context("failed to read demo/concepts/")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let filename = entry.file_name();
        let slug = path.file_stem().unwrap_or_default().to_string_lossy();
        if slug == ".gitkeep" {
            continue;
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let permalink = format!("/{slug}/");
        let processed = inject_frontmatter(&content, "concept.liquid", &permalink);
        let processed = rewrite_media_paths(&processed);

        fs::write(
            format!("{dest_dir}/{}", filename.to_string_lossy()),
            &processed,
        )?;
        count += 1;
    }

    eprintln!("  Processed {count} concept cards");
    Ok(count)
}

fn generate_index(source_count: usize, concept_count: usize) -> Result<()> {
    let content = format!(
        "---\nlayout: default.liquid\ntitle: Home\npermalink: /\n---\n\n\
         # Haleiki Demo\n\n\
         A demonstration knowledge base with {source_count} source pages and {concept_count} concept cards.\n\n\
         ## Sample Source Pages\n\n\
         - [Dzogchen](/source/dzogchen/) — Tibetan Buddhism\n\
         - [Quantum Mechanics](/source/quantum-mechanics/) — Theoretical Physics\n\
         - [Rust (programming language)](/source/rust-language/) — Programming Languages\n\
         - [Padmasambhava](/source/padmasambhava/) — Tibetan Buddhism\n\
         - [Johann Sebastian Bach](/source/johann-sebastian-bach/) — Music\n\n\
         ## Sample Concept Cards\n\n\
         - [Ownership (Rust)](/ownership-rust/) — Programming Languages\n\
         - [Uncertainty Principle](/uncertainty-principle/) — Physics\n\
         - [Nyingma](/nyingma/) — Tibetan Buddhism\n\
         - [Counterpoint](/counterpoint/) — Music\n\
         - [Plate Tectonics](/plate-tectonics/) — Geology\n"
    );

    fs::write(format!("{BUILD_DIR}/index.md"), content)?;
    Ok(())
}

fn setup_site_media() -> Result<()> {
    let site_media = "demo/_site/media";
    if !Path::new(site_media).exists() {
        std::os::unix::fs::symlink("../media", site_media)
            .context("failed to symlink media into _site/")?;
    }
    Ok(())
}

fn run_haleiki_build() -> Result<()> {
    eprintln!("  Running haleiki build (graph + data)...");
    let content_dir = Path::new("demo");
    let output_dir = Path::new("demo/_build/_data");
    let corpus = crate::parser::parse_corpus(content_dir)?;
    let report = crate::validator::validate(&corpus, content_dir);
    if report.has_errors() {
        report.print();
        bail!("Demo content validation failed.");
    }
    if !report.warnings.is_empty() {
        report.print();
    }
    let graph = crate::graph::HaleikiGraph::build(&corpus);
    crate::generator::generate(&corpus, &graph, output_dir)?;
    let stats = graph.stats(&corpus);
    eprintln!(
        "  Generated data: {} nodes, {} edges, 6 JSON files",
        stats.total_nodes, stats.total_edges
    );
    Ok(())
}

fn run_cobalt_build() -> Result<()> {
    eprintln!("  Running cobalt build...");
    let output = Command::new("cobalt")
        .arg("build")
        .current_dir(BUILD_DIR)
        .output()
        .context("failed to run 'cobalt build' — is cobalt-bin installed? (cargo install cobalt-bin --locked)")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("cobalt build failed:\n{stderr}");
    }

    Ok(())
}

/// Transform frontmatter for Cobalt compatibility.
///
/// Cobalt only accepts known fields at the top level (title, layout, permalink,
/// slug, description, tags, etc.). All other fields must go under `data:`.
/// This function extracts the title, injects layout/permalink, and wraps
/// remaining fields under `data:`.
fn inject_frontmatter(content: &str, layout: &str, permalink: &str) -> String {
    let Some(first_fence) = content.find("---\n") else {
        return content.to_string();
    };
    let after_first = first_fence + 4;

    let Some(second_fence) = content[after_first..].find("\n---") else {
        return content.to_string();
    };
    let fm_content = &content[after_first..after_first + second_fence];
    let body = &content[after_first + second_fence + 4..];

    // Extract title from original frontmatter
    let title = fm_content
        .lines()
        .find(|l| l.starts_with("title:"))
        .map(|l| {
            l.trim_start_matches("title:")
                .trim()
                .trim_matches('"')
                .to_string()
        })
        .unwrap_or_default();

    // For concept cards: fall back to concept + disambiguator if no title field
    let title = if title.is_empty() && layout == "concept.liquid" {
        let concept = fm_content
            .lines()
            .find(|l| l.starts_with("concept:"))
            .map(|l| {
                l.trim_start_matches("concept:")
                    .trim()
                    .trim_matches('"')
                    .to_string()
            })
            .unwrap_or_default();
        let disambiguator = fm_content
            .lines()
            .find(|l| l.starts_with("disambiguator:"))
            .map(|l| {
                l.trim_start_matches("disambiguator:")
                    .trim()
                    .trim_matches('"')
                    .to_string()
            })
            .filter(|d| !d.is_empty());
        match disambiguator {
            Some(d) => format!("{concept} ({d})"),
            None => concept,
        }
    } else {
        title
    };

    // Build Cobalt-compatible frontmatter:
    // - Known Cobalt fields at top level
    // - Everything else indented under `data:`
    let mut new_fm = String::new();
    let _ = writeln!(new_fm, "layout: {layout}");
    let _ = writeln!(new_fm, "permalink: \"{permalink}\"");
    let _ = writeln!(new_fm, "title: {title}");
    new_fm.push_str("data:\n");

    for line in fm_content.lines() {
        // Skip title (already promoted) and blank lines at start
        if line.starts_with("title:") {
            continue;
        }
        // Skip YAML comment-only section headers for cleanliness
        if line.starts_with("# ===") {
            continue;
        }
        if line.is_empty() {
            continue;
        }
        // Indent everything else under data:
        new_fm.push_str("  ");
        new_fm.push_str(line);
        new_fm.push('\n');
    }

    // Wrap body in {% raw %} to prevent Liquid from parsing {{ }} in content
    format!("---\n{new_fm}---\n{{% raw %}}{body}{{% endraw %}}")
}

/// Rewrite relative media paths to absolute for correct resolution after permalink routing.
fn rewrite_media_paths(content: &str) -> String {
    content.replace("](../media/", "](/media/")
}
