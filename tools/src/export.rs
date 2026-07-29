//! Static export writer — Arc 02's capstone, refactored in Arc 03 slice 01
//! onto the shared `server::render_routes` walk ("one renderer, two sinks").
//! Writes the rendered route map to a complete `_site/` of static HTML, copies
//! the site asset tree, then (Arc 04 slice 01) builds the Pagefind search
//! index into `_site/pagefind/`. The pivot's default deliverable.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::server::{self, ASSET_ALLOWLIST};

/// Parses, validates, and renders the full site from `content_dir` to
/// `out_dir` as static HTML (via `templates_dir`, through
/// [`server::render_routes`]), copies the asset allowlist from `assets_dir`
/// into `out_dir/assets/`, then builds the Pagefind search index into
/// `out_dir/pagefind/` ([`crate::search::build_index`]).
///
/// Cleans `out_dir` first (`remove_dir_all`, if present) so a stale file
/// from a prior export cannot linger. `render_routes`' route iteration order
/// is sorted (slugs, category names) and its `pages` map is a `BTreeMap`, and
/// `search::build_records` sorts the same way, so repeated exports of the
/// same corpus are byte-stable.
///
/// # Errors
///
/// Returns an error if the corpus fails to parse or validate, the profile
/// fails to resolve, a template fails to render, a filesystem operation
/// fails, or the search index fails to build.
pub fn export_site(
    content_dir: &Path,
    templates_dir: &Path,
    assets_dir: &Path,
    out_dir: &Path,
) -> Result<()> {
    let routes = server::render_routes(content_dir, templates_dir)?;

    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir)
            .with_context(|| format!("failed to clean {}", out_dir.display()))?;
    }

    for (url_path, html) in &routes.pages {
        write_route(out_dir, url_path, html)?;
    }
    write_404(out_dir, &routes.not_found)?;

    copy_assets(assets_dir, &out_dir.join("assets"))?;

    crate::search::build_index(content_dir, &out_dir.join("pagefind"))
        .context("failed to build the pagefind search index")?;

    Ok(())
}

/// Writes `html` to the file a URL path maps to: `out_dir/index.html` for
/// `"/"`, `out_dir/{trimmed}/index.html` otherwise (e.g. `"/browse/"` →
/// `out_dir/browse/index.html`). Creates parent directories as needed.
fn write_route(out_dir: &Path, url_path: &str, html: &str) -> Result<()> {
    let trimmed = url_path.trim_matches('/');
    let dir: PathBuf = if trimmed.is_empty() {
        out_dir.to_path_buf()
    } else {
        out_dir.join(trimmed)
    };
    std::fs::create_dir_all(&dir).with_context(|| format!("failed to create {}", dir.display()))?;
    let path = dir.join("index.html");
    std::fs::write(&path, html).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

/// Writes the 404 page to `out_dir/404.html` — not `404/index.html`, the
/// path static hosts look for.
fn write_404(out_dir: &Path, html: &str) -> Result<()> {
    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;
    let path = out_dir.join("404.html");
    std::fs::write(&path, html).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

/// Copies the allowlisted asset subdirectories from `assets_dir` into
/// `dest`. A missing allowlisted subdir is skipped, not an error — a
/// deployment profile might not need every asset kind (e.g. no `vendor/`
/// `KaTeX` if the corpus has no math).
fn copy_assets(assets_dir: &Path, dest: &Path) -> Result<()> {
    for name in ASSET_ALLOWLIST {
        let src = assets_dir.join(name);
        if !src.exists() {
            continue;
        }
        copy_dir_recursive(&src, &dest.join(name))?;
    }
    Ok(())
}

/// Recursively copies `src` into `dest`. Symlinks are skipped (none are
/// expected under `assets/`; skipping avoids following one somewhere
/// unexpected).
fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    std::fs::create_dir_all(dest)
        .with_context(|| format!("failed to create {}", dest.display()))?;
    for entry in
        std::fs::read_dir(src).with_context(|| format!("failed to read {}", src.display()))?
    {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dest.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dest_path)?;
        } else if file_type.is_file() {
            std::fs::copy(entry.path(), &dest_path).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    entry.path().display(),
                    dest_path.display()
                )
            })?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn demo_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo")
    }

    fn templates_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../templates")
    }

    fn assets_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets")
    }

    /// A fresh temp directory for one test's export output, named after the
    /// caller so parallel `cargo test` runs never collide.
    fn temp_out_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("haleiki-export-test-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    fn export_demo(out_dir: &Path) {
        export_site(&demo_dir(), &templates_dir(), &assets_dir(), out_dir)
            .expect("demo corpus exports without error");
    }

    /// Every file under `dir`, relative to `root`, sorted.
    fn walk_files(dir: &Path, root: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                walk_files(&path, root, out);
            } else {
                out.push(path.strip_prefix(root).unwrap().to_path_buf());
            }
        }
    }

    fn relative_files(root: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        walk_files(root, root, &mut files);
        files.sort();
        files
    }

    // F-1: export_site writes a complete _site/ from the demo corpus: every
    // route at its contract path, with counts matching the real corpus.
    #[test]
    fn test_export_site_writes_complete_tree_with_correct_counts() {
        let corpus = crate::parser::parse_corpus(&demo_dir()).expect("demo corpus parses");
        let out = temp_out_dir("complete-tree");
        export_demo(&out);

        for route in ["", "browse", "about", "attribution", "search"] {
            let path = if route.is_empty() {
                out.join("index.html")
            } else {
                out.join(route).join("index.html")
            };
            assert!(path.exists(), "{} should exist", path.display());
        }
        assert!(out.join("404.html").exists());
        assert!(!out.join("404").join("index.html").exists());

        let graph = crate::graph::HaleikiGraph::build(&corpus);
        let category_index = graph.category_index(&corpus);
        let category_dirs: Vec<_> = std::fs::read_dir(out.join("category"))
            .expect("category dir exists")
            .filter_map(std::result::Result::ok)
            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
            .collect();
        assert_eq!(category_dirs.len(), category_index.len());
        for name in category_index.keys() {
            assert!(
                out.join("category").join(name).join("index.html").exists(),
                "category/{name}/index.html should exist"
            );
        }

        for slug in corpus.concepts.keys() {
            assert!(
                out.join(slug).join("index.html").exists(),
                "{slug}/index.html should exist"
            );
        }
        for slug in corpus.sources.keys() {
            assert!(
                out.join("source").join(slug).join("index.html").exists(),
                "source/{slug}/index.html should exist"
            );
        }

        let source_dirs: Vec<_> = std::fs::read_dir(out.join("source"))
            .expect("source dir exists")
            .filter_map(std::result::Result::ok)
            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
            .collect();
        assert_eq!(source_dirs.len(), corpus.sources.len());

        let _ = std::fs::remove_dir_all(&out);
    }

    // F-2: asset allowlist copied; ai/mockups excluded.
    #[test]
    fn test_export_site_copies_asset_allowlist_only() {
        let out = temp_out_dir("asset-allowlist");
        export_demo(&out);

        for name in ASSET_ALLOWLIST {
            assert!(
                out.join("assets").join(name).exists(),
                "assets/{name} should be copied"
            );
        }
        assert!(!out.join("assets").join("ai").exists());
        assert!(!out.join("assets").join("mockups").exists());

        // Spot-check actual file content made it through, not just empty dirs.
        assert!(out.join("assets/css/haleiki.css").exists());
        assert!(
            std::fs::read_dir(out.join("assets/fonts"))
                .expect("fonts dir readable")
                .count()
                >= 8
        );

        let _ = std::fs::remove_dir_all(&out);
    }

    // F-3: clean-before-write — a stale file planted before a re-export is
    // gone afterward.
    #[test]
    fn test_export_site_cleans_before_write() {
        let out = temp_out_dir("clean-before-write");
        std::fs::create_dir_all(&out).unwrap();
        let stale = out.join("stale-from-a-previous-build.txt");
        std::fs::write(&stale, "stale").unwrap();
        assert!(stale.exists());

        export_demo(&out);

        assert!(
            !stale.exists(),
            "a stale pre-existing file must not survive re-export"
        );
        assert!(out.join("index.html").exists());

        let _ = std::fs::remove_dir_all(&out);
    }

    // F-4: internal links resolve within _site/ — home -> a category page,
    // a category -> an article, a concept's breadcrumb/see-also targets.
    #[test]
    fn test_export_site_internal_links_resolve_to_real_files() {
        // Several templates print a whole href from a single interpolated
        // variable (e.g. breadcrumb.jinja's `href="{{ crumb.url }}"`, unlike
        // the partially-literal `/category/{{ cat.slug }}/` pattern), and
        // minijinja's attribute auto-escaping turns every "/" in a fully
        // interpolated value into "&#x2f;" — hit this same thing rendering
        // attribution's license links in slice 03. Check both forms.
        fn contains_href(html: &str, path: &str) -> bool {
            let literal = format!("href=\"{path}\"");
            let escaped = format!("href=\"{}\"", path.replace('/', "&#x2f;"));
            html.contains(&literal) || html.contains(&escaped)
        }

        let out = temp_out_dir("link-resolution");
        export_demo(&out);

        // Home links to /category/mathematics/.
        let home_html = std::fs::read_to_string(out.join("index.html")).unwrap();
        assert!(contains_href(&home_html, "/category/mathematics/"));
        assert!(out.join("category/mathematics/index.html").exists());

        // The mathematics category page links to real articles (its
        // tier-grouped articles include sources); resolve every /source/
        // reference (literal or escaped) it contains to a real written file.
        let category_html =
            std::fs::read_to_string(out.join("category/mathematics/index.html")).unwrap();
        let mut resolved_any = false;
        for candidate in [category_html.as_str()] {
            for marker in ["href=\"/source/", "href=\"&#x2f;source&#x2f;"] {
                let mut rest = candidate;
                while let Some(start) = rest.find(marker) {
                    let after = &rest[start + marker.len()..];
                    let end = after.find('"').expect("href has a closing quote");
                    let raw_slug = &after[..end];
                    let slug = raw_slug.trim_end_matches('/').trim_end_matches("&#x2f;");
                    assert!(
                        out.join("source").join(slug).join("index.html").exists(),
                        "linked source {slug} should exist"
                    );
                    resolved_any = true;
                    rest = &after[end..];
                }
            }
        }
        assert!(
            resolved_any,
            "expected at least one /source/ link on the mathematics category page"
        );

        // abelian-group's breadcrumb (-> /category/mathematics/) and
        // see-also targets (-> /source/group-mathematics/, /ring-mathematics/)
        // all exist as real files.
        let concept_html = std::fs::read_to_string(out.join("abelian-group/index.html")).unwrap();
        assert!(contains_href(&concept_html, "/category/mathematics/"));
        assert!(contains_href(&concept_html, "/source/group-mathematics/"));
        assert!(out.join("source/group-mathematics/index.html").exists());
        assert!(contains_href(&concept_html, "/ring-mathematics/"));
        assert!(out.join("ring-mathematics/index.html").exists());

        let _ = std::fs::remove_dir_all(&out);
    }

    // F-5: written pages match the renderers — spot-check chrome on a
    // written concept page and the home page.
    #[test]
    fn test_export_site_written_pages_match_renderers() {
        let out = temp_out_dir("chrome-spotcheck");
        export_demo(&out);

        let home_html = std::fs::read_to_string(out.join("index.html")).unwrap();
        assert!(home_html.contains("class=\"hero-title\""));
        assert!(home_html.contains("class=\"category-grid\""));
        assert!(home_html.contains("class=\"featured-grid\""));

        let concept_html = std::fs::read_to_string(out.join("abelian-group/index.html")).unwrap();
        assert!(concept_html.contains("trust-bar trust-bar--credible"));
        assert!(concept_html.contains("Abelian Group"));
        assert!(concept_html.contains(r#"data-math-style="inline""#));
        assert!(concept_html.contains("id=\"see-also\""));

        let source_html =
            std::fs::read_to_string(out.join("source/vimalamitra/index.html")).unwrap();
        assert!(source_html.contains("Vimalamitra"));
        assert!(source_html.contains("class=\"prose\""));

        let _ = std::fs::remove_dir_all(&out);
    }

    // F-7 (determinism half): re-exporting the same corpus produces the same
    // file tree and byte-identical content, outside `pagefind/`.
    //
    // The pagefind bundle's `filter/*.pf_filter` and `pagefind.*.pf_meta`
    // filenames embed a content hash whose internal byte order is not yet
    // stable across separate process runs, upstream, in pagefind 1.5.2
    // itself (its `src/index/mod.rs` carries its own "TODO: Change filter
    // indexes to BTree to give them a stable hash" — our own record builder
    // is already fully sorted/deterministic; this is not our code). Verified
    // by hand: two real `haleiki export` runs against the demo produced
    // identical file trees except for exactly 2 `.pf_filter` files and 1
    // `.pf_meta` file (of ~330 total files) — every rendered HTML page and
    // the rest of the pagefind bundle (JS/wasm entry, fragments) is
    // unaffected. `pagefind/` presence and file-kind shape is G-5's job.
    #[test]
    fn test_export_site_is_deterministic_across_repeated_exports() {
        let out_a = temp_out_dir("determinism-a");
        let out_b = temp_out_dir("determinism-b");
        export_demo(&out_a);
        export_demo(&out_b);

        let files_a: Vec<PathBuf> = relative_files(&out_a)
            .into_iter()
            .filter(|p| !p.starts_with("pagefind"))
            .collect();
        let files_b: Vec<PathBuf> = relative_files(&out_b)
            .into_iter()
            .filter(|p| !p.starts_with("pagefind"))
            .collect();
        assert_eq!(
            files_a, files_b,
            "two exports must produce the same file tree outside pagefind/"
        );
        assert!(!files_a.is_empty());

        for rel in &files_a {
            let bytes_a = std::fs::read(out_a.join(rel)).unwrap();
            let bytes_b = std::fs::read(out_b.join(rel)).unwrap();
            assert_eq!(
                bytes_a,
                bytes_b,
                "{} must be byte-identical across exports",
                rel.display()
            );
        }

        // The pagefind bundle exists in both exports (file-kind assertions
        // live in the search-index tests; here only presence matters).
        assert!(out_a.join("pagefind").join("pagefind.js").exists());
        assert!(out_b.join("pagefind").join("pagefind.js").exists());

        let _ = std::fs::remove_dir_all(&out_a);
        let _ = std::fs::remove_dir_all(&out_b);
    }

    // F-7 (no-Liquid half): the export path references no Liquid template.
    // Needle built by concatenation, not a literal — include_str! pulls in
    // this test's own source, and a literal anywhere in this function would
    // make the check find itself (the standing lesson since Arc 01 slice 02).
    #[test]
    fn test_export_path_references_no_liquid_templates() {
        let liquid_extension = format!(".{}{}", "liqu", "id");
        let export_src = include_str!("export.rs");

        assert!(
            !export_src.contains(&liquid_extension),
            "export.rs must not reference any Liquid template"
        );
    }
}
