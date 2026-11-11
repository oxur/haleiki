# M3 Implementation Plan: Design System CSS Extraction

## Context

M1 and M2 produced a working but unstyled demo site (bare HTML with correct structure, populated `_data/` directory). M3 extracts the design system from the self-contained mockup HTML files into a proper layered CSS architecture, adds font loading and theme toggling, and wires the assets into the Cobalt build pipeline. After M3, pages look polished (typography, color, spacing) even without M4's full chrome (sidebar, breadcrumbs, nav).

## Design Decisions (Resolved)

### Typography: Red Hat Display + Newsreader + IBM Plex Mono

**Source of truth:** Brand guidelines (`docs/design/06-final/0004-haleiki-brand-guidelines.md`) — state: Final.

This is consistent with the home-page mockup (10), brand-guidelines mockup (09), and architecture doc §4.2. The earlier arch doc draft mentioned "DM Sans / Source Serif 4" but that was superseded by the Final brand guidelines. No conflict.

- **Display/headings:** Red Hat Display (variable, 300–900)
- **Body/long-form:** Newsreader (variable, opsz 6–72, wght 200–800)
- **Code/mono:** IBM Plex Mono (300, 400, 500, italic)

### Font hosting: Local (woff2)

Local files in `assets/fonts/`. Rationale: offline-friendly, reproducible builds, no third-party dependency, woff2 is compact. Google Fonts variable-font files are freely downloadable under OFL.

### Theme default: System preference, fallback day

On first load: check `localStorage('haleiki-theme')` → if unset, check `prefers-color-scheme` → if no preference, default to `day`.

### Asset location: Repo root (`assets/`)

Framework-level assets live at repo root (`assets/css/`, `assets/js/`, `assets/fonts/`). The M1 build orchestrator symlinks them into `demo/_build/assets/` for Cobalt, same pattern as layouts. This matches arch doc §3.6 and keeps the framework/demo distinction clean.

### Trust token forward-compat (M6)

M3 structures `01-tokens.css` so M6 simply adds compound-selector blocks (`[data-theme="day"][data-trust="credible"]`) inside the existing file or a new `07-trust-tokens.css`. The mockup's `data-trust="established"` uses canon-axis values (not trust-axis) — flagged as M6 reconciliation; M3 doesn't write trust tokens.

## Files to Create

| File | Purpose | Approx lines |
|------|---------|------|
| `assets/css/main.css` | Entry point; `@layer` declarations + `@import` | ~15 |
| `assets/css/01-tokens.css` | CSS custom properties: scale, spacing, color, layout | ~180 |
| `assets/css/02-reset.css` | Normalization + antialiasing | ~30 |
| `assets/css/03-base.css` | Element-level styles (body, headings, links, lists, code) | ~120 |
| `assets/css/04-layout.css` | Every Layout primitives (Stack, Cluster, Sidebar, Switcher) | ~80 |
| `assets/css/05-components.css` | Page-type components (topnav, footer, article, sidebar, cards) | ~250 |
| `assets/css/06-utilities.css` | Single-purpose modifiers (visually-hidden, flow, etc.) | ~30 |
| `assets/js/theme.js` | Day/night toggle with localStorage + system-preference | ~35 |
| `assets/fonts/RedHatDisplay-Variable.woff2` | Display font | binary |
| `assets/fonts/RedHatDisplay-Italic-Variable.woff2` | Display font italic | binary |
| `assets/fonts/Newsreader-Variable.woff2` | Body font | binary |
| `assets/fonts/Newsreader-Italic-Variable.woff2` | Body font italic | binary |
| `assets/fonts/IBMPlexMono-Regular.woff2` | Mono 400 | binary |
| `assets/fonts/IBMPlexMono-Medium.woff2` | Mono 500 | binary |
| `assets/fonts/IBMPlexMono-Light.woff2` | Mono 300 | binary |
| `assets/fonts/IBMPlexMono-Italic.woff2` | Mono 400 italic | binary |

## Files to Modify

| File | Changes |
|------|---------|
| `demo/_layouts/default.liquid` | Add CSS link, JS script, `data-theme` inline init |
| `demo/_layouts/source.liquid` | Add body class for page-type styling |
| `demo/_layouts/concept.liquid` | Add body class for page-type styling |
| `tools/src/demo/build.rs` | Add `setup_assets()` to symlink `assets/` into `demo/_build/` |
| `demo/_cobalt.yml` | No change needed (Cobalt copies all non-md/liquid files by default) |

## Implementation Details

### 1. `assets/css/main.css`

```css
@layer tokens, reset, base, layout, components, utilities;

@import "01-tokens.css" layer(tokens);
@import "02-reset.css" layer(reset);
@import "03-base.css" layer(base);
@import "04-layout.css" layer(layout);
@import "05-components.css" layer(components);
@import "06-utilities.css" layer(utilities);
```

### 2. `assets/css/01-tokens.css` (~180 lines)

Extract from mockup `10-home-page.html` (most comprehensive). Contents:

- `:root` block: ratio, step--2 through step-4, space-3xs through space-3xl, space pairs, font families, layout constants (content-max, page-max, sidebar-width, nav-width), radius values
- `[data-theme="day"]` block: complete semantic color tokens (surfaces, borders, text, accent, info, code, footer, shadows)
- `[data-theme="night"]` block: same tokens, night values
- `@font-face` declarations for the three font families (pointing to `/assets/fonts/`)

### 3. `assets/css/02-reset.css` (~30 lines)

- `*, *::before, *::after { margin: 0; padding: 0; box-sizing: border-box; }`
- `html { font-size: 100%; -webkit-font-smoothing: antialiased; scroll-behavior: smooth; }`
- `body` base setup: font-family, font-size, line-height, color, background, transition for theme switch, flex column, min-height 100vh

### 4. `assets/css/03-base.css` (~120 lines)

Element styles extracted from mockups:
- Headings (h1–h6): font-display family, step sizing, letter-spacing, line-height
- Body text: font-body, step-0, opsz 14, line-height 1.65
- Links: accent-text color, underline offset
- Lists (ul, ol): spacing, markers
- Code: inline `code` (mono font, code-surface bg, code-border, code-text, radius)
- Pre/code blocks: overflow, padding, border
- Blockquotes: left border, italic, padding
- Tables: border-collapse, th/td padding, border
- Images: max-width 100%, height auto
- Figure/figcaption: spacing, caption styling

### 5. `assets/css/04-layout.css` (~80 lines)

Every Layout primitives:
- `.stack > * + *` — vertical spacing (flow)
- `.cluster` — flexbox wrap with gap
- `.sidebar` — intrinsic sidebar layout (flex, min-width)
- `.switcher` — flex wrap with calculated break
- `.content-max` — max-width constraint
- `.page-max` — page-level max-width + auto margins

### 6. `assets/css/05-components.css` (~250 lines)

Page-specific components extracted from mockups:
- `.topnav` — sticky top, backdrop blur, flex, brand-mark
- `.site-footer` — dark surface, columns, links
- `.article` / `.source-page` / `.concept-card` — content wrappers
- `.overline` — mono font, small caps, accent color
- `.meta` — secondary text styling
- `.disambiguator` — muted parenthetical
- `.sidebar` (taxonomy info card) — placeholder class for M4
- `.category-card` — card component (border, radius, hover lift)
- `.hero` — centered section with large title
- `.theme-toggle` — pill-shaped toggle button styling

### 7. `assets/css/06-utilities.css` (~30 lines)

- `.visually-hidden` — accessible hide
- `.flow > * + *` — generic vertical rhythm (alias for stack)
- `.text-center`, `.text-small`, `.text-mono` — single-property overrides

### 8. `assets/js/theme.js` (~35 lines)

```javascript
(function() {
  const STORAGE_KEY = 'haleiki-theme';
  
  function getPreferredTheme() {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) return stored;
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) return 'night';
    return 'day';
  }
  
  function setTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem(STORAGE_KEY, theme);
  }
  
  // Apply immediately (called from inline script in <head>)
  setTheme(getPreferredTheme());
  
  // Expose for toggle button (M4 will wire this)
  window.haleikiToggleTheme = function() {
    const current = document.documentElement.getAttribute('data-theme');
    setTheme(current === 'day' ? 'night' : 'day');
  };
})();
```

### 9. Build orchestrator update (`tools/src/demo/build.rs`)

Add `setup_assets()` after `setup_layouts()`:

```rust
fn setup_assets() -> Result<()> {
    let link = format!("{BUILD_DIR}/assets");
    std::os::unix::fs::symlink("../../assets", &link)
        .context("failed to symlink assets")?;
    Ok(())
}
```

### 10. Default layout update (`demo/_layouts/default.liquid`)

```html
<!DOCTYPE html>
<html lang="en" data-theme="day">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{{ page.title }} — Haleiki Demo</title>
  <script>
    // Prevent flash of wrong theme
    (function(){
      var t = localStorage.getItem('haleiki-theme');
      if (!t) t = window.matchMedia('(prefers-color-scheme:dark)').matches ? 'night' : 'day';
      document.documentElement.setAttribute('data-theme', t);
    })();
  </script>
  <link rel="stylesheet" href="/assets/css/main.css">
  <script src="/assets/js/theme.js" defer></script>
</head>
<body>
  ...
</body>
</html>
```

Source and concept layouts: add appropriate body class (`<body class="page-source">` / `<body class="page-concept">`).

## Implementation Order

1. Download font files to `assets/fonts/`
2. Write CSS files (01-tokens through 06-utilities + main.css)
3. Write `assets/js/theme.js`
4. Update `tools/src/demo/build.rs` with `setup_assets()`
5. Update `demo/_layouts/*.liquid` with asset references
6. Build and verify: `make check && ./target/release/haleiki demo build`
7. Visual verify: `cd demo/_build && cobalt serve` → browser check

## Verification

After implementation:
- `make check` passes
- `./target/release/haleiki demo build` succeeds
- `demo/_site/assets/css/main.css` exists and imports all layers
- `demo/_site/assets/js/theme.js` exists
- `demo/_site/assets/fonts/` contains woff2 files
- Opening `http://localhost:1024/source/dzogchen/` shows styled page (correct fonts, warm colors, proper spacing)
- Opening `http://localhost:1024/ownership-rust/` shows styled concept card
- Theme toggle works (changing `data-theme` attribute via browser devtools switches day/night)
- Night mode has amber accent, dark surfaces

## Open Questions (For Review)

1. **Trust token mockup discrepancy**: Mockup 21 uses `data-trust="established"` (a canon-axis value). M3 won't write trust tokens, but the compound-selector pattern (`[data-theme][data-trust]`) should inform the token file structure. Flagged for M6 — no action in M3.

2. **Font file source**: Download from Google Fonts API as variable woff2? Or extract from another source? Google Fonts serves variable fonts via their API; I'll use that.

## Approximate Scope

- **CSS**: ~700 lines across 7 files
- **JS**: ~35 lines (theme toggle)
- **Fonts**: 8 binary files (~200-400KB total for variable woff2)
- **Rust**: ~10 lines (setup_assets function)
- **Liquid**: ~15 lines modified across 3 layouts
