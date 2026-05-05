# M1 Implementation Plan: Cobalt Configuration + Minimal Rendering

## Context

The Haleiki demo corpus is complete (91 source pages, 186 concept cards, ~1900 images) but has no rendering pipeline. M1 is the first milestone of the site build: wire Cobalt to produce navigable HTML from the markdown content. No styling, no graph data — just the bones proving the toolchain works end-to-end.

## Architecture Decision: Build Directory

Create `demo/_build/` as an ephemeral working directory that assembles everything Cobalt needs (config, layouts, processed content, media). The `haleiki demo build` command creates this directory fresh each run, then shells out to `cobalt build` inside it. Output goes to `demo/_site/`.

**Why not run Cobalt directly on `demo/`?** Source pages lack a `layout:` frontmatter field and use relative media paths (`../media/...`) that don't resolve after permalink routing. The build step injects layout/permalink fields and rewrites media paths to absolute.

### Note on architectural divergence

Architecture doc `0001-haleiki-architecture-document.md` §3.6 puts `_cobalt.yml` and `_layouts/` at the repo root. This M1 plan puts them at `demo/_cobalt.yml` and `demo/_layouts/`, internally consistent with the existing demo-self-contained pattern (`demo/manifest.yaml`, `demo/taxonomy.yaml`). The build-directory pattern (`demo/_build/`) is also a divergence from arch doc §6, which describes a simpler "symlink demo content into `content/`, run cobalt from repo root" wiring.

Both divergences are pragmatic improvements and should be reflected back into arch doc 0001 §3.6 and §6 as part of this milestone (or a follow-up doc-update task). The demo-self-contained pattern keeps the demo a complete deployment example until the framework-extraction phase (originally Phase 5 of the plan), at which point framework-level defaults can be promoted to repo root with the demo keeping local overrides.

The architectural call is captured here so future readers (and arch-doc updates) can stay in sync.

## Files to Create

### 1. `demo/_cobalt.yml` (committed)

Cobalt configuration. Key settings:
- `source: "."` (looks at `_build/` itself)
- `destination: "../_site"` (outputs to `demo/_site/`)
- Site title/description metadata

### 2. `demo/_layouts/default.liquid` (committed)

Skeleton HTML5 wrapper: `<html>`, `<head>` with title, `<body>` with minimal header/main/footer. No CSS. Renders `{{ page.content }}`.

### 3. `demo/_layouts/source.liquid` (committed)

Extends default. Renders source page: overline "Source Page", `<h1>` from `page.title`, category/tier meta line, body content.

### 4. `demo/_layouts/concept.liquid` (committed)

Extends default. Renders concept card: overline "Concept", `<h1>` from `page.data.concept`, optional disambiguator, category/tier meta, body content.

### 5. `tools/src/demo/build.rs` (new Rust module)

The build orchestrator. Functions:

- `run()` — entry point, orchestrates all steps
- `clean_build_dir()` — rm -rf `demo/_build/`, recreate
- `setup_cobalt_config()` — copy `demo/_cobalt.yml` → `demo/_build/_cobalt.yml`
- `setup_layouts()` — symlink each `.liquid` from `demo/_layouts/` into `demo/_build/_layouts/`
- `setup_media()` — relative symlink `demo/_build/media` → `../media`
- `setup_data()` — create `demo/_build/_data/`, symlink taxonomy
- `process_sources()` — for each source .md: inject `layout: source.liquid` + `permalink: /source/{slug}/`, rewrite `](../media/` → `](/media/`, write to `demo/_build/sources/`
- `process_concepts()` — for each concept .md: inject `layout: concept.liquid` + `permalink: /{slug}/`, **rewrite `](../media/` → `](/media/`** (concept cards contain image references when figures are present; e.g., the LFE logo concept card produced by the demo pipeline has `![LFE Logo](../media/lfe-language/...)` in the body), write to `demo/_build/concepts/`
- `run_cobalt_build()` — `Command::new("cobalt").arg("build").current_dir("demo/_build")`
- `generate_index()` — write a minimal `demo/_build/index.md` (path explicit) with links to a few pages (for easy verification)

### 6. `.gitignore` additions

```
/demo/_build/
/demo/_site/
```

## Files to Modify

### `tools/src/demo/mod.rs`

- Add `pub mod build;`
- Change `DemoCommand::Build => println!(...)` to `DemoCommand::Build => build::run()?`

## Key Technical Details

### Frontmatter injection (string-level, not YAML round-trip)

```rust
fn inject_frontmatter(content: &str, layout: &str, permalink: &str) -> String {
    // Find "---\n", insert layout + permalink after it
    // Preserves YAML comments, field order, quoting
}
```

Uses literal permalinks (e.g., `/source/dzogchen/`) computed from the filename slug — avoids Cobalt template variable edge cases.

### Media path rewriting

Simple string replace: `](../media/` → `](/media/`

Apply to **both** source pages and concept cards (concept cards may contain figure references with the same relative-path pattern, produced by the demo pipeline's round-4 figure rewriting).

After Cobalt renders a source page at `/source/dzogchen/index.html` or a concept card at `/ownership-rust/index.html`, the absolute path `/media/<slug>/file.ext` resolves correctly because Cobalt copies `_build/media/` to `_site/media/`.

### Symlinks (relative, Unix)

```
demo/_build/media           → ../media
demo/_build/_layouts/*.liquid → ../../_layouts/*.liquid
demo/_build/_data/taxonomy.yaml → ../../taxonomy.yaml
```

## Implementation Order

1. Create `demo/_layouts/` directory with the three `.liquid` files
2. Create `demo/_cobalt.yml`
3. Update `.gitignore`
4. Implement `tools/src/demo/build.rs`
5. Wire into `mod.rs`
6. Build: `cargo build --release --features demo --manifest-path tools/Cargo.toml`
7. Test: `./tools/target/release/haleiki demo build`
8. Verify: `cd demo/_build && cobalt serve` → browser check

(Use the release build throughout for consistency with prior demo-pipeline conventions; debug works for quick iteration during development.)

## Verification

After implementation:

```bash
cargo build --release --features demo --manifest-path tools/Cargo.toml
./tools/target/release/haleiki demo build
cd demo/_build && cobalt serve
```

Then in browser:
- `http://localhost:1024/source/dzogchen/` — renders Dzogchen article with title, body, images
- `http://localhost:1024/ownership-rust/` — renders Ownership concept card with title, definition, code blocks
- `http://localhost:1024/` — minimal index page with links to sample pages
- Images load (e.g., the Dzogchen White-A SVG)
- Internal markdown links between pages resolve

## Not In Scope (M1 explicitly excludes)

- CSS styling
- Graph data / see-also / breadcrumbs
- Navigation pages (home, category, browse)
- `haleiki demo serve` wrapper (user runs `cobalt serve` manually)
- Search, trust bar, attribution page
