# M6 Implementation Plan: Profile System — Trust Bar and Info Card Tinting

## Context

M1–M5 produced a fully navigable wiki with styled pages, but no epistemic signaling. M6 adds the trust system: a colored trust bar below the topnav, trust glyphs in article overlines, and trust-tinted info card headers. Every page communicates its epistemic posture visually per the encyclopedia profile.

## Design Decisions (Resolved)

### Source-page trust default: Path B (template default)
Templates use `{% assign trust_level = page.data.trust_level | default: site.data.profile.trust.default %}`. No source-page frontmatter retrofit needed. The profile's `trust.default: "credible"` applies to all pages without an explicit `trust_level` field.

### CSS file organization: New `assets/css/07-trust-tokens.css`
Separate file in the `@layer tokens` cascade. Added as `@import "07-trust-tokens.css" layer(tokens)` in `main.css` AFTER `01-tokens.css`. This keeps trust tokens cleanly separated and follows the M3 forward-compat design.

### `data-trust` attribute: On `<html>`, per addendum §8.3
Set alongside `data-theme` on the `<html>` element. Compound selectors `[data-theme="day"][data-trust="credible"]` activate based on both. Theme toggle changes `data-theme` only; `data-trust` is static per page.

### Mockup value-space: Use correct trust values
Ignore mockup 21's `data-trust="established"` (canon-axis value). Use `corroborated | credible | provisional | speculative` per addendum §3.2. Flag mockup update as post-M6 cleanup.

### Glyph approach: Inline SVG with `currentColor`
Four SVG glyphs in `trust-glyph.liquid`, sized at 1em, using `currentColor` for stroke/fill. Conditional via `{% case trust_level %}`.

## Data Shape Verification ✅

### profile.json
Shape confirmed: `trust.enabled` (boolean), `trust.default` ("credible"), `trust.levels.<level>.label` and `.description` (strings), `trust.level_order` (array). All present and correctly typed.

### page.data.trust_level
- Concept cards: ✅ all have `trust_level: "credible"` in frontmatter
- Source pages: ❌ field absent — handled by Path B template default

## CSS Class Gaps

Trust-related classes needed (NONE exist in current CSS):
- `.trust-bar`, `.trust-bar-label`, `.trust-bar-desc`, `.trust-bar-link`
- `.trust-glyph`
- `.trust-overline` (for the glyph+label in article overline)
- Info card header trust tinting (via `data-trust` attribute selectors, not new classes)

All will be added in `05-components.css` (trust-bar component) and `07-trust-tokens.css` (color tokens).

## Files to Create

| File | Purpose | Approx lines |
|------|---------|------|
| `assets/css/07-trust-tokens.css` | 8 compound-selector blocks (4 trust × 2 themes) with 7 tokens each | ~120 |
| `demo/_includes/trust-bar.liquid` | Trust bar component below topnav | ~25 |
| `demo/_includes/trust-glyph.liquid` | 4 SVG glyphs (solid, half, outlined, dashed circle) | ~40 |

## Files to Modify

| File | Changes |
|------|---------|
| `assets/css/main.css` | Add `@import "07-trust-tokens.css" layer(tokens)` |
| `assets/css/05-components.css` | Add trust-bar component styles (~40 lines) |
| `demo/_layouts/source.liquid` | Set `data-trust` on `<html>`, add trust-bar include, update overline with glyph+label |
| `demo/_layouts/concept.liquid` | Same as source |
| `demo/_layouts/default.liquid` | Set `data-trust` on `<html>` (for index/home page) |
| `demo/_layouts/home.liquid` | Set `data-trust` on `<html>` |
| `demo/_layouts/category.liquid` | Set `data-trust` on `<html>` |
| `demo/_layouts/browse.liquid` | Set `data-trust` on `<html>` |

## Implementation Details

### 1. Trust tokens CSS (`assets/css/07-trust-tokens.css`)

8 compound-selector blocks, each defining 7 CSS custom properties:

```css
/* Corroborated — terracotta/amber (inherits wiki accent) */
[data-theme="day"][data-trust="corroborated"] {
    --trust-surface: oklch(0.97 0.015 45);
    --trust-border: oklch(0.85 0.05 45);
    --trust-accent: oklch(0.55 0.14 45);
    --trust-text: oklch(0.45 0.12 40);
    --trust-subtle: oklch(0.93 0.03 45);
    --trust-bar-bg: oklch(0.96 0.01 45);
    --trust-on-accent: oklch(0.97 0.01 80);
}

[data-theme="night"][data-trust="corroborated"] {
    --trust-surface: oklch(0.20 0.025 70);
    --trust-border: oklch(0.35 0.05 70);
    --trust-accent: oklch(0.72 0.14 70);
    --trust-text: oklch(0.78 0.12 72);
    --trust-subtle: oklch(0.24 0.04 65);
    --trust-bar-bg: oklch(0.18 0.02 65);
    --trust-on-accent: oklch(0.16 0.02 55);
}

/* credible — sage/teal (hue ~165°) */
/* provisional — dusty violet (hue ~295°) */
/* speculative — warm ochre/yellow (hue ~85°) */
```

Extract exact OKLCH values from mockup 21's CSS (substituting the correct trust level names for the mockup's canon-axis names).

### 2. Trust bar component (`demo/_includes/trust-bar.liquid`)

```liquid
{% if site.data.profile.trust.enabled %}
{% assign trust_level = page.data.trust_level | default: site.data.profile.trust.default %}
{% assign trust_info = site.data.profile.trust.levels[trust_level] %}
<div class="trust-bar trust-bar--{{ trust_level }}">
  <div class="trust-bar-label">
    {% include "trust-glyph.liquid" %}
    <span>{{ trust_info.label }}</span>
  </div>
  <div class="trust-bar-desc">{{ trust_info.description }}</div>
  <a href="/about/#trust-levels" class="trust-bar-link">What does this mean?</a>
</div>
{% endif %}
```

### 3. Trust glyph SVG (`demo/_includes/trust-glyph.liquid`)

```liquid
{% assign trust_level = page.data.trust_level | default: site.data.profile.trust.default %}
<span class="trust-glyph">
{% case trust_level %}
{% when "corroborated" %}
  <svg width="1em" height="1em" viewBox="0 0 16 16"><circle cx="8" cy="8" r="6" fill="currentColor"/></svg>
{% when "credible" %}
  <svg width="1em" height="1em" viewBox="0 0 16 16"><path d="M8 2a6 6 0 0 1 0 12V2z" fill="currentColor"/><circle cx="8" cy="8" r="6" fill="none" stroke="currentColor" stroke-width="1.5"/></svg>
{% when "provisional" %}
  <svg width="1em" height="1em" viewBox="0 0 16 16"><circle cx="8" cy="8" r="6" fill="none" stroke="currentColor" stroke-width="1.5"/></svg>
{% when "speculative" %}
  <svg width="1em" height="1em" viewBox="0 0 16 16"><circle cx="8" cy="8" r="6" fill="none" stroke="currentColor" stroke-width="1.5" stroke-dasharray="3 2"/></svg>
{% endcase %}
</span>
```

### 4. Trust bar component CSS (append to `05-components.css`)

```css
.trust-bar {
    display: flex;
    align-items: center;
    gap: var(--space-s);
    padding: var(--space-2xs) var(--space-s-l);
    background: var(--trust-bar-bg, var(--surface-secondary));
    border-bottom: 1px solid var(--trust-border, var(--border-subtle));
    font-family: var(--font-mono);
    font-size: var(--step--2);
    color: var(--trust-text, var(--text-secondary));
    transition: background 0.3s ease, border-color 0.3s ease;
}
.trust-bar--provisional { border-bottom-width: 2px; }
.trust-bar--speculative { border-bottom-width: 3px; border-bottom-style: dashed; }
.trust-bar-label { display: flex; align-items: center; gap: var(--space-3xs); font-weight: 600; }
.trust-bar-desc { flex: 1; }
.trust-bar-link { color: var(--trust-accent, var(--accent-text)); text-decoration: underline; }
.trust-glyph { display: inline-flex; align-items: center; }
```

### 5. Layout updates (`data-trust` on `<html>`)

In every self-contained layout, the `<html>` tag becomes:

```liquid
{% assign trust_level = page.data.trust_level | default: site.data.profile.trust.default %}
<html lang="en" data-theme="day"{% if site.data.profile.trust.enabled %} data-trust="{{ trust_level }}"{% endif %}>
```

The inline theme-init script remains unchanged (it only sets `data-theme`). The `data-trust` attribute is server-rendered and doesn't change at runtime.

### 6. Overline trust integration

In source.liquid and concept.liquid, the overline section becomes:

```liquid
<div class="article-overline">
  {% if site.data.profile.trust.enabled %}
    {% include "trust-glyph.liquid" %}
    <span class="trust-overline">{{ trust_info.label }}</span> ·
  {% endif %}
  <span>{{ existing_overline_content }}</span>
</div>
```

### 7. Info card header tinting

In `sidebar-taxonomy.liquid`, the info-card header background uses trust accent:

```liquid
<div class="info-card-header" style="{% if site.data.profile.trust.enabled %}background: var(--trust-accent); color: var(--trust-on-accent);{% endif %}">
```

This is an inline style override — when trust is disabled, it falls back to the base CSS.

## Implementation Order

1. Write `assets/css/07-trust-tokens.css` (extract values from mockup 21, correct the trust level names)
2. Update `assets/css/main.css` (add import)
3. Add trust-bar styles to `assets/css/05-components.css`
4. Write `demo/_includes/trust-glyph.liquid`
5. Write `demo/_includes/trust-bar.liquid`
6. Update all layouts: `data-trust` on `<html>`, trust-bar include, overline glyph
7. Update `demo/_includes/sidebar-taxonomy.liquid` (header tinting)
8. Build and verify: `make check && haleiki demo build`
9. Visual verify: compare corroborated vs credible pages (different tinting)

## Verification

- `make check` passes
- `haleiki demo build` succeeds
- `/source/dzogchen/` shows trust bar with "Credible" (default for source pages)
- `/ownership-rust/` shows trust bar with "Credible" (from concept frontmatter)
- Trust bar has sage/teal tinting (hue ~165° for credible)
- Article overline includes trust glyph (half-filled circle) + "Credible" label
- Info card header tinted with trust accent
- Day/night toggle preserves trust tinting (both data attributes coexist)
- If we manually set a concept card's trust_level to "provisional", the page renders with dusty violet tinting and outlined circle glyph

## Open Questions (For Review)

1. **Mockup cleanup**: Trust-bar mockup should be updated from canon-axis to trust-axis values. Flag as post-M6 task.
2. **Trust levels across the corpus**: All 186 concept cards are "credible" (encyclopedia default). To see visual differentiation, we'd need to manually vary some. Not blocking M6 — the CSS works regardless; it's just that the demo shows one color everywhere. Suggest manually setting 2-3 cards to other levels for visual testing.

## Approximate Scope

- **CSS**: ~160 lines (120 trust tokens + 40 trust-bar component)
- **Liquid includes**: ~65 lines (25 trust-bar + 40 trust-glyph)
- **Layout mods**: ~30 lines across 6 layouts (data-trust attribute + trust-bar include + overline update)
- **Total**: ~255 lines
