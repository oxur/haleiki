---
number: 5
title: "Deployment Profiles & Epistemic Architecture"
author: "the pre"
component: All
tags: [change-me]
created: 2026-03-19
updated: 2026-03-19
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# Deployment Profiles & Epistemic Architecture

**Addendum to `haleiki-architecture.md`**

Version 0.1 · March 2026 · Living Document

---

## 1. Overview

Haleiki is designed to serve multiple use cases — from technical encyclopedias to creative worldbuilding tools to simple knowledge-sharing sites. Rather than hardcoding assumptions about content epistemology into the framework, Haleiki uses **deployment profiles** (also called "personalities") to configure how content trust, creative status, and reader-facing language behave for each deployment.

This document defines the profile system, the two configurable metadata axes it manages (trust level and canon status), the visual design tokens that express them, and the impact on the existing architecture.

---

## 2. Deployment Personalities

Three built-in personalities cover the primary deployment patterns. Each personality sets defaults for which features are enabled and how they present to readers. All defaults are overridable in `profile.yaml`.

### 2.1 Knowledge Site (default)

The leanest configuration. Pages, graph, search, taxonomy — no epistemic signaling by default. Suitable for knowledge bases where all content is equally authoritative by nature.

| Feature | Default |
|---------|---------|
| Trust level | Disabled (opt-in via `trust.enabled: true`) |
| Canon status | Disabled |
| Trust descriptions | Epistemic dialect (if enabled) |
| Default trust assignment | `provisional` (if enabled) |

**Target use cases:** Conversation travelogues, MCP server concept browsers, internal documentation sites, curated link collections, tool reference wikis.

### 2.2 Encyclopedia

Enables trust-level signaling so readers can assess the epistemic standing of each article. Suitable for reference material where source quality varies and readers need calibration.

| Feature | Default |
|---------|---------|
| Trust level | Enabled |
| Canon status | Disabled |
| Trust descriptions | Epistemic dialect |
| Default trust assignment | `provisional` |

**Target use cases:** Rust programming knowledge base, R&D research wiki, domain-specific reference sites.

### 2.3 Writer

Enables both axes — trust level (using a fact-to-fiction dialect) and canon status. Designed for creative worldbuilding where readers need to know both "how grounded is this in reality?" and "how committed is the author to this?"

| Feature | Default |
|---------|---------|
| Trust level | Enabled |
| Canon status | Enabled |
| Trust descriptions | Worldbuilding dialect |
| Default trust assignment | `credible` |

**Target use cases:** Scifi/fantasy worldbuilding wikis, collaborative fiction projects, game design lore databases.

---

## 3. The Trust Level Axis

### 3.1 Purpose

Trust level answers: **how much should a reader rely on this content?** It communicates the epistemic quality of the content itself — the sourcing, corroboration, and factual grounding. It is a *reading posture* cue: it shapes how the reader holds the information as they read.

### 3.2 Enum Values

Four fixed levels. The IDs are stable across all personalities; only the reader-facing labels and descriptions change.

```
trust_level: corroborated | credible | provisional | speculative
```

| ID | Semantic | Visual intensity |
|----|----------|-----------------|
| `corroborated` | Highest confidence — multiple reliable sources, well-verified | Quietest (bedrock; the default posture) |
| `credible` | Trustworthy sources, but framed or angled for a purpose | Moderate |
| `provisional` | Partial basis — may be revised as more information emerges | Noticeable |
| `speculative` | Fragmentary, loosely grounded, or purely constructed | Loudest (furthest from bedrock) |

### 3.3 Profile-Customizable Descriptions

The same four-level system speaks different dialects depending on the personality. Labels and descriptions are defined in `profile.yaml` and rendered in the trust bar, the definitions page, and search facets.

**Epistemic dialect** (encyclopedia, knowledge site):

| Level | Label | Description |
|-------|-------|-------------|
| `corroborated` | Corroborated | Verified from multiple reliable sources |
| `credible` | Credible | Well-sourced, reasonable interpretation |
| `provisional` | Provisional | Partial evidence, subject to revision |
| `speculative` | Speculative | Limited or fragmentary basis |

**Worldbuilding dialect** (writer):

| Level | Label | Description |
|-------|-------|-------------|
| `corroborated` | Corroborated | Factual information from reliable sources |
| `credible` | Credible | Real sources, framed for narrative purpose |
| `provisional` | Provisional | Fictional information based loosely on fact |
| `speculative` | Speculative | A work of pure fiction |

Custom deployments can define their own labels and descriptions. The underlying enum values and visual tokens remain consistent.

### 3.4 Trust Level in Frontmatter

Added to both source pages and concept cards:

```yaml
# In source page or concept card frontmatter
trust_level: "corroborated"    # corroborated | credible | provisional | speculative
```

If omitted, the profile's `trust.default` value is used. The validator warns on missing trust levels when trust is enabled.

### 3.5 Visual Treatment

Trust level is the **ambient, pre-attentive** signal. It colors the reader's entire experience of the page. Three coordinated touchpoints change simultaneously:

1. **Trust bar** — a slim horizontal bar below the top navigation, tinted with the trust-level color. Border weight increases with distance from bedrock (1px for corroborated, 3px dashed for speculative).

2. **Article overline** — the category/type label above the title includes a trust glyph and the trust level label, colored to match.

3. **Info card header** — the right sidebar's header background shifts to the trust-level accent color.

**Glyph system** (geometric, works regardless of color vision):

| Level | Glyph | Shape logic |
|-------|-------|-------------|
| `corroborated` | Solid filled circle | Complete, whole, bedrock |
| `credible` | Half-filled circle | Mostly solid, slight opening |
| `provisional` | Outlined circle | Hollow — shape is there, filling is not |
| `speculative` | Dashed-outline circle | Fragmentary, provisional, open |

When trust is disabled in the profile, none of these elements render. The trust bar is absent, the overline uses the base accent color, and the info card header uses `--accent-primary`.

---

## 4. The Canon Status Axis

### 4.1 Purpose

Canon status answers: **where does this content sit in the creative work?** It communicates the author's commitment to this content as part of the story world. It is a *workflow state* cue: it tells readers and collaborators whether they can build on this, contradict it, or ignore it.

Canon status is **orthogonal to trust level**. Content can be simultaneously corroborated (grounded in real science) and seed (an early idea not yet committed to the story). Or speculative (pure fiction) and established (settled canon the narrative depends on). All combinations are valid.

### 4.2 Enum Values

```
canon_status: established | proposed | developing | seed
```

| ID | Meaning |
|----|---------|
| `established` | Settled, load-bearing in the narrative — the story depends on this |
| `proposed` | Author intends this to become canon; under review or awaiting integration |
| `developing` | Actively being shaped, not yet committed — may change substantially |
| `seed` | Early idea; might go anywhere or nowhere |

### 4.3 Canon Status in Frontmatter

```yaml
# In source page or concept card frontmatter (writer personality only)
canon_status: "proposed"    # established | proposed | developing | seed
```

If omitted when canon is enabled, defaults to `developing`.

### 4.4 Visual Treatment

Canon status is **metadata, not chrome**. It appears as a small inline badge on the info card, using a *different visual language* from trust to prevent conflation:

- **Color strategy:** Lightness/chroma variation within a single warm-bronze hue family (~30° OKLCH). Established is darkest/most saturated; seed is lightest/most washed out. This contrasts with trust's hue-rotation strategy (terracotta → teal → violet → ochre).

- **Shape strategy:** Badges contain a small square mark (not circle — circles belong to the trust axis). Solid square for established, outlined square for developing, dashed square for seed.

- **Placement:** A single row in the info card, between core identification rows and the Categories section.

When canon is disabled in the profile, the row does not render and the frontmatter field is ignored.

---

## 5. Design Tokens

### 5.1 Trust Level Tokens

Trust tokens are set via compound selectors on the root element: `[data-theme][data-trust]`. Each combination of theme × trust level defines a complete set of semantic tokens.

```css
/* Semantic trust tokens (set per theme × trust combination) */
--trust-surface:     /* tinted background for trust bar */
--trust-border:      /* bar and component borders */
--trust-accent:      /* primary accent (info card header, active states) */
--trust-text:        /* text and glyph color */
--trust-subtle:      /* hover/selection backgrounds */
--trust-bar-bg:      /* trust bar background wash */
--trust-on-accent:   /* text on accent backgrounds */
```

**Hue assignments:**

| Trust level | Day hue (OKLCH) | Night hue (OKLCH) | Color family |
|-------------|-----------------|-------------------|--------------|
| `corroborated` | ~45° | ~70° | Terracotta / Amber (inherits wiki accent) |
| `credible` | ~165° | ~165° | Sage / Teal |
| `provisional` | ~295° | ~295° | Dusty violet |
| `speculative` | ~85° | ~85° | Warm ochre / Yellow |

Design principle: **the further from bedrock, the louder the signal.** Corroborated uses the wiki's own accent palette (it IS the default). Speculative departs furthest in both hue and visual weight.

### 5.2 Canon Status Tokens

Canon tokens use a single hue family with lightness/chroma variation. They do not interact with the trust axis tokens.

```css
/* Per canon status, per theme */
--canon-{status}-bg:      /* badge background */
--canon-{status}-text:    /* badge text */
--canon-{status}-border:  /* badge border */
```

**Hue family:** ~30–45° (warm bronze/copper) across all four statuses. Differentiation is through lightness and chroma only.

| Canon status | Day treatment | Night treatment |
|-------------|---------------|-----------------|
| `established` | Darkest, highest chroma | Brightest, highest chroma |
| `proposed` | Mid-dark, moderate chroma | Mid-bright, moderate chroma |
| `developing` | Mid-light, low chroma | Mid-dark, low chroma |
| `seed` | Lightest, nearly washed out | Dimmest, nearly invisible |

---

## 6. Profile Configuration Schema

### 6.1 File: `profile.yaml`

Lives at the root of the content directory, alongside `taxonomy.yaml` (which it may eventually absorb). Read by the pre-build tool before any content processing.

```yaml
# ── Profile identity ──
profile:
  name: "Worlds of Kāla Sāra"
  personality: "writer"            # knowledge-site | encyclopedia | writer

# ── Trust level axis ──
trust:
  enabled: true                    # derived from personality default if omitted
  default: "credible"              # assigned when frontmatter omits trust_level
  descriptions:                    # override personality defaults
    corroborated:
      label: "Corroborated"
      description: "Factual information from reliable sources"
    credible:
      label: "Credible"
      description: "Real sources, framed for narrative purpose"
    provisional:
      label: "Provisional"
      description: "Fictional information based loosely on fact"
    speculative:
      label: "Speculative"
      description: "A work of pure fiction"

# ── Canon status axis ──
canon:
  enabled: true                    # derived from personality default if omitted
  default: "developing"            # assigned when frontmatter omits canon_status
  # descriptions use built-in defaults; override same pattern as trust

# ── Page types ──
page_types:
  - source
  - concept
  # Writer personality might add:
  - brief                          # narrative-biased synthesis
  - sketch                         # working notes

# ── Relationship vocabulary ──
relationships:
  # Core (all personalities)
  - prerequisite
  - extends
  - related
  - contrasts_with
  # Writer additions
  - inspires
  - supersedes
  - real_world_analogue
  - narrative_adaptation_of
  - contradicts_in_universe

# ── Taxonomy ──
# Can be inline or reference taxonomy.yaml
taxonomy:
  categories:
    - "physics"
    - "biology"
    - "culture"
    - "technology"
    - "politics"
  tiers:
    - "foundational"
    - "intermediate"
    - "advanced"
```

### 6.2 Defaults by Personality

When fields are omitted from `profile.yaml`, defaults are derived from the declared personality:

| Field | knowledge-site | encyclopedia | writer |
|-------|---------------|-------------|--------|
| `trust.enabled` | `false` | `true` | `true` |
| `trust.default` | `provisional` | `provisional` | `credible` |
| `trust.descriptions` | epistemic dialect | epistemic dialect | worldbuilding dialect |
| `canon.enabled` | `false` | `false` | `true` |
| `canon.default` | — | — | `developing` |
| `page_types` | `[source, concept]` | `[source, concept]` | `[source, concept, brief, sketch]` |
| `relationships` | core set | core set | core + creative set |

---

## 7. Definitions Page

### 7.1 Purpose

A non-article page at a reserved URL (`/about/` or `/guide/`) that defines all active axes for the deployment. This page is critical for setting reader expectations — particularly for trust levels, where misunderstanding can cause real confusion.

The definitions page is **conditionally generated**: it only includes sections for axes that are enabled in the profile. A knowledge site with trust disabled gets no trust section. A writer deployment gets both trust and canon sections.

### 7.2 Content Requirements

**Trust level definitions** must include:

- A clear statement that trust levels describe *the sourcing and evidence basis of the content*, not a value judgment on the subject matter. Example: an article on Buddhism marked "Corroborated" means the information presented is verified by experts in Buddhist studies — it does not mean the wiki endorses Buddhism as truth.

- Each level with its profile-customized label, description, glyph, and color swatch.

- At least one concrete example per level, drawn from the deployment's own content where possible.

- A note that trust levels may change as content is updated and additional sources are incorporated.

**Canon status definitions** (if enabled) must include:

- A clear statement that canon status describes the author's commitment to the content within the creative work.

- Each status with its label, description, and badge appearance.

- Guidance on what it means to "build on" content at each status level (e.g., "Established canon should not be contradicted without updating the canon entry; Seed content may be freely contradicted or abandoned").

### 7.3 Linking

The trust bar label links to the definitions page (anchor: `#trust-levels`). The canon badge in the info card links to the definitions page (anchor: `#canon-status`). These links are the reader's always-available escape hatch: "I don't understand this badge — click to learn."

### 7.4 URL Convention

Reserved path: `/about/` — not a content page, not a category page. Generated by the build pipeline from profile data and a Liquid template. The template conditionally renders sections based on which axes are enabled.

---

## 8. Cobalt Integration

### 8.1 Profile-Aware Template Selection

Cobalt templates need to conditionally render trust and canon elements based on profile configuration. The pre-build tool writes profile-derived flags into `_data/profile.json`, which Liquid templates read.

```json
// _data/profile.json (generated by pre-build)
{
  "personality": "writer",
  "trust": {
    "enabled": true,
    "default": "credible",
    "levels": {
      "corroborated": {
        "label": "Corroborated",
        "description": "Factual information from reliable sources"
      },
      // ...
    }
  },
  "canon": {
    "enabled": true,
    "default": "developing"
  },
  "page_types": ["source", "concept", "brief", "sketch"],
  "relationships": ["prerequisite", "extends", "related", "..."]
}
```

### 8.2 Conditional Rendering in Templates

```liquid
{% comment %} Trust bar: only if trust is enabled {% endcomment %}
{% if site.data.profile.trust.enabled %}
  {% include "trust-bar" trust_level: page.trust_level %}
{% endif %}

{% comment %} Canon badge in info card: only if canon is enabled {% endcomment %}
{% if site.data.profile.canon.enabled %}
  {% include "canon-badge" canon_status: page.canon_status %}
{% endif %}

{% comment %} Overline: trust-aware or plain {% endcomment %}
{% if site.data.profile.trust.enabled %}
  <div class="article-overline" data-trust="{{ page.trust_level }}">
    {% include "trust-glyph" level: page.trust_level %}
    {{ site.data.profile.trust.levels[page.trust_level].label }} · {{ page.category }}
  </div>
{% else %}
  <div class="article-overline">{{ page.category }}</div>
{% endif %}
```

### 8.3 Page-Level Data Attributes

The rendered HTML sets `data-trust` on the root element (for CSS token resolution) and `data-canon` on the badge element. These are populated from frontmatter via Liquid:

```html
<html lang="en" data-theme="day"
  {% if site.data.profile.trust.enabled %} data-trust="{{ page.trust_level }}"{% endif %}>
```

### 8.4 Definitions Page Generation

The pre-build tool generates `/about/index.md` from the profile configuration. This is a Liquid template that iterates over the enabled axes:

```liquid
{% if site.data.profile.trust.enabled %}
## Trust Levels {#trust-levels}
{{ site.data.profile.trust_preamble }}
{% for level in site.data.profile.trust.level_order %}
  {% assign info = site.data.profile.trust.levels[level] %}
### {{ info.label }}
{{ info.description }}
{% endfor %}
{% endif %}
```

The preamble text (the careful framing about what trust levels do and don't mean) is defined in the profile or provided as a default per personality.

### 8.5 Template Selection by Page Type

Currently two templates: `concept.liquid` and `source.liquid`. The profile system extends this:

- Template naming convention: `_layouts/{page_type}.liquid`
- Fallback: if no template exists for a declared page type, fall back to `source.liquid` (freeform prose is the safest default)
- The pre-build tool validates that every declared page type has either a dedicated template or an explicit fallback mapping in the profile

### 8.6 Build Pipeline Extension

The build command gains profile awareness:

```bash
# Pre-build reads profile.yaml first
haleiki build
  1. Parse profile.yaml → resolve personality defaults
  2. Parse all content (parameterized by profile's page types)
  3. Validate (parameterized by profile's trust/canon rules)
  4. Build graph (with trust_level and canon_status as node attributes)
  5. Generate _data/ (including profile.json, trust/canon facet data)
  6. Generate definitions page (/about/index.md)
  7. → Cobalt build (reads _data/, applies conditional templates)
  8. → Pagefind (facets include trust_level and canon_status if enabled)
```

---

## 9. Impact on Search

When trust and/or canon are enabled, Pagefind facets are extended:

```html
<!-- In page template, Pagefind data attributes -->
<article
  data-pagefind-meta="trust_level:{{ page.trust_level }}"
  {% if site.data.profile.canon.enabled %}
  data-pagefind-meta="canon_status:{{ page.canon_status }}"
  {% endif %}
  data-pagefind-filter="trust_level:{{ page.trust_level }}"
>
```

The search UI shows filter chips for trust level (and canon status, if enabled) alongside existing category and tier filters. Search results display the trust glyph inline next to each result title.

---

## 10. Impact on Graph & Validation

### 10.1 Graph Attributes

Trust level and canon status become node attributes in the unified graph. The pre-build tool stores them in the graph JSON:

```json
{
  "nodes": {
    "ownership": {
      "page_type": "concept",
      "trust_level": "corroborated",
      "canon_status": null,
      "category": "memory-model",
      "tier": "foundational"
    }
  }
}
```

This enables trust/canon-aware queries: "prerequisite chain for this concept, filtered to corroborated content only."

### 10.2 Validation Rules

The validator applies personality-aware rules:

**Encyclopedia personality:**

- WARN if a `corroborated` page has no `derived_from` or `external_references`
- WARN if a `speculative` page is referenced as a prerequisite by a `corroborated` page
- INFO on trust level distribution (catch accidental clustering)

**Writer personality:**

- All encyclopedia rules, plus:
- WARN if an `established` canon page has trust level `speculative` (settled fiction should at least be internally consistent)
- WARN if a `seed` canon page is a prerequisite for an `established` canon page

**Knowledge site (trust disabled):**

- Trust and canon fields are ignored; no validation against them
- If trust is opt-in enabled, encyclopedia rules apply

### 10.3 Concept Card Inheritance

**Open design question (to be resolved in implementation):**

When concept cards are extracted from source pages, they must inherit the trust level and canon status of their source material. If a source page's trust level or canon status changes after extraction, derived concept cards must be flagged for review — the pre-build validator should detect the mismatch and warn.

Inheritance rules (proposed, not yet implemented):

- A concept card derived from a single source inherits that source's trust level and canon status as defaults
- A concept card derived from multiple sources inherits the *lowest* trust level among its sources (conservative: the card is only as strong as its weakest source)
- Canon status is not automatically inherited from multiple sources — if sources disagree, the concept card's canon status must be set explicitly
- Re-extraction or source status changes trigger a validation warning, not an automatic update (manual review required)

---

## 11. Impact on Existing Architecture

### 11.1 Changes to Content Model (Architecture Doc §3)

**Source page frontmatter** gains two optional fields:

```yaml
trust_level: "corroborated"      # only meaningful if trust is enabled
canon_status: "established"       # only meaningful if canon is enabled
```

**Concept card frontmatter** gains the same two fields, plus the inheritance relationship to source material trust/canon is tracked in the analysis record.

### 11.2 Changes to Pre-Build Tool (Architecture Doc §5)

- **New:** Profile parser (`profile.rs`) — reads `profile.yaml`, resolves personality defaults, produces `ProfileConfig` struct used throughout the pipeline
- **Modified:** `parser.rs` — validates frontmatter against profile-declared page types and optional axes; page type handling becomes data-driven rather than match-on-string
- **Modified:** `validator.rs` — gains personality-aware validation rules for trust/canon
- **Modified:** `generator.rs` — writes `_data/profile.json` and generates the definitions page
- **Modified:** `graph.rs` — stores trust_level and canon_status as node attributes; supports filtered graph queries
- **Modified:** `search.rs` — adds trust_level and canon_status to Pagefind facet data

### 11.3 Changes to Design System (Architecture Doc §4)

- **New token layer:** Trust-level semantic tokens (8 theme × trust combinations)
- **New token layer:** Canon-status semantic tokens (2 themes × 4 statuses)
- **New component:** Trust bar (`.trust-bar`)
- **New component:** Canon badge (`.canon-badge`)
- **Modified component:** Article overline (conditionally includes trust glyph)
- **Modified component:** Info card header (background driven by `--trust-accent` when enabled)
- **New data attribute:** `data-trust` on `<html>` element
- SVG glyph set for trust levels (4 glyphs) and canon marks (4 marks)

### 11.4 Changes to Directory Structure

```
haleiki/
├── content/
│   ├── profile.yaml              # NEW — deployment profile configuration
│   ├── taxonomy.yaml             # Unchanged (may be absorbed into profile.yaml later)
│   ├── sources/
│   └── concepts/
├── _layouts/
│   ├── about.liquid              # NEW — definitions page template
│   └── ...
├── _includes/
│   ├── trust-bar.liquid          # NEW
│   ├── trust-glyph.liquid        # NEW
│   ├── canon-badge.liquid        # NEW
│   └── ...
├── tools/
│   └── src/
│       ├── profile.rs            # NEW — profile parsing + personality defaults
│       └── ...
```

### 11.5 Impact on Demo Site (demo-site-project-plan.md)

The demo site pipeline (phases 1–5, currently implemented) is minimally affected:

- `demo/profile.yaml` is created with `personality: "encyclopedia"` and trust enabled
- `demo/manifest.yaml` entries gain a `trust_level` default (likely `credible` — Wikipedia content is well-sourced but editorially framed)
- Frontmatter injection (phase 5, `frontmatter.rs`) adds `trust_level` from the manifest
- Hand-authored demo concept cards (phase 9) include `trust_level` in their frontmatter
- The definitions page is auto-generated as part of the demo build

No changes are required to the fetch, clean, media, or conversion stages.

---

## 12. Open Questions

1. **Profile inheritance:** Should a profile be able to extend another? e.g., `personality: "writer" extends: "encyclopedia"`. Useful if writer-specific additions keep growing, but adds complexity. Defer until needed.

2. **Trust-level transitions:** Should the framework track when a page's trust level changes over time? A changelog in the analysis record would enable "this was upgraded from provisional to corroborated on date X." Useful for editorial workflow but adds storage overhead.

3. **Visual treatment of trust in see-also cards:** Should see-also cards on a corroborated page show the trust glyph of each linked page? Helps readers anticipate what they're clicking into, but adds visual noise. Prototype and evaluate.

4. **Definitions page authoring:** Is the auto-generated definitions page sufficient, or should authors be able to hand-edit it (with merge protection similar to concept cards)? The preamble text about "what trust levels mean" is sensitive enough that hand-authoring may be preferable to generated boilerplate.

5. **Multi-source concept card canon status:** When a concept is derived from sources with conflicting canon statuses, the proposed rule is "require explicit assignment." Should the validator enforce this, or just warn?

6. **Profile migration:** When a deployment changes personality (e.g., knowledge-site → encyclopedia), what happens to existing content that lacks `trust_level` fields? The default assignment handles new content, but a migration tool for backfilling existing pages would be useful.
