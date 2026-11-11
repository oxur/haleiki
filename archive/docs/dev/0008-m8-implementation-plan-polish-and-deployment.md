# M8 Implementation Plan: Polish, Dev Experience, Deployment Readiness

## Context

M1–M7 produced a functional, styled, trust-aware, navigable wiki with attribution and definitions. M8 closes out Phase 6 with polish: math rendering, code highlighting, responsive/accessibility audits, performance, dev-experience commands, documentation, and CI/CD.

## Design Decisions (Resolved)

### Math rendering: KaTeX (runtime, self-hosted)
Smaller (~30KB), faster, auto-render plugin scans `$...$` and `$$...$$`. Self-hosted in `assets/vendor/katex/` per local-first architecture. Runtime (client-side) — no Node.js build toolchain needed.

### Code highlighting: Cobalt's built-in syntect (no additional library needed)
Cobalt already applies syntax highlighting via syntect during its markdown-to-HTML build step. The rendered HTML already has colored `<span>` elements with inline styles. No Prism or highlight.js needed — Cobalt handles it natively. We just need to ensure the theme colors work with day/night modes.

### `haleiki demo serve`: Full rebuild on change
Simpler than incremental. Watch `demo/sources/`, `demo/concepts/`, `demo/_layouts/`, `demo/_includes/`, `assets/`. On change: re-run `haleiki demo build`, cobalt re-serves. Use `notify` crate for file watching.

### CI/CD: Author workflow, defer deployment
Write `.github/workflows/demo.yml` but don't enable deployment until GitHub Pages is configured. PR builds verify the pipeline; pushes to main deploy when ready.

### CSS minification: Skip for M8
Cobalt doesn't have built-in minification. Post-build minification would require a Node.js tool (csso, clean-css) or a Rust crate. The CSS is ~55KB total — not a meaningful performance bottleneck for a static demo. Defer to a future optimization pass.

## Implementation by Track

### Track A — KaTeX math rendering (M8.3)

**Files:**
- Download KaTeX dist to `assets/vendor/katex/` (katex.min.js, katex.min.css, auto-render.min.js, fonts/)
- Update all self-contained layouts to add KaTeX CSS + JS links
- Add math bootstrap script

**KaTeX config:**
```javascript
document.addEventListener("DOMContentLoaded", function() {
  renderMathInElement(document.body, {
    delimiters: [
      {left: "$$", right: "$$", display: true},
      {left: "$", right: "$", display: false}
    ],
    ignoredTags: ["script", "noscript", "style", "textarea", "pre", "code"],
    throwOnError: false
  });
});
```

### Track B — Code highlighting (M8.4)

**Finding:** Cobalt's syntect already applies syntax highlighting during build. Verify the existing highlighting works with day/night theme. If the inline styles conflict with night mode, add a small CSS override for syntect's generated spans.

No additional library download needed. This track is verification + minor CSS tweaks only.

### Track C — Responsive audit (M8.1)

Test at 360px, 768px, 1024px, 1440px. Fix issues found. Likely fixes:
- Topnav: wrap/stack on narrow viewports
- Trust bar: truncate description on narrow
- Sidebar: stack below content on narrow
- Code blocks: overflow-x scroll

### Track D — Accessibility audit (M8.2)

- Add skip-to-content link in all layouts
- Verify ARIA on theme toggle, trust bar
- Verify heading hierarchy
- Color contrast check on trust-tinted regions

### Track E — Performance (M8.5)

- Add `loading="lazy"` to images in rendered HTML (via build.rs post-processing or Liquid filter)
- Font defer already done (M3)
- CSS minification deferred

### Track F — `haleiki demo serve` (M8.6)

Implement in `tools/src/demo/mod.rs` → `DemoCommand::Serve`. Shell out to `cobalt serve` from `demo/_build/` after initial build. File watching deferred to a follow-up (cobalt serve has its own rebuild-on-change for templates).

Simple implementation: build once, then spawn cobalt serve as a child process.

### Track G — `haleiki dev` (M8.7)

Thin wrapper: checks `content/` exists, calls the same build+serve logic with `content/` as content_dir. Forward-compat for non-demo deployments.

### Track H — README (M8.8)

Update with quickstart, project description, links.

### Track I — CI/CD (M8.9)

`.github/workflows/demo.yml` — build + validate on PR, deploy on push to main (when configured).

## Implementation Order

1. Track A (KaTeX) — most visible improvement
2. Track B (code highlighting verification)
3. Track D (accessibility — skip-to-content link)
4. Track F (demo serve)
5. Track G (haleiki dev)
6. Track C (responsive fixes)
7. Track E (image lazy-load)
8. Track H (README)
9. Track I (CI/CD workflow)

## Verification

- `make check` passes
- `haleiki demo build` succeeds
- Math renders on quantum-mechanics, general-relativity pages
- Code blocks have syntax highlighting on rust-language, lfe-language
- `haleiki demo serve` starts and serves the site
- Skip-to-content link present
- Responsive at 360px (no overflow)
- CI workflow YAML is valid

## Approximate Scope

- **KaTeX vendor**: ~30 binary asset files (JS, CSS, fonts)
- **Layout updates**: ~20 lines across 8 layouts (KaTeX links + skip-to-content)
- **Rust**: ~50 lines (demo serve + dev commands)
- **CSS fixes**: ~30 lines (responsive + accessibility)
- **CI/CD**: ~60 lines YAML
- **README**: ~50 lines markdown
- **Total**: ~210 lines of authored code + vendor assets
