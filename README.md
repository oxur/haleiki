# Haleiki planning

Planning branch: **`planning` (orphan)**. Worktree: **`.worktrees/planning`**.
Source checkout: `/Users/oubiwann/lab/oxur/haleiki` on `framework-base/tiny-http`.
Source baseline: `ffe2dd78994e94cd003f039f8c706c73dfec9dd4` (2026-08-21). Migration: 2026-11-11.
Framework loaded: collaboration-framework **3.0.1**, from the Operator-named installed skill.

## Chronological project index

| Project | Historical period | Disposition |
|---------|-------------------|-------------|
| [01 — Demo site](project01-demo-site/project-plan.md) | March–May 2026 | Best-effort historical map; phases 1–5 detailed, later scope retained as carryover |
| [02 — Concept cards](project02-concept-cards/project-plan.md) | May 3, 2026 | Pilot, corpus extraction and backlink backfill; historical acceptance not reconstructed |
| [03 — Cobalt site build](project03-cobalt-site-build/project-plan.md) | May 3–8, 2026 | M1–M8 mapped to three coarse arcs; superseded rendering lineage |
| [04 — tiny_http pivot](project04-tiny-http/project-plan.md) | July–August 2026; unfinished | Real framework records retained; Arcs 01–04 closed, Arc 05 active, Arc 06 roadmap only |

The first three projects are finding aids, not newly approved implementation work.
Their arc grouping is approximate; historical milestone identities and subjects
remain visible in slice plans and the [Git history map](history-commits.txt).
There are no fabricated success reports, reviewer decisions, dates or prompts.
All projects use the canonical `projectNN-<slug>` naming convention.

## Original records and path resolution

The shared [archive](archive/) retains **all** files from `docs/design/`,
`docs/dev/`, and ignored `workbench/`, including draft/overwritten versions,
duplicate prompts, ODM state, design assets and ZIP. It intentionally retains
their original structure, spelling, whitespace, links and timestamps; content
inside historical code blocks is not current execution authority. Even OS
metadata is preserved, so the source removal need not discard any file.

The [migration manifest](migration-manifest.json) maps each old repository path
to its exact retained payload and records SHA-256, size, mode, original mtime and
tracked/untracked origin. For tiny_http, unedited files live at the new project
root. Six original project/arc plans are archived because current copies separate
their embedded ledgers. Issued prompts and acceptance records are unchanged.
Old links may already have been stale before migration; resolve old paths via
the manifest rather than editing issued history. Canonical entrypoint links are checked.

ODM configuration is preserved at [archive/odm.toml](archive/odm.toml) for provenance;
ODM's numbered state folders are retired as the active planning layout.
Source branches other than `framework-base/tiny-http` are not rewritten.

## Resume point

Read the [tiny_http project plan](project04-tiny-http/project-plan.md), its [ledger](project04-tiny-http/ledger.md),
then Arc 05 and its current Slice 02 `slice-plan.md`. The Operator separately approved
the four content-model decisions and Autonomous Mode through end of Arc 05 on
2026-11-11. Slice 01 is independently closed at source `5fe3cf46`; its accepted
final assignment is preserved as
`arc05-news-blog-content-type/slice01-news-model-and-post/cc-prompt-iteration02.md`
(relative to `project04-tiny-http`). The Operator has now approved all four
archive/taxonomy/feed-deferral/news-search recommendations; see the actual
approval record in the arc plan. Current ready-for-CC assignment:
`arc05-news-blog-content-type/slice02-news-index-archives-taxonomy/cc-prompt.md`
(relative to `project04-tiny-http`). Parent resumes the explicitly selected CC;
CDC independently reviews after source ownership returns.
Arc 06 is outside execution authority. The migration
itself did not provide this separate approval.

## Migration checks

At the migration baseline, run `python3 verify-migration.py --source ../..` here.
This checker is frozen migration evidence, not a new-development gate: it
expects the original live ledgers and unchanged source. Do not weaken it after
Arc 05 development legitimately changes those inputs. It checks every retained
payload, current original bytes if still present (or pinned Git bytes after removal),
unchanged tiny_http ledger rows, canonical links, and source-code/fixture stability.
See [migration report](migration-report.md) for outcomes and limitations.
