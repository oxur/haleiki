# Slice 01 — minijinja shell · CDC Verification

> Independent verification of [`closing-report.md`](./closing-report.md) against
> the artifacts, per the collaboration-framework's `LEDGER-DISCIPLINE.md`
> Section A (CDC protocol) and the slice bubble-up check in
> `PROJECT-MANAGEMENT.md` Part IV. The closer (CC) is structurally separate from
> this verifier. Verdict: **CLOSED** · 2026-07-28.

## Verifier and method

Verified by an independent Claude context (Cowork), **not** the implementing
context. Reproduction was done in a clean cloud build, not the doer's working
tree: the slice's source (`tools/`, workspace `Cargo.toml`/`Cargo.lock`) and the
repo-root `templates/` were staged into a fresh container, dependencies resolved
from crates.io (`minijinja 2.21`, `serde_yaml 0.9`, …), and the toolchain was
`cargo 1.95.0 / rustc 1.95.0`. Every `done` row's Verify command was re-run
there; grep/inspection rows were re-run against the committed tree at `d6f882e`
on the device. This is the `attested → reproduced` transition.

## Row-count check

Ledger opened with **8** rows (F-1…F-8); the closing report walks **8**; this
verification reproduces **8**. No missing rows, no silent drops.

## Per-row verification

| ID | Closer status | CDC verdict | Strength | How reproduced |
|----|---------------|-------------|----------|----------------|
| F-1 | done | **confirmed** | reproduced | `cargo test --bin haleiki render` → `test_environment_loads_and_renders_404_template ... ok`. `pages/404.jinja` + its `extends`/`include` chain load and render non-empty HTML. |
| F-2 | done | **confirmed** | reproduced | Same suite → `test_resolve_profile_sample_yaml_parses_full_shape ... ok`. `Profile` deserializes `templates/profile.sample.yaml`; trust levels/order, taxonomy, footer all populated and typed. |
| F-3 | done | **confirmed** | reproduced | Same suite → `test_base_jinja_renders_chrome_from_profile ... ok`. `<title>`, the `haleiki.css` link, the no-flash theme script, topnav + footer all present. |
| F-4 | done (disclosed deviation) | **confirmed** | reproduced | Same suite → `test_escaping_safe_vs_autoescaped_fields ... ok`. Injected `<b>x</b>` into `about_intro` renders as `&lt;b&gt;x&lt;…` (escaped) while `about_body` renders unescaped. **The auto-escape finding is real and the fix is correct** (see below). |
| F-5 | done | **confirmed** | reproduced | `grep` of `templates/` for the 8 old modular CSS names → 0 matches; `haleiki.css` referenced at `base.jinja:30`; `assets/js/` has `search.js`, `toc-scrollspy.js`, `browse-scrollspy.js`, `theme.js` (+ `math.js`). |
| F-6 | done | **confirmed** | reproduced | `assets/fonts/` has exactly 8 `.woff2`; `assets/css/haleiki.css` has 8 `@font-face` blocks with 8 `url('../fonts/…woff2')` refs (RedHatDisplay ×2, Newsreader ×2, IBMPlexMono ×4). |
| F-7 | done (disclosed fixture) | **confirmed** | reproduced | Same suite → `test_active_nav_block_capture_marks_topnav_item ... ok`. A child setting `{% block active_nav %}browse{% endblock %}` marks Browse `class="active" aria-current="page"` in the included topnav; Categories not marked. No route file added under `templates/pages/`. |
| F-8 | done (scoped to new code) | **confirmed, with a tracked finding** | reproduced | `cargo fmt --check` → exit 0 (whole crate). `cargo clippy --all-targets` (default) **and** `--all-features -- -D warnings`: `render.rs` produces **0** findings in both. The failures are entirely pre-existing (finding F below). |

Full render suite result reproduced verbatim: `running 5 tests … test result:
ok. 5 passed; 0 failed; 0 ignored`.

## Deviations reviewed

- **F-4 field choice** — the closing report injects the control string into
  `profile.about_intro` and renders `pages/about.jinja` (the README-documented
  consumer of both `about_intro` and `about_body`) instead of `profile.name`.
  **Accepted:** this is stronger evidence, not weaker — it exercises the `| safe`
  and auto-escaped fields the contract actually pairs, on a real template.
- **F-7 in-memory template** — the active-nav test registers its child via
  `env.add_template` rather than adding a file under `templates/pages/`.
  **Accepted:** it demonstrates the block-capture mechanism while staying inside
  the "no non-404 route" boundary. Correct call.

Neither deviation is spec-softening: in both cases the demonstrated guarantee is
equal to or stronger than the criterion stated.

## The auto-escape finding (independently confirmed)

The load-bearing finding is real. minijinja's default auto-escape callback keys
off `.html`/`.htm`/`.xml` and treats `.jinja` as an ignored suffix, so this
template set (all `*.jinja`) would resolve to `AutoEscape::None` — the entire
`| safe` discipline would ship as a silent no-op, everything unescaped. The fix
(`build_environment()` sets `set_auto_escape_callback(|_| AutoEscape::Html)`) is
correct and is what F-4 proves. This is exactly the class of defect the ledger's
per-row-with-evidence discipline exists to catch, and it was caught because F-4
was written as its own row rather than folded into a prose "escaping works."

## Bubble-up check (PROJECT-MANAGEMENT.md Part IV)

1. **Did the slice deliver its assigned piece?** Yes. Against `arc-plan.md`'s
   slice breakdown, slice 01 is "the minijinja shell: Environment + path_loader,
   profile resolution, asset swap, base/topnav/footer for a smoke route." All
   delivered and reproduced. Reuse boundary honored: `git show d6f882e --stat`
   confirms `parser.rs`/`graph.rs`/`validator.rs` are untouched; `main.rs` is
   +2 lines (`mod render;`).
2. **Silent-drop diff complete and honest?** Yes. 8/8 rows walked; the closing
   report's "what was NOT done" list (CLI wiring, markdown, article context,
   `demo/profile.yaml` migration, the pre-existing lint debt) matches the slice
   boundary. Nothing is buried.
3. **Do the findings require an arc-plan change?** Yes — three, dispositioned:
   - **Finding A — auto-escape is per-`Environment`.** Already recorded in
     `arc-plan.md` v1.1. Concur; A-6 must verify the callback is set on whatever
     `Environment` slices 02/03 use, not just that one page's output is escaped.
     No further arc-plan change needed beyond confirming it stands.
   - **Finding B — pre-existing lint debt blocks `make lint`/CI.**
     Independently confirmed and reproduced: `generator.rs:541`
     (`unnecessary_sort_by`), `main.rs:57` (`map_unwrap_or`, in the pre-existing
     `Dev` command), and 4 in `demo/{convert,frontmatter,media,rewrite}.rs`
     (surfaced under `--all-features`). All predate this slice (`git diff HEAD --
     <file>` empty; none in `d6f882e`). **Disposition:** out of scope for slice
     01; tracked as an arc finding routed to a dedicated small cleanup slice
     (fold into Arc 06 "remove Cobalt/Liquid remnants + lint", or a standalone
     near-term slice). Recorded in `arc-plan.md` v1.2. Consequence to flag: do
     not treat `make check`/`make lint` as a green merge gate on this branch
     until it is fixed.
   - **Finding C — `demo/profile.yaml` not migrated to the new shape.** The
     `Profile` struct's required fields mean the Cobalt-era `demo/profile.yaml`
     will not deserialize; slice 01 correctly tested against
     `templates/profile.sample.yaml`. **Disposition:** belongs to the first slice
     that renders real demo content (Arc 01 slice 03, or Arc 02); name it in that
     slice's scope so it isn't assumed done. Recorded in `arc-plan.md` v1.2.

## What worked (Safety-II)

- Writing **F-4 as its own ledger row** is what surfaced the auto-escape
  landmine. Had escaping been a sub-clause of "chrome renders," the no-op would
  have shipped. The discipline paid for itself in the first slice.
- CC's closing report modeled the standard well: per-row walk, disclosed
  deviations, git-verified pre-existing-debt claim, and an explicit "not done"
  list. Verification was fast because the evidence was concrete and reproducible.

## Verdict

**Slice 01 is CLOSED.** 8 rows: 8 `done`, 0 deferred, 0 no-op — all reproduced at
slice scale. The proposed-done state in the closing report is upgraded to closed.
Arc ledger row **A-1** is satisfied (evidence: this file). Findings B and C are
tracked in `arc-plan.md` v1.2; Finding A stands from v1.1.
