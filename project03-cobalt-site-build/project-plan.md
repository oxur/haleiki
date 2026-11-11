---
project: project03-cobalt-site-build
status: historical-reconstructed
depends-on: ["project01-demo-site", "project02-concept-cards"]
blocks: []
related: ["project-04-tiny-http"]
---

# Cobalt templates and static site build

Historical period: 2026-05-03–2026-05-08. Reconstruction date: 2026-11-11.

Reconstructed on 2026-11-11 for historical navigation only. Git subjects attest implementation; no historical tests were rerun and no new acceptance is claimed. Missing reports, prompts and independent review have not been invented. Original documents, including stale instructions and links, are preserved in `archive/`; use the root migration manifest to resolve their former paths.

## Definition of done and boundaries

Turn the existing demo sources, concepts and media into a navigable Cobalt/Liquid static wiki. The original M1–M8 plan defines the historical DoD. All eight implementation milestones appear in Git; that alone does not reconstruct independent close or deployed-site acceptance.

This is a finding aid for past work, not an approved new implementation plan.
Historical commands/specifications remain in the preserved sources. Closure,
publication and current runtime correctness are not established by this migration.

## Roadmap and recorded status

| Arc | Capability |
|-----|------------|
| [arc01-render-foundation](arc01-render-foundation/arc-plan.md) | Bare rendering, data and visual foundation |
| [arc02-navigable-wiki](arc02-navigable-wiki/arc-plan.md) | Articles and navigation |
| [arc03-profile-and-polish](arc03-profile-and-polish/arc-plan.md) | Trust, attribution and usable demo |

Numbers follow the first coherent delivery effort in the source ancestry, not the
filesystem modification time of ignored drafts. Early architecture, brand and
profile designs also informed later projects; links do not imply new chronology.

## Source records

- [docs/design/06-final/0006-haleiki-site-build-cobalt-templates-and-static-rendering-project-plan.md](../archive/docs/design/06-final/0006-haleiki-site-build-cobalt-templates-and-static-rendering-project-plan.md)
- [docs/design/06-final/0004-haleiki-brand-guidelines.md](../archive/docs/design/06-final/0004-haleiki-brand-guidelines.md)
- [docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md](../archive/docs/design/06-final/0005-deployment-profiles-epistemic-architecture.md)
- [docs/dev/0001-haleiki-search-architecture-ab-plan.md](../archive/docs/dev/0001-haleiki-search-architecture-ab-plan.md)
- [workbench/plans/template-conversion.md](../archive/workbench/plans/template-conversion.md)
- [workbench/TODO.md](../archive/workbench/TODO.md)

The root [manifest](../migration-manifest.json) maps every former path
to its byte-preserved copy, including untracked workbench drafts and binary assets.
[History](../history-commits.txt) pins commit subjects to the source baseline.

## Unresolved scope and carryover

The early A+B search/template-conversion proposals include search/news/RSS ambitions; the later M1–M8 plan explicitly leaves search, news and production deployment as follow-up. Project 04 supersedes Cobalt rendering; its recorded Pagefind reversal remains authoritative. Do not reinterpret the older TypeScript/npm examples as current browser policy. Public deployment, visual acceptance, and missing historical verification remain unestablished.

See [project ledger](ledger.md). Rows are deliberately
open where historical acceptance is missing. No modern retrospective has been
backdated or substituted for an original acceptance record.

## Workflow and authority

Reconstruction only under the Operator's 2026-11-11 migration request; no code
execution assignment or new contributor arrangement. Future implementation uses
the default CDC + CC workflow unless the Operator selects otherwise. Existing
independent/Operator gates remain binding. Plans, ledgers and supporting records
live on orphan branch `planning` in `.worktrees/planning`.

## Version history

- 2026-11-11: v1.0 finding aid reconstructed from existing files and Git messages;
  missing content intentionally not supplied, no historical acceptance upgraded.
