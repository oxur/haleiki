# A Guide for Parallel Concept Card Extraction

**Haleiki v1.**

This guide documents how to extract concept cards from a Haleiki source-page corpus using Claude Code with Opus agents working in parallel. It assumes familiarity with [`0001-howto-wiki-concept-card-extraction.md`](./0001-howto-wiki-concept-card-extraction.md), which covers the schema and per-card extraction mechanics — this doc focuses on coordinating extraction at corpus scale.

The procedure is designed to be merge-aware from the start. Whether the target deployment has zero, some, or many existing concept cards, the extraction pattern is the same: agents check for existing cards at each candidate slug, augment if present (multi-source `derived_from`), create fresh otherwise.

---

## Lineage

This guide derives from Billo Systems' AI Engineering parallel re-extraction guide v3.2, adapted for the Haleiki wiki framework. Major adaptations:

- **Source shape** — Reframed from book-with-chapters (single source split into chapter files) to wiki-source-page-cluster (many source pages, each its own document, networked by manifest and graph). Phase 1 analysis uses the manifest and source-page taxonomy rather than chapter sequence.
- **Agent assignment** — Agents work on coherent source-page *clusters* (typically a category subset, sometimes a tier subset within a category) rather than sequential chapter ranges. The unit of work is "all source pages in this cluster" rather than "chapters N through M."
- **Cross-source merging** — Promoted from a re-extraction afterthought to a first-class step. The wiki use case has cross-page concept overlap as the *normal case* (encyclopedia articles cross-reference each other constantly), so the validation phase explicitly checks for missed merges.
- **Schema** — Phase 2 agent prompts use the Haleiki concept-card schema from 0001 (with `derived_from`/`external_references` instead of book-citation fields, plus `disambiguator`, `subject_headings`, `trust_level`, `canon_status`, etc.).
- **Paths** — `content/concepts/` (flat) and `content/sources/` rather than ai-engineering's `knowledge/<kb>/...` layout.

---

## Overview

This is a **four-phase process**:

1. **Phase 0: Requirements & Scope** — Define competency questions, domain customizations, scope (which source pages, which deployment profile)
2. **Phase 1: Analysis & Planning** — Analyze source-page corpus, identify cross-page concept overlap, build agent assignments
3. **Phase 2: Parallel Extraction** — Five Opus agents extract concept cards simultaneously from assigned clusters, merge-aware
4. **Phase 3: Validation & Cross-Source Merge** — Verify structural compliance, resolve missed merges, check graph integrity

---

## Prerequisites

### 1. Environment Setup

- **Claude Code** installed and configured
- **Model access**: Coordinator should use **Opus model**, with access to spawn Opus sub-agents
- **Source pages** in place at `content/sources/` (one `.md` per source page, with valid Haleiki frontmatter)
- **Existing concept cards** at `content/concepts/` if any (will be merged into during Phase 2)
- **Profile and taxonomy** declared in `content/profile.yaml` and/or `content/taxonomy.yaml`
- **Howto guide** available: reference `docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md` for the schema, body section structure, and per-card quality bar

### 2. Directory Structure

```
haleiki/
├── content/
│   ├── profile.yaml                # deployment profile (encyclopedia/writer/knowledge-site)
│   ├── taxonomy.yaml               # categories, subcategories, tiers
│   ├── sources/                    # input — wiki source pages
│   │   ├── rust-language.md
│   │   ├── music-theory.md
│   │   ├── padmasambhava.md
│   │   └── ...
│   ├── concepts/                   # output — concept cards (flat)
│   │   ├── ownership-rust.md
│   │   ├── major-scale.md
│   │   └── ...
│   └── _analysis/                  # extraction analysis records (machine-generated)
│       └── <source-slug>/
│           ├── analysis.json
│           ├── concept-map.json
│           └── merge-log.json
├── docs/
│   └── dev/
│       └── concept-cards/
│           ├── 0001-howto-wiki-concept-card-extraction.md
│           └── 0002-a-guide-for-parallel-concept-card-extraction.md
└── ...
```

The flat `content/concepts/{slug}.md` layout means agents writing concurrently can collide on filename — see *Phase 2: Coordination* for the avoidance strategy.

### 3. Source Material Requirements

- Source pages have valid Haleiki frontmatter (slug, title, page_type: source, category, tier, genre)
- The slug index is unique (validator should confirm pre-flight)
- For mixed-genre corpora (encyclopedia + conversations + worldbuilding), each source page's `genre` is set; trust-level inheritance defaults follow accordingly
- For demo corpora pulled by `haleiki demo fetch`: the pipeline guarantees frontmatter validity

---

## Phase 0: Requirements & Scope

**Purpose**: Establish what the concept-card extraction must accomplish before touching the source corpus.

### Step 0.0: Audit the Corpus

Before planning, audit current state. Run from the repo root:

```bash
SOURCES="content/sources"
CONCEPTS="content/concepts"

# Count source pages
echo "Source pages: $(find "${SOURCES}" -name '*.md' -not -name '.gitkeep' | wc -l)"

# Count existing concept cards (may be 0 for fresh extraction)
echo "Existing concept cards: $(find "${CONCEPTS}" -name '*.md' -not -name '.gitkeep' | wc -l)"

# Categorize source pages by category
for f in "${SOURCES}"/*.md; do
  category=$(grep '^category:' "$f" | head -1 | sed 's/^category: *//; s/"//g')
  echo "$category"
done | sort | uniq -c | sort -rn

# Categorize source pages by tier
for f in "${SOURCES}"/*.md; do
  tier=$(grep '^tier:' "$f" | head -1 | sed 's/^tier: *//; s/"//g')
  echo "$tier"
done | sort | uniq -c

# Categorize source pages by genre (if present)
for f in "${SOURCES}"/*.md; do
  genre=$(grep '^genre:' "$f" | head -1 | sed 's/^genre: *//; s/"//g')
  echo "${genre:-encyclopedia}"   # default to encyclopedia if unset
done | sort | uniq -c

# Check for existing concept cards' provenance
if [ -d "${CONCEPTS}" ] && [ "$(ls -A "${CONCEPTS}" 2>/dev/null | grep -v gitkeep)" ]; then
  echo "Existing cards' derived_from references:"
  grep -A 5 '^derived_from:' "${CONCEPTS}"/*.md 2>/dev/null | grep '^\s*-\s*slug:' | sort | uniq -c | sort -rn
fi

# LLM artifact check (if existing cards)
grep -rl "Wait.*let me\|Let me recalculate\|I apologize\|As an AI" "${CONCEPTS}/" 2>/dev/null
```

Record the results:

- Total source pages (and breakdown by category, tier, genre)
- Existing concept-card count (zero is normal for fresh extraction)
- Source-page categories with high or low concept-card density (informs Phase 1 priorities)
- Any LLM artifacts in existing cards

### Step 0.1: Competency Question Elicitation

Identify **30-50 competency questions** the concept-card layer should answer. For wiki deployments, CQs typically cover:

| Type | Count Target | Example |
|------|--------------|---------|
| Definitional | 10-15 | "What is rigpa?" "What is a major scale?" |
| Relational | 8-12 | "How does dzogchen relate to nyingma?" "How do scales relate to keys?" |
| Procedural | 5-10 | "How do I construct a major scale?" "How is ownership transferred in Rust?" |
| Prerequisite | 5-8 | "What must I know before learning sonata form?" |
| Diagnostic | 5-8 | "What distinguishes the encyclopedia trust levels from the writer ones?" |
| Comparative | 5-8 | "How does Rust ownership differ from C++ RAII?" |

**Methods for generating CQs:**

1. Review source-page titles and categories — what questions does this corpus naturally invite?
2. Identify learning objectives implicit in source-page tier assignments (foundational pages often introduce CQ-worthy concepts)
3. Consider typical user queries for the deployment domain
4. Ask: "What questions would a learner at each tier ask?"
5. For mixed-corpus deployments: include CQs that span genres (e.g., "How do real and fictional treatments of X differ?")

Save to `content/_analysis/_corpus/competency-questions.md`:

```markdown
# Competency Questions for <Deployment Name>

## Definitional (What is X?)
1. What is rigpa?
2. What is a major scale?
...

## Relational (How does X relate to Y?)
1. How does dzogchen relate to nyingma?
...

## Procedural (How do I do X?)
1. How do I construct a major scale?
...

## Prerequisite (What before X?)
1. What concepts must I know before understanding sonata form?
...

## Diagnostic (What distinguishes X from Y?)
1. What is the difference between rigpa and shamatha?
...
```

### Step 0.2: Domain Taxonomy Confirmation

Confirm the deployment's `taxonomy.yaml` matches what extraction will need. Categories should be:

- Mutually exclusive (each concept fits one primary category)
- Collectively exhaustive (every concept has a category)
- Aligned with the source-page corpus

For deployments using subject_headings (cross-cutting topical tags), enumerate the controlled vocabulary upfront. Subject headings are looser than categories — they're tags for cross-tagging, not the primary classification.

**Example (for the Haleiki demo wiki):**

```yaml
categories:
  tibetan-buddhism, theoretical-physics, mathematics, music,
  programming-languages, artificial-intelligence, philosophy-of-mind,
  linguistics, botany, geology

subject_headings (cross-cutting):
  history, biography, foundations, systems, mathematics, philosophy,
  practice, language, symbiosis, structure
```

Note that some category names also appear as subject_headings — that's fine. A concept primarily classified under `mathematics` (category) might also cross-tag into `foundations` (subject heading) if it's foundational.

### Step 0.3: Notation and Domain Conventions

Document domain-specific notation requirements that agents will need to honor:

- Mathematical notation: LaTeX in `$...$` or `$$...$$` (matches the convention from the demo math fix)
- Code: fenced blocks with language hints
- Multilingual scripts: preserve as Unicode (Tibetan, Sanskrit, etc.)
- Pronunciation guides: optional `pronunciation:` field in concept frontmatter (or in body Quick Definition)
- Romanization conventions: per source

### Step 0.4: Profile and Trust/Canon Decisions

Confirm the deployment profile (`encyclopedia` / `writer` / `knowledge-site`). For each:

- Determine whether trust_level is enabled and what the default is
- Determine whether canon_status is enabled and what the default is
- Resolve any per-genre trust overrides (e.g., conversations default to `provisional`, encyclopedia articles to `credible`)

This matters for Phase 2 agent prompts: agents need to know what trust/canon to assign by default, and when to override.

---

## Phase 1: Analysis & Planning

**Model**: Use Opus for this phase.

### Step 1.1: Corpus Analysis

Use the **Explore agent** with "very thorough" mode:

```
Please analyze the source-page corpus in content/sources/ to create a comprehensive concept extraction plan. Use very thorough exploration mode.

Cross-reference against the existing concept-card inventory in content/concepts/:
- How many existing concept cards are there?
- Which source-page categories are well-covered vs. sparse?
- Are there source pages whose concepts are not yet captured as cards?
- For existing cards, which source pages are listed in derived_from?
```

The analysis should produce:

1. **Corpus metadata**:
   - Total source-page count, breakdown by category, tier, genre
   - Most-cross-referenced source pages (the "hub" pages whose concepts likely appear in many other pages)
   - Source pages with the highest concept density (long technical articles vs. short biographies)

2. **Concept inventory**:
   - Per source page: estimated concept count and key concepts
   - Total estimated concept cards (typical: ~3-7 cards per source page, varying by depth)
   - Comparison with existing card count

3. **Cross-page overlap detection** (the wiki-specific step):
   - Concepts likely to appear in multiple source pages (these become merge candidates)
   - Concepts with high disambiguation risk (multiple source pages with the same concept name but different domains)
   - Source pages that primarily cite or extend other source pages (should be processed after the cited pages)

4. **Categorization plan**:
   - Category and tier assignment per concept (typically inherits from source page, with exceptions noted)
   - Subject headings for cross-cutting concepts

5. **Disambiguation list**:
   - Concept names likely to collide (e.g., "Ownership" — Rust + property law)
   - Proposed disambiguator suffixes for each

### Step 1.2: Competency Question Mapping

Map each competency question to the source pages and candidate concepts that should answer it:

```markdown
# CQ-to-Source Mapping

## "What is rigpa?" (Definitional)
- Source pages: dzogchen, longchenpa, longchen-nyingtik, jikme-lingpa
- Candidate concept: rigpa (foundational, derived_from multiple)

## "How does Rust manage memory without a garbage collector?" (Procedural)
- Source pages: rust-language
- Candidate concepts: ownership-rust, borrow-checker, lifetimes (derived_from rust-language)

## "What distinguishes major from minor mode?" (Diagnostic)
- Source pages: music-theory
- Candidate concepts: major-scale, minor-scale, mode-quality (with contrasts_with relationships)
```

This mapping reveals:

- Concepts answering multiple CQs (high-priority, may justify hand-authoring as seed cards)
- Coverage gaps (CQs with no clear source-page match — indicates the corpus may need expansion or the CQ is out of scope)
- Cross-source concept candidates (concepts with multiple source pages in the mapping → multi-source `derived_from`)

### Step 1.2.5: Pre-Flight Disambiguation Audit

Before assigning agents, sweep the corpus for slug collisions and build the disambiguation list upfront. Three sources of collision to detect:

**(a) Cross-domain collisions** — concept names likely to appear with different meanings across the corpus. Common collision-prone names: `ownership`, `group`, `composition`, `traits`, `borrowing`, `lifetimes`, `pattern matching`, `quantum mechanics`, `field`. For each, identify which source pages would extract a concept with that name and decide on disambiguator suffixes.

**(b) Source-page-vs-concept-card collisions** — for each source page, anticipate whether the concept extracted from it will need a slug different from the source page's own slug. Source pages about a single entity (biography, "X (programming language)") will typically produce a concept card with a role/category suffix (`-teacher`, `-language`, `-field`).

**(c) Within-corpus same-name source pages** — the manifest itself may have multiple source pages with overlapping concept names. For Haleiki's demo corpus, both `group-theory` (the field) and `group-mathematics` (the algebraic structure) are present; concepts extracted from these need careful slug coordination.

Save the disambiguation list to `content/_analysis/_corpus/disambiguation-list.md`:

```markdown
# Disambiguation List for <Deployment Name>

## Cross-domain collisions (case a)

| Bare concept | Disambiguated slug | Disambiguator | Source page(s) |
|--------------|--------------------|--------------|-----------------|
| Ownership | ownership-rust | "Rust" | rust-language |
| ... | | | |

## Source-page collisions (case b)

| Source page slug | Concept card slug | Disambiguator |
|------------------|-------------------|---------------|
| padmasambhava | padmasambhava-teacher | "teacher" |
| longchenpa | longchenpa-scholar | "scholar" |
| group-theory | group-theory-mathematics | "mathematics" |
| ... | | |

## Anticipated future collisions (case c)

| Bare concept | Suffixed slug | Rationale |
|--------------|---------------|-----------|
| Traits | traits-rust | Also psychology, biology |
| ... | | |
```

This list goes into every Phase 2 agent prompt. Agents apply it directly rather than independently judging disambiguation case-by-case.

### Step 1.3: Agent Cluster Assignment

Divide work among **exactly 5 agents**. Unlike the upstream book-shaped guide (where agents take chapter ranges), wiki extraction divides by *cluster* — typically a category subset, sometimes a tier subset.

Cluster strategies:

- **By category**: Agent A takes math + foundations of physics; Agent B takes the rest of physics + theoretical-physics; etc. Best when categories are roughly balanced.
- **By tier within category**: Agent A takes foundational across all categories; Agent B takes intermediate; Agent C takes advanced; D and E split rebalancing. Best when categories are imbalanced and you want each agent to extract concepts at a similar depth.
- **By topical cluster (LoC-style)**: Agent A takes the math/physics/foundations cluster; Agent B takes the music cluster; etc. Best when subject_headings reveal natural cross-cutting clusters that don't follow the category boundary.

Whichever you pick, optimize for:

1. **Balanced workload** — Aim for ±20% concept-card-count variance across agents.
2. **Coherent clusters** — An agent should work on related source pages so cross-references within their work are local.
3. **Minimized merge collisions** — Concepts likely to appear in multiple agents' clusters should be flagged upfront. Agents should know which concepts may already be in flight by other agents.
4. **CQ coverage distribution** — Each agent should address some CQs in their cluster.

**Assignment verification checklist:**

- [ ] Every source page assigned to exactly one agent
- [ ] No gaps in coverage
- [ ] Concept count estimates balanced (±20%)
- [ ] Each agent has a thematically coherent cluster
- [ ] CQ coverage distributed across agents
- [ ] Cross-cluster concepts (likely to need merging post-extraction) flagged in agent prompts

### Step 1.4: Coordination Strategy for Cross-Cluster Concepts

Some concepts will appear in source pages assigned to multiple agents. Two strategies:

**Strategy A — Defer to Phase 3**: Each agent extracts independently. If the same concept gets two cards (from different agents), Phase 3's merge resolution catches it and merges. Simpler agent prompts; more Phase 3 work.

**Strategy B — Pre-assign cross-cutting concepts**: Identify likely cross-cutting concepts in Phase 1.3 and assign each to a specific agent (the one whose cluster has the most authoritative source). Other agents skip those concepts. Less Phase 3 work; more complex agent prompts.

For the demo and most fresh extractions, **Strategy A is recommended** — agents work independently, Phase 3 handles merging. The merge-aware extraction in 0001 (each agent checks for existing cards at the candidate slug) handles the within-agent case; Phase 3 handles the cross-agent case.

### Step 1.5: Compile Agent Instructions

For each agent, compile:

1. **Cluster assignment** — list of source-page slugs and their categories/tiers/genres
2. **Category guidance** for their cluster
3. **Tier defaults** for expected concept depth
4. **Notation conventions** (math, code, multilingual)
5. **CQs to address** — competency questions their concepts should help answer
6. **Cross-cluster concept alerts** — concepts that may appear in other agents' clusters (so they know to write disambiguator if needed, or expect Phase 3 merging)
7. **Profile axes** — trust/canon defaults and inheritance rules
8. **Existing cards in cluster** — slugs of existing concept cards in their assigned source-page range (for merge-aware extraction)
9. **Disambiguation list** — the corpus-level list from Step 1.2.5 (every agent gets the full list, not just their cluster's slice)

### Step 1.6: Source-Page Processing Order Within a Cluster

Within an agent's cluster, processing order can affect merge outcomes. Concepts that appear across multiple source pages get their first card from whichever page is processed first; subsequent pages augment via merge.

For the cleanest provenance, suggest agents process in this order within their cluster:

1. **Foundational-tier source pages first** — these typically introduce the concepts most likely to be cross-referenced from intermediate/advanced pages.
2. **Concept-dense source pages before biographical pages** — a topic article ("Music theory") usually defines a concept more cleanly than a biographical article that mentions the concept ("Bach"). Process the topic article first so the concept card is established with strong definitional content; the biographical article augments with examples.
3. **Within a tier, process by category coherence** — keep related source pages adjacent in the processing order so cross-references within the cluster resolve naturally.

Order doesn't always matter — many concepts are cluster-local. But when it matters, foundational-first is the default.

---

## Phase 2: Parallel Extraction

### Concept Card Template

Use the v1 Haleiki concept card template as documented in `0001-howto-wiki-concept-card-extraction.md` § "Complete Frontmatter Example". Reproduced briefly here for reference:

```yaml
---
# === CORE IDENTIFICATION ===
concept: "[Concept Name]"
slug: "[concept-slug]"
page_type: "concept"
disambiguator: "[only when needed for collision]"

# === CLASSIFICATION ===
category: "[primary]"
subcategory: "[optional]"
tier: "[foundational/intermediate/advanced]"
subject_headings:
  - "[cross-cutting tag]"
keywords:
  - "[search hint]"

# === PROVENANCE ===
derived_from:
  - slug: "[source-page-slug]"
    title: "[Source Page Title]"
external_references:
  - title: "[external source]"
    url: "[url]"
    license: "[license]"

# === CONFIDENCE ===
extraction_confidence: "[high/medium/low]"

# === VARIANTS ===
aliases:
  - "[variant]"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "[concept-slug]"
extends:
  - "[concept-slug]"
related:
  - "[concept-slug]"
contrasts_with:
  - "[concept-slug]"

# === PROFILE AXES ===
trust_level: "[corroborated/credible/provisional/speculative]"
canon_status: "[established/proposed/developing/seed]"   # writer profile only

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "[CQ this card addresses]"

# === LIFECYCLE ===
status: "published"
last_reviewed: "[YYYY-MM-DD]"
---

# Quick Definition
# Core Definition
# Prerequisites
# Key Properties
# Construction / Recognition
# Context & Application
# Examples
# Relationships
# Common Errors
# Common Confusions
# Source Reference
# Verification Notes
```

See 0001 for full body section guidance.

### Agent Prompt Template (Fresh Extraction, Merge-Aware)

```
Extract concept cards from the Haleiki source-page cluster <ClusterName>, writing them to content/concepts/. Be merge-aware: if a candidate slug already has a card in content/concepts/, augment it (add this source to derived_from) rather than overwriting.

## Your Cluster

Source pages assigned to you:
- content/sources/<slug-1>.md (category: X, tier: Y, genre: encyclopedia)
- content/sources/<slug-2>.md (category: X, tier: Y, genre: encyclopedia)
... [list all source pages in your cluster]

Total source pages: <N>
Estimated concept cards: ~<RangeStart>-<RangeEnd>
Existing concept cards in this cluster's expected slugs: <ExistingCount>

## Reference Documents

- **Schema and per-card extraction**: docs/dev/concept-cards/0001-howto-wiki-concept-card-extraction.md (read this first)
- **Deployment profile**: content/profile.yaml (currently <profile name>)
- **Taxonomy**: content/taxonomy.yaml

## Profile Axes

- trust_level: <enabled/disabled>; default for this profile: <default>; per-genre defaults apply (see 0001 § "Genre and Trust Defaults")
- canon_status: <enabled/disabled>; default: <default>

## Competency Questions to Address

Your concepts should help answer:
<CQsForThisCluster>

## Cross-Cluster Concept Alerts

The following concepts may also be extracted by other agents working on different clusters. If you encounter them in your source pages, extract them normally but expect that Phase 3 may merge with or augment your cards:

- <slug>: also appears in <other-cluster>'s <source-page-slug>
- ...

## Disambiguation List (corpus-wide, from Phase 1.2.5)

This is the corpus-level disambiguation list, applicable to every agent. Apply these slugs as written; do not invent variants.

**Cross-domain collisions:**

- "Ownership" → use slug `ownership-rust` (Rust sense), NOT `ownership`
- "Group" → use slug `group-mathematics` (math sense), NOT `group`
- ...

**Source-page collisions** (your concept card slug must differ from the source page slug it derives from):

- Source page `padmasambhava` → concept card slug `padmasambhava-teacher`
- Source page `longchenpa` → concept card slug `longchenpa-scholar`
- Source page `group-theory` → concept card slug `group-theory-mathematics`
- ...

**Anticipated future collisions:**

- "Traits" → use slug `traits-rust` (Rust sense), NOT `traits`
- "Borrowing" → use slug `borrowing-rust`, NOT `borrowing`
- ...

## Existing Concept Cards in Your Cluster's Slug Range

The following concept cards already exist in `content/concepts/`. When you encounter a candidate that matches one of these slugs, you must MERGE (augment `derived_from`, augment body, reconcile `trust_level` to lowest) rather than overwrite. Per 0001 § "Merge Mechanics."

- <slug>: derived from [<source-page-slugs>]
- ...

If your cluster's concepts produce a slug that matches an existing card from a previous extraction round, the same merge rule applies — read the existing card before writing.

## Extraction Process

For each source page in your cluster:

1. **Read the source page** in full to identify substantive concepts (not just the page's main topic)
2. **List candidate concepts** with their proposed slugs (apply disambiguation suffix where needed)
3. **For each candidate slug, check** content/concepts/<slug>.md:
   - **If the file does not exist**: extract fresh per the v1 template in 0001
   - **If the file exists**: read it, then merge-extract:
     - Add this source page to `derived_from`
     - Augment Examples and Context & Application sections with new source content
     - Reconcile trust_level (lowest among combined sources)
     - Update last_reviewed to today
     - Note the merge in Verification Notes
4. **Write the card** to content/concepts/<slug>.md
5. **Update the source page's frontmatter** (per 0001 § "Source Page Updates"):
   - Set `extraction_status: complete`
   - Set `concepts_generated` to the list of concept slugs you extracted from THIS source page (substantive only — not skipped mentions)
   - Set `last_extracted: <today's date>`
   - Use line-based edits to preserve existing frontmatter formatting; do not round-trip through a YAML parser unless you can guarantee field-order preservation
6. **Track your progress** — note which concepts came from which source pages, which were merges, which were fresh, in `content/_analysis/<source-slug>/extraction-log.md`

## Quality Requirements

Each card MUST have:
- [ ] All frontmatter fields populated (use null/empty array where N/A)
- [ ] page_type: concept
- [ ] disambiguator set if the concept name is in the disambiguation list
- [ ] Quick Definition ≤ 2 sentences
- [ ] Core Definition with source-page citation (use slug-based markdown links)
- [ ] At least one example from the source page text
- [ ] Prerequisites section (even if "foundational, no prerequisites")
- [ ] At least one competency question in answers_questions
- [ ] Confidence rationale in Verification Notes
- [ ] Multi-source merge note in Verification Notes if you augmented an existing card
- [ ] trust_level set per profile and genre defaults
- [ ] canon_status set if profile enables canon

## File Naming

- Filename: <slug>.md (lowercase, hyphenated, no numerical prefix)
- The slug MUST match the slug: frontmatter field exactly
- The file path is content/concepts/<slug>.md (flat, no subdirectories)
- OK: ownership-rust.md, major-scale.md, rigpa.md
- BAD: 001-ownership-rust.md, Ownership_Rust.md, content/concepts/programming/ownership.md

Work systematically through your cluster. Prioritize quality over speed. Coordinate with other agents only via the merge-aware reads — agents do not need to communicate directly.
```

**Model:** opus

### Coordination: Concurrent Filename Collisions

Five agents writing concurrently to a flat `content/concepts/` directory can race on the same filename when both try to handle a cross-cluster concept. Three avoidance strategies:

- **A. Pre-assign in Phase 1.4** (Strategy B from earlier): each cross-cluster concept is owned by exactly one agent. Other agents skip it. Eliminates the race.
- **B. Lock-by-write on first arrival**: agents check for the file and create-with-O_EXCL semantics. The second agent gets a "already exists" and switches to merge-mode. The merge-mode read-then-write isn't fully atomic but in practice agents work on different concepts most of the time, so collisions are rare.
- **C. Phase-3 deduplication**: agents create with their own scoped slug suffix (e.g., `mark-and-sweep-from-garbage-collection.md`), Phase 3 detects duplicates by `concept` field and merges into a canonical slug.

For most fresh extractions, **B is the right choice** — agents handle the rare race naturally as a merge. Strategy A adds prompt complexity; Strategy C adds Phase-3 complexity and produces ugly intermediate filenames.

---

## Phase 3: Validation & Cross-Source Merge

### Step 3.1: Structural Validation

After all agents complete, run validation:

```bash
CONCEPTS="content/concepts"

# Frontmatter completeness
for file in "${CONCEPTS}"/*.md; do
  for field in concept slug page_type category tier extraction_confidence \
               aliases prerequisites answers_questions derived_from \
               last_reviewed; do
    grep -q "^${field}:" "$file" || echo "Missing $field: $file"
  done
done

# Slug/filename consistency
for file in "${CONCEPTS}"/*.md; do
  filename=$(basename "$file" .md)
  slug=$(grep '^slug:' "$file" | head -1 | sed 's/^slug: *//; s/"//g')
  if [ "$filename" != "$slug" ]; then
    echo "MISMATCH: file=$filename slug=$slug"
  fi
done

# page_type sanity
grep -L '^page_type: *concept' "${CONCEPTS}"/*.md | head

# Profile axes presence (only required if profile enables them)
PROFILE_TRUST=$(grep -A 5 '^trust:' content/profile.yaml | grep '^\s*enabled:' | head -1)
if [[ "$PROFILE_TRUST" == *"true"* ]]; then
  for file in "${CONCEPTS}"/*.md; do
    grep -q '^trust_level:' "$file" || echo "Missing trust_level: $file"
  done
fi
```

### Step 3.2: Slug Uniqueness and Cross-Reference Resolution

```bash
# Extract all concept slugs
ls "${CONCEPTS}"/*.md | xargs -n1 basename | sed 's/.md$//' > /tmp/concept-slugs.txt

# Extract all source-page slugs
ls content/sources/*.md | xargs -n1 basename | sed 's/.md$//' > /tmp/source-slugs.txt

# All defined slugs (concepts + sources, since both use the same namespace)
cat /tmp/concept-slugs.txt /tmp/source-slugs.txt | sort | uniq > /tmp/all-slugs.txt

# Check for duplicate slugs (should be impossible since filename = slug, but YAML body could disagree)
sort /tmp/concept-slugs.txt | uniq -d

# Extract all cross-references from typed relationship fields
grep -hE '^\s*-\s*"?[a-z][a-z0-9-]*"?\s*$' "${CONCEPTS}"/*.md | sed 's/^\s*-\s*"\?//; s/"\?\s*$//' | sort | uniq > /tmp/refs.txt

# Find references to non-existent concepts/sources
comm -23 /tmp/refs.txt /tmp/all-slugs.txt > /tmp/dangling-refs.txt
[ -s /tmp/dangling-refs.txt ] && echo "Dangling references:" && cat /tmp/dangling-refs.txt
```

### Step 3.3: Cross-Source Merge Resolution

This is the wiki-specific Phase 3 step. After parallel extraction, look for concept cards that should have been merged but ended up as separate cards.

**Detection signals:**

- Multiple cards with the same `concept` field but different `slug` values, where the disambiguator is *not* set (suggests accidental separate cards rather than deliberate disambiguation)
- Cards whose body content overlaps substantially (same examples, same prerequisites)
- Cards whose `derived_from` lists overlap (same source pages cited by both → likely the same concept extracted twice)

```bash
# Find concept-name collisions where disambiguator isn't set
declare -A concepts_seen
for file in "${CONCEPTS}"/*.md; do
  concept=$(grep '^concept:' "$file" | head -1 | sed 's/^concept: *//; s/"//g')
  disambig=$(grep '^disambiguator:' "$file" | head -1 | sed 's/^disambiguator: *//; s/"//g')
  if [ -n "${concepts_seen[$concept]}" ] && [ -z "$disambig" ]; then
    echo "Collision (no disambiguator): '$concept' in ${concepts_seen[$concept]} and $file"
  fi
  concepts_seen[$concept]="$file"
done
```

**Resolution per collision:**

1. **If they're truly the same concept**: merge — combine `derived_from`, `aliases`, `prerequisites`, `related`, `contrasts_with`; combine body sections (Examples especially); reconcile trust_level (lowest); delete the redundant file; update cross-references in other cards.
2. **If they're different concepts that happen to share a name**: add disambiguator to both (or one) and rename slugs accordingly; update cross-references.
3. **If unclear**: flag for human review. Do not silently merge or split.

### Step 3.4: Trust/Canon Inheritance Audit

If the profile enables trust_level:

```bash
# For each concept with multiple derived_from, verify trust_level is the lowest among sources
for file in "${CONCEPTS}"/*.md; do
  trust=$(grep '^trust_level:' "$file" | head -1 | sed 's/^trust_level: *//; s/"//g')
  source_slugs=$(awk '/^derived_from:/,/^[a-z]/{if (/^\s*-\s*slug:/) print $0}' "$file" | sed 's/^\s*-\s*slug: *//; s/"//g')

  # Skip if no derived_from
  [ -z "$source_slugs" ] && continue

  # For each source, get its trust_level
  source_trusts=()
  while read -r slug; do
    src_trust=$(grep '^trust_level:' "content/sources/${slug}.md" 2>/dev/null | head -1 | sed 's/^trust_level: *//; s/"//g')
    source_trusts+=("$src_trust")
  done <<< "$source_slugs"

  # Compute the minimum trust (corroborated > credible > provisional > speculative)
  # ... [comparison logic, skipped here for brevity]

  # Compare with the card's trust_level; flag if not the conservative minimum
done
```

In practice this is easier in the validator (Rust) than in shell — but the audit principle stands: cards should default to the conservative minimum, and explicit overrides should be documented in Verification Notes.

### Step 3.5: CQ Coverage Check

For each competency question, verify that:

1. At least one concept card lists it in `answers_questions`
2. The card(s) actually contain enough information to answer it (sample-check)

```bash
# CQs not covered by any card
while IFS= read -r cq; do
  matches=$(grep -l "$cq" "${CONCEPTS}"/*.md | wc -l)
  if [ "$matches" -eq 0 ]; then
    echo "Uncovered CQ: $cq"
  fi
done < <(grep '^[0-9]\.' content/_analysis/_corpus/competency-questions.md | sed 's/^[0-9]*\.\s*//')
```

### Step 3.5.5: Bidirectional Provenance Consistency

Verify that every concept card's `derived_from` slug appears in the corresponding source page's `concepts_generated`, and vice versa. Any inconsistency is a back-link maintenance failure during extraction.

```bash
CONCEPTS="content/concepts"
SOURCES="content/sources"

# For each concept card, check that each derived_from source has this concept in its concepts_generated
for card in "${CONCEPTS}"/*.md; do
  [ "$(basename "$card")" = ".gitkeep" ] && continue
  card_slug=$(basename "$card" .md)
  # Extract derived_from source slugs
  source_slugs=$(awk '/^derived_from:/,/^[a-z_]+:/{if (/^\s*-\s*slug:/) print}' "$card" | sed 's/^\s*-\s*slug: *//; s/"//g')
  for src_slug in $source_slugs; do
    src_file="${SOURCES}/${src_slug}.md"
    [ ! -f "$src_file" ] && echo "MISSING SOURCE: $card_slug derives from $src_slug (not found)" && continue
    # Check the source page's concepts_generated includes this concept
    if ! grep -q "^\s*-\s*${card_slug}\s*$" "$src_file"; then
      echo "BACK-LINK MISSING: $src_slug should list $card_slug in concepts_generated"
    fi
  done
done

# Reverse direction: for each source page, every concepts_generated entry should have a card with this source in its derived_from
for src in "${SOURCES}"/*.md; do
  [ "$(basename "$src")" = ".gitkeep" ] && continue
  src_slug=$(basename "$src" .md)
  generated=$(awk '/^concepts_generated:/,/^[a-z_]+:/{if (/^\s*-\s*[a-z]/) print}' "$src" | sed 's/^\s*-\s*//; s/"//g')
  for concept_slug in $generated; do
    card_file="${CONCEPTS}/${concept_slug}.md"
    [ ! -f "$card_file" ] && echo "MISSING CARD: $src_slug claims to have generated $concept_slug (card not found)" && continue
    if ! grep -A 20 '^derived_from:' "$card_file" | grep -q "slug: *\"\?${src_slug}\"\?"; then
      echo "FORWARD-LINK MISSING: $concept_slug should list $src_slug in derived_from"
    fi
  done
done
```

If either direction shows missing links, fix them before reporting Phase 3 complete.

### Step 3.6: LLM Artifact Check

```bash
grep -rl "Wait.*let me\|Let me recalculate\|I apologize\|As an AI\|I'm sorry" "${CONCEPTS}/"
```

These signal unedited generation artifacts. Each should be reviewed and cleaned.

### Step 3.7: Quality Verification Sampling

Sample 3-5 cards from each agent's output and verify:

- [ ] Definitions accurate to source pages
- [ ] Confidence scoring justified in Verification Notes
- [ ] Prerequisites form valid chains (no circular dependencies)
- [ ] Examples cite specific source pages
- [ ] Variants capture alternative terminology
- [ ] Relationships use valid slugs
- [ ] Section names match the v1 template exactly
- [ ] Multi-source cards have merge documentation in Verification Notes
- [ ] Disambiguators set wherever names collide

---

## Troubleshooting

### Issue: Low confidence scores across many cards

**Cause**: Source pages don't provide explicit definitions (common when source pages are conversations or worldbuilding entries rather than encyclopedia articles).

**Solution**:
- Accept medium confidence as normal for synthesis-heavy genres
- Flag low-confidence cards for expert review
- Consider whether the trust_level inheritance default (per genre) is doing the right thing

### Issue: Circular prerequisites detected

**Cause**: Mutually dependent concepts extracted without proper ordering.

**Solution**:
- Identify the foundational concept in the cycle
- Mark it as `tier: foundational` (no prerequisites)
- Re-run affected agent with guidance on which concept is base

### Issue: Many orphaned concepts

**Cause**: Insufficient relationship extraction; concepts have no incoming or outgoing edges.

**Solution**:
- Review orphans — are they truly isolated foundational concepts, or missing relationships?
- Re-extract with explicit instruction to connect each concept to at least one other
- Some foundational concepts legitimately have only outgoing edges

### Issue: Competency questions unanswered

**Cause**: Coverage gap — needed concepts not extracted.

**Solution**:
- Map unanswered CQs to source pages
- Re-extract from those pages with specific CQ guidance
- If source pages don't cover the CQ, document as out-of-scope

### Issue: Multiple cards for the same concept

**Cause**: Two agents extracted the same concept independently (cross-cluster overlap).

**Solution**:
- Run Phase 3.3 cross-source merge resolution
- For each collision: merge cleanly (combining `derived_from`, examples, etc.) or disambiguate (different concepts that happen to share a name)
- If the corpus has many cross-cluster concepts, switch to Strategy A (pre-assign in Phase 1.4) for the next extraction round

### Issue: Disambiguator collisions discovered late

**Cause**: A concept name collision wasn't anticipated in Phase 1.3, so two cards have the same bare slug.

**Solution**:
- Rename the conflicting slugs with disambiguators (`ownership-rust`, `ownership-property-law`)
- Update cross-references via search-and-replace in `content/concepts/*.md` and `content/sources/*.md`
- Add to the disambiguation list for future extractions

### Issue: Trust level inheritance produces too-conservative defaults

**Cause**: A concept that's well-established in domain literature is extracted from a source page with a `provisional` trust level (e.g., a conversation), so the default conservative inheritance assigns it `provisional`.

**Solution**:
- Override `trust_level` explicitly in the card frontmatter
- Document in Verification Notes: "Trust level overridden from default `provisional` (inherited from conversation source) to `credible` because the concept is well-established in [domain]; the conversation merely re-introduces it."

### Issue: Cards written to the wrong path

**Cause**: An agent used a legacy path (e.g., `concepts/<source-slug>/<concept-slug>.md` from the upstream book-shaped doc) instead of the flat Haleiki layout.

**Solution**:
- Verify the agent prompt spells out `content/concepts/<concept-slug>.md` as the flat target
- Move any mis-located cards to the correct location
- Update cross-references if any cards in the wrong location were referenced

---

## Workflow Summary

### Phase 0: Requirements (30-60 minutes)

1. Audit corpus state (run gap-analysis scripts)
2. Generate 30-50 competency questions
3. Confirm domain taxonomy and subject_headings vocabulary
4. Document notation conventions
5. Confirm profile axes (trust/canon) settings
6. Save to `content/_analysis/_corpus/`

### Phase 1: Analysis (20-40 minutes)

1. Run Explore agent on source-page corpus
2. Map CQs to source pages and candidate concepts
3. Identify cross-page concept overlap and disambiguation candidates
4. Choose cluster strategy (by category, by tier, by topical cluster)
5. Compile 5 balanced agent assignments
6. Verify assignments (no gaps, balanced, CQ coverage, cross-cluster concepts flagged)

### Phase 2: Parallel Extraction (30-90 minutes wall, depending on corpus size)

1. Launch all 5 agents in parallel with cluster-specific prompts
2. Monitor for completion; agents handle merge-aware extraction independently
3. Optionally: post-processing cleanup (whitespace, heading consistency)

### Phase 3: Validation & Cross-Source Merge (20-40 minutes)

1. Structural validation (frontmatter, body sections)
2. Slug uniqueness and cross-reference resolution
3. Cross-source merge resolution (the wiki-specific step)
4. Trust/canon inheritance audit
5. CQ coverage check
6. LLM artifact check
7. Quality verification sampling
8. Re-extract or merge as needed

**Total: ~100-230 minutes** for complete, validated extraction across a corpus.

For Haleiki's 91-source-page demo, expect Phase 2 to take 60-90 minutes (the source pages are diverse and many; CC's parallel agents are throughput-bound on extraction quality, not API latency) and Phase 3 to take 30-45 minutes if cross-cluster concepts are common.

---

## Version History

- **v1** (Haleiki): First version. Adapted from Billo Systems' AI Engineering parallel re-extraction guide v3.2 (`workbench/0010-a-guide-for-parallel-concept-card-re-extraction-v3.2.md` for reference). Reframed from book-shaped re-extraction to wiki-shaped fresh-and-merge extraction; replaced chapter-based agent assignment with source-page-cluster-based assignment; promoted cross-source concept merging to a first-class Phase 3 step; updated path conventions to Haleiki's flat `content/concepts/` layout; updated agent prompts to use the v1 Haleiki concept-card schema (with `derived_from`, `external_references`, `disambiguator`, `subject_headings`, `trust_level`, `canon_status`); added genre-aware trust inheritance to agent prompts. The four-phase structure, parallel-Opus pattern, validation checklist, and quality bar are unchanged.
- **v1.1** (Haleiki): Refinements from the 5-article pilot extraction. Added Step 1.2.5 — pre-flight disambiguation audit before agent assignment, producing a corpus-level disambiguation list covering all three cases (cross-domain, source-page, anticipated future). Added Step 1.6 — source-page processing order within an agent's cluster (foundational and concept-dense first). Updated the agent prompt template to embed the corpus-wide disambiguation list and to explicitly call out merge-aware augmentation of any pre-existing concept cards (e.g., from prior pilot extractions). No changes to the four-phase structure or validation checklist.
- **v1.2** (Haleiki): Source-page back-link maintenance. Updated the Phase 2 agent prompt template to include a mandatory source-page-update step (set `extraction_status`, populate `concepts_generated`, set `last_extracted`) after extracting concepts from each source page. Added Step 3.5.5 — bidirectional provenance consistency check verifying that concept cards' `derived_from` and source pages' `concepts_generated` agree in both directions. Pre-v1.2 corpora need a one-time back-fill (the concept cards' `derived_from` is correct; only source pages need updating from a scan of the concept-card index).

---

## Appendix: Quick Reference

### Confidence Scoring

| Level | Criteria |
|-------|----------|
| High | Source page explicitly defines concept; extraction is straightforward |
| Medium | Concept discussed but not formally defined; requires synthesis |
| Low | Concept inferred or reconstructed from context |

### Relationship Types

| Type | Direction | Semantics |
|------|-----------|-----------|
| prerequisites | Incoming | Must know this first |
| extends | Outgoing | This builds upon |
| related | Symmetric | Associated, non-hierarchical |
| contrasts_with | Symmetric | Often confused with |

Profiles may add: `inspires`, `supersedes`, `real_world_analogue`, `narrative_adaptation_of`, `contradicts_in_universe` (writer profile).

### Trust Level Inheritance

For multi-source concept cards (`derived_from` has multiple entries):

- **Default**: lowest trust_level among the source pages (conservative)
- **Override**: explicit per-card setting, documented in Verification Notes

### Genre Trust Defaults (per source page)

| Genre | Default trust_level |
|-------|---------------------|
| encyclopedia | credible |
| conversation | provisional |
| worldbuilding | speculative |
| essay / blog | credible |

### Required Frontmatter Fields

```yaml
concept:                # REQUIRED
slug:                   # REQUIRED
page_type: concept      # REQUIRED
category:               # REQUIRED
tier:                   # REQUIRED
derived_from:           # REQUIRED (empty array allowed for hand-authored seed cards)
extraction_confidence:  # REQUIRED
aliases:                # REQUIRED (empty array if none)
prerequisites:          # REQUIRED (empty array if foundational)
answers_questions:      # REQUIRED (at least one CQ)
last_reviewed:          # REQUIRED (date)
status:                 # REQUIRED (typically "published")
trust_level:            # REQUIRED if profile enables trust
canon_status:           # REQUIRED if profile enables canon
disambiguator:          # OPTIONAL (only when concept name collides)
subcategory:            # OPTIONAL
subject_headings:       # OPTIONAL
keywords:               # OPTIONAL
extends:                # OPTIONAL (empty array allowed)
related:                # OPTIONAL (empty array allowed)
contrasts_with:         # OPTIONAL (empty array allowed)
external_references:    # OPTIONAL
stable_since:           # OPTIONAL
```

### Per-Card Extraction Checklist (Agent Self-Check)

```
[ ] Source page read in full
[ ] Concept boundaries identified
[ ] Disambiguator suffix applied to slug if name collides
[ ] Existing card checked at candidate slug (merge if exists, fresh otherwise)
[ ] All frontmatter fields populated per schema
[ ] All body sections present per template
[ ] Slug matches filename
[ ] Card path is content/concepts/<slug>.md (flat)
[ ] derived_from cites the source page slug correctly
[ ] trust_level set per profile + genre defaults (or explicit override with rationale)
[ ] At least one competency question in answers_questions
[ ] Verification Notes document confidence rationale and any merge actions
```
