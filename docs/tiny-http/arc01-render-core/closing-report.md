# Arc 01 — Render Core · Closing Report

> The arc-level close, per `PROJECT-MANAGEMENT.md` Part V and
> `LEDGER-DISCIPLINE.md` Section B: capability verdict, slice walk, the
> composition check (arc ledger A-4…A-7 **reproduced at arc scale**), the
> accumulated arc-plan change log, and the bubble-up to the project. Status:
> **CLOSED** · 2026-07-28.

## 1. Capability — restated and verdict

**Capability (from `arc-plan.md`):** render a fully-chromed wiki article — both
concept and source page types — to correct HTML from the real demo corpus, using
minijinja over Claude Design's refactor templates and the unified stylesheet,
replacing the Cobalt/Liquid renderer for the article path.

**Verdict: delivered.** A real demo concept (`abelian-group`) and a real demo
source (`vimalamitra`) both render end-to-end with full chrome — trust bar, TOC,
breadcrumb, prose with client-side-KaTeX math, see-also grouped by relationship,
taxonomy info card, provenance — through `render::build_environment` +
`pages/concept.jinja` / `pages/source.jinja`, from `demo/` alone, with no Liquid
in the path.

## 2. Slice walk

Three slices, all CDC-closed against independent reproduction. Count matches the
`arc-plan.md` breakdown (3), no arc-scale silent drop.

| Slice | Outcome | Close evidence |
|-------|---------|----------------|
| 01 — minijinja shell | delivered | `slice01-minijinja-shell/cdc-verification.md` — 8/8 rows reproduced. Surfaced the auto-escape-per-`Environment` finding (v1.1). |
| 02 — markdown render + TOC | delivered | `slice02-markdown-toc/cdc-verification.md` — 9/9; F-9 returned for a hidden pedantic lint, fixed, re-verified. Bubble-ups (KaTeX on source, dead `auto-render`) resolved (v1.3). |
| 03 — wiki-article context | delivered | `slice03-wiki-article-context/cdc-verification.md` — 9/9 reproduced against the full corpus. |

## 3. Composition check (arc ledger, reproduced at arc scale)

The composition rows were **reproduced** — not inherited from slice attestations
— by running the article render pipeline against the full demo corpus in a clean
independent build (91 sources + 186 concepts; cross-references resolve). All
pass:

| ID | Criterion | Verdict | Evidence (reproduced) |
|----|-----------|---------|-----------------------|
| A-1 | Slice 01 closed | done | ptr: slice01 cdc-verification |
| A-2 | Slice 02 closed | done | ptr: slice02 cdc-verification |
| A-3 | Slice 03 closed | done | ptr: slice03 cdc-verification |
| A-4 | Real concept renders end-to-end, full chrome | done | `test_render_concept_abelian_group_full_chrome` green — trust bar, TOC (ids match anchors), breadcrumb, math spans, see-also, sidebar, provenance link `/source/group-mathematics/`. |
| A-5 | Real source renders end-to-end, full chrome | done | `test_render_source_vimalamitra_full_chrome` green. |
| A-6 | Escaping correct (only contract fields `\| safe`) | done | `test_render_concept_escaping_holds_at_page_scale` green — injected alias escaped; `page.content` intact. |
| A-7 | No Liquid/Cobalt in the article path | done | `test_render_path_references_no_liquid_templates` green. |

Composition verdict: **the slices compose into the capability.** Nothing the arc
promised is missing. Gate reviewed by: the independent CDC context that ran the
reproduction (distinct from the implementer).

## 4. Accumulated arc-plan change log

The arc-plan moved five times as the work revealed what it couldn't have known
(full detail in `arc-plan.md` Version History):

- **v1.1** — auto-escape is per-`Environment` (slice 01): reuse
  `render::build_environment`; the default is wrong for `.jinja`.
- **v1.2** — pre-existing lint debt + `demo/profile.yaml` migration named
  (slice 01 CDC).
- **v1.3** — source-page KaTeX + dead `auto-render` include fixed (slice 02 CDC).
- **v1.4** — operator remediation: pre-existing lint debt fixed; `make check`
  green.
- **v1.5** — a new `--all-targets`-only pre-existing-debt category in the
  feature-gated `demo/*` pipeline (slice 03).

## 5. Bubble-up to the project

Answered against `project-plan.md`, per Part V:

1. **Did Arc 01 deliver its capability as the project roadmap defined it?**
   Yes. The roadmap's Arc 01 — "render a fully-chromed wiki article (concept
   and source) via minijinja, replacing the Liquid renderer for the article
   path" — is delivered and composition-reproduced. No project-plan change is
   forced (itself a valid, recorded bubble-up outcome).
2. **What did Arc 01 reveal that the project plan did not anticipate?**
   - **A CI-gate definition item for the project:** the lint gate must run
     `cargo clippy --all-targets --all-features -- -D warnings` (both flags).
     Two remediation rounds missed debt because they used neither `--all-targets`
     nor the full feature set. This belongs in Arc 06 (polish/deploy/CI) — noted
     so it isn't rediscovered. The feature-gated `demo/*` debt (v1.5) is the
     concrete backlog for that pass.
   - **A carried template observation for Arc 02:** `source.jinja` has no info
     card, so source-page provenance ("Concepts in This Source") never renders.
     If sources should show it, that is an Arc 02 template decision, not an
     Arc 01 gap.
   - **No routing/CLI yet:** `render_concept`/`render_source` are library
     functions; wiring them into routes + static export is Arc 02, and into the
     server is Arc 03 — as the roadmap already sequences.
3. **Silent-drop diff at arc scale:** none. Every roadmap expectation of Arc 01
   landed; the items above are forward-looking findings, not gaps.

## 6. Arc-close decision

**Go.** Arc 01 closes. Project ledger row **P-1** (Arc 01 closed + composed) is
satisfied; `project-plan.md` is updated (P-1 done; status: Arc 02 is next to
plan deep). No remediation slice required.
