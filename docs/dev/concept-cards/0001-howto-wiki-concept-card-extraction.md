# HOWTO: Wiki Concept Card Extraction

**Haleiki v1.**

A practical guide to creating high-quality concept cards from Haleiki source pages — Wikipedia-derived articles, captured conversations, worldbuilding entries, essays, blog posts, and any other authored prose serving as primary content. For parallel extraction across many source pages at once, see [`0002-a-guide-for-parallel-concept-card-extraction.md`](./0002-a-guide-for-parallel-concept-card-extraction.md).

---

## Lineage

This guide derives from Billo Systems' AI Engineering concept-card howto (v3.2), adapted for the Haleiki wiki framework. The confidence scoring, typed relationship vocabulary, body section structure, and competency-question framework are unchanged from the upstream — those parts of the design transfer directly. Major adaptations from the upstream version:

- **Provenance model** — Replaced book-citation fields (`source`, `source_slug`, `chapter`, `pdf_page`) with wiki-pointer fields (`derived_from`, `external_references`). Concept cards point to Haleiki source pages, which carry the upstream attribution in their own frontmatter; concept cards inherit the chain transitively.
- **Disambiguation** — Added `disambiguator` field (display-form for collision cases like `ownership-rust` vs `ownership-property-law`). Slug is the file is the URL is the cross-reference; disambiguation lives in the slug suffix when needed.
- **Cross-cutting topical organization** — Added `subject_headings` (LoC-style controlled vocabulary, multi-valued) alongside the single primary `category`. Concepts that span topical buckets (e.g., a math concept also relevant to physics) get cross-tagged via `subject_headings`.
- **Profile-aware metadata** — Added `trust_level` and `canon_status` from `0005-deployment-profiles-epistemic-architecture.md`, with explicit inheritance rules for the multi-source case.
- **Source-page genre awareness** — Concept extraction works the same regardless of source-page genre (encyclopedia article, conversation, worldbuilding entry, essay), but trust-level inheritance defaults vary by genre. See *Genre and Trust Defaults* below.
- **Cross-source merging** — Explicit guidance for the case where the same concept is extractable from multiple source pages, with provenance combining cleanly via `derived_from`.
- **File organization** — Flat `content/concepts/{slug}.md` layout, no per-source subdirectories. Disambiguation lives in the slug, not the path.

---

## What Are Concept Cards?

Concept cards are atomic knowledge units that capture a single concept in a structured, queryable format. Each card has:

1. **YAML Frontmatter** — Machine-readable metadata for search, filtering, graph construction, profile-aware rendering
2. **Markdown Body** — Human-readable exposition organized into standardized sections

The dual format serves three consumers:

- **Humans** reading for understanding
- **The Haleiki build pipeline** building the relationship graph, computing derived data, and validating cross-references
- **LLMs** accessing structured knowledge via MCP servers, agent tools, or other downstream consumers

In the Haleiki content model, concept cards complement source pages. A source page is freeform authored prose (a Wikipedia article, a worldbuilding entry, a captured conversation, an essay) — published as written. A concept card is a synthesis: it extracts a single concept from one or more source pages and structures it for graph navigation.

---

## The Golden Rules

### 1. One Concept, One Card

Never combine multiple concepts into a single card, even if they're related. If you find yourself writing "X and Y" as the concept name, you need two cards.

**Bad**: "Major and Minor Scales" — One card trying to cover two concepts
**Good**: "Major Scale" + "Minor Scale" — Two cards, cross-referenced

### 2. Source-Faithful, Not Source-Copied

Extract the concept as the source presents it, but in your own structured format. Don't plagiarize paragraphs — synthesize and structure.

**Bad**: Copy-pasting source paragraphs into sections
**Good**: Extracting the essence into structured sections with source citations

### 3. Explicit Relationships

Every concept exists in a web of relationships. Make these explicit in typed fields, not buried in prose.

**Bad**: "This concept is related to several others including..."
**Good**:

```yaml
prerequisites:
  - interval
  - scale-degree
extends:
  - triad
related:
  - chord-inversion
contrasts_with:
  - minor-triad
```

### 4. Confidence Is Information

Not all extractions are equally reliable. Signal this in the `extraction_confidence` field.

| Confidence | When to Use |
|------------|-------------|
| **high** | Source explicitly defines the concept with clear terminology |
| **medium** | Concept is present but definition must be synthesized from context |
| **low** | Concept is inferred, implied, or reconstructed |

### 5. Provenance Is Sacred

Every assertion should be traceable. In Haleiki, concept cards point to source pages via `derived_from`; source pages carry upstream attribution (Wikipedia URL, license, revision_id) in their own frontmatter. Concept cards inherit the upstream chain transitively — they don't need to re-cite the upstream URL.

When a source page itself adapts external material that warrants an explicit reference at the concept-card level (e.g., a specific RFC, a technical paper, a primary text not represented as a source page), use `external_references`. This is rare in practice — most concept cards just need `derived_from`.

Within the body, cite source pages by slug for traceability:

> The major scale is a diatonic scale consisting of seven distinct pitch classes... ([music-theory](/source/music-theory/))

### 6. Disambiguate When Concepts Collide

Slugs are globally unique across both source pages and concept cards (they share a single namespace). When the bare concept name would produce a slug collision, add a disambiguator suffix.

There are three cases that require disambiguation:

**(a) Cross-domain collision** — the same name names different concepts in different domains:

- `ownership-rust` (Rust's ownership concept)
- `ownership-property-law` (property law's ownership concept)
- `quantum-mechanics` (the physics theory) / `quantum-mechanics-kala-sara` (a fictional worldbuilding version)

**(b) Source-page-vs-concept-card collision** — a concept card extracted from a source page about the same entity needs a slug different from the source page's slug:

- Source page `padmasambhava` (the Wikipedia article) + concept card `padmasambhava-teacher` (the cross-cutting concept of Padmasambhava as a teacher figure)
- Source page `longchenpa` (the Rigpa Wiki article) + concept card `longchenpa-scholar` (Longchenpa as a scholarly authority)
- Source page `group-theory` (the Wikipedia article) + concept card `group-theory-mathematics` (group theory as the mathematical field)

For this case, choose a role/category suffix that captures the concept's *function* in the wiki: `-teacher`, `-scholar`, `-field`, `-mathematics`, `-language`, etc. The suffix should make sense as a disambiguator value in display (e.g., "Padmasambhava (teacher)" reads naturally).

When a concept card would have the same identity as a source page about it, you have two options:

1. Don't extract a concept card for the entity — let the source page be the canonical representation, and use concept cards for atomic facts *about* the entity.
2. Extract the concept card with a role/category suffix (the case above) — useful when the concept is heavily cross-referenced from other articles and benefits from cross-cutting metadata.

Use option 2 when the concept is cited or extended by many other source pages; use option 1 when the source page is mostly self-contained and other articles only mention the entity by name.

**(c) Anticipated future collision** — the bare name is so widely used across domains that even if no collision exists in the corpus today, one is plausible:

- `traits-rust` (Rust traits) — "traits" is also used in psychology, biology, character analysis
- `pattern-matching-rust` (Rust pattern matching) — also used in cognition, vision systems
- `borrowing-rust` (Rust's borrow rules) — also used for loans, library books, etc.

For (c), apply the suffix proactively even when the bare slug is currently free. The cost is small (a few extra characters in the slug); the benefit is stable URLs as the corpus grows.

For names that are *nearly uniquely* tied to one domain — `rigpa`, `eigenvalue`, `palestrina`, `tectonics` — bare slugs are fine. Don't add suffixes "just in case" for unambiguous names. The judgment call is whether the bare name plausibly means something else in another domain you might add later.

The `concept` field carries the human-readable name (which can repeat); the `slug` is unique with disambiguation; the `disambiguator` field drives display in UI tiles, breadcrumbs, search results, and the article overline.

If you discover a collision after the fact, the validator will surface it. Renaming a slug means updating cross-references in all cards that point to it — search-and-replace works since slugs appear as bare strings in YAML lists.

### 7. Profile-Aware Metadata

Concept cards include `trust_level` and `canon_status` fields when the deployment profile enables those axes (per `0005-deployment-profiles-epistemic-architecture.md`).

When a concept is derived from multiple source pages, the inheritance rule is:

- **`trust_level`** — Defaults to the *lowest* among source pages. Conservative: a concept is only as well-sourced as its weakest source. Override explicitly if you have a reason.
- **`canon_status`** — Does *not* auto-inherit from multiple sources; if sources disagree, set explicitly. The validator warns if multi-source canon is ambiguous.

When a single source's trust or canon changes after extraction, derived concept cards are flagged for review by the validator.

---

## Frontmatter: Field by Field

### Core Identification

```yaml
concept: "Ownership"
slug: "ownership-rust"
page_type: "concept"
disambiguator: "Rust"           # optional, only when needed for display
```

- **concept** — Human-readable name, properly capitalized. Can repeat across cards if disambiguator distinguishes them.
- **slug** — The filename (without `.md`), URL fragment, and cross-reference target. Must be globally unique. Lowercase, hyphens, no special characters.
- **page_type** — Always `concept` for concept cards. Discriminates from `source`, `brief`, `sketch`, etc.
- **disambiguator** — Display-form for collision cases. Optional. When present, the wiki UI renders the article overline as "Ownership (Rust)" rather than just "Ownership."

**Slug rules:**

- All lowercase
- Hyphens for spaces
- No special characters
- Match the filename exactly
- Append a domain-or-context suffix when the bare name would collide (`ownership-rust`, not `ownership`)

### Classification

```yaml
category: "memory-model"
subcategory: "ownership-system"
tier: "foundational"
subject_headings:
  - "memory-management"
  - "type-systems"
keywords:
  - "move semantics"
  - "RAII"
  - "resource management"
```

- **category** — Primary topical bucket (single value, required). Drawn from the deployment's `taxonomy.yaml` or `profile.yaml`.
- **subcategory** — Optional finer classification (single value).
- **tier** — Prerequisite depth: `foundational` | `intermediate` | `advanced`. See *Tier Assignment* below.
- **subject_headings** — Cross-cutting topical tags (multi-valued). LoC-style controlled vocabulary. A concept lives in one primary `category` but can be cross-tagged into other topical buckets it relates to. The build graph exposes concepts via primary category *and* via shared subject heading.
- **keywords** — Free-form query terms. Distinct from `aliases` (authority-control variants of the concept's name) — keywords are search hints, not alternative names.

### Provenance

```yaml
derived_from:
  - slug: "rust-language"
    title: "Rust (programming language)"
  - slug: "memory-management-rust-conversation-2026-04-15"
    title: "Conversation on Memory Management in Rust"
external_references:
  - title: "The Rust Programming Language"
    chapter: "Understanding Ownership"
    url: "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html"
    license: "MIT/Apache-2.0"
```

- **derived_from** — One or more Haleiki source pages this concept was extracted from. Each entry has `slug` (the source page's slug, used for cross-reference resolution) and `title` (human-readable, for display in the provenance section). Required for non-foundational concepts.
- **external_references** — Optional. Use when a concept warrants an explicit external citation that isn't carried by any of the `derived_from` source pages. Rare. Each entry has `title`, optional `chapter`, `url`, and `license`.

For concepts derived from multiple source pages, the build pipeline aggregates the upstream attribution (Wikipedia URLs, licenses, revision IDs) from each `derived_from` source page into the concept card's rendered provenance section.

### Confidence

```yaml
extraction_confidence: "high"
```

Document the rationale in the *Verification Notes* body section. Vary realistically: not every extraction is high-confidence. A concept that requires synthesizing across paragraphs is `medium`; one inferred from context is `low`.

### Variants (Authority Control)

```yaml
aliases:
  - "ownership system"
  - "ownership model"
  - "Rust ownership"
```

Capture all alternative names readers might search for, including:

- Synonyms
- Abbreviations
- Notational forms
- Historical names
- Common misspellings (for search)

**YAML quoting rule:** Any alias containing a colon (`:`) MUST be quoted. YAML interprets `key: value` as a mapping, not a string.

```yaml
# BAD — YAML parses this as a map, not a string:
  - Kotlin variant: panache-kotlin

# GOOD — quotes prevent misparse:
  - "Kotlin variant: panache-kotlin"
```

This applies to all string-valued list entries, not just aliases.

### Typed Relationships

```yaml
prerequisites:
  - "variable-binding"
  - "stack-and-heap"
extends:
  - "raii"
related:
  - "borrowing"
  - "lifetimes"
  - "clone-and-copy"
contrasts_with:
  - "garbage-collection"
  - "manual-memory-management"
```

**Use exact slugs** that match existing or planned card filenames.

**Relationship semantics:**

- **prerequisites** — Concepts that MUST be understood before this one (directed)
- **extends** — Concepts this one builds upon or elaborates (directed)
- **related** — Non-hierarchical associations (symmetric)
- **contrasts_with** — Commonly confused concepts (symmetric)

Profiles may define additional relationship types (the `writer` personality adds `inspires`, `supersedes`, `real_world_analogue`, `narrative_adaptation_of`, `contradicts_in_universe`). When extracting for a profile that enables additional types, use them where appropriate.

### Profile Axes (Trust and Canon)

```yaml
trust_level: "credible"          # corroborated | credible | provisional | speculative
canon_status: "established"       # established | proposed | developing | seed (writer profile only)
```

Required when the deployment profile enables these axes. Multi-source inheritance rules:

- `trust_level` — defaults to the *lowest* among `derived_from` source pages
- `canon_status` — does not auto-inherit; set explicitly when sources disagree

If omitted when the profile requires them, the build defaults from `profile.yaml` apply, and the validator warns.

### Competency Questions

```yaml
answers_questions:
  - "What is ownership in Rust?"
  - "How does Rust manage memory without a garbage collector?"
  - "What are the three rules of ownership?"
```

Link to the competency questions this card helps answer. At least one CQ per card.

### Lifecycle Metadata

```yaml
status: "published"               # draft | review | published
stable_since: "1.0"               # optional, for concepts in versioned domains (programming languages, etc.)
last_reviewed: "2026-05-01"       # date of last manual review
```

- **status** — Editorial state. `published` is the default; `draft` and `review` flag work-in-progress.
- **stable_since** — Optional. For concepts tied to a versioned external system (e.g., a Rust language feature stable since 1.0), record the version it became stable.
- **last_reviewed** — Date of last human review. Useful for the validator to flag concepts that haven't been touched since their source pages were updated.

### Complete Frontmatter Example

```yaml
---
# === CORE IDENTIFICATION ===
concept: "Ownership"
slug: "ownership-rust"
page_type: "concept"
disambiguator: "Rust"

# === CLASSIFICATION ===
category: "memory-model"
subcategory: "ownership-system"
tier: "foundational"
subject_headings:
  - "memory-management"
  - "type-systems"
keywords:
  - "move semantics"
  - "RAII"
  - "resource management"

# === PROVENANCE ===
derived_from:
  - slug: "rust-language"
    title: "Rust (programming language)"
external_references:
  - title: "The Rust Programming Language"
    chapter: "Understanding Ownership"
    url: "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html"
    license: "MIT/Apache-2.0"

# === CONFIDENCE ===
extraction_confidence: "high"

# === VARIANTS ===
aliases:
  - "ownership system"
  - "ownership model"
  - "Rust ownership"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - "variable-binding"
  - "stack-and-heap"
extends:
  - "raii"
related:
  - "borrowing"
  - "lifetimes"
  - "clone-and-copy"
contrasts_with:
  - "garbage-collection"
  - "manual-memory-management"

# === PROFILE AXES ===
trust_level: "credible"

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is ownership in Rust?"
  - "How does Rust manage memory without a garbage collector?"
  - "What are the three rules of ownership?"

# === LIFECYCLE ===
status: "published"
stable_since: "1.0"
last_reviewed: "2026-05-01"
---
```

---

## Body Sections: Best Practices

The body structure is unchanged from the upstream v3 howto. All concept cards include the following sections in this order. Sections marked *recommended* may be omitted only when truly inapplicable.

### Quick Definition

**Purpose**: Fast lookup for someone who knows the concept but needs a reminder.

**Guidelines:**

- Maximum 2 sentences
- No jargon that isn't in the concept name itself
- Write for quick scanning

**Example:**
> A major scale is a seven-note diatonic scale with a specific pattern of whole and half steps (W-W-H-W-W-W-H). It's the most fundamental scale in Western tonal music.

### Core Definition

**Purpose**: Authoritative technical reference definition.

**Guidelines:**

- Quote the source directly if it provides a clear definition (cite source page slug)
- Include all defining characteristics
- Use proper technical terminology

**Example:**
> The major scale is a diatonic scale consisting of seven distinct pitch classes arranged in the intervallic pattern: whole step, whole step, half step, whole step, whole step, whole step, half step (W-W-H-W-W-W-H). When constructed from C, this produces the "white key" scale C-D-E-F-G-A-B-C with no sharps or flats. The major scale defines the intervallic foundation for major keys and serves as the reference point for describing other scales and modes ([music-theory](/source/music-theory/)).

### Prerequisites

**Purpose**: Explicit dependency chain for learning paths.

**Guidelines:**

- List every concept needed to understand this one
- State WHY each is required
- Use bold for concept names
- For foundational concepts: explicitly state "No prerequisites"

**Example:**
>
> - **Half step** — The major scale's structure is defined by half-step placements
> - **Whole step** — Most intervals in the major scale are whole steps
> - **Scale degree** — Understanding major scale requires knowing how to identify positions within a scale

**For foundational concepts:**
> This is a foundational concept with no prerequisites within the wiki.

### Key Properties (recommended)

**Purpose**: Enumerated characteristics that define the concept.

**Guidelines:**

- Use numbered list
- Each property should be verifiable
- Include edge cases and boundary conditions

### Construction / Recognition (if applicable)

**Purpose**: Procedural knowledge — how to construct, identify, or apply the concept.

**Guidelines:**

- Use numbered steps
- Separate construction from recognition if both apply
- Be explicit enough that someone could follow the steps

### Context & Application (recommended)

**Purpose**: When, where, and why this concept matters.

**Guidelines:**

- Be specific to the domain
- Include historical/stylistic context if relevant
- Mention typical usage patterns

### Examples

**Purpose**: Concrete examples drawn from source material.

**Guidelines:**

- Always cite the source page slug
- Use source-page examples, not generic ones
- Include any worked examples the source provides

**Example:**
> **Example 1** ([music-theory](/source/music-theory/)): The C major scale (C-D-E-F-G-A-B-C) demonstrates the pattern using all natural notes.
>
> **Example 2** ([johann-sebastian-bach](/source/johann-sebastian-bach/)): Bach's Prelude in C from the Well-Tempered Clavier exemplifies extensive use of the C major scale.

### Relationships (recommended)

**Purpose**: Prose elaboration of the typed relationships in frontmatter.

**Guidelines:**

- Repeat frontmatter relationships with additional context
- Explain the nature of each relationship
- Use concept names that match card slugs

### Common Errors (if applicable)

**Purpose**: Practical mistakes when applying the concept.

**Guidelines:**

- Focus on procedural errors (doing it wrong)
- Provide corrections
- These are "how you mess up" not "what you misunderstand"

### Common Confusions (if applicable)

**Purpose**: Conceptual misunderstandings about what this IS.

**Guidelines:**

- Focus on what people wrongly believe about the concept
- Provide clarifications
- These are "what you misunderstand" not "how you mess up"

### Source Reference

**Purpose**: Full citation for traceability.

**Guidelines:**

- List each source page in `derived_from` with a brief note about what it contributes
- Note any external references and why they're cited at the concept level rather than via the source page

**Example:**
> Derived from:
> - [music-theory](/source/music-theory/) — provides the formal definition and interval pattern
> - [johann-sebastian-bach](/source/johann-sebastian-bach/) — provides historical context and exemplary uses
>
> External references:
> - Hutchinson, "Music Theory for the 21st-Century Classroom," ch. 3 — cited because the formal pattern definition predates Wikipedia coverage and is foundational to Western music theory

### Verification Notes

**Purpose**: Internal tracking for quality assurance.

**Guidelines:**

- Document what was directly quoted vs. synthesized
- Justify confidence level
- Note any uncertainties
- Flag cross-references for verification
- For multi-source extractions, note how content from different sources was combined

**Example:**
> - Definition: Adapted from [music-theory](/source/music-theory/), section "Diatonic Scales"
> - Key Properties: Items 1-4 explicit in source; item 5 synthesized from discussion of leading-tone resolution
> - Construction steps: Adapted from worked examples in [music-theory](/source/music-theory/)
> - Confidence: HIGH — explicit definition and extensive examples in source
> - Multi-source merge: Examples in `Examples` section combine illustrative cases from [music-theory](/source/music-theory/) (theoretical) and [johann-sebastian-bach](/source/johann-sebastian-bach/) (applied); the bach reference is purely illustrative and does not change the concept's tier or trust level
> - Cross-references: All slug references verified against current concept inventory
> - Uncertainties: None

---

## File Organization

Concept cards live flat in `content/concepts/`. The slug is the filename is the URL fragment is the cross-reference target.

```
content/
├── concepts/
│   ├── ownership-rust.md
│   ├── ownership-property-law.md
│   ├── quantum-mechanics.md
│   ├── quantum-mechanics-kala-sara.md
│   ├── major-scale.md
│   ├── ...
│   └── padmasambhava.md
├── sources/
│   ├── rust-language.md
│   ├── music-theory.md
│   ├── ...
│   └── padmasambhava.md
├── taxonomy.yaml
└── profile.yaml
```

**Why flat?**

- The slug is globally unique by validator construction; uniqueness obviates the need for path-based namespacing.
- A concept that spans multiple categories (e.g., `homotopy` legitimately in both math/topology and physics/string-theory) doesn't need to live in two places — `category` is the primary bucket, `subject_headings` carries the cross-cutting tags.
- Slug lookups are O(1). Cross-reference resolution is direct.
- Recategorization is a frontmatter edit, not a file move. `git log` and `git blame` keep working.

If you find yourself wanting subdirectories, the answer is almost always "use `subject_headings` and let the build graph drive the navigation, not the directory structure."

### Naming Conventions

| Element | Format | Example |
|---------|--------|---------|
| Concept slug | `<concept-slug>` (kebab-case) | `major-scale`, `ownership-rust` |
| Concept card filename | `<concept-slug>.md` | `major-scale.md`, `ownership-rust.md` |
| Source page slug | `<source-slug>` (kebab-case) | `music-theory`, `rust-language` |
| Source page filename | `<source-slug>.md` | `music-theory.md`, `rust-language.md` |

### Slug Rules

- All lowercase
- Hyphens for word separation
- No underscores, spaces, or special characters
- No numerical prefixes
- Match exactly between `slug:` field and filename
- Globally unique across both source pages and concept cards (the validator enforces)

**Good slugs:**

- `major-scale`
- `dominant-seventh-chord`
- `ownership-rust`
- `quantum-mechanics-kala-sara`

**Bad slugs:**

- `Major-Scale` (uppercase)
- `major_scale` (underscore)
- `001-major-scale` (prefix)
- `majorscale` (no separation)
- `ownership` (would collide with property-law and other ownership concepts; use disambiguator)

### Disambiguation Strategy

Three cases (recap from Golden Rule 6):

**(a) Cross-domain name collision:**

| Bare concept | Disambiguated slugs | Disambiguator field |
|--------------|---------------------|---------------------|
| Ownership | `ownership-rust`, `ownership-property-law` | "Rust", "property law" |
| Quantum mechanics | `quantum-mechanics`, `quantum-mechanics-kala-sara` | (none for canonical), "Kāla Sāra" |
| Group | `group-mathematics`, `group-music-theory` | "mathematics", "music theory" |
| Composition | `composition-music`, `composition-functional-programming` | "music", "functional programming" |

**(b) Source-page-vs-concept-card collision:**

| Source page slug | Concept card slug | Disambiguator field |
|------------------|-------------------|---------------------|
| `padmasambhava` | `padmasambhava-teacher` | "teacher" |
| `longchenpa` | `longchenpa-scholar` | "scholar" |
| `group-theory` | `group-theory-mathematics` | "mathematics" |
| `rust-language` | (no card needed; use atomic concepts derived from the source page) | — |

For (b), the suffix is a role or category that captures the concept's function in the wiki. Pick a suffix that reads naturally as a disambiguator value in display ("Padmasambhava (teacher)", "Group theory (mathematics)").

**(c) Anticipated future collision:**

| Bare concept | Suffixed slug | Rationale |
|--------------|---------------|-----------|
| Traits | `traits-rust` | "traits" is also psychology, biology, character — likely future collision |
| Borrowing | `borrowing-rust` | "borrowing" is also library, loans, etc. |
| Pattern matching | `pattern-matching-rust` | "pattern matching" appears in cognition, vision, etc. |

For (c), apply the suffix proactively. For names nearly uniquely tied to one domain (`rigpa`, `eigenvalue`, `palestrina`, `tectonics`, `parinirvana`), use bare slugs — don't add suffixes for unambiguous names.

**Choosing the canonical slug:**

When two concepts genuinely share a name across domains, one typically gets the bare slug and others get suffixes. The canonical (bare-slug) version is usually:

- The most widely searched-for in the wiki's domain
- The most extensively cross-referenced from other articles
- The earliest established in the corpus

There's no automated rule. Use judgment. If unclear, give *both* concepts a suffix and reserve the bare slug for future use.

**The `disambiguator` field in display:**

- Article overline: "Ownership (Rust) · memory-model"
- Search result: "Ownership (Rust)"
- See-also tile: "Ownership (Rust)"

Without `disambiguator`, the display falls back to just the `concept` name. For non-collision cases, leave `disambiguator` unset.

---

## Genre and Trust Defaults

Source pages carry a `genre` field that signals what kind of authored prose they contain. Concept extraction works the same regardless of genre, but trust-level inheritance defaults vary.

| Source genre | Default trust | Canon usage | Notes |
|--------------|---------------|-------------|-------|
| `encyclopedia` | `credible` | Disabled (encyclopedia profile) | Wikipedia-derived, third-person reference voice |
| `conversation` | `provisional` | Disabled (encyclopedia) / Enabled (writer) | Captured dialog. Trust default is provisional because exploratory dialog is exploratory. |
| `worldbuilding` | `speculative` | Enabled (writer) | In-universe entries. Default trust is speculative; canon status carries the load. |
| `essay` / `blog` | `credible` | Disabled (encyclopedia) | Long-form opinion or exploration. Default trust is credible — well-considered — but may vary. |

When extracting concepts from a single source page, the concept's `trust_level` defaults to the source page's trust level (which may itself be the genre default or an override).

When extracting from multiple source pages of mixed genre, the conservative default applies: take the lowest trust among the sources. A concept that appears in both an encyclopedia article (`credible`) and a conversation (`provisional`) inherits `provisional`. Override explicitly if the concept is well-established despite being mentioned in a low-trust source.

---

## Tier Assignment

The `tier` field captures prerequisite depth:

- **`foundational`** — No prerequisites within the wiki. Must be understood before other concepts.
- **`intermediate`** — Requires foundational concepts.
- **`advanced`** — Requires intermediate concepts.

Tier is a global judgment about the concept itself, not about how the source page presents it. A "foundational" concept in one knowledge base may be "advanced" in another — assign per the deployment's overall corpus.

For the demo wiki: tier is taken from the source page's `tier` field by default, but a concept extracted from an `advanced`-tier source page may itself be `foundational` (e.g., a basic concept casually invoked in an advanced article). Override per concept as appropriate.

---

## Cross-Source Concept Merging

When a concept is genuinely the same across multiple source pages, produce a single concept card with multiple `derived_from` entries — don't create multiple cards.

### When to Merge vs. Create Separate Cards

**Merge** when:
- The same concept appears in multiple source pages with consistent meaning
- One source page introduces, another exemplifies; combining gives richer coverage
- Cross-references across the corpus benefit from a single canonical card

**Separate cards** when:
- The "same name" actually denotes different concepts in different domains (use disambiguation suffixes — `ownership-rust` vs `ownership-property-law`)
- Source pages disagree fundamentally about what the concept is (this is rare and usually a sign that the wiki has a real disagreement to surface, possibly via `contrasts_with`)

### Merge Mechanics

When extracting from a source page that covers a concept already represented as a card:

1. Read the existing card.
2. Identify what the new source contributes — additional examples, alternative framings, contextual applications, etc.
3. Add the new source page to `derived_from`.
4. Augment body sections (Examples, Context & Application, sometimes Relationships) with content from the new source.
5. Reconcile `trust_level` per the multi-source rule (lowest among sources, conservative).
6. Update `last_reviewed` to the merge date.
7. Document the merge in *Verification Notes* — what the new source contributed and what was preserved from the existing card.

The Haleiki build pipeline persists merge analysis in `_analysis/{concept-slug}/merge-log.json` (per architecture doc 0001 §3.3). For manual merges, you can rely on the in-frontmatter `derived_from` list and the verification notes to capture provenance — automated merge tooling will populate the analysis records when implemented.

### Conflict Detection

Watch for:

- **Scope drift** — One source treats the concept as a wide category, another as a narrow special case. Surface in *Common Confusions* or split into related cards.
- **Trust mismatch** — A `corroborated` source page and a `speculative` source page both mention the concept. Use the conservative default (lowest trust) and note the mismatch in verification notes.
- **Naming variation** — One source calls it "rigpa," another "pure awareness," a third "intrinsic awareness." Capture all variants in `aliases`; pick a primary slug.

---

## Quality Checklist

Before considering a card complete, verify:

### Frontmatter

- [ ] `concept` is properly capitalized
- [ ] `slug` matches filename (without `.md`)
- [ ] `slug` is globally unique (validator will catch collisions)
- [ ] `page_type: concept`
- [ ] `disambiguator` set when concept name collides with another concept
- [ ] `category` is from the deployment's taxonomy
- [ ] `subcategory` set if the deployment uses them
- [ ] `tier` is assigned (foundational/intermediate/advanced)
- [ ] `subject_headings` set if the concept legitimately spans categories
- [ ] `keywords` includes search hints distinct from `aliases`
- [ ] `derived_from` lists at least one source page (unless this is a hand-authored seed concept)
- [ ] `extraction_confidence` is set with rationale documented in Verification Notes
- [ ] `aliases` includes all known variants (empty array if none)
- [ ] `prerequisites` lists all dependencies (empty array if foundational)
- [ ] `answers_questions` has at least one CQ
- [ ] `trust_level` set if the deployment profile enables trust
- [ ] `canon_status` set if the deployment profile enables canon
- [ ] `status` defaults to `published` unless work-in-progress
- [ ] `last_reviewed` set to the date the card was written or last reviewed

### Body

- [ ] Quick Definition is ≤ 2 sentences
- [ ] Core Definition cites source page slug(s)
- [ ] Prerequisites explains WHY each is needed
- [ ] Key Properties are enumerated, not prose
- [ ] Construction/Recognition uses numbered steps (when applicable)
- [ ] Examples cite specific source pages
- [ ] Relationships use valid concept slugs
- [ ] Common Errors vs. Confusions are distinguished (when present)
- [ ] Source Reference summarizes what each `derived_from` source contributes
- [ ] Verification Notes justify confidence and document multi-source merges

### Cross-References

- [ ] All concepts in prerequisites/extends/related/contrasts_with exist as cards
- [ ] Relationship types are correctly applied
- [ ] No circular prerequisites
- [ ] Source page slugs in `derived_from` exist in `content/sources/`
- [ ] External reference URLs resolve

---

## Common Mistakes to Avoid

### 1. Combining Concepts

**Wrong**: Creating "Triads and Seventh Chords" as one card
**Right**: Separate cards for "Triad" and "Seventh Chord"

### 2. Generic Examples

**Wrong**: "For example, C major is a common key"
**Right**: "Example ([music-theory](/source/music-theory/)): The C major scale (C-D-E-F-G-A-B-C) demonstrates the major scale pattern using all natural notes"

### 3. Buried Relationships

**Wrong**: "This concept relates to several others, including intervals and scale degrees..."
**Right**: Use the structured `prerequisites`/`related` fields

### 4. Missing Provenance

**Wrong**: "The major scale has seven notes" (no source citation)
**Right**: "The major scale has seven notes ([music-theory](/source/music-theory/))"

### 5. Flat Confidence

**Wrong**: Assigning `high` to everything
**Right**: Vary based on source clarity; document rationale

### 6. Undefined Slugs

**Wrong**: `prerequisites: [some-concept-maybe]`
**Right**: Only use slugs that match actual or planned cards

### 7. Prose in Properties

**Wrong**: "The major scale has several key properties. First, it contains..."
**Right**: Numbered list of discrete properties

### 8. Confusing Errors and Confusions

**Wrong**: Mixing procedural mistakes with conceptual misunderstandings
**Right**:

- Common Errors = "You did it wrong" (procedural)
- Common Confusions = "You understood it wrong" (conceptual)

### 9. Forgetting Disambiguation

**Wrong**: Creating `ownership.md` when extracting from `rust-language` and ignoring that the property-law sense exists
**Right**: Use `ownership-rust.md` from the start; add `disambiguator: "Rust"`

### 10. Multiple Cards for the Same Concept

**Wrong**: Creating `mark-and-sweep-from-garbage-collection.md` and `mark-and-sweep-from-reference-counting.md` because the concept appears in two source pages
**Right**: One `mark-and-sweep.md` card with both source pages in `derived_from`

### 11. Letting Trust Inheritance Drift

**Wrong**: Defaulting all concept cards to `credible` regardless of source
**Right**: Take the lowest trust level among `derived_from` sources (conservative); override only with explicit reason

---

## Extraction Workflow

### For Each Source Page (Single-Page Extraction)

1. **Read the source page** to identify concepts. Note all the substantive concepts the page discusses, not just the topic of the page itself.
2. **List concepts** with estimated importance/complexity. Apply the *substantive coverage* threshold (see below) — extract a concept only if the source page provides substantive treatment, not just a passing mention.
3. **Determine tiers** based on prerequisite analysis within the corpus.
4. **Check for existing cards** at the candidate slugs. If a card exists, plan a merge (add this source page to its `derived_from`); if not, plan a fresh extraction.
5. **Extract in dependency order** (foundational first within the page).
6. **Cross-reference** as you extract — note slugs for later cards.
7. **Disambiguate** when needed (check for slug collisions before writing).
8. **Validate** frontmatter and body completeness via the Quality Checklist.

### Substantive Coverage Threshold

A concept is **substantively covered** by a source page (and therefore extractable from that page) when at least one of these holds:

- The source provides a definition, explicit or recoverable from context
- The source dedicates a paragraph or more to the concept
- The source provides examples or worked treatments of the concept
- The source places the concept in relationship with other concepts the page covers

A concept is a **mere mention** (and not extractable from this page) when:

- The source name-drops the concept in a single sentence without definition or elaboration
- The concept appears only in a "see also" or "related topics" list
- The concept is referenced only as the name of a person, work, or thing, with no semantic content beyond the name

When in doubt, err toward skipping. A thin card derived from a passing mention is worse than no card — the same concept will likely be substantively covered by another source page in the corpus, and that's where the card should originate.

**Examples:**

- `quantum-mechanics` source page mentions "group theory" in a single sentence about symmetries. **Skip** — extract group-theory from the `group-theory` source page instead.
- `padmasambhava` source page dedicates several paragraphs to the founding of the Nyingma school. **Extract** — `nyingma` is substantively covered.
- `longchenpa` source page describes Longchenpa's role in systematizing Nyingma teachings, with multiple references to the school's structure. **Extract and merge** — `nyingma` already has a card from `padmasambhava`; this becomes a multi-source card.
- `rust-language` source page lists "traits, generics, and macros" in a feature summary without elaborating on macros. **Skip macros** — would be extracted from a dedicated `rust-macros` source page if one were added; otherwise the corpus has a coverage gap for that concept.

### For Each Concept

1. **Identify** the concept boundaries (what's in, what's out).
2. **Find** the source's definition (if explicit).
3. **Assign** confidence based on definition clarity.
4. **List** prerequisites by tracing the "what must you know first" chain.
5. **Extract** key properties as discrete, enumerable items.
6. **Document** construction/recognition procedures.
7. **Pull** examples directly from source page(s) with citations.
8. **Map** relationships to other concepts (use exact slugs).
9. **Note** errors vs. confusions from your domain knowledge.
10. **Set** trust_level (default = lowest among `derived_from` sources; override explicitly if needed).
11. **Set** canon_status if the profile enables it (default per profile).
12. **Generate** 1-3 competency questions.
13. **Complete** verification notes.

### When to Create Multiple Cards From One Source Page

Sometimes a single source page section covers what should be multiple cards. Split when:

1. **Different definitions**: The source defines X and Y separately
2. **Different prerequisites**: X requires A, Y requires B
3. **Different applications**: X is used for purpose P, Y for purpose Q
4. **Common confusion**: People often mix up X and Y (they need separate cards to disambiguate)

### When to Merge Across Source Pages

Sometimes multiple source pages cover what should be one card. Merge when:

1. **Same concept, different contexts**: Source A introduces X, Source B applies X
2. **Progressive elaboration**: Basic definition in one source, formal definition in another
3. **Cross-references**: Sources explicitly cite each other for the same concept

When merging, list all source pages in `derived_from` and use the most comprehensive definition. See *Cross-Source Concept Merging* above for the full procedure.

---

## Handling Edge Cases

### Concept Not Explicitly Defined

- Set `extraction_confidence: medium` or `low`
- In Core Definition, note: "The source does not explicitly define this concept. Synthesized from discussion in [source-slug]."
- In Verification Notes, document the synthesis process

### Circular Dependencies

If A requires B and B requires A:

1. Identify which is more fundamental
2. Make the fundamental one `foundational` (no prerequisites)
3. Add a note in Verification Notes explaining the mutual dependency

### Multiple Source Treatments Conflict

If different source pages present conflicting information about the same concept:

1. Note the conflict in Verification Notes
2. Use the most authoritative treatment for the definition (typically the foundational source)
3. Document alternatives in Context & Application
4. Consider whether `contrasts_with` would more cleanly capture the disagreement (linking to a concept that represents the alternative interpretation)

### No Suitable Examples in Source

- First, look harder — sources usually have examples, even if implicit
- If truly none, note: "The source provides no explicit examples for this concept"
- Do NOT invent generic examples

### Concept Appears in Multiple Source Pages

Decide based on treatment:

- **Same treatment**: One card, list both sources in `derived_from`
- **Different but consistent treatments**: One card, merge per the cross-source merging procedure
- **Disambiguating treatments** (the same name actually means different things): Separate cards with disambiguation suffixes

### Source Pages with Different Genres

When a concept is extracted from source pages of mixed genre (e.g., one encyclopedia article and one captured conversation):

- Trust defaults to the conservative (lowest) — see the genre defaults table
- `derived_from` carries both regardless of genre
- Verification Notes should document any tension between the genres' framings

### Hand-Authored "Seed" Concept Cards

The demo and bootstrap deployments may include hand-authored concept cards that don't derive from any source page yet. For these:

- `derived_from: []` (empty list, valid)
- `external_references` carries any relevant citations
- The card is a "seed" that future source-page extractions may augment via merging
- Set `extraction_confidence: high` if the author is a domain expert; lower if it's a sketch

---

## Final Checklist Before Committing

- [ ] All cards pass frontmatter validation
- [ ] All cards have complete body sections
- [ ] All cross-referenced slugs exist
- [ ] No orphaned cards (unless legitimately foundational seeds)
- [ ] No circular prerequisites
- [ ] Confidence distribution is realistic (not all high)
- [ ] At least one CQ per card
- [ ] Competency questions have coverage
- [ ] Filenames match slugs
- [ ] No numerical prefixes on files
- [ ] All files have `.md` extension
- [ ] Cards are under `content/concepts/` (flat, not in subdirectories)
- [ ] Disambiguators set wherever concept names collide
- [ ] Profile-aware fields (`trust_level`, `canon_status`) set per profile requirements
- [ ] No LLM generation artifacts (search for "Wait", "Let me recalculate", "I apologize", "As an AI")

---

## Version History

- **v1** (Haleiki): First version. Adapted from Billo Systems' AI Engineering concept-card howto v3.2 (`workbench/0009-howto-concept-card-extraction-with-claude-code-v3.2.md` for reference). Replaced book-citation provenance with wiki-pointer provenance (`derived_from` / `external_references`); added `disambiguator`, `subject_headings`, `trust_level`, `canon_status`, `page_type`, `status`, `last_reviewed`, and `stable_since` fields; added genre-aware trust defaults; added cross-source concept merging guidance; flattened file organization to `content/concepts/{slug}.md`; updated examples to draw from the Haleiki demo cluster (Tibetan Buddhism, theoretical physics, mathematics, music, programming languages, AI, philosophy of mind, linguistics, botany, geology). The confidence framework, typed relationship vocabulary, body section structure, and competency-question framework are unchanged.
- **v1.1** (Haleiki): Refinements from the 5-article pilot extraction (group-theory, quantum-mechanics, rust-language, padmasambhava, longchenpa). Added the source-page-vs-concept-card slug-collision case to disambiguation guidance (Golden Rule 6 and the Disambiguation Strategy section), with role/category suffix examples (`-teacher`, `-scholar`, `-field`, `-mathematics`). Added a "Substantive Coverage Threshold" section to the Extraction Workflow with concrete extract-vs-skip examples. Refined disambiguation guidance into three explicit cases: cross-domain collision (a), source-page collision (b), anticipated future collision (c). No schema changes; no existing pilot cards need renaming.
