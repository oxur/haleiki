# Milestone 1.1 — Demo Directory Structure and Manifest File

**Version:** 1.0
**Depends on:** Nothing (first milestone)
**Produces:** `demo/manifest.yaml`, `demo/taxonomy.yaml`, `demo/.staging/.gitkeep`, directory skeleton for `demo/` and `content/`

---

## Overview

Create the `demo/` and `content/` directory trees with `.gitkeep` sentinel files, write the full `demo/manifest.yaml` with all 12 article entries (11 Wikipedia + 1 Wikibooks), and write `demo/taxonomy.yaml` with the demo-specific categories and tiers.

---

## Step 1: Create the `demo/` directory structure

Create the following directory tree. Empty directories get `.gitkeep` files so they're tracked by git.

```
demo/
├── manifest.yaml          # Created in Step 3
├── taxonomy.yaml          # Created in Step 4
├── .staging/              # Raw fetched HTML landing zone (milestone 2.1)
│   └── .gitkeep
├── sources/               # Converted Markdown source pages (milestone 5.3)
│   └── .gitkeep
├── concepts/              # Hand-seeded or AI-extracted concept cards (milestone 9.1)
│   └── .gitkeep
├── media/                 # Downloaded images by article slug (milestone 4.1)
│   └── .gitkeep
└── _analysis/             # Extraction analysis records
    └── .gitkeep
```

### Commands

```bash
cd /Users/oubiwann/lab/oxur/haleiki
mkdir -p demo/.staging demo/sources demo/concepts demo/media demo/_analysis
touch demo/.staging/.gitkeep demo/sources/.gitkeep demo/concepts/.gitkeep demo/media/.gitkeep demo/_analysis/.gitkeep
```

---

## Step 2: Create the `content/` directory structure

This is the user-facing content directory (empty in the repo — users who fork put their own content here). The demo pipeline symlinks `demo/` subdirectories into `content/` at build time.

```
content/
├── sources/
│   └── .gitkeep
├── concepts/
│   └── .gitkeep
└── taxonomy.yaml          # Starter template (not demo-specific)
```

### Commands

```bash
mkdir -p content/sources content/concepts
touch content/sources/.gitkeep content/concepts/.gitkeep
```

### `content/taxonomy.yaml` — Starter template

Write this file with a comment explaining its purpose and a minimal example structure:

```yaml
# Haleiki Content Taxonomy
#
# Define your categories and tiers here.
# The demo site has its own taxonomy in demo/taxonomy.yaml.
#
# Example:
#
# categories:
#   - "my-category"
#   - "another-category"
#
# tiers:
#   - "foundational"
#   - "intermediate"
#   - "advanced"

categories: []
tiers:
  - "foundational"
  - "intermediate"
  - "advanced"
```

---

## Step 3: Write `demo/manifest.yaml`

This is the central manifest that drives the entire demo fetch pipeline. Write it exactly as specified in design doc 0002, section 3.

### File: `demo/manifest.yaml`

```yaml
# Haleiki Demo Site — Content Manifest
# All content sourced under CC BY-SA 4.0 (or compatible) from Wikimedia projects.

# Default settings (can be overridden per-article)
defaults:
  project: "en.wikipedia.org"
  license: "CC BY-SA 4.0"
  media:
    max_width: 1024
    formats:
      - "svg"
      - "png"
      - "jpg"
    skip_patterns:
      - "Flag_of_*"
      - "Wiki-*.svg"
      - "Commons-logo*"
      - "Ambox_*"

# Classification defaults for the demo taxonomy
taxonomy:
  categories:
    - "memory-management"
    - "type-systems"
    - "programming-concepts"
    - "data-structures"
  tiers:
    - "foundational"
    - "intermediate"
    - "advanced"

# ─── Articles ───────────────────────────────────────────

articles:

  - title: "Memory management"
    slug: "memory-management"
    category: "memory-management"
    tier: "foundational"
    tags: ["overview", "foundational"]
    keywords: ["allocation", "deallocation", "memory model"]

  - title: "Garbage collection (computer science)"
    slug: "garbage-collection"
    category: "memory-management"
    tier: "intermediate"
    tags: ["runtime", "automatic-memory"]
    keywords: ["GC", "mark-and-sweep", "tracing"]

  - title: "Reference counting"
    slug: "reference-counting"
    category: "memory-management"
    tier: "intermediate"
    tags: ["runtime", "automatic-memory"]
    keywords: ["ref count", "weak reference", "cycle detection"]

  - title: "Stack-based memory allocation"
    slug: "stack-memory"
    category: "memory-management"
    tier: "foundational"
    tags: ["allocation"]
    keywords: ["stack frame", "LIFO", "automatic storage"]

  - title: "Region-based memory management"
    slug: "region-based-memory"
    category: "memory-management"
    tier: "advanced"
    tags: ["allocation", "research"]
    keywords: ["arena", "region inference", "Cyclone"]

  - title: "Resource acquisition is initialization"
    slug: "raii"
    category: "memory-management"
    tier: "intermediate"
    tags: ["pattern", "C++", "Rust"]
    keywords: ["RAII", "destructor", "scope-based cleanup"]

  - title: "Smart pointer"
    slug: "smart-pointer"
    category: "data-structures"
    tier: "intermediate"
    tags: ["pointer", "automatic-memory"]
    keywords: ["unique_ptr", "shared_ptr", "Box", "Rc"]

  - title: "Pointer (computer programming)"
    slug: "pointer"
    category: "programming-concepts"
    tier: "foundational"
    tags: ["fundamentals"]
    keywords: ["address", "dereference", "null pointer"]

  - title: "Dangling pointer"
    slug: "dangling-pointer"
    category: "memory-management"
    tier: "intermediate"
    tags: ["bug", "safety"]
    keywords: ["use-after-free", "wild pointer", "stale reference"]

  - title: "Memory safety"
    slug: "memory-safety"
    category: "type-systems"
    tier: "foundational"
    tags: ["safety", "verification"]
    keywords: ["buffer overflow", "bounds checking", "spatial safety"]

  - title: "Type safety"
    slug: "type-safety"
    category: "type-systems"
    tier: "foundational"
    tags: ["safety", "type-theory"]
    keywords: ["type checking", "strong typing", "soundness"]

  # ─── Cross-project sources ───────────────────────────

  - title: "Introduction to Computer Science/Memory Management"
    project: "en.wikibooks.org"
    slug: "wikibooks-memory-management"
    category: "memory-management"
    tier: "foundational"
    tags: ["tutorial", "beginner-friendly"]
    keywords: ["introduction", "textbook"]
    license: "CC BY-SA 3.0"
```

---

## Step 4: Write `demo/taxonomy.yaml`

The demo-specific taxonomy, separate from the general-purpose `content/taxonomy.yaml`.

### File: `demo/taxonomy.yaml`

```yaml
# Haleiki Demo Site — Taxonomy
#
# Categories and tiers used by the demo content cluster.
# These must match the values used in demo/manifest.yaml.

categories:
  - name: "memory-management"
    description: "Memory allocation, deallocation, and management strategies"
  - name: "type-systems"
    description: "Type safety, type checking, and type-theoretic concepts"
  - name: "programming-concepts"
    description: "General programming concepts and abstractions"
  - name: "data-structures"
    description: "Data structures and their implementations"

tiers:
  - name: "foundational"
    description: "Core concepts — prerequisites for most other topics"
    order: 1
  - name: "intermediate"
    description: "Builds on foundational knowledge"
    order: 2
  - name: "advanced"
    description: "Specialized or research-level topics"
    order: 3
```

---

## Step 5: Update `.gitignore`

Add entries for generated demo content that should not be committed (staging HTML, generated media, etc.). The manifest and taxonomy YAML files ARE committed.

### Additions to `.gitignore`

```
# Demo generated content (fetched/converted, not committed)
/demo/.staging/*.html
/demo/media/manifest.json
```

**Important:** Do NOT gitignore `demo/manifest.yaml`, `demo/taxonomy.yaml`, `demo/sources/`, `demo/concepts/`, or `demo/media/` directories themselves — the design doc says demo content IS committed to the repo for reproducible CI builds (see design doc section 10: "Demo content is committed to the repo, not fetched during CI").

Actually, re-reading the design doc more carefully: `demo/sources/*.md` and `demo/media/*/` ARE committed. Only `.staging/` HTML files (intermediate fetched HTML before conversion) should be gitignored. Update the `.gitignore` additions to:

```
# Demo staging files (intermediate, not committed)
/demo/.staging/*.html
```

---

## Verification

Run these checks to confirm the milestone is complete:

```bash
# 1. Directory structure exists
test -d demo/.staging && echo "OK: demo/.staging"
test -d demo/sources && echo "OK: demo/sources"
test -d demo/concepts && echo "OK: demo/concepts"
test -d demo/media && echo "OK: demo/media"
test -d demo/_analysis && echo "OK: demo/_analysis"
test -d content/sources && echo "OK: content/sources"
test -d content/concepts && echo "OK: content/concepts"

# 2. .gitkeep files exist
test -f demo/.staging/.gitkeep && echo "OK: .staging/.gitkeep"
test -f demo/sources/.gitkeep && echo "OK: sources/.gitkeep"
test -f demo/concepts/.gitkeep && echo "OK: concepts/.gitkeep"
test -f demo/media/.gitkeep && echo "OK: media/.gitkeep"
test -f demo/_analysis/.gitkeep && echo "OK: _analysis/.gitkeep"
test -f content/sources/.gitkeep && echo "OK: content/sources/.gitkeep"
test -f content/concepts/.gitkeep && echo "OK: content/concepts/.gitkeep"

# 3. YAML files parse correctly
python3 -c "import yaml; yaml.safe_load(open('demo/manifest.yaml'))" && echo "OK: manifest.yaml parses"
python3 -c "import yaml; yaml.safe_load(open('demo/taxonomy.yaml'))" && echo "OK: taxonomy.yaml parses"
python3 -c "import yaml; yaml.safe_load(open('content/taxonomy.yaml'))" && echo "OK: content/taxonomy.yaml parses"

# 4. Manifest has exactly 12 articles
python3 -c "
import yaml
m = yaml.safe_load(open('demo/manifest.yaml'))
articles = m['articles']
assert len(articles) == 12, f'Expected 12 articles, got {len(articles)}'
slugs = [a['slug'] for a in articles]
assert len(slugs) == len(set(slugs)), 'Duplicate slugs found!'
print(f'OK: {len(articles)} articles, all slugs unique')
"

# 5. All article slugs listed
python3 -c "
import yaml
m = yaml.safe_load(open('demo/manifest.yaml'))
expected = [
    'memory-management', 'garbage-collection', 'reference-counting',
    'stack-memory', 'region-based-memory', 'raii', 'smart-pointer',
    'pointer', 'dangling-pointer', 'memory-safety', 'type-safety',
    'wikibooks-memory-management'
]
actual = sorted([a['slug'] for a in m['articles']])
assert sorted(expected) == actual, f'Mismatch: {set(expected) ^ set(actual)}'
print('OK: all expected slugs present')
"
```

---

## Acceptance Criteria

- [ ] `demo/` directory tree exists with all subdirectories and `.gitkeep` files
- [ ] `content/` directory tree exists with `sources/`, `concepts/`, and `taxonomy.yaml`
- [ ] `demo/manifest.yaml` parses as valid YAML
- [ ] `demo/manifest.yaml` contains exactly 12 articles with unique slugs
- [ ] `demo/manifest.yaml` has correct defaults block (project, license, media settings)
- [ ] `demo/manifest.yaml` has taxonomy block matching the 4 categories and 3 tiers
- [ ] All articles have required fields: `title`, `slug`, `category`, `tier`
- [ ] The Wikibooks article overrides `project` and `license`
- [ ] `demo/taxonomy.yaml` parses as valid YAML with 4 categories and 3 tiers
- [ ] `content/taxonomy.yaml` exists as a starter template
- [ ] `.gitignore` updated for demo staging files
- [ ] No existing files are modified (except `.gitignore`)

---

## Gotchas

1. **Do not put Cargo.toml or Rust code in this milestone** — that's milestone 1.2
2. **The `demo/.staging/` directory** is referenced in milestone 2.1 (fetch pipeline writes raw HTML here). Create it now so the directory structure is complete.
3. **The Wikibooks article title** contains a `/` character: `"Introduction to Computer Science/Memory Management"`. This is a valid Wikimedia page title — the title field must preserve it exactly. The `slug` field is what's used for filenames.
4. **YAML string quoting**: The design doc does not quote most string values in the manifest. Follow the same convention — only quote strings that contain special YAML characters (`:`, `#`, `{`, etc.).
5. **Taxonomy in manifest vs. standalone file**: The manifest embeds a `taxonomy` block for validation purposes (milestones 1.3 checks articles against it). The standalone `demo/taxonomy.yaml` is the richer version with descriptions and ordering. Both must list the same categories and tiers.
