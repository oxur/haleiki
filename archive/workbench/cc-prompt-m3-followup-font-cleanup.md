# M3 Follow-Up: Font Cleanup

Three small but firm fixes to close out M3 cleanly. Visual rendering and theme-toggle behaviour are confirmed working — these are pure cleanup of the font asset pipeline plus a doc-text correction.

**Each numbered item below is a MUST. Do not opt out, partially apply, or argue scope. Report back with all three completed.**

---

## Reference

- `assets/css/01-tokens.css` — current `@font-face` declarations
- `assets/fonts/` — current font file inventory
- `docs/dev/0003-m3-implementation-plan-design-system-css-extraction.md` — M3 plan (also mirrored at `docs/design/05-active/0008-m3-implementation-plan-design-system-css-extraction.md` if that copy is the canonical one — apply the same edit there)
- Brand guidelines `docs/design/06-final/0004-haleiki-brand-guidelines.md` § 3 — IBM Plex Mono uses 300 / 400 / 500 / italic

---

## 1. MUST: switch IBM Plex Mono to `.woff2` everywhere and add the missing weights

The current `01-tokens.css` references `IBMPlexMono-*.ttf` for Regular, Medium, and Italic. `.ttf` files are roughly 3–4× the size of `.woff2`. The Regular and Medium `.woff2` files are *already on disk* (`assets/fonts/IBMPlexMono-Regular.woff2`, `assets/fonts/IBMPlexMono-Medium.woff2`). The brand guidelines also call for a Light (300) weight which is currently missing entirely.

You **MUST**:

(a) **Download** `.woff2` versions of `IBMPlexMono-Italic` and `IBMPlexMono-Light` (300 weight) into `assets/fonts/`. The IBM Plex Mono fonts ship under the SIL Open Font License — the canonical source is the IBM/Plex GitHub release; the `IBM-Plex-Mono/web/` directory has `.woff2` files for all weights and styles.

(b) **Update** every `@font-face` declaration in `assets/css/01-tokens.css` for IBM Plex Mono to reference the `.woff2` file with `format('woff2')`. Specifically:

```css
@font-face {
    font-family: 'IBM Plex Mono';
    src: url('/assets/fonts/IBMPlexMono-Light.woff2') format('woff2');
    font-weight: 300;
    font-style: normal;
    font-display: swap;
}

@font-face {
    font-family: 'IBM Plex Mono';
    src: url('/assets/fonts/IBMPlexMono-Regular.woff2') format('woff2');
    font-weight: 400;
    font-style: normal;
    font-display: swap;
}

@font-face {
    font-family: 'IBM Plex Mono';
    src: url('/assets/fonts/IBMPlexMono-Medium.woff2') format('woff2');
    font-weight: 500;
    font-style: normal;
    font-display: swap;
}

@font-face {
    font-family: 'IBM Plex Mono';
    src: url('/assets/fonts/IBMPlexMono-Italic.woff2') format('woff2');
    font-weight: 400;
    font-style: italic;
    font-display: swap;
}
```

After this, no `@font-face` rule in `01-tokens.css` should reference a `.ttf` URL.

(c) **Delete** the now-unused IBM Plex Mono `.ttf` files from `assets/fonts/`:

- `assets/fonts/IBMPlexMono-Regular.ttf`
- `assets/fonts/IBMPlexMono-Medium.ttf`
- `assets/fonts/IBMPlexMono-Italic.ttf`

After this, `assets/fonts/` should contain *only* `.woff2` files.

---

## 2. MUST: delete the unused `.ttf` files for Red Hat Display and Newsreader

The variable-font `.woff2` files for Red Hat Display and Newsreader (regular + italic) are already in use by CSS. The `.ttf` versions sitting alongside them are unreferenced dead weight and **MUST** be deleted:

- `assets/fonts/RedHatDisplay-Variable.ttf`
- `assets/fonts/RedHatDisplay-Italic-Variable.ttf`
- `assets/fonts/Newsreader-Variable.ttf`
- `assets/fonts/Newsreader-Italic-Variable.ttf`

Verify after deletion that `_site/assets/fonts/` (after a fresh `haleiki demo build`) contains only `.woff2` files.

---

## 3. MUST: update the M3 plan doc to reflect copy-not-symlink

The plan document at `docs/dev/0003-m3-implementation-plan-design-system-css-extraction.md` (and its mirror at `docs/design/05-active/0008-m3-implementation-plan-design-system-css-extraction.md` if present) describes `setup_assets()` as **symlinking** the repo-root `assets/` directory into `demo/_build/assets/`. The actual implementation in `tools/src/demo/build.rs` uses `copy_dir_for_cobalt` because cobalt-bin doesn't reliably traverse symlinks — see commit `06284a7 Fixed cobalt asset pipeline and dry-run test failures`.

You **MUST** update the plan-doc text where it says "symlink" (specifically in the Files-to-Modify table for `tools/src/demo/build.rs` and in the implementation sketch for `setup_assets()`) to say "copy" with a brief note: *"Copy rather than symlink because cobalt-bin doesn't reliably traverse symlinks for asset directories — see commit `06284a7`."*

Apply the same edit to both copies of the plan if both exist.

---

## Verification

1. Confirm `assets/fonts/` contains only `.woff2` files, including new `IBMPlexMono-Italic.woff2` and `IBMPlexMono-Light.woff2`.
2. `grep -i 'truetype\|\.ttf' assets/css/01-tokens.css` returns zero matches.
3. `cargo build --release --features demo --manifest-path tools/Cargo.toml && ./tools/target/release/haleiki demo build` succeeds end-to-end.
4. `_site/assets/fonts/` contains only `.woff2` files (no `.ttf` orphans).
5. Open `demo/_site/source/dzogchen/index.html` (or any rendered page) in a browser and confirm fonts still load correctly — code blocks should now use IBM Plex Mono `.woff2` instead of `.ttf`. Check Network tab to confirm.
6. Plan doc says "copy" not "symlink" in the relevant places.

---

## Reporting

Standard pattern. Report:

1. New font files added (filenames + sizes).
2. Font files deleted (count + total bytes saved).
3. CSS changes (number of `@font-face` blocks updated).
4. Plan doc edits applied (which file(s), what changed).
5. Verification output (the four checks above).
6. Anything surprising — IBM Plex Mono `.woff2` source URL used, any download issues, anything else.

Don't argue scope. The three items are firm requirements. Report when all three are complete.
