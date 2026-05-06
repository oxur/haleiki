---
number: 7
title: "`haleiki build` Core"
author: "splitting on"
component: All
tags: [change-me]
created: 2026-05-04
updated: 2026-05-05
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# M2 Implementation Plan: `haleiki build` Core

## Context

M1 established the Cobalt rendering pipeline. M2 adds the **pre-build data layer**: a Rust pipeline that parses the demo corpus (91 source pages + 186 concept cards), builds a relationship graph, validates cross-references, and writes JSON data files that M4+ templates will consume for see-also sections, breadcrumbs, category indices, and provenance displays.

After M2, `haleiki demo build` produces the same rendered HTML *plus* a populated `demo/_build/_data/` directory. No template changes in M2.

## Files to Create

| File | Purpose | Approx lines |
|------|---------|------|
| `demo/profile.yaml` | Encyclopedia deployment profile | ~20 |
| `tools/src/parser.rs` | Parse source pages + concept cards into Rust types | ~250 |
| `tools/src/graph.rs` | Build petgraph, compute see-also/breadcrumbs/categories | ~300 |
| `tools/src/validator.rs` | Validation rules (errors + warnings) | ~200 |
| `tools/src/generator.rs` | Write `_data/*.json` files | ~250 |

## Files to Modify

| File | Changes |
|------|---------|
| `tools/src/main.rs` | Wire Build/Validate/Stats commands to real implementations |
| `tools/src/cli.rs` | Add `--content-dir` and `--output-dir` flags to Build/Validate/Stats |
| `tools/src/demo/build.rs` | Call framework build between content processing and cobalt build |
| `tools/Cargo.toml` | No changes needed — petgraph, serde_yaml, serde_json already present |

## Implementation Details

### 1. `demo/profile.yaml`

```yaml
profile:
  name: "Haleiki Demo Wiki"
  personality: "encyclopedia"

trust:
  enabled: true
  default: "credible"

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
```

### 2. `tools/src/parser.rs`

**Key types:**

```rust
pub struct Corpus {
    pub sources: HashMap<String, SourcePage>,
    pub concepts: HashMap<String, ConceptCard>,
    pub taxonomy: Taxonomy,
    pub profile: Option<Profile>,
}
```

**Design choices:**

- Use `#[serde(default)]` on all optional/list fields for forward-compat
- Use `#[serde(rename_all = "snake_case")]` where applicable
- Parse frontmatter by splitting on `---` delimiters, then `serde_yaml::from_str` on the YAML block
- Store raw markdown body as `String` (not parsed further)
- YAML comments (`# === SECTION ===`) are handled by serde_yaml automatically (ignored during parse)
- Taxonomy struct: `{ categories: Vec<TaxonomyEntry>, tiers: Vec<TierEntry> }`
- Profile struct mirrors the YAML shape

**Public API:** `pub fn parse_corpus(content_dir: &Path) -> anyhow::Result<Corpus>`

Reads:

- `{content_dir}/sources/*.md`
- `{content_dir}/concepts/*.md`
- `{content_dir}/taxonomy.yaml`
- `{content_dir}/profile.yaml` (optional)

**Frontmatter parsing approach:**

```rust
fn parse_frontmatter<T: DeserializeOwned>(content: &str) -> Result<(T, String)> {
    let after_first = content.find("---\n").map(|i| i + 4)?;
    let second_fence = content[after_first..].find("\n---")?;
    let yaml_str = &content[after_first..after_first + second_fence];
    let body = &content[after_first + second_fence + 4..];
    let frontmatter: T = serde_yaml::from_str(yaml_str)?;
    Ok((frontmatter, body.to_string()))
}
```

### 3. `tools/src/graph.rs`

**Graph structure:**

- `petgraph::Graph<NodeData, EdgeKind>` (directed graph)
- `HashMap<String, NodeIndex>` for slug → node lookup
- `NodeData` enum: `Source { slug }` or `Concept { slug }`
- `EdgeKind` enum: `Prerequisite`, `Extends`, `Related`, `ContrastsWith`, `DerivedFrom`, `ConceptsGenerated`

**Build process:**

1. Add all source pages and concept cards as nodes
2. For each concept: add edges from its `prerequisites`, `extends`, `related`, `contrasts_with`, and `derived_from`
3. For symmetric relationships (`Related`, `ContrastsWith`): add edges in both directions

**Derived data methods:**

- `see_also(slug)` → collect all neighbors by edge type, resolve titles
- `breadcrumbs(slug, corpus)` → Home / Category / Subcategory / Page
- `category_index(corpus)` → group all pages by category/subcategory
- `orphans()` → nodes with zero typed-relationship edges (excluding provenance)
- `stats()` → counts

**Breadcrumb logic:**

```
For slug "padmasambhava" with category "tibetan-buddhism", subcategory "teachers":
  [Home (/), Tibetan Buddhism (/category/tibetan-buddhism/),
   Teachers (/category/tibetan-buddhism/teachers/),
   Padmasambhava (/source/padmasambhava/)]
```

### 4. `tools/src/validator.rs`

**Validation rules (from spec):**

| Check | Severity | Logic |
|-------|----------|-------|
| Duplicate slug | error | Check sources + concepts for any slug appearing twice |
| Slug-filename mismatch | error | slug field ≠ filename stem |
| Unknown category | error | page.category not in taxonomy.categories |
| Unknown tier | error | page.tier not in taxonomy.tiers |
| Broken cross-ref | error | prerequisites/extends/related/contrasts_with slug not in corpus |
| Dangling provenance | error | concept's derived_from slug not a source page |
| concepts_generated missing card | error | source lists slug but no concept card exists |
| Bidirectional mismatch | warning | derived_from ↔ concepts_generated inconsistency |
| Orphan concept | warning | no typed-relationship edges |
| Orphan source | warning | `extraction_status: complete` AND `concepts_generated` empty AND no concept's `derived_from` lists this slug. (NB: `extraction_status: skipped` is a deliberate state, not an orphan — the back-fill skipped 5 source pages legitimately and we don't want to re-warn on those every build.) |

**Public API:** `pub fn validate(corpus: &Corpus) -> ValidationReport`

### 5. `tools/src/generator.rs`

Write 6 JSON files:

- `graph.json` — full node/edge graph
- `categories.json` — per-category indices with subcategories
- `breadcrumbs.json` — per-slug breadcrumb trails
- `see-also.json` — per-slug typed relationships (expanded with titles/URLs)
- `provenance.json` — bidirectional source↔concept links
- `profile.json` — resolved profile with trust level descriptions

**Trust description resolution:** when `profile.yaml` doesn't override `trust.descriptions`, fall back to the personality's defaults. For the encyclopedia personality, use the epistemic-dialect descriptions from `0005-deployment-profiles-epistemic-architecture.md` §3.3:

| Level | Label | Description |
|-------|-------|-------------|
| `corroborated` | Corroborated | Verified from multiple reliable sources |
| `credible` | Credible | Well-sourced, reasonable interpretation |
| `provisional` | Provisional | Partial evidence, subject to revision |
| `speculative` | Speculative | Limited or fragmentary basis |

These get written into the `profile.json` so M6's trust-bar template doesn't have to re-derive them.

**Public API:** `pub fn generate(corpus: &Corpus, graph: &HaleikiGraph, output_dir: &Path) -> anyhow::Result<()>`

Uses `serde_json::to_string_pretty` for readability during development.

### 6. CLI Wiring (`main.rs` + `cli.rs`)

Add to `Command` enum variants (or as global flags):

```rust
/// Build the knowledge base (graph JSON, indices, derived data)
Build {
    #[arg(long, default_value = "content")]
    content_dir: PathBuf,
    #[arg(long)]
    output_dir: Option<PathBuf>,
},
/// Validate all content references and frontmatter
Validate {
    #[arg(long, default_value = "content")]
    content_dir: PathBuf,
},
/// Show statistics about the knowledge base
Stats {
    #[arg(long, default_value = "content")]
    content_dir: PathBuf,
},
```

Default `output_dir`: `{content_dir}/_data`.

### 7. Demo Build Integration (`demo/build.rs`)

Add `run_haleiki_build()` call between `process_concepts()` and `run_cobalt_build()`:

```rust
fn run_haleiki_build() -> Result<()> {
    eprintln!("  Running haleiki build (graph + data)...");
    let corpus = crate::parser::parse_corpus(Path::new("demo"))?;
    let report = crate::validator::validate(&corpus);
    if report.has_errors() {
        report.print();
        anyhow::bail!("Demo content validation failed.");
    }
    if !report.warnings.is_empty() {
        report.print();
    }
    let graph = crate::graph::HaleikiGraph::build(&corpus)?;
    crate::generator::generate(&corpus, &graph, Path::new("demo/_build/_data"))?;
    Ok(())
}
```

## Implementation Order

1. Create `demo/profile.yaml`
2. Implement `tools/src/parser.rs` (types + parsing)
3. Implement `tools/src/validator.rs` (uses parsed corpus)
4. Implement `tools/src/graph.rs` (uses parsed corpus)
5. Implement `tools/src/generator.rs` (uses corpus + graph)
6. Wire CLI commands in `main.rs` / `cli.rs`
7. Integrate into `demo/build.rs`
8. Build, test, verify end-to-end

## Testing Strategy

- **parser.rs**: Parse sample source page, concept card, full demo corpus (91 sources, 186 concepts)
- **validator.rs**: Synthetic corpus with each error/warning type
- **graph.rs**: Build from sample corpus, assert node/edge counts, query see-also
- **generator.rs**: Write to temp dir, parse JSON back, assert structure
- Integration: `haleiki validate --content-dir demo` exits 0

## Verification

```bash
# Build
cargo build --release --features demo --manifest-path tools/Cargo.toml

# Stats
./target/release/haleiki stats --content-dir demo

# Validate
./target/release/haleiki validate --content-dir demo

# Build standalone
./target/release/haleiki build --content-dir demo --output-dir /tmp/haleiki-data

# Demo build (full pipeline)
./target/release/haleiki demo build

# Spot-check data files
cat demo/_build/_data/profile.json
jq '.["padmasambhava"]' demo/_build/_data/breadcrumbs.json
jq '.["nyingma"]' demo/_build/_data/see-also.json
```

Expected: zero validation errors, all 6 JSON files populated, demo build succeeds end-to-end.
