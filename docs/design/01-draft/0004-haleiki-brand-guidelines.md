---
number: 4
title: "Haleiki Brand Guidelines"
author: "multiplying the"
component: All
tags: [change-me]
created: 2026-03-18
updated: 2026-03-18
state: Draft
supersedes: null
superseded-by: null
version: 1.0
---

# Haleiki Brand Guidelines

**Version 1.0 -- March 2026**

Visual identity system for Haleiki -- covering the brand mark, colour architecture, typography, spacing, and implementation. Built in OKLCH, scaled with a perfect fourth, fluid from phone to ultrawide.

---

## 1. Brand Identity

### The Brand Mark

The Haleiki brand mark is the letter **H** set in Red Hat Display (weight 700-800) inside a rounded square. The background uses `--accent-primary` (terracotta in day, amber in night); the letter uses `--text-on-invert` (day) or `--surface-ground` (night).

- Day: terracotta square, warm cream H
- Night: amber square, deep oxide H
- Minimum size: 24px square
- Border radius: `--radius-s` (4px) at nav size, `--radius-m` (6px) at larger display sizes

### Name and Tagline

- **Name:** Haleiki (Hawaiian: *hale* "house" + *ike* "knowledge")
- **Tagline:** House of Knowledge -- a read-only wiki for structured knowledge

### Brand Personality

Haleiki communicates scholarly warmth, navigational clarity, and the quiet confidence of a well-organized library. The design language draws from Wikipedia's information architecture, modernized with perceptually uniform colour, geometric type scale, and intrinsic layout composition.

The design is warm but never casual, structured but never rigid, scholarly but never cold.

---

## 2. Colour System

All colours defined in OKLCH for perceptual uniformity -- equal lightness steps produce equal perceived brightness across all hues. Warm neutrals throughout (low chroma at warm hue, never pure grey).

### Day Theme -- Warm Ochre, Sandstone, Terracotta

#### Surfaces

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--surface-ground` | `oklch(0.97 0.01 80)` | Page background |
| `--surface-primary` | `oklch(0.99 0.005 80)` | Card/panel backgrounds |
| `--surface-secondary` | `oklch(0.95 0.015 75)` | Secondary panels, inputs |
| `--surface-tertiary` | `oklch(0.92 0.02 72)` | Hover states, wells |
| `--surface-invert` | `oklch(0.25 0.03 55)` | Inverted sections |

#### Borders

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--border-subtle` | `oklch(0.88 0.025 75)` | Soft dividers |
| `--border-default` | `oklch(0.82 0.03 72)` | Standard borders |
| `--border-strong` | `oklch(0.70 0.04 65)` | Emphasis borders |

#### Text

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--text-primary` | `oklch(0.22 0.02 55)` | Body text, headings |
| `--text-secondary` | `oklch(0.40 0.03 60)` | Captions, metadata |
| `--text-tertiary` | `oklch(0.55 0.025 65)` | Placeholder, disabled |
| `--text-on-invert` | `oklch(0.97 0.01 80)` | Text on dark backgrounds |

#### Accent

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--accent-primary` | `oklch(0.55 0.14 45)` | Terracotta -- primary interactive |
| `--accent-hover` | `oklch(0.48 0.15 42)` | Hover state |
| `--accent-subtle` | `oklch(0.92 0.04 50)` | Subtle accent background |
| `--accent-text` | `oklch(0.45 0.12 40)` | Accent-coloured text, links |

#### Info and Code

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--info-surface` | `oklch(0.94 0.03 80)` | Callout background |
| `--info-border` | `oklch(0.82 0.05 75)` | Callout border |
| `--info-text` | `oklch(0.40 0.06 65)` | Callout text |
| `--code-surface` | `oklch(0.95 0.01 80)` | Code block background |
| `--code-border` | `oklch(0.88 0.02 75)` | Code block border |
| `--code-text` | `oklch(0.38 0.08 40)` | Code text |

#### Footer

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--footer-surface` | `oklch(0.22 0.025 55)` | Footer background (always dark) |
| `--footer-text` | `oklch(0.75 0.015 70)` | Footer body text |
| `--footer-text-dim` | `oklch(0.55 0.015 65)` | Footer secondary text |
| `--footer-heading` | `oklch(0.90 0.01 75)` | Footer headings |
| `--footer-link` | `oklch(0.75 0.015 70)` | Footer links |
| `--footer-link-hover` | `oklch(0.92 0.01 75)` | Footer link hover |
| `--footer-border` | `oklch(0.32 0.025 55)` | Footer dividers |
| `--footer-accent` | `oklch(0.65 0.12 50)` | Footer accent (brand mark) |

### Night Theme -- Deep Oxide, Amber

#### Surfaces

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--surface-ground` | `oklch(0.16 0.02 55)` | Page background |
| `--surface-primary` | `oklch(0.20 0.02 55)` | Card/panel backgrounds |
| `--surface-secondary` | `oklch(0.24 0.025 52)` | Secondary panels, inputs |
| `--surface-tertiary` | `oklch(0.28 0.03 50)` | Hover states, wells |
| `--surface-invert` | `oklch(0.92 0.02 75)` | Inverted sections |

#### Borders

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--border-subtle` | `oklch(0.30 0.03 50)` | Soft dividers |
| `--border-default` | `oklch(0.36 0.035 48)` | Standard borders |
| `--border-strong` | `oklch(0.48 0.04 50)` | Emphasis borders |

#### Text

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--text-primary` | `oklch(0.90 0.015 75)` | Body text, headings |
| `--text-secondary` | `oklch(0.72 0.02 65)` | Captions, metadata |
| `--text-tertiary` | `oklch(0.55 0.025 58)` | Placeholder, disabled |
| `--text-on-invert` | `oklch(0.20 0.02 55)` | Text on light backgrounds |

#### Accent

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--accent-primary` | `oklch(0.72 0.14 70)` | Amber -- primary interactive |
| `--accent-hover` | `oklch(0.78 0.15 72)` | Hover state |
| `--accent-subtle` | `oklch(0.28 0.06 60)` | Subtle accent background |
| `--accent-text` | `oklch(0.78 0.12 72)` | Accent-coloured text, links |

#### Info and Code

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--info-surface` | `oklch(0.24 0.03 55)` | Callout background |
| `--info-border` | `oklch(0.36 0.04 52)` | Callout border |
| `--info-text` | `oklch(0.72 0.04 68)` | Callout text |
| `--code-surface` | `oklch(0.22 0.02 55)` | Code block background |
| `--code-border` | `oklch(0.32 0.03 52)` | Code block border |
| `--code-text` | `oklch(0.78 0.1 72)` | Code text |

#### Footer (Night)

| Token | OKLCH Value | Role |
|-------|-------------|------|
| `--footer-surface` | `oklch(0.22 0.02 55)` | Footer background |
| `--footer-text` | `oklch(0.65 0.015 65)` | Footer body text |
| `--footer-text-dim` | `oklch(0.45 0.015 60)` | Footer secondary text |
| `--footer-heading` | `oklch(0.82 0.01 70)` | Footer headings |
| `--footer-link` | `oklch(0.65 0.015 65)` | Footer links |
| `--footer-link-hover` | `oklch(0.85 0.01 72)` | Footer link hover |
| `--footer-border` | `oklch(0.30 0.025 55)` | Footer dividers |
| `--footer-accent` | `oklch(0.72 0.12 70)` | Footer accent (brand mark) |

### Shadows

| Token | Day | Night |
|-------|-----|-------|
| `--shadow-soft` | `0 1px 3px oklch(0.5 0.02 60 / 0.08)` | `0 1px 3px oklch(0.1 0.02 55 / 0.3)` |
| `--shadow-medium` | `0 2px 8px oklch(0.4 0.03 55 / 0.1)` | `0 2px 8px oklch(0.08 0.02 55 / 0.4)` |

---

## 3. Typography

Three typefaces, three roles, one mathematical skeleton.

### Red Hat Display -- Display and Headlines

Geometric sans-serif with humanist touches. Variable weight axis (300--900). Clean and structured, communicating the navigational quality of a knowledge base. The slightly rounded stroke endings and open counters keep it warm rather than clinical.

- **Usage:** Article titles, section headings, nav labels, UI elements, overlines, brand name
- **CSS:** `font-family: var(--font-display)` = `'Red Hat Display', system-ui, sans-serif`

### Newsreader -- Body and Long-form

Production Type's variable serif with optical sizing. Transitional old-style designed for sustained reading on screens. The editorial energy fits a knowledge base -- it says "here is something worth reading carefully."

- **Usage:** Article prose, definitions, subtitles, callout text, lead paragraphs
- **CSS:** `font-family: var(--font-body)` = `'Newsreader', Georgia, serif`
- **Optical sizing:** `font-variation-settings: 'opsz' 14` for body, `'opsz' 24` for subtitles, `'opsz' 72` for display use

### IBM Plex Mono -- Code and System

Monospace with human warmth. Rounded dots and open counters prevent it from feeling robotic. The systematic, engineered layer that signals "this is a considered, well-built thing."

- **Usage:** Code blocks, inline code, metadata, tags, overlines, footer text, breadcrumbs
- **CSS:** `font-family: var(--font-mono)` = `'IBM Plex Mono', 'Menlo', monospace`

### The Modular Scale -- Perfect Fourth (4:3 = 1.333)

Every font size is generated by multiplying the base (1rem) by powers of 4/3 -- the same ratio as the musical interval of a perfect fourth. Fluid between major third (1.25) at 360px viewport and perfect fourth at 1440px.

| Step | Clamp Value | Min | Max | Usage |
|------|-------------|-----|-----|-------|
| step 4 | `clamp(3.157rem, 2.93rem + 0.92vi, 3.553rem)` | ~50px | ~57px | Display / hero (Red Hat Display) |
| step 3 | `clamp(2.369rem, 2.2rem + 0.69vi, 2.665rem)` | ~38px | ~43px | Page title (Red Hat Display) |
| step 2 | `clamp(1.777rem, 1.65rem + 0.52vi, 2rem)` | ~28px | ~32px | Section heading (Red Hat Display) |
| step 1 | `clamp(1.333rem, 1.24rem + 0.39vi, 1.5rem)` | ~21px | ~24px | Lead paragraph / large body (Newsreader) |
| step 0 | `clamp(1rem, 0.93rem + 0.29vi, 1.125rem)` | ~16px | ~18px | Body text (Newsreader) |
| step -1 | `clamp(0.75rem, 0.7rem + 0.22vi, 0.844rem)` | ~12px | ~14px | Caption / small text (Newsreader) |
| step -2 | `clamp(0.563rem, 0.52rem + 0.18vi, 0.633rem)` | ~9px | ~10px | Micro / metadata (IBM Plex Mono) |

---

## 4. Spacing System

Same perfect fourth ratio as typography. Fluid via `clamp()` with `vi` units. 4px base, 8px primary rhythm.

### Base Tokens

| Token | Clamp Value | Min | Max | Typical Use |
|-------|-------------|-----|-----|-------------|
| `--space-3xs` | `clamp(0.25rem, 0.23rem + 0.1vi, 0.313rem)` | 4px | 5px | Tight inline gaps |
| `--space-2xs` | `clamp(0.5rem, 0.47rem + 0.15vi, 0.563rem)` | 8px | 9px | Icon-to-label, tag padding |
| `--space-xs` | `clamp(0.75rem, 0.7rem + 0.22vi, 0.844rem)` | 12px | 14px | Label-to-input, compact padding |
| `--space-s` | `clamp(1rem, 0.93rem + 0.29vi, 1.125rem)` | 16px | 18px | Default component padding |
| `--space-m` | `clamp(1.5rem, 1.4rem + 0.44vi, 1.688rem)` | 24px | 27px | Card padding, field-to-field |
| `--space-l` | `clamp(2rem, 1.86rem + 0.58vi, 2.25rem)` | 32px | 36px | Section padding |
| `--space-xl` | `clamp(3rem, 2.79rem + 0.88vi, 3.375rem)` | 48px | 54px | Major section gaps |
| `--space-2xl` | `clamp(4rem, 3.72rem + 1.17vi, 4.5rem)` | 64px | 72px | Hero padding |
| `--space-3xl` | `clamp(6rem, 5.58rem + 1.76vi, 6.75rem)` | 96px | 108px | Page-level vertical rhythm |

### Space Pairs

Space pairs interpolate more steeply between two named tokens, useful for responsive padding that needs a bigger jump.

| Token | Clamp Value | From | To |
|-------|-------------|------|----|
| `--space-s-m` | `clamp(1rem, 0.75rem + 1.24vi, 1.688rem)` | space-s | space-m |
| `--space-s-l` | `clamp(1rem, 0.5rem + 2.48vi, 2.25rem)` | space-s | space-l |
| `--space-m-l` | `clamp(1.5rem, 1.26rem + 1.18vi, 2.25rem)` | space-m | space-l |
| `--space-m-xl` | `clamp(1.5rem, 0.93rem + 2.82vi, 3.375rem)` | space-m | space-xl |
| `--space-l-xl` | `clamp(2rem, 1.49rem + 2.53vi, 3.375rem)` | space-l | space-xl |

### The Proximity Principle

**Intra-group spacing must always be tighter than inter-group spacing.** This activates gestalt proximity grouping -- the perceptual mechanism that makes related elements feel connected. Title-to-subtitle: `space-2xs`. Card padding: `space-m`. Between cards: `space-l`. Three different scales communicating three different structural relationships.

---

## 5. Layout

Every Layout primitives + Utopia fluid spacing. Zero media queries for layout. Single content-decision breakpoint at 60rem hides the left nav.

### Layout Tokens

| Token | Value | Purpose |
|-------|-------|---------|
| `--content-max` | `58ch` | Maximum prose width (optimal line length) |
| `--sidebar-width` | `280px` | Right sidebar (taxonomy card) |
| `--nav-width` | `220px` | Left nav (table of contents) |
| `--page-max` | `1320px` | Maximum page width |
| `--radius-s` | `4px` | Small radius (tags, brand mark) |
| `--radius-m` | `6px` | Medium radius (cards, code blocks) |
| `--radius-l` | `8px` | Large radius (info cards, search input) |
| `--radius-pill` | `100vmax` | Pill shape (theme toggle, tag pills) |

### Three-Column Structure

The wiki article layout uses three columns via flexbox with intrinsic sizing:

1. **Left nav** (`--nav-width`, sticky) -- Table of contents. Collapses below 60rem.
2. **Article** (flex-grow, min 60%) -- Main content area, prose capped at `--content-max`.
3. **Right sidebar** (`--sidebar-width`, sticky) -- Taxonomy info card. Wraps below article when space is tight.

The layout is built with the Sidebar primitive from Every Layout: columns collapse when the content area would be squeezed below its minimum inline size, with no media queries governing the arrangement.

---

## 6. Implementation

### CSS Cascade Layers

```css
@layer tokens, reset, base, layout, components, utilities;
```

Explicit layer ordering eliminates specificity wars. Tokens are lowest priority (overridable), utilities are highest.

### Token Architecture

Three tiers following the convergent pattern from Material Design, Carbon, and Polaris:

1. **Primitive tokens** -- Raw OKLCH values (`oklch(0.55 0.14 45)`)
2. **Semantic tokens** -- Role-based aliases (`--accent-primary`, `--text-secondary`)
3. **Component tokens** -- Implementation bindings (applied via CSS classes)

All semantic tokens are defined on `[data-theme="day"]` and `[data-theme="night"]`. Switching theme requires only changing the `data-theme` attribute on `<html>`.

### Fluid Type and Space

All type and spacing values use CSS `clamp()` for fluid interpolation between 360px and 1440px viewports. The `vi` unit (viewport inline) respects writing direction and zoom level, satisfying WCAG accessibility requirements. No breakpoints needed for sizing.

### Build Pipeline

1. **Haleiki CLI** (Rust) -- parses content, builds relationship graph, generates `_data/` JSON
2. **Cobalt** (Rust static site generator) -- reads Markdown + Liquid templates, outputs HTML
3. **Pagefind** (Rust search indexer) -- generates client-side search index from rendered HTML

Templates use `data-pagefind-body` on article content and `data-pagefind-filter` on taxonomy elements for faceted search.

### Design Principles

The seven principles from the Cowboys & Beans manifesto, as applied to Haleiki:

1. **Multiply, never add** -- geometric type scale (perfect fourth, 1.333)
2. **Perception is the only coordinate system** -- OKLCH colour, `rem`/`vi` units
3. **Name the role, not the value** -- semantic tokens over primitives in components
4. **Constraint liberates** -- 3 type families, 9 spacing values, constrained palette
5. **Unity is not uniformity** -- day/night themes from same token architecture
6. **Proximity carries meaning** -- spacing as syntax (tighter within, looser between)
7. **Design the invariants** -- tokens and scales ARE the design; pages are instances

---

## 7. Component Reference

### Topnav

Sticky, backdrop-blur, contains: brand mark + name, search input (with `⌘K` shortcut badge), nav links (Browse, Categories, Random), theme toggle (day/night pill).

### Info Card (Taxonomy Sidebar)

Right sidebar card with accent-coloured header, key-value rows, tag sections. Used on both source pages and concept pages.

### Prose

Body text container capped at `--content-max` (58ch). Styles for: paragraphs, h2/h3, links, strong, inline code, lists.

### Code Block

Bordered panel with header (language label), monospace pre, syntax colouring via span classes (`.kw`, `.fn`, `.cm`, `.st`, `.ty`).

### Callout

Info panel with left accent border. Title in display font, body in small body text. Used for "Why this matters" and cross-reference notes.

### See Also Grid

Auto-fill grid of link cards at the bottom of articles. Cards have title + description, hover shows accent border + subtle background.

### Tags

Pill-shaped labels in display font at step -2. Used in article meta, info card sections, and the hero tag row.

### Footer

Dark-surface columnar footer with: brand column (mark + tagline), three link columns (Knowledge Base, About, Development), content notice in mono, bottom bar with copyright + meta links.
