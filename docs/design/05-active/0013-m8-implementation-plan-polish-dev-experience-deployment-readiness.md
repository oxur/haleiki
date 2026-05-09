---
number: 13
title: "Polish, Dev Experience, Deployment Readiness"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-05-08
updated: 2026-05-08
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# M8 Implementation Plan: Polish, Dev Experience, Deployment Readiness

Pre-implementation planning pass for milestone M8 of Phase 6 — the final milestone. Same pattern as M1–M7 plans: draft the implementation plan as a design doc, Duncan and I will review, then you execute.

**This pass is plan-only — do not implement yet.** Land the plan doc, report what's in it, surface design decisions you've made or want to surface for review.

M8 is wider than previous milestones — it's seven distinct tracks that close out the phase. Plan should treat each as its own clearly-scoped section.

---

## Reference docs

- `docs/design/06-final/0006-...site-build...project-plan.md` § M8 — milestone scope and task breakdown (M8.1 through M8.9)
- `docs/design/05-active/0002-haleiki-demo-site-design-document.md` § 9 — CI/CD pipeline spec (the GitHub Actions workflow shape)
- `docs/design/01-draft/0001-haleiki-architecture-document.md` § 5.1 — `haleiki dev` command spec (the user-facing equivalent for non-demo deployments)
- Demo source pages with math notation: `demo/sources/quantum-mechanics.md`, `demo/sources/general-relativity.md`, `demo/concepts/uncertainty-principle.md`, etc. — these have `$...$` and `$$...$$` from M2's math preprocessor
- Demo source pages with code blocks: `demo/sources/rust-language.md`, `demo/sources/erlang-language.md`, `demo/sources/lfe-language.md`, `demo/sources/lisp-language.md` — these have language-tagged fenced code blocks from M2's syntaxhighlight preprocessor
- M3's CSS layered files at `assets/css/*` — already use `font-display: swap`; need minification consideration
- M3's `assets/js/theme.js` — minimal, defer-loaded already
- `tools/src/demo/build.rs` — orchestrator extended in M1/M3/M5/M7; M8.6 adds `serve` integration

---

## Scope (from project plan § M8)

Goal: shippable demo. Responsive, accessible, fast, with a clean dev-experience flow and a path to deployment.

Tasks per the project plan (M8.1–M8.9):

- **M8.1** — Responsive audit at 360px–1440px viewports
- **M8.2** — Accessibility audit (semantic HTML, ARIA, color contrast, keyboard nav, screen reader)
- **M8.3** — Math notation rendering (KaTeX or MathJax)
- **M8.4** — Code block syntax highlighting (Prism or highlight.js)
- **M8.5** — Performance polish (CSS minification, font defer, image lazy-load, SVG optimization)
- **M8.6** — Implement `haleiki demo serve` (build + watch + live-reload)
- **M8.7** — Implement `haleiki dev` (user-facing equivalent for non-demo deployments)
- **M8.8** — Update `README.md` with quickstart and demo link
- **M8.9** — Author CI/CD GitHub Action (deferrable per project plan)

After M8, the demo is shippable: navigable, styled, trust-aware, attribution-complete, math-rendering, code-highlighted, accessible, performant, with a clean local-dev flow and a deployment path.

---

## Plan document requirements

Write the implementation plan to `docs/dev/0008-m8-implementation-plan-polish-and-deployment.md`. Mirror under `docs/design/05-active/` if that's the project's settled convention.

The plan should organize by track. Each track is independent enough to be implemented and verified separately. Suggested ordering: M8.3, M8.4 (visible content rendering) → M8.1, M8.2 (audits) → M8.5 (performance) → M8.6, M8.7 (dev experience) → M8.8 (docs) → M8.9 (CI/CD).

The plan should cover:

### Track A — Math notation rendering (M8.3)

**Decision: KaTeX or MathJax.**

- **KaTeX**: faster, smaller (~30KB compressed), supports most common LaTeX. Active development. Auto-render plugin scans rendered HTML for `$...$` and `$$...$$` and replaces in-place.
- **MathJax**: heavier (~200KB+), more complete LaTeX, mature accessibility. Slower client render.

**Recommend KaTeX with auto-render.** Self-host the script + CSS in `assets/js/` and `assets/css/` (or a vendor subdir) per the architecture's local-first preference. Wire via `default.liquid` (or via a math-aware include).

**Implementation sketch:**
- Download KaTeX dist (`katex.min.js`, `katex.min.css`, `auto-render.min.js`, fonts) into `assets/vendor/katex/`
- Update `default.liquid` to add `<link rel="stylesheet" href="/assets/vendor/katex/katex.min.css">` and `<script defer src="/assets/vendor/katex/katex.min.js"></script>` `<script defer src="/assets/vendor/katex/auto-render.min.js"></script>`
- Add a small bootstrap script that calls `renderMathInElement(document.body, { delimiters: [{left: "$$", right: "$$", display: true}, {left: "$", right: "$", display: false}] })` after DOMContentLoaded
- Verify against `quantum-mechanics`, `general-relativity`, `homotopy-type-theory`, `category-theory`, `topological-qft`, `supersymmetry` (the math-heavy articles from earlier rounds)

**Edge cases to plan for:**
- Math inside code blocks should NOT render as math (escape with `\$` or use `\\(\\)` form, or KaTeX's `ignoredTags: ['code', 'pre']` option)
- Math in image alt text: KaTeX shouldn't touch alt
- Multi-line display math with internal newlines

**Build-time vs runtime decision:** runtime (client-side) recommended for simplicity. Build-time KaTeX would require integrating a Rust crate or shelling out to a node tool, which adds toolchain complexity for marginal benefit. Plan should confirm runtime.

### Track B — Code block syntax highlighting (M8.4)

**Decision: Prism or highlight.js.**

- **Prism**: themable, requires explicit language hints, smaller core, plugin ecosystem
- **highlight.js**: auto-detects language, larger out-of-box, simpler API

**Recommend Prism.** Reasons: (1) M2's syntaxhighlight preprocessor produced fenced code blocks with explicit language hints (`lisp`, `rust`, `erlang`, etc.); (2) themeable via CSS to match Haleiki's day/night cascade; (3) smaller for the language subset we actually use.

**Languages to bundle:** rust, erlang, lisp/scheme, javascript (for theme.js examples in docs), bash (for shell snippets), yaml (frontmatter examples in docs), markdown (for nested examples). Not all 200 of Prism's languages — pick the demo's actual needs.

**Implementation sketch:**
- Download Prism with selected languages from prismjs.com download builder, save to `assets/vendor/prism/`
- Choose a Prism theme that fits Haleiki's warm palette — probably custom, derived from a base theme (Solarized Light or similar) and colorshifted to use OKLCH tokens. Plan should propose a path: copy a base theme and remap colors to match `--code-text`, `--code-surface`, etc., or use a starter theme as-is for M8 and customize later.
- Day/night theme awareness: Prism's syntax tokens should pick up theme tokens. Probably a small custom CSS file that maps Prism token classes to Haleiki tokens (`.token.keyword { color: var(--code-keyword); }` etc.).
- Wire via `default.liquid` similar to KaTeX
- Verify against `rust-language`, `lfe-language`, `erlang-language`, `lisp-language`

**Plain code blocks (no language) should still render**, just without highlighting — Prism handles this gracefully.

### Track C — Responsive audit (M8.1)

The mockups are designed fluid via `clamp()` and Every Layout primitives. M3 extracted these. M8.1 verifies they hold up.

**Audit method:**
- Browser devtools at fixed breakpoints: 360px, 480px, 768px, 1024px, 1440px
- Pages to test: home, browse, category (e.g., `/category/tibetan-buddhism/`), source (`/source/dzogchen/`, `/source/quantum-mechanics/`), concept (`/ownership-rust/`), about, attribution
- Specifically check: topnav (does it collapse / wrap cleanly?), sidebar info card (stacks below content on narrow, beside content on wide?), see-also/provenance grid layouts (reflow gracefully?), trust bar (legible at 360px?), code blocks (overflow-x scroll vs wrap?)

**Implementation:** likely small CSS additions to `05-components.css` for narrow-viewport behaviors (the existing fluid sizing should mostly handle it; specific component re-flows may need explicit @media or Every-Layout switcher tweaks). Plan should propose the audit-and-fix loop, not pre-write fixes.

### Track D — Accessibility audit (M8.2)

**Audit method:**
- Automated: axe-core or Pa11y against rendered pages (browser extension or CLI)
- Manual: keyboard navigation through home → category → source → concept; semantic HTML check; color contrast at trust-tinted regions
- Screen reader smoke test: VoiceOver (Mac) or NVDA (Windows) on at least home, source, concept

**Specific concerns to verify:**
- Trust bar conveys epistemic posture — does the screen reader announce trust level + label clearly?
- Color contrast on trust-tinted info card headers (especially `provisional` violet and `speculative` ochre on text)
- Theme toggle button: labeled, focus-visible, state announced
- Skip-to-content link present (probably missing — likely an addition needed)
- All images have meaningful alt text — the demo pipeline should have set these via M2 round-2 alt-text fix; spot-check
- Heading hierarchy: source-page bodies are user prose; concept-card bodies have standardized H2/H3 structure; verify no skipped levels
- Form controls: theme toggle button, eventually search input — ensure proper labels

**Implementation:** likely small markup additions (skip-to-content link, ARIA where the audit flags), color-token tweaks where contrast fails. Plan proposes the audit-and-fix loop.

### Track E — Performance polish (M8.5)

**Sub-tasks:**
- **CSS minification**: cobalt build may or may not minify CSS. Either configure cobalt to minify, or run a post-build pass. Recommend a minification step in the build orchestrator that processes `_site/assets/css/*.css` (or use Cobalt's built-in if available). Plan should investigate cobalt-bin's capabilities.
- **Font defer**: already done in M3 (`font-display: swap`).
- **Image lazy-load**: add `loading="lazy"` to `<img>` elements that are below the fold. Implementation: a Liquid filter in source.liquid and concept.liquid that adds `loading="lazy"` to all `<img>` inside the article body. Or a build-time markdown post-processor.
- **SVG optimization**: SVGO pass on `assets/icons/` (favicons are PNG, but the trust glyphs from M6 are inline SVG — already minimal). Possibly minor.
- **Asset cache headers**: not directly under M8 control (depends on hosting), but worth noting in plan.
- **Critical CSS**: probably overkill for static demo. Skip.

Plan should prioritize: CSS minification (real win), image lazy-load (real win for image-heavy articles like `quantum-mechanics` with many figures), defer the rest.

### Track F — `haleiki demo serve` (M8.6)

**Behavior:**
1. Run `haleiki demo build` once (initial)
2. Start `cobalt serve` from `demo/_build/`
3. Watch for changes in `demo/sources/`, `demo/concepts/`, `_layouts/` (or `demo/_layouts/`), `_includes/` (if any), `assets/`, `demo/profile.yaml`, `demo/taxonomy.yaml`
4. On change: re-run the relevant build step (or full `haleiki demo build` for simplicity), trigger cobalt to re-serve
5. Live-reload via cobalt's built-in mechanism (it has a `--drafts` and serve mode; check whether it supports live-reload natively or needs a small JS injection)

**Implementation:**
- Use `notify` crate (or similar) for file watching
- For each watch event: invoke the function-call equivalent of `haleiki demo build` (already factored into a function in M1+ orchestrator)
- Cobalt's `serve` mode auto-rebuilds the cobalt step; we may only need to handle the upstream pipeline (clean, rewrite, media, convert, frontmatter, haleiki build)
- Ctrl-C handling: clean shutdown of both file watcher and cobalt subprocess

**Plan should specify** the watch granularity (full rebuild vs incremental) and the live-reload mechanism (if any). Full rebuild is simpler; incremental is faster but requires more orchestration. Recommend full rebuild for M8 with a note about incremental as future work.

### Track G — `haleiki dev` (M8.7)

The user-facing version of `demo serve`, for non-demo deployments per arch 0001 § 5.1. Currently the project has only the demo deployment, so this is mostly forward-compat.

**Behavior:** identical to `demo serve` but operates on `content/` instead of `demo/`.

**Implementation:** since the demo deployment IS the only deployment right now, `haleiki dev` likely just calls into the same orchestrator code with `content/` as the path. May be a thin wrapper that checks `content/` exists and is non-empty, then delegates.

Plan should propose: thin shared implementation that takes a `content_dir` argument; `demo serve` and `dev` each pass their respective directories. Avoids code duplication.

### Track H — README + docs (M8.8)

**README updates:**
- Quickstart: `cd demo && haleiki demo serve` (after the binary is built; provide build command too)
- Brief project description (probably already there from earlier; just verify)
- Link to live demo URL — placeholder until M8.9 deploys
- Link to documentation (project plan, architecture, deployment profiles, concept-card howto)

**Other doc updates worth doing:**
- `docs/dev/concept-cards/0001-...` and `0002-...` — verify v1.3 is current and references are consistent
- Project README for the framework (vs the demo) — clarify the framework/demo split if not already

Plan should propose the README structure and call out any other docs that need touching.

### Track I — CI/CD GitHub Action (M8.9)

Per design doc 0002 § 9, the workflow YAML structure is already specified. M8.9 implements it.

**Workflow shape:**
- Trigger: push to main (deploy), pull request to main (build-only, no deploy)
- Steps: checkout → Rust toolchain → cargo build (release, demo feature) → haleiki demo build → install cobalt-bin → cobalt build → haleiki demo validate → deploy to GitHub Pages (peaceiris/actions-gh-pages or actions/deploy-pages)

**Plan should decide:**
- Use `actions/deploy-pages` (newer, official) or `peaceiris/actions-gh-pages` (older, more documented). Recommend `actions/deploy-pages`.
- GitHub Pages source config (deploy from Actions vs deploy from branch)
- Build cache strategy: `Swatinem/rust-cache@v2` for Rust, default cobalt-bin install (or cache that too)
- Whether M8.9 actually deploys (if the GitHub Pages site doesn't exist yet, the workflow can be authored but disabled / on a feature branch until pages are configured). Project plan says: "may be deferred to a follow-up phase if local-demo readiness is the milestone."

**Recommend:** author the workflow YAML in M8 but don't enable deployment until Duncan has configured GitHub Pages for the repo. The workflow can run on PR builds without deploying to verify the build pipeline is CI-clean.

---

## Out of scope (what M8 explicitly doesn't do)

- **Search** — separate phase entirely (Pagefind integration per design doc 0001 § 6).
- **Writer profile features** — canon badges, brief/sketch page types. Forward-compat only.
- **News section** — mockups 16-20 sketch a news/blog area. Forward design.
- **Per-article social sharing, comments, dynamic anything** — never; Haleiki is read-only static.
- **Image proxying / CDN integration** — out of scope.

---

## Open questions to surface in the plan

1. **KaTeX vs MathJax** — recommend KaTeX; confirm.
2. **Prism vs highlight.js** — recommend Prism; confirm.
3. **Build-time vs runtime math/code rendering** — recommend runtime; confirm.
4. **Prism theme strategy** — start with a starter theme remapped to Haleiki tokens, or custom from scratch?
5. **CSS minification approach** — cobalt config vs post-build pass. Plan should investigate cobalt-bin.
6. **`haleiki demo serve` watch granularity** — full rebuild on change vs incremental. Recommend full for M8.
7. **CI/CD deployment trigger** — author workflow now, deploy when Duncan configures GitHub Pages.
8. **Class vocabulary check** — does any new component (KaTeX wrappers, Prism token classes) need additions to `05-components.css`?
9. **Math + code in conjunction** — pages with both (e.g., a Rust article that mentions trait constraints with math notation in the trait bound). Unlikely in current corpus but plan should note no conflict between renderers (KaTeX scans body, Prism applies to fenced code; they shouldn't overlap).
10. **Self-hosted vs CDN for KaTeX/Prism** — recommend self-hosted for offline-friendly + reproducibility (consistent with M3's font choice).

---

## Reporting

When the plan doc is landed, report:

1. **File path** — `docs/dev/0008-m8-implementation-plan-...` (or design-doc mirror).
2. **Doc structure** — section headings only.
3. **Design decisions you made** — math renderer, code highlighter, runtime vs build-time, watch strategy, CI/CD deployment posture.
4. **Design decisions you surfaced for review** — any open questions.
5. **Class vocabulary check** — new classes needed for KaTeX/Prism/skip-to-content/etc.
6. **Approximate scope per track** — line counts (CSS, JS, Rust, YAML, docs), total LOC and binary-asset estimate.
7. **Anything surprising** during the planning pass.

Don't commit the plan yet — Duncan will review and either approve as-is or ask for revisions before kicking off execution.

---

If you encounter a substantive design question that you can't resolve confidently from the project plan, design docs, mockups, and existing M1–M7 code, surface it in the plan with a "for review" note rather than picking silently. M8 closes out the phase — it's worth getting the design calls right rather than racing.
