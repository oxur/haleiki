# Planning migration — 2026-11-11

Operator request: adopt the current collaboration framework, establish the usual
planning worktree, preserve `docs/design/`, `docs/dev/`, `workbench/` and
`docs/tiny-http/`, and create a low-effort chronological finding aid. Missing
historical content is deliberately not supplied. This is documentation migration,
not feature execution, fresh acceptance, a retrospective audit, or a release.

## Inputs and layout

- Source: `/Users/oubiwann/lab/oxur/haleiki`, `framework-base/tiny-http`,
  `ffe2dd7` (the full authoritative SHA is in `migration-manifest.json`).
- Clean source checkout at intake; existing five Claude agent worktrees retained.
- Framework: Operator-named installed `collaboration-framework/SKILL.md`, version
  3.0.1, with current project-management layout/planning/confirmation guidance
  and ledger discipline. No vendored framework copy or upstream update was needed.
- Planning: orphan branch `planning`, `.worktrees/planning`.
- Projects 01–03 use canonical `projectNN-<slug>`; `project-04-tiny-http`
  follows the Operator's explicit `project-<next index>-tiny-http` spelling.
- Shared `archive/` retains original path structure so historical drafts,
  overwritten ODM versions, duplicate records and cross-project design inputs
  remain identifiable. Plans link originals instead of expanding their payloads.
  The manifest provides old-to-new resolution where links were already stale.

## Chronological reconstruction

| Project | Basis | Reconstruction |
|---------|-------|----------------|
| 01 Demo site | Architecture/demo plans, 15 detailed milestone files, March 17–May 3 ancestry | Five arcs mirror original phases 1–5; fifteen slices mirror their milestones. Original phases 6–10 remain explicit unresolved/cross-project carryover. April–May repair commits are indexed without inventing missing repair plans. |
| 02 Concept cards | May 3 pilot, corpus extraction and backfill prompts/commits | One arc, three slices; guide lineage remains reference material rather than an invented re-extraction run. |
| 03 Cobalt site build | May 3–8 project plan, M1–M8 plans/prompts and implementation commits | Three approximate arcs, eight milestone slices. Earlier brand/profile/search proposals are inputs, not later delivery claims. |
| 04 tiny_http | Existing July–August framework records | Five existing detailed arcs, thirteen original slices. Added canonical slice-plan entrypoints; separated project/arc ledgers. Arc 06 remains roadmap-level. |

Git subjects were compared with document identities, dependencies, scopes and
historical checks. Inspection focused on planning narrative, versions, scope and
milestone structure; embedded example implementations and design runtime assets
were preserved, not audited for current correctness. Exact duplicate drafts were
identified rather than interpreted as separate work. No code or historical test
suite was run to upgrade an old claim. Early design inputs span projects, so
directory order records the coherent delivery sequence, not every input's date.

The named chat **Review Haleiki slice handoff** was consulted read-only. It
records independent Arc 04 verification and commit `aafe96f`, matching Git and
the retained verification file. Arc 05 remains open at its authoring-location,
URL, frontmatter and fixture discussion gate. Cleanup grants no approval for
those decisions. Source baseline `ffe2dd7` still opens that gate; no news-model
implementation has been inferred.

## Preservation and standards

The manifest records **181 payloads**: all 180 files under the four requested
roots plus retired `odm.toml`. That includes **117 tracked** and **64 formerly
ignored/untracked** files. All bytes, modes and copy-time modification times
were preserved before removal. Git does not preserve filesystem modification
times on a later checkout; original nanosecond timestamps remain in the manifest.
OS metadata is retained too, rather than silently discarded during removal.

The design ZIP's integrity check passed for all 41 members. It includes the
original `templates/` tree missing from the unpacked workbench directory.
The ZIP and unpacked assets are retained unchanged; no extraction or recreation
was necessary. Current implementation templates remain in source `templates/`.

Six tiny_http project/arc plans have exact archived originals and current copies
with separate ledgers. All **46 project/arc rows** retain exact criterion,
verification, status and evidence text. All other original tiny_http payloads,
including slice ledgers, issued prompts and verification records, are byte
identical at their migrated paths. New `slice-plan.md` wrappers link the original
`slice-doc.md` without reducing or rewriting its contract. Historical paths
inside issued prompts remain identifiable through the manifest; a future
assignment must use a preserved numbered follow-up with current paths.

Historical reconstructed ledgers remain open with only attested Git pointers.
No new closing reports, independent reviewer verdicts, success dates or CC
assignments were fabricated. Existing tiny_http acceptance remains historical:
Arcs 01–04 closed, Arc 05 active/gated, Arc 06 not yet detailed; P-5–P-12 remain
open, including project-scale P-9 search recomposition.

## Verification receipts

Before source removal, `python3 verify-migration.py --source ../..` checked all
181 originals against retained bytes, all 117 tracked input paths, and all 46
extracted project/arc ledger rows. Its first pass stopped at a missing link to
this report while the report was still being authored; no source had been removed.
The completed check also validates authored links and implementation stability.
The completed pre-removal run passed:

- 181/181 exact retained payloads against originals still on disk; all 181
  original filesystem modification times also matched their copies.
- Complete population: 117 tracked paths plus 64 ignored/untracked files.
- 46/46 project/arc ledger rows extracted unchanged.
- 285 links in authored/canonical Markdown resolved; historical links retained.
- Implementation, fixtures, shipped assets and CI unchanged from the baseline.
- Unborn `planning` branch confirmed before the first commit.
- Whitespace checks passed for all 107 authored files. Original whitespace is
  preserved separately and intentionally not normalized.

The first whitespace-check harness interpreted `git diff --no-index` exit 1
(files differ) as a whitespace failure despite empty diagnostics. The corrected
check distinguishes that normal diff status from actual whitespace diagnostics;
no content changes were needed. Final post-removal verification follows below.

No Rust/browser gates are needed for unchanged implementation, fixtures, assets
and CI. Migration checks cover the changed document/layout surface. Historical
sandbox/localhost failures and successful privileged reruns remain in their
original records; no new localhost permission was required for this migration.

Worktree setup initially needed Git metadata permission. The app-created detached
worktree was moved to the requested repository-local path, then switched to
orphan `planning`. An initial move lacked the parent directory and was retried
after creating it. A sandboxed orphan switch was denied access to the shared Git
index and succeeded with Git metadata permission. Git registers the resulting
worktree correctly; the desktop app's managed-worktree attachment API rejects
the relocated repository-local path. Use its explicit filesystem path.

## Final post-removal receipts

Self-checked on 2026-11-11, after preserving all originals in planning commit
`60f09d5` and removing the exact manifest-listed source files:

```text
PASS: 181 byte-identical payloads; original present=0, pinned Git original=117, formerly ignored/untracked=64
PASS: complete tracked population (117); existing local roots covered
PASS: 46 project/arc ledger rows extracted without changes
PASS: 285 links in authored/canonical Markdown; historical links retained as-is
PASS: implementation, fixtures, assets and CI unchanged from migration baseline
PASS: orphan planning history; no implementation ancestry
```

`git diff --check` passed for the source cleanup. The scoped source commit is
`166ef2e` (`Move planning records off the tiny_http source branch`): 117 tracked
originals removed, three source guidance/ignore files updated, no other tracked
changes. All four old content roots (`docs/design`, `docs/dev`, `docs/tiny-http`,
`workbench`) were removed after byte/hash checks against committed archive blobs.
Retired `odm.toml` is also preserved and removed from the source branch.
Source status was clean after commit. Final planning receipts are committed
separately from the preservation commit; neither branch has been pushed.

## Remaining boundaries

- Only the checked-out `framework-base/tiny-http` source branch is cleaned;
  `main`, remote refs and existing agent worktrees retain their own history.
- This request does not authorize pushing either branch. Local commits preserve
  the migration; remote availability awaits a later push decision.
- Historical Markdown links and commands are preserved evidence, not guaranteed
  current entrypoints. Use canonical plans and the manifest when resuming.
- Unknown historical validation/acceptance remains unknown. The lightweight
  project/arc grouping is a finding aid, not a retroactive claim that those
  exact units were formally planned or independently closed at the time.

## Directory-name correction — 2026-11-11

After the initial migration, the Operator renamed `project-04-tiny-http`
to canonical `project04-tiny-http` (rename commit `504536e`). Current
navigation, metadata, source-checkout guidance, 69 manifest destination
paths and the verifier now use the corrected directory. The initial-layout
description above remains the historical record of the original migration.
Payload hashes, issued prompts, archive contents and ledger rows are unchanged.
