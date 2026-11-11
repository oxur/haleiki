# M2 Implementation: `haleiki build` Core

Second milestone of the Phase 6 site build (per `docs/design/01-draft/0006-haleiki-site-build-project-plan.md` § M2). M1 produced the cobalt-rendering pipeline; M2 produces the relationship graph and derived data that templates will consume in M4+.

This is purely a build-time data layer — no template changes in M2. After M2, `haleiki demo build` produces the same rendered HTML as before, *plus* a populated `demo/_build/_data/` directory ready for M4 templates to read.

---

## Reference docs

- `docs/design/01-draft/0006-haleiki-site-build-project-plan.md` § M2 — milestone scope
- `docs/design/01-draft/0001-haleiki-architecture-document.md` §3 (content model), §5 (pre-build CLI), §3.4 (concept merging) — schema and design
- `docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md` §6 (profile schema), §8 (cobalt integration) — profile.yaml shape
- `docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md` (v1.2) — concept card frontmatter spec
- `docs/dev/concept-cards/0002-a-guide-for-parallel-concept-card-extraction.md` (v1.2) — bidirectional provenance check (§3.5.5)
- `tools/src/demo/build.rs` — M1 orchestrator (you'll extend it to call the new framework build)

---

## Scope

Goal: implement the framework's pre-build pipeline. Modules currently exist as 4-5 line stubs in `tools/src/`:

- `parser.rs` — parse source pages and concept cards into structured Rust types
- `graph.rs` — construct unified petgraph; edges per typed relationships and provenance
- `validator.rs` — run validation rules, emit ValidationReport with errors and warnings
- `generator.rs` — write `_data/*.json` files for cobalt templates to consume
- `merger.rs` — leave for now (Phase 3 of original framework plan; M2 doesn't need it)
- `search.rs` — leave for now (separate Pagefind phase)

CLI commands to wire in `tools/src/main.rs`:

- `Command::Build` → run parse → validate → graph → generate
- `Command::Validate` → run parse → validate, exit non-zero on errors
- `Command::Stats` → print graph statistics (node counts, edge counts by type, orphan counts)

Plus a small `demo/profile.yaml` to land alongside the rest of the demo's self-contained config.

---

## Design decisions baked in

These were settled before this prompt; implement directly.

**1. Build path: source-of-truth, not build-directory.** `haleiki build` reads canonical content files, not the cobalt-processed M1 output. Reasoning: the M1 `inject_frontmatter` transforms frontmatter into a `data:` indirection for cobalt; the parser shouldn't have to peel that back. The framework operates on canonical content; the cobalt prep is downstream.

CLI signature: `haleiki build [--content-dir <path>] [--output-dir <path>]`. Defaults: `--content-dir content`, `--output-dir <content-dir>/_data`. For the demo, `haleiki demo build` calls `haleiki build --content-dir demo --output-dir demo/_build/_data` (or invokes the relevant function directly — your call on factoring).

**2. Profile placement: `demo/profile.yaml`.** Matches the demo-self-contained pattern (manifest, taxonomy, layouts, cobalt-config all at `demo/<thing>`). Create it as part of M2 with these contents (adjust formatting to match project conventions):

```yaml
profile:
  name: "Haleiki Demo Wiki"
  personality: "encyclopedia"

trust:
  enabled: true
  default: "credible"
  # descriptions inherit from encyclopedia personality defaults

canon:
  enabled: false

page_types:
  - source
  - concept

relationships:
  - prerequisite
  - extends
  - related
  - contrasts_with

# Taxonomy reference — reads from demo/taxonomy.yaml, no inline override
```

**3. Validator strictness.** Two severity levels: error (blocks build) and warning (continues). Behavior:

| Issue | Severity |
|-------|----------|
| Duplicate slug across the corpus | error |
| Slug-filename mismatch | error |
| Unknown category (vs taxonomy.yaml) | error |
| Unknown tier (vs taxonomy.yaml) | error |
| Broken cross-reference (prerequisite/extends/related/contrasts_with slug doesn't exist) | error |
| Dangling provenance (concept's `derived_from` slug doesn't point to a source page) | error |
| Slug listed in `concepts_generated` but no concept card with that slug exists | error |
| Concept card's `derived_from` slug missing from source's `concepts_generated` (or vice versa) | warning |
| Orphan concept (no incoming or outgoing typed-relationship edges) | warning |
| Orphan source page (no concept extracted from it) | warning |

`haleiki validate` exits non-zero if any errors. `haleiki build` runs `validate` first; aborts on errors, continues on warnings (printing the warnings).

**4. Data file shapes.** Single consolidated files per concept (rather than per-page-slug files). Templates access via `site.data.<name>[page.slug]`.

---

## Module specifications

### `parser.rs`

Parse YAML frontmatter (using `serde_yaml`) and the markdown body. Define structs that map the v1.2 schema cleanly:

```rust
// Sketch — adjust to taste
pub struct Corpus {
    pub sources: HashMap<String, SourcePage>,
    pub concepts: HashMap<String, ConceptCard>,
    pub taxonomy: Taxonomy,
    pub profile: Option<Profile>,
}

pub struct SourcePage {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub tier: String,
    pub keywords: Vec<String>,
    pub tags: Vec<String>,
    pub author: String,
    pub original_source: OriginalSource,
    pub extraction_status: String,        // "pending" | "in-progress" | "complete" | "skipped"
    pub concepts_generated: Vec<String>,  // concept-card slugs derived from this page
    pub last_extracted: Option<String>,
    pub trust_level: Option<String>,
    pub canon_status: Option<String>,
    pub status: String,
    pub body: String,                     // raw markdown post-frontmatter
}

pub struct ConceptCard {
    pub slug: String,
    pub concept: String,
    pub disambiguator: Option<String>,
    pub category: String,
    pub subcategory: Option<String>,
    pub tier: String,
    pub subject_headings: Vec<String>,
    pub keywords: Vec<String>,
    pub aliases: Vec<String>,
    pub derived_from: Vec<DerivedFromEntry>,
    pub external_references: Vec<ExternalReference>,
    pub extraction_confidence: String,
    pub prerequisites: Vec<String>,
    pub extends: Vec<String>,
    pub related: Vec<String>,
    pub contrasts_with: Vec<String>,
    pub trust_level: Option<String>,
    pub canon_status: Option<String>,
    pub answers_questions: Vec<String>,
    pub status: String,
    pub stable_since: Option<String>,
    pub last_reviewed: Option<String>,
    pub body: String,
}

pub struct DerivedFromEntry {
    pub slug: String,
    pub title: String,
}
```

Public API: `pub fn parse_corpus(content_dir: &Path) -> Result<Corpus>`. Reads `<content_dir>/sources/*.md`, `<content_dir>/concepts/*.md`, `<content_dir>/taxonomy.yaml`, `<content_dir>/profile.yaml` (if present). Returns the populated `Corpus`.

Frontmatter parsing should be tolerant of fields not in the schema (forward-compat) — `#[serde(default)]` and `#[serde(flatten)]` patterns. Missing required fields are validation errors, not parse errors.

Tests: parse a sample source page, parse a sample concept card, parse the full demo corpus (integration test with `assert_eq!(corpus.sources.len(), 91)` and `assert!(corpus.concepts.len() >= 180)`).

### `graph.rs`

Build a unified petgraph using the parsed `Corpus`. Both source pages and concept cards become nodes. Edge types:

- `Prerequisite` — concept → concept (from concept's `prerequisites` field)
- `Extends` — concept → concept (from `extends`)
- `Related` — concept ↔ concept (from `related`, treat as undirected during graph construction or store both directions)
- `ContrastsWith` — concept ↔ concept (from `contrasts_with`)
- `DerivedFrom` — concept → source page (from concept's `derived_from`)
- `ConceptsGenerated` — source page → concept (from source's `concepts_generated`; this is the inverse of DerivedFrom and should be checked for consistency in the validator)

```rust
// Sketch
pub struct HaleikiGraph {
    pub graph: petgraph::Graph<NodeId, EdgeKind>,
    pub node_index: HashMap<String, NodeIndex>,  // slug -> NodeIndex
}

pub enum NodeId {
    Source(String),  // slug
    Concept(String), // slug
}

pub enum EdgeKind {
    Prerequisite,
    Extends,
    Related,
    ContrastsWith,
    DerivedFrom,
    ConceptsGenerated,
}

impl HaleikiGraph {
    pub fn build(corpus: &Corpus) -> Result<Self>;
    pub fn see_also(&self, slug: &str) -> SeeAlso;
    pub fn breadcrumbs(&self, slug: &str, corpus: &Corpus) -> Vec<BreadcrumbEntry>;
    pub fn category_index(&self, corpus: &Corpus) -> HashMap<String, CategoryEntry>;
    pub fn orphans(&self) -> Vec<String>;
    pub fn stats(&self) -> GraphStats;
}
```

Breadcrumbs: per architecture, breadcrumbs derive from category hierarchy. For a page with `category: tibetan-buddhism, subcategory: teachers, slug: padmasambhava`:

```
Home / Tibetan Buddhism / Teachers / Padmasambhava
```

The breadcrumb function returns `Vec<BreadcrumbEntry>` where each entry has `{ title, url }`. URL conventions:
- Home: `/`
- Category: `/category/<category>/`
- Subcategory: `/category/<category>/<subcategory>/` (if subcategory exists)
- Page: `/source/<slug>/` for sources, `/<slug>/` for concepts

Tests: build graph from sample corpus, assert node and edge counts, walk relationships, query see-also for a known concept.

### `validator.rs`

Run the validation rules from "Design decisions baked in" §3 above. Return a structured report:

```rust
pub struct ValidationReport {
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
}

pub struct ValidationIssue {
    pub kind: ValidationKind,
    pub slug: Option<String>,
    pub message: String,
}

pub enum ValidationKind {
    DuplicateSlug,
    SlugFilenameMismatch,
    UnknownCategory,
    UnknownTier,
    BrokenCrossReference,
    DanglingProvenance,
    BackLinkInconsistency,
    OrphanConcept,
    OrphanSource,
}

impl ValidationReport {
    pub fn has_errors(&self) -> bool;
    pub fn print(&self);  // human-readable to stderr
}
```

Public API: `pub fn validate(corpus: &Corpus) -> ValidationReport`.

Tests: synthetic corpus with each issue type; assert the right errors/warnings are emitted.

### `generator.rs`

Write the `_data/` files. Public API: `pub fn generate(corpus: &Corpus, graph: &HaleikiGraph, output_dir: &Path) -> Result<()>`.

Files to write (all consolidated, single file per concept):

**`<output_dir>/graph.json`** — full graph:
```json
{
  "nodes": {
    "<slug>": {
      "page_type": "source" | "concept",
      "title": "...",
      "category": "...",
      "subcategory": "...",
      "tier": "...",
      "trust_level": "...",
      "concept": "...",          // for concepts only
      "disambiguator": "...",    // for concepts only
      "subject_headings": [...]  // for concepts only
    }
  },
  "edges": [
    { "from": "<slug>", "to": "<slug>", "type": "prerequisite" }
  ]
}
```

**`<output_dir>/categories.json`** — category index:
```json
{
  "tibetan-buddhism": {
    "name": "tibetan-buddhism",
    "description": "Tibetan Buddhist philosophy, practices, teachers, and lineages",
    "sources": ["dzogchen", "padmasambhava", ...],
    "concepts": ["nyingma", "rigpa", ...],
    "subcategories": {
      "teachers": { "sources": [...], "concepts": [...] },
      "schools": { ... }
    }
  }
}
```

**`<output_dir>/breadcrumbs.json`** — per-slug breadcrumb trail:
```json
{
  "padmasambhava": [
    { "title": "Home", "url": "/" },
    { "title": "Tibetan Buddhism", "url": "/category/tibetan-buddhism/" },
    { "title": "Teachers", "url": "/category/tibetan-buddhism/teachers/" },
    { "title": "Padmasambhava", "url": "/source/padmasambhava/" }
  ]
}
```

**`<output_dir>/see-also.json`** — per-slug typed relationships, expanded:
```json
{
  "ownership-rust": {
    "prerequisites": [],
    "extends": [],
    "related": [
      { "slug": "borrowing-rust", "title": "Borrowing", "page_type": "concept", "disambiguator": "Rust", "url": "/borrowing-rust/" }
    ],
    "contrasts_with": [],
    "derived_from": [
      { "slug": "rust-language", "title": "Rust (programming language)", "page_type": "source", "url": "/source/rust-language/" }
    ]
  }
}
```

**`<output_dir>/provenance.json`** — bidirectional provenance:
```json
{
  "rust-language": {
    "page_type": "source",
    "concepts_generated": [
      { "slug": "ownership-rust", "concept": "Ownership", "disambiguator": "Rust", "url": "/ownership-rust/" }
    ]
  },
  "ownership-rust": {
    "page_type": "concept",
    "derived_from": [
      { "slug": "rust-language", "title": "Rust (programming language)", "url": "/source/rust-language/" }
    ],
    "external_references": [
      { "title": "The Rust Programming Language", "chapter": "Understanding Ownership", "url": "https://doc.rust-lang.org/book/...", "license": "MIT/Apache-2.0" }
    ]
  }
}
```

**`<output_dir>/profile.json`** — per addendum 0005 §8.1, derived from `<content_dir>/profile.yaml` plus personality defaults:
```json
{
  "personality": "encyclopedia",
  "trust": {
    "enabled": true,
    "default": "credible",
    "levels": {
      "corroborated": { "label": "Corroborated", "description": "Verified from multiple reliable sources" },
      "credible": { "label": "Credible", "description": "Well-sourced, reasonable interpretation" },
      "provisional": { "label": "Provisional", "description": "Partial evidence, subject to revision" },
      "speculative": { "label": "Speculative", "description": "Limited or fragmentary basis" }
    },
    "level_order": ["corroborated", "credible", "provisional", "speculative"]
  },
  "canon": { "enabled": false },
  "page_types": ["source", "concept"],
  "relationships": ["prerequisite", "extends", "related", "contrasts_with"],
  "taxonomy": {
    "categories": [...],
    "tiers": [...]
  }
}
```

For trust descriptions when not overridden in `profile.yaml`, pull the encyclopedia-personality defaults from addendum 0005 §3.3.

Tests: write data to a temp directory, parse the JSON back, assert structure.

### CLI wiring (`tools/src/main.rs`)

Replace the stubs:

```rust
Command::Build => {
    let corpus = parser::parse_corpus(content_dir)?;
    let report = validator::validate(&corpus);
    if report.has_errors() {
        report.print();
        bail!("Validation failed; build aborted.");
    }
    if !report.warnings.is_empty() {
        report.print();  // warnings only
    }
    let graph = graph::HaleikiGraph::build(&corpus)?;
    generator::generate(&corpus, &graph, output_dir)?;
    println!("Build complete. Wrote {} files to {}", file_count, output_dir.display());
}
Command::Validate => {
    let corpus = parser::parse_corpus(content_dir)?;
    let report = validator::validate(&corpus);
    report.print();
    if report.has_errors() {
        std::process::exit(1);
    }
}
Command::Stats => {
    let corpus = parser::parse_corpus(content_dir)?;
    let graph = graph::HaleikiGraph::build(&corpus)?;
    let stats = graph.stats();
    println!("{}", stats);  // human-readable summary
}
```

Add `--content-dir` and `--output-dir` flags to the `Cli` struct in `cli.rs` (or as flags on the relevant commands). Default `--content-dir`: `content`. Default `--output-dir`: `<content-dir>/_data`.

### Demo build integration (`tools/src/demo/build.rs`)

Extend the M1 orchestrator. After `process_concepts` and before `run_cobalt_build`, call the framework build:

```rust
fn run_haleiki_build() -> Result<()> {
    eprintln!("  Running haleiki build...");
    // Either invoke the function directly:
    let corpus = crate::parser::parse_corpus(Path::new("demo"))?;
    let report = crate::validator::validate(&corpus);
    if report.has_errors() {
        report.print();
        bail!("Demo content validation failed.");
    }
    if !report.warnings.is_empty() {
        report.print();
    }
    let graph = crate::graph::HaleikiGraph::build(&corpus)?;
    crate::generator::generate(&corpus, &graph, Path::new("demo/_build/_data"))?;
    Ok(())
    // ...or shell out: Command::new(self_exe).args(["build", "--content-dir", "demo", "--output-dir", "demo/_build/_data"])
}
```

The function-call approach is simpler (no subprocess overhead, error handling stays Rust-native). Use it.

Note: the existing `setup_data()` symlinks `taxonomy.yaml` into `demo/_build/_data/`. Keep that — it lets templates access `site.data.taxonomy` directly without going through the framework's resolved profile. The new generated data files (`graph.json`, etc.) live alongside.

---

## File creation summary

New files:
- `demo/profile.yaml` — encyclopedia profile (per spec above)
- (Real implementations of) `tools/src/parser.rs`, `graph.rs`, `validator.rs`, `generator.rs`

Modified files:
- `tools/src/main.rs` — wire Build/Validate/Stats commands
- `tools/src/cli.rs` — add `--content-dir` and `--output-dir` flags
- `tools/src/demo/build.rs` — call framework build between content processing and cobalt build

---

## Verification

1. **Build**: `cargo build --release --features demo --manifest-path tools/Cargo.toml` should succeed without warnings.

2. **Stats**: `./tools/target/release/haleiki stats --content-dir demo` should print:
   - Source-page count: 91
   - Concept-card count: 186 (or whatever the current state is — check `ls demo/concepts | wc -l`)
   - Edge counts by type (prerequisite, related, derived_from, etc.)
   - Orphan counts

3. **Validate**: `./tools/target/release/haleiki validate --content-dir demo` should:
   - Exit 0 (zero errors)
   - May print warnings (orphans are likely)
   - No errors expected per the corpus's validated state

4. **Build standalone**: `./tools/target/release/haleiki build --content-dir demo --output-dir /tmp/haleiki-test-data` should produce `/tmp/haleiki-test-data/{graph,categories,breadcrumbs,see-also,provenance,profile}.json`.

5. **Demo build integration**: `./tools/target/release/haleiki demo build` should run end-to-end, producing `demo/_build/_data/` with the full data set, and the cobalt build should still succeed (no template breakage since M2 doesn't change templates).

6. **Spot-check the data files** — paste:
   - First 80 lines of `demo/_build/_data/graph.json` (formatted)
   - The `tibetan-buddhism` entry in `categories.json`
   - The `padmasambhava` entry in `breadcrumbs.json` (should be Home / Tibetan Buddhism / Teachers / Padmasambhava)
   - The `nyingma` entry in `see-also.json` (multi-source — let's see what it looks like)
   - The `nyingma` entry in `provenance.json` (multi-source `derived_from`, plus its `concepts_generated` if it exists as a source page too)
   - The full `profile.json`

7. **Regression on the rendered demo**: after the full demo build, open `demo/_site/source/dzogchen/index.html` in a browser (or curl it) and confirm it still renders correctly. Templates haven't changed, but the build pipeline now does more work.

---

## Reporting

Standard pattern. Branch the work or commit to main per your judgment (pattern from M1 was commit-to-main with a clear message; same is fine here unless you want to PR for review).

Report:

1. **Branch/commit details** — `git diff main --stat` if branched; commit hash if landed.
2. **Module sizes** — line counts for each new module (parser.rs, graph.rs, validator.rs, generator.rs).
3. **Test counts** — total tests, breakdown by module. Use the existing `test_<fn>_<scenario>_<expectation>` naming convention from CLAUDE.md.
4. **Verification report** — output of each step above (stats, validate, build, spot-checks).
5. **Validation findings** — any warnings emitted against the demo corpus (orphans, back-link mismatches). Errors should be zero; warnings are interesting data about corpus shape.
6. **Schema rough edges** — anything in the v1.2 concept-card schema that didn't fit cleanly when implementing the parser (forward-compat surprises, ambiguous field semantics).
7. **Anything surprising** — any data shape that emerged differently than expected, performance notes (parsing 91+186 markdown files should be fast), etc.

If something substantive comes up that requires a design call (e.g., the data file shape doesn't fit a real template need we haven't anticipated, or the validator's strictness defaults produce too many false errors against the demo corpus), stop and report rather than guessing.

---

## Out of scope (explicitly)

- **Templates consuming the new data** — that's M4. M2 is data-only.
- **Trust-bar / canon-badge components** — that's M6. M2 produces `profile.json` for M6 to use.
- **Definitions/about page** — that's M7.
- **Search index** — separate phase entirely.
- **Concept merging logic** — `merger.rs` stays a stub. Phase 3 of the original framework plan; not needed for M2.
